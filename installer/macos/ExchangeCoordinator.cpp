#include "ExchangeCoordinator.h"
#include <cerrno>
#include <cstring>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

namespace fstr::installer {
namespace {
constexpr const char* lockName = ".fstr-install-lock-v1";
bool same(Identity a, Identity b) { return a.device == b.device && a.inode == b.inode; }
bool same(PreparedExchange a, PreparedExchange b) {
    return same(a.previous,b.previous) && same(a.candidate,b.candidate);
}
bool component(const char* n) {
    return n && *n && std::strlen(n)<=255 && !std::strchr(n,'/') &&
           std::strcmp(n,".") && std::strcmp(n,"..");
}
int parentError(int fd) {
    struct stat s{};
    if (fstat(fd,&s)) return errno;
    return S_ISDIR(s.st_mode) && s.st_uid==geteuid() && !(s.st_mode&0022) ? 0 : EACCES;
}
int locationError(ExchangeLocations p) {
    if (!component(p.activeName) || !component(p.backupName)) return EINVAL;
    if (int e=parentError(p.activeParent)) return e;
    if (int e=parentError(p.backupParent)) return e;
    return parentError(p.transactionDirectory);
}
int identity(int parent,const char* name,Identity& result) {
    struct stat s{};
    if (fstatat(parent,name,&s,AT_SYMLINK_NOFOLLOW)) return errno;
    if (S_ISLNK(s.st_mode)) return ELOOP;
    if (!S_ISDIR(s.st_mode)) return ENOTDIR;
    result={s.st_dev,s.st_ino}; return 0;
}
class Lock {
    int fd=-1;
public:
    Lock()=default;
    Lock(const Lock&)=delete;
    Lock& operator=(const Lock&)=delete;
    ~Lock() { if (fd>=0) close(fd); } // Kernel releases the advisory lock.
    int acquire(int parent) {
        bool created=true;
        fd=openat(parent,lockName,O_CREAT|O_EXCL|O_RDWR|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK,0600);
        if (fd<0 && errno==EEXIST) {
            created=false;
            fd=openat(parent,lockName,O_RDWR|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK);
        }
        if (fd<0) return errno;
        if (created && fchmod(fd,0600)) return errno;
        struct stat s{}, current{};
        if (fstat(fd,&s)) return errno;
        if (!S_ISREG(s.st_mode) || s.st_uid!=geteuid() || (s.st_mode&0777)!=0600 ||
            s.st_nlink!=1 || s.st_size!=0) return EACCES;
        if (flock(fd,LOCK_EX|LOCK_NB)) return errno;
        if (fstatat(parent,lockName,&current,AT_SYMLINK_NOFOLLOW)) return errno;
        if (current.st_dev!=s.st_dev || current.st_ino!=s.st_ino) return EAGAIN;
        return 0;
    }
};
int position(ExchangeLocations p,PreparedExchange expected,RecoveryPosition& out) {
    Identity a{},b{};
    if (int e=identity(p.activeParent,p.activeName,a)) return e;
    if (int e=identity(p.backupParent,p.backupName,b)) return e;
    out=locateExchange(expected,a,b);
    return out==RecoveryPosition::Unknown ? EAGAIN : 0;
}
int check(ExchangeLocations p,PreparedExchange expected,RecoveryPosition wanted,
          Revalidate verifier,void* context) {
    RecoveryPosition actual=RecoveryPosition::Unknown;
    if (int e=position(p,expected,actual)) return e;
    if (actual!=wanted) return EAGAIN;
    if (int e=verifier(context,wanted)) return e;
    // The verifier may be lengthy; bind the layout again after collecting bytes.
    if (int e=position(p,expected,actual)) return e;
    return actual==wanted ? 0 : EAGAIN;
}
}
TransactionResult replaceVerified(ExchangeLocations p,PreparedExchange expected,
                                  Revalidate verifier,void* context) {
    if (!verifier) return {TransactionState::Refused,EINVAL};
    if (int e=locationError(p)) return {TransactionState::Refused,e};
    Lock lock;
    if (int e=lock.acquire(p.activeParent)) return {TransactionState::Refused,e};
    if (int e=check(p,expected,RecoveryPosition::Unexchanged,verifier,context))
        return {TransactionState::Refused,e};
    const auto journal=prepareJournal(p.transactionDirectory,expected);
    if (journal.state!=JournalState::Durable)
        return {journal.state==JournalState::NotCreated ? TransactionState::Refused : TransactionState::NeedsRecovery,journal.error};
    if (int e=check(p,expected,RecoveryPosition::Unexchanged,verifier,context))
        return {TransactionState::Prepared,e};
    const auto swapped=exchangeDirectories(p.activeParent,p.activeName,expected.previous,
                                           p.backupParent,p.backupName,expected.candidate);
    if (swapped.state==ExchangeState::NotExchanged) return {TransactionState::Prepared,swapped.error};
    if (swapped.state==ExchangeState::ExchangedNeedsRecovery) return {TransactionState::NeedsRecovery,swapped.error};
    if (int e=check(p,expected,RecoveryPosition::Exchanged,verifier,context))
        return {TransactionState::NeedsRecovery,e};
    return {TransactionState::Installed,0};
}
TransactionResult recoverVerified(ExchangeLocations p,PreparedExchange expected,
                                  Revalidate verifier,void* context,RecoveryAction action) {
    if (!verifier || (action!=RecoveryAction::Inspect && action!=RecoveryAction::Restore))
        return {TransactionState::Refused,EINVAL};
    if (int e=locationError(p)) return {TransactionState::Refused,e};
    Lock lock;
    if (int e=lock.acquire(p.activeParent)) return {TransactionState::Refused,e};
    PreparedExchange recorded{};
    if (int e=readPreparedJournal(p.transactionDirectory,recorded))
        return {TransactionState::NeedsRecovery,e};
    if (!same(recorded,expected)) return {TransactionState::NeedsRecovery,EAGAIN};
    RecoveryPosition current=RecoveryPosition::Unknown;
    if (int e=position(p,expected,current)) return {TransactionState::NeedsRecovery,e};
    if (int e=check(p,expected,current,verifier,context)) return {TransactionState::NeedsRecovery,e};
    if (current==RecoveryPosition::Unexchanged) return {TransactionState::Unchanged,0};
    if (action==RecoveryAction::Inspect) return {TransactionState::Installed,0};
    const auto swapped=exchangeDirectories(p.activeParent,p.activeName,expected.candidate,
                                           p.backupParent,p.backupName,expected.previous);
    if (swapped.state!=ExchangeState::Exchanged) return {TransactionState::NeedsRecovery,swapped.error};
    if (int e=check(p,expected,RecoveryPosition::Unexchanged,verifier,context))
        return {TransactionState::NeedsRecovery,e};
    return {TransactionState::Restored,0};
}
}
