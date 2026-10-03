#include "ExchangeCoordinator.h"
#include "PayloadSnapshot.h"
#include <cassert>
#include <cerrno>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#include <poll.h>
#include <signal.h>
#include <sys/wait.h>
namespace fs=std::filesystem;
using namespace fstr::installer;
Identity id(int fd,const char* name) {
    struct stat s{}; assert(!fstatat(fd,name,&s,AT_SYMLINK_NOFOLLOW));
    return {s.st_dev,s.st_ino};
}
struct Fixture {
    fs::path root;
    int a,b,tx;
    PreparedExchange expected;
    Fixture(fs::path parent,const char* name) : root(parent/name) {
        for (const auto& p : {root,root/"active",root/"backup",root/"tx"}) {
            fs::create_directory(p); assert(!chmod(p.c_str(),0700));
        }
        fs::create_directory(root/"active"/"target.plugin");
        fs::create_directory(root/"backup"/"prior.plugin");
        std::ofstream(root/"active"/"target.plugin"/"payload")<<"old";
        std::ofstream(root/"backup"/"prior.plugin"/"payload")<<"new";
        a=open((root/"active").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
        b=open((root/"backup").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
        tx=open((root/"tx").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
        assert(a>=0 && b>=0 && tx>=0);
        expected={id(a,"target.plugin"),id(b,"prior.plugin")};
    }
    ~Fixture() { close(a);close(b);close(tx); }
    ExchangeLocations paths() { return {a,"target.plugin",b,"prior.plugin",tx}; }
    void unchanged() { assert(locateExchange(expected,id(a,"target.plugin"),id(b,"prior.plugin"))==RecoveryPosition::Unexchanged); }
};
struct Verification {
    Fixture* fixture;
    int calls=0, rejectAt=0;
    int pauseAt=0, notify=-1;
    PayloadSnapshot previous{},candidate{};
    explicit Verification(Fixture* f,int c=0,int r=0,int p=0,int n=-1)
        : fixture(f),calls(c),rejectAt(r),pauseAt(p),notify(n) {
        assert(!snapshotPayload(f->a,"target.plugin",previous));
        assert(!snapshotPayload(f->b,"prior.plugin",candidate));
    }
    static int run(void* opaque,RecoveryPosition position) noexcept {
        auto& v=*static_cast<Verification*>(opaque);
        ++v.calls;
        if (v.calls==v.pauseAt) {
            const char ready='R';
            if (write(v.notify,&ready,1)!=1) _exit(12);
            for (;;) pause();
        }
        if (v.calls==v.rejectAt) return EPERM;
        PayloadSnapshot active{},backup{};
        if(int e=snapshotPayload(v.fixture->a,"target.plugin",active)) return e;
        if(int e=snapshotPayload(v.fixture->b,"prior.plugin",backup)) return e;
        const bool swapped=position==RecoveryPosition::Exchanged;
        return active.sha256==(swapped ? v.candidate.sha256 : v.previous.sha256) &&
               backup.sha256==(swapped ? v.previous.sha256 : v.candidate.sha256) ? 0 : EIO;
    }
};
int main() {
    char temp[]="/private/tmp/egfx-installer-coordinator-XXXXXX";
    const auto created=mkdtemp(temp); assert(created);
    const fs::path root(created);
    {
        Fixture f(root,"normal"); Verification v{&f};
        auto r=replaceVerified(f.paths(),f.expected,Verification::run,&v);
        assert(r.state==TransactionState::Installed && r.error==0 && v.calls==3);
        r=recoverVerified(f.paths(),f.expected,Verification::run,&v,RecoveryAction::Inspect);
        assert(r.state==TransactionState::Installed);
        r=recoverVerified(f.paths(),f.expected,Verification::run,&v,RecoveryAction::Restore);
        assert(r.state==TransactionState::Restored && r.error==0); f.unchanged();
        r=recoverVerified(f.paths(),f.expected,Verification::run,&v,RecoveryAction::Restore);
        assert(r.state==TransactionState::Unchanged); f.unchanged();
        r=replaceVerified(f.paths(),f.expected,Verification::run,&v);
        assert(r.state==TransactionState::Refused && r.error==EEXIST); f.unchanged();
    }
    for (int rejected : {1,2,3}) {
        Fixture f(root, rejected==1 ? "first" : rejected==2 ? "second" : "third");
        Verification v{&f,0,rejected};
        const auto r=replaceVerified(f.paths(),f.expected,Verification::run,&v);
        assert(r.error==EPERM);
        if (rejected==1) {
            assert(r.state==TransactionState::Refused);
            assert(!fs::exists(f.root/"tx"/"prepared-v1")); f.unchanged();
        } else if (rejected==2) {
            assert(r.state==TransactionState::Prepared); f.unchanged();
            PreparedExchange record{}; assert(!readPreparedJournal(f.tx,record));
        } else {
            assert(r.state==TransactionState::NeedsRecovery);
            assert(locateExchange(f.expected,id(f.a,"target.plugin"),id(f.b,"prior.plugin"))==RecoveryPosition::Exchanged);
        }
        v.rejectAt=0;
        if (rejected>1) {
            const auto recovered=recoverVerified(f.paths(),f.expected,Verification::run,&v,RecoveryAction::Restore);
            assert(recovered.state==(rejected==2 ? TransactionState::Unchanged : TransactionState::Restored));
            f.unchanged();
        }
    }
    {
        Fixture f(root,"changed"); Verification v{&f};
        assert(replaceVerified(f.paths(),f.expected,Verification::run,&v).state==TransactionState::Installed);
        std::ofstream(f.root/"active"/"target.plugin"/"payload",std::ios::trunc)<<"changed";
        auto r=recoverVerified(f.paths(),f.expected,Verification::run,&v,RecoveryAction::Restore);
        assert(r.state==TransactionState::NeedsRecovery && r.error==EIO);
        assert(locateExchange(f.expected,id(f.a,"target.plugin"),id(f.b,"prior.plugin"))==RecoveryPosition::Exchanged);
        std::string content; std::ifstream(f.root/"active"/"target.plugin"/"payload")>>content;
        assert(content=="changed");
        auto wrong=f.expected; ++wrong.previous.inode;
        r=recoverVerified(f.paths(),wrong,Verification::run,&v,RecoveryAction::Restore);
        assert(r.state==TransactionState::NeedsRecovery && r.error==EAGAIN);
    }
    {
        Fixture f(root,"locked"); Verification v{&f};
        const int lock=openat(f.a,".fstr-install-lock-v1",O_CREAT|O_EXCL|O_RDWR|O_NOFOLLOW,0600);
        assert(lock>=0 && !flock(lock,LOCK_EX|LOCK_NB));
        auto r=replaceVerified(f.paths(),f.expected,Verification::run,&v);
        assert(r.state==TransactionState::Refused && (r.error==EWOULDBLOCK || r.error==EAGAIN));
        assert(!v.calls && !fs::exists(f.root/"tx"/"prepared-v1")); f.unchanged(); close(lock);
        assert(replaceVerified(f.paths(),f.expected,Verification::run,&v).state==TransactionState::Installed);
    }
    {
        Fixture f(root,"linked-lock"); Verification v{&f};
        assert(!symlink("target.plugin",(f.root/"active"/".fstr-install-lock-v1").c_str()));
        const auto r=replaceVerified(f.paths(),f.expected,Verification::run,&v);
        assert(r.state==TransactionState::Refused && r.error==ELOOP);
        assert(!v.calls); f.unchanged();
    }
    for (int stoppedAt : {2,3}) {
        Fixture f(root,stoppedAt==2 ? "killed-prepared" : "killed-installed");
        Verification recovery{&f}; // Preserve the authenticated pre-install snapshots.
        int ready[2]; assert(!pipe(ready));
        const auto child=fork(); assert(child>=0);
        if (!child) {
            close(ready[0]);
            Verification v=recovery;
            v.pauseAt=stoppedAt; v.notify=ready[1];
            replaceVerified(f.paths(),f.expected,Verification::run,&v);
            _exit(13); // Must stop at the selected coordinator checkpoint.
        }
        close(ready[1]);
        struct pollfd notification{ready[0],POLLIN,0};
        const int available=poll(&notification,1,5000);
        char marker=0;
        const bool received=available>0 && read(ready[0],&marker,1)==1 && marker=='R';
        close(ready[0]); assert(!kill(child,SIGKILL));
        int status=0; assert(waitpid(child,&status,0)==child);
        assert(received && WIFSIGNALED(status) && WTERMSIG(status)==SIGKILL);
        const auto inspected=recoverVerified(f.paths(),f.expected,Verification::run,&recovery,RecoveryAction::Inspect);
        assert(inspected.state==(stoppedAt==2 ? TransactionState::Unchanged : TransactionState::Installed));
        const auto recovered=recoverVerified(f.paths(),f.expected,Verification::run,&recovery,RecoveryAction::Restore);
        assert(recovered.state==(stoppedAt==2 ? TransactionState::Unchanged : TransactionState::Restored));
        f.unchanged();
    }
    fs::remove_all(root);
    std::cout<<"PASS: coordinator install/inspect/restore, retained journal, refusal gates and changed-byte protection\n";
}
