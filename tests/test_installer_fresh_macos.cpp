#include "FreshPublication.h"
#include <cassert>
#include <cerrno>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <fcntl.h>
#include <poll.h>
#include <signal.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>
namespace fs=std::filesystem;
using namespace fstr::installer;
struct Fixture {
    fs::path root;
    int active,stage,tx;
    PayloadSnapshot expected{};
    explicit Fixture(fs::path p):root(p) {
        for(auto n:{"","active","stage","tx"}){fs::create_directory(root/n);assert(!chmod((root/n).c_str(),0700));}
        fs::create_directory(root/"stage"/"new.plugin");std::ofstream(root/"stage"/"new.plugin"/"payload")<<"new";
        active=open((root/"active").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
        stage=open((root/"stage").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
        tx=open((root/"tx").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
        assert(active>=0 && stage>=0 && tx>=0);assert(!snapshotPayload(stage,"new.plugin",expected));
    }
    ~Fixture(){close(active);close(stage);close(tx);}
    FreshLocations paths(){return {active,"target.plugin",stage,"new.plugin",tx};}
};
struct Verify {
    Fixture* f;
    int calls=0,rejectAt=0,pauseAt=0,notify=-1;
    bool interfere=false;
    static int run(void* opaque,bool) noexcept {
        auto& v=*static_cast<Verify*>(opaque);++v.calls;
        if(v.calls==v.pauseAt){const char c='R';if(write(v.notify,&c,1)!=1)_exit(21);for(;;)pause();}
        if(v.calls==v.rejectAt)return EPERM;
        if(v.interfere && v.calls==2){std::ofstream(v.f->root/"active"/"target.plugin")<<"foreign";}
        return 0;
    }
};
int main(){
    char tmp[]="/private/tmp/fstr-fresh-XXXXXX";assert(mkdtemp(tmp));const fs::path root(tmp);
    {
        Fixture f(root/"normal");Verify v{&f};
        const auto r=publishFreshVerified(f.paths(),f.expected,Verify::run,&v);
        assert(r.state==TransactionState::Installed && !r.error && v.calls==3);
        assert(!fs::exists(f.root/"stage"/"new.plugin"));
        PayloadSnapshot actual{};assert(!snapshotPayload(f.active,"target.plugin",actual));assert(matchesSnapshot(actual,f.expected));
        const auto recovered=inspectFreshVerified(f.paths(),Verify::run,&v);assert(recovered.state==TransactionState::Installed && !recovered.error);
        // Repeat must preserve the existing installed bundle, not replace it.
        assert(publishFreshVerified(f.paths(),f.expected,Verify::run,&v).error==EEXIST);
    }
    for(int reject=1;reject<=3;++reject){
        Fixture f(root/("reject-"+std::to_string(reject)));Verify v{&f,0,reject};
        const auto r=publishFreshVerified(f.paths(),f.expected,Verify::run,&v);assert(r.error==EPERM);
        assert(r.state==(reject==1?TransactionState::Refused:reject==2?TransactionState::Prepared:TransactionState::NeedsRecovery));
        assert(fs::exists(f.root/"tx"/"snapshots-v1")== (reject!=1));
        v.rejectAt=0;const auto recovered=inspectFreshVerified(f.paths(),Verify::run,&v);
        assert(recovered.state==(reject==1?TransactionState::NeedsRecovery:reject==2?TransactionState::Prepared:TransactionState::Installed));
    }
    for(int pauseAt:{2,3}){
        Fixture f(root/("interrupt-"+std::to_string(pauseAt)));int pipefd[2];assert(!pipe(pipefd));
        const pid_t child=fork();assert(child>=0);
        if(!child){close(pipefd[0]);Verify v{&f,0,0,pauseAt,pipefd[1]};publishFreshVerified(f.paths(),f.expected,Verify::run,&v);_exit(22);}
        close(pipefd[1]);pollfd wait{pipefd[0],POLLIN,0};assert(poll(&wait,1,5000)==1);char ready=0;assert(read(pipefd[0],&ready,1)==1 && ready=='R');close(pipefd[0]);
        assert(!kill(child,SIGKILL));int status=0;assert(waitpid(child,&status,0)==child && WIFSIGNALED(status));
        Verify v{&f};const auto r=inspectFreshVerified(f.paths(),Verify::run,&v);
        assert(!r.error && r.state==(pauseAt==2?TransactionState::Prepared:TransactionState::Installed));
        assert(fs::exists(f.root/"tx"/"snapshots-v1"));
    }
    {
        Fixture f(root/"collision");Verify v{&f,0,0,0,-1,true};
        const auto r=publishFreshVerified(f.paths(),f.expected,Verify::run,&v);
        assert(r.state==TransactionState::Prepared && r.error==EEXIST);
        std::ifstream in(f.root/"active"/"target.plugin");std::string s;in>>s;assert(s=="foreign");
        assert(fs::exists(f.root/"stage"/"new.plugin"/"payload"));
        assert(inspectFreshVerified(f.paths(),Verify::run,&v).state==TransactionState::NeedsRecovery);
    }
    {
        Fixture f(root/"modified");Verify v{&f};assert(publishFreshVerified(f.paths(),f.expected,Verify::run,&v).state==TransactionState::Installed);
        std::ofstream(f.root/"active"/"target.plugin"/"payload",std::ios::app)<<"external";
        assert(inspectFreshVerified(f.paths(),Verify::run,&v).state==TransactionState::NeedsRecovery);
        assert(fs::exists(f.root/"active"/"target.plugin"/"payload"));
    }
    {
        Fixture f(root/"link");Verify v{&f};fs::create_symlink(f.root/"stage"/"new.plugin",f.root/"active"/"target.plugin");
        assert(publishFreshVerified(f.paths(),f.expected,Verify::run,&v).error==EEXIST);
        assert(!fs::exists(f.root/"tx"/"snapshots-v1"));
    }
    {
        Fixture f(root/"lock");Verify v{&f};int fd=openat(f.active,".fstr-install-lock-v1",O_CREAT|O_EXCL|O_RDWR,0600);assert(fd>=0 && !flock(fd,LOCK_EX|LOCK_NB));
        const auto r=publishFreshVerified(f.paths(),f.expected,Verify::run,&v);assert(r.state==TransactionState::Refused && (r.error==EWOULDBLOCK || r.error==EAGAIN));close(fd);
        assert(v.calls==0 && !fs::exists(f.root/"tx"/"snapshots-v1"));
    }
    fs::remove_all(root);std::cout<<"Exclusive publication, interruption and refusal PASS\n";
}
