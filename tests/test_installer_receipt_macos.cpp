#include "SnapshotReceipt.h"
#include "ExchangeCoordinator.h"
#include <cassert>
#include <cerrno>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <fcntl.h>
#include <membership.h>
#include <sys/acl.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>
namespace fs=std::filesystem;
using namespace fstr::installer;
struct Directory {
    fs::path path;
    int fd;
    explicit Directory(fs::path p):path(p) {
        fs::create_directory(path);assert(!chmod(path.c_str(),0700));
        fd=open(path.c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);assert(fd>=0);
    }
    ~Directory(){close(fd);}
};
SnapshotReceipt payloads(Directory& d) {
    for(const char* name:{"old.plugin","new.plugin"}) {
        fs::create_directory(d.path/name);
        std::ofstream(d.path/name/"payload")<<name;
    }
    SnapshotReceipt r{};r.previousPresent=true;
    assert(!snapshotPayload(d.fd,"old.plugin",r.previous));
    assert(!snapshotPayload(d.fd,"new.plugin",r.candidate));return r;
}
struct Verify {
    Directory* d;
    SnapshotReceipt expected;
    static int run(void* opaque,RecoveryPosition pos) noexcept {
        auto& v=*static_cast<Verify*>(opaque);PayloadSnapshot a{},b{};
        if(int e=snapshotPayload(v.d->fd,"old.plugin",a))return e;
        if(int e=snapshotPayload(v.d->fd,"new.plugin",b))return e;
        const bool swapped=pos==RecoveryPosition::Exchanged;
        return matchesSnapshot(a,swapped?v.expected.candidate:v.expected.previous) &&
            matchesSnapshot(b,swapped?v.expected.previous:v.expected.candidate)?0:EAGAIN;
    }
};
int main() {
    char tmp[]="/private/tmp/fstr-receipt-XXXXXX";assert(mkdtemp(tmp));const fs::path root(tmp);
    {
        Directory d(root/"replace");auto r=payloads(d);
        auto result=prepareSnapshotReceipt(d.fd,r);assert(result.state==JournalState::Durable && !result.error);
        result=prepareSnapshotReceipt(d.fd,r);assert(result.state==JournalState::NotCreated && result.error==EEXIST);
        SnapshotReceipt read{};assert(!readSnapshotReceipt(d.fd,read));
        assert(read.previousPresent && matchesSnapshot(read.previous,r.previous) && matchesSnapshot(read.candidate,r.candidate));
        Verify v{&d,read};ExchangeLocations p{d.fd,"old.plugin",d.fd,"new.plugin",d.fd};
        PreparedExchange expected{read.previous.root,read.candidate.root};
        assert(replaceVerified(p,expected,Verify::run,&v).state==TransactionState::Installed);
        // A new process receives authentic expectations from protected durable
        // state, rather than trusting the now-exchanged files as the baseline.
        const pid_t child=fork();assert(child>=0);
        if(!child) {
            SnapshotReceipt saved{};if(readSnapshotReceipt(d.fd,saved))_exit(10);
            Verify recovery{&d,saved};
            auto outcome=recoverVerified(p,{saved.previous.root,saved.candidate.root},Verify::run,&recovery,RecoveryAction::Restore);
            _exit(outcome.state==TransactionState::Restored && !outcome.error?0:11);
        }
        int status=0;assert(waitpid(child,&status,0)==child && WIFEXITED(status) && !WEXITSTATUS(status));
        assert(!Verify::run(&v,RecoveryPosition::Unexchanged));
    }
    {
        Directory d(root/"tamper");auto r=payloads(d);assert(prepareSnapshotReceipt(d.fd,r).state==JournalState::Durable);
        Verify v{&d,r};ExchangeLocations p{d.fd,"old.plugin",d.fd,"new.plugin",d.fd};
        PreparedExchange expected{r.previous.root,r.candidate.root};
        assert(replaceVerified(p,expected,Verify::run,&v).state==TransactionState::Installed);
        std::ofstream(d.path/"old.plugin"/"payload",std::ios::app)<<"foreign update";
        SnapshotReceipt saved{};assert(!readSnapshotReceipt(d.fd,saved));Verify recovery{&d,saved};
        const auto outcome=recoverVerified(p,expected,Verify::run,&recovery,RecoveryAction::Restore);
        assert(outcome.state==TransactionState::NeedsRecovery && outcome.error==EAGAIN);
        PayloadSnapshot preserved{};assert(!snapshotPayload(d.fd,"old.plugin",preserved));
        assert(preserved.root.inode==r.candidate.root.inode);
        assert(fs::exists(d.path/"snapshots-v1") && fs::exists(d.path/"new.plugin"/"payload"));
    }
    {
        Directory d(root/"fresh");auto r=payloads(d);r.previousPresent=false;r.previous={};
        assert(prepareSnapshotReceipt(d.fd,r).state==JournalState::Durable);
        SnapshotReceipt read{};assert(!readSnapshotReceipt(d.fd,read) && !read.previousPresent);
        assert(matchesSnapshot(r.candidate,read.candidate));
    }
    for(int bad=0;bad<7;++bad) {
        Directory d(root/("invalid-"+std::to_string(bad)));auto r=payloads(d);
        switch(bad){
            case 0:r.candidate.root.inode=0;break;
            case 1:r.candidate.entries=4097;break;
            case 2:r.candidate.fileBytes=32ULL*1024*1024+1;break;
            case 3:r.previous.root=r.candidate.root;break;
            case 4:r.previous.root.device=r.candidate.root.device+1;break;
            case 5:r.previousPresent=false;break;
            case 6:r.candidate.root.device=-1;break;
        }
        assert(prepareSnapshotReceipt(d.fd,r).state==JournalState::NotCreated);
        assert(!fs::exists(d.path/"snapshots-v1"));
    }
    {
        Directory d(root/"shared-parent");auto r=payloads(d);assert(!chmod(d.path.c_str(),0755));
        assert(prepareSnapshotReceipt(d.fd,r).error==EACCES);assert(!fs::exists(d.path/"snapshots-v1"));
    }
    {
        Directory d(root/"symlink");auto r=payloads(d);std::ofstream(root/"foreign")<<"preserve";
        fs::create_symlink(root/"foreign",d.path/"snapshots-v1");
        assert(prepareSnapshotReceipt(d.fd,r).error==EEXIST);SnapshotReceipt read{};
        assert(readSnapshotReceipt(d.fd,read)==ELOOP);
        std::ifstream in(root/"foreign");std::string text;in>>text;assert(text=="preserve");
    }
    for(int bad=0;bad<6;++bad) {
        Directory d(root/("bad-file-"+std::to_string(bad)));auto r=payloads(d);
        assert(prepareSnapshotReceipt(d.fd,r).state==JournalState::Durable);
        const auto path=d.path/"snapshots-v1";
        switch(bad){
            case 0:assert(!chmod(path.c_str(),0644));break;
            case 1:assert(!link(path.c_str(),(root/"linked").c_str()));break;
            case 2:std::ofstream(path,std::ios::app)<<"extra";break;
            case 3:std::ofstream(path)<<"FSTR-SNAPSHOTS-2\n";break;
            case 4:std::ofstream(path)<<std::string(513,'X');break;
            case 5:std::ofstream(path)<<"";break;
        }
        SnapshotReceipt out=r;assert(readSnapshotReceipt(d.fd,out));
        assert(matchesSnapshot(out.candidate,r.candidate)); // Failed parse never writes output.
    }
    {
        Directory d(root/"acl-file");auto r=payloads(d);assert(prepareSnapshotReceipt(d.fd,r).state==JournalState::Durable);
        acl_t acl=acl_init(1);assert(acl);acl_entry_t entry{};assert(!acl_create_entry(&acl,&entry));
        assert(!acl_set_tag_type(entry,ACL_EXTENDED_ALLOW));uuid_t user{};assert(!mbr_uid_to_uuid(getuid(),user));
        assert(!acl_set_qualifier(entry,user));acl_permset_t permissions{};assert(!acl_get_permset(entry,&permissions));
        assert(!acl_add_perm(permissions,ACL_READ_DATA));assert(!acl_set_permset(entry,permissions));
        assert(!acl_set_file((d.path/"snapshots-v1").c_str(),ACL_TYPE_EXTENDED,acl));acl_free(acl);
        SnapshotReceipt read{};assert(readSnapshotReceipt(d.fd,read)==ENOTSUP);
    }
    fs::remove_all(root);std::cout<<"Protected snapshot receipt and restart recovery PASS\n";
}
