#include "FreshPublication.h"
#include <cerrno>
#include <cstring>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/stdio.h>
#include <unistd.h>

namespace fstr::installer {
namespace {
bool component(const char* n){return n && *n && std::strlen(n)<=255 && !std::strchr(n,'/') && std::strcmp(n,".") && std::strcmp(n,"..");}
int parentError(int fd) {
    struct stat s{};if(fstat(fd,&s))return errno;
    return S_ISDIR(s.st_mode) && s.st_uid==geteuid() && !(s.st_mode&0022)?0:EACCES;
}
int pathsError(FreshLocations p) {
    if(!component(p.activeName) || !component(p.stagingName))return EINVAL;
    if(int e=parentError(p.activeParent))return e;
    if(int e=parentError(p.stagingParent))return e;
    return parentError(p.transactionDirectory);
}
class Lock {
    int fd=-1;
public:
    ~Lock(){if(fd>=0)close(fd);}
    int acquire(int parent) {
        bool created=true;
        fd=openat(parent,".fstr-install-lock-v1",O_RDWR|O_CREAT|O_EXCL|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK,0600);
        if(fd<0 && errno==EEXIST){created=false;fd=openat(parent,".fstr-install-lock-v1",O_RDWR|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK);}
        if(fd<0)return errno;
        if(created && fchmod(fd,0600))return errno;
        struct stat a{},b{};
        if(fstat(fd,&a))return errno;
        if(!S_ISREG(a.st_mode) || a.st_uid!=geteuid() || (a.st_mode&0777)!=0600 || a.st_nlink!=1 || a.st_size!=0)return EACCES;
        if(flock(fd,LOCK_EX|LOCK_NB))return errno;
        if(fstatat(parent,".fstr-install-lock-v1",&b,AT_SYMLINK_NOFOLLOW))return errno;
        return a.st_dev==b.st_dev && a.st_ino==b.st_ino?0:EAGAIN;
    }
};
int absent(int fd,const char* n) {
    struct stat s{};if(!fstatat(fd,n,&s,AT_SYMLINK_NOFOLLOW))return EEXIST;
    return errno==ENOENT?0:errno;
}
int check(FreshLocations p,const PayloadSnapshot& expected,bool published,
          FreshRevalidate verifier,void* context) {
    if(int e=absent(published?p.stagingParent:p.activeParent,published?p.stagingName:p.activeName))return e;
    PayloadSnapshot actual{};
    if(int e=snapshotPayload(published?p.activeParent:p.stagingParent,published?p.activeName:p.stagingName,actual))return e;
    if(!matchesSnapshot(expected,actual))return EAGAIN;
    if(int e=verifier(context,published))return e;
    if(int e=absent(published?p.stagingParent:p.activeParent,published?p.stagingName:p.activeName))return e;
    if(int e=snapshotPayload(published?p.activeParent:p.stagingParent,published?p.activeName:p.stagingName,actual))return e;
    return matchesSnapshot(expected,actual)?0:EAGAIN;
}
int sameDevice(FreshLocations p,const PayloadSnapshot& expected) {
    struct stat a{},b{};
    if(fstat(p.activeParent,&a) || fstat(p.stagingParent,&b))return errno;
    return a.st_dev==b.st_dev && a.st_dev==expected.root.device?0:EXDEV;
}
}
TransactionResult publishFreshVerified(FreshLocations p,const PayloadSnapshot& expected,
                                       FreshRevalidate verifier,void* context) {
    if(!verifier)return {TransactionState::Refused,EINVAL};
    if(int e=pathsError(p))return {TransactionState::Refused,e};
    Lock lock;if(int e=lock.acquire(p.activeParent))return {TransactionState::Refused,e};
    if(int e=sameDevice(p,expected))return {TransactionState::Refused,e};
    if(int e=check(p,expected,false,verifier,context))return {TransactionState::Refused,e};
    SnapshotReceipt receipt{};receipt.candidate=expected;
    const auto prepared=prepareSnapshotReceipt(p.transactionDirectory,receipt);
    if(prepared.state!=JournalState::Durable)return {
        prepared.state==JournalState::NotCreated?TransactionState::Refused:TransactionState::NeedsRecovery,prepared.error};
    if(int e=check(p,expected,false,verifier,context))return {TransactionState::Prepared,e};
    // Exclusive rename is a single atomic publication. No overwrite, mkdir of
    // the live bundle, copy fallback, unlink, or externally selected destination.
    if(renameatx_np(p.stagingParent,p.stagingName,p.activeParent,p.activeName,RENAME_EXCL|RENAME_NOFOLLOW_ANY))
        return {TransactionState::Prepared,errno};
    if(fsync(p.activeParent) || fsync(p.stagingParent))return {TransactionState::NeedsRecovery,errno};
    if(int e=check(p,expected,true,verifier,context))return {TransactionState::NeedsRecovery,e};
    return {TransactionState::Installed,0};
}
TransactionResult inspectFreshVerified(FreshLocations p,FreshRevalidate verifier,void* context) {
    if(!verifier)return {TransactionState::Refused,EINVAL};
    if(int e=pathsError(p))return {TransactionState::Refused,e};
    Lock lock;if(int e=lock.acquire(p.activeParent))return {TransactionState::Refused,e};
    SnapshotReceipt saved{};
    if(int e=readSnapshotReceipt(p.transactionDirectory,saved))return {TransactionState::NeedsRecovery,e};
    if(saved.previousPresent)return {TransactionState::NeedsRecovery,EINVAL};
    if(int e=sameDevice(p,saved.candidate))return {TransactionState::NeedsRecovery,e};
    const bool published=absent(p.activeParent,p.activeName)==EEXIST;
    if(int e=check(p,saved.candidate,published,verifier,context))return {TransactionState::NeedsRecovery,e};
    return {published?TransactionState::Installed:TransactionState::Prepared,0};
}
}
