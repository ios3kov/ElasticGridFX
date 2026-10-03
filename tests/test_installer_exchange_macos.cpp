#include "AtomicExchange.h"
#include "PreparedJournal.h"
#include <cassert>
#include <cerrno>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/xattr.h>
#include <sys/wait.h>
#include <signal.h>
#include <poll.h>
#include <unistd.h>
using namespace fstr::installer;
namespace fs = std::filesystem;
Identity id(int parent, const char* name) {
    struct stat s{}; assert(!fstatat(parent, name, &s, AT_SYMLINK_NOFOLLOW));
    return {s.st_dev, s.st_ino};
}
int main() {
    char temp[] = "/private/tmp/egfx-installer-exchange-XXXXXX";
    char* created = mkdtemp(temp); assert(created);
    fs::path root(created);
    fs::create_directory(root/"active"); fs::create_directory(root/"backups");
    fs::create_directory(root/"active"/"target.plugin");
    fs::create_directory(root/"backups"/"previous.plugin");
    chmod((root/"active").c_str(),0700); chmod((root/"backups").c_str(),0700);
    std::ofstream(root/"active"/"target.plugin"/"original") << "old";
    std::ofstream(root/"backups"/"previous.plugin"/"candidate") << "new";
    const auto original = root/"active"/"target.plugin"/"original";
    assert(!chmod(original.c_str(),0751));
    const char attribute[] = "metadata-retained";
    assert(!setxattr(original.c_str(),"com.fstr.installer-fixture",attribute,sizeof(attribute),0,0));
    struct stat oldFile{}; assert(!lstat(original.c_str(),&oldFile));
    int a=open((root/"active").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
    int b=open((root/"backups").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
    assert(a>=0 && b>=0);
    auto old=id(a,"target.plugin"), next=id(b,"previous.plugin");
    fs::create_directory(root/"transaction");
    assert(!chmod((root/"transaction").c_str(),0700));
    int transaction=open((root/"transaction").c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
    assert(transaction>=0);
    PreparedExchange prepared{old,next}, loaded{};
    auto journal=prepareJournal(transaction,prepared);
    assert(journal.state==JournalState::Durable && journal.error==0);
    assert(!readPreparedJournal(transaction,loaded));
    assert(locateExchange(loaded,id(a,"target.plugin"),id(b,"previous.plugin"))==RecoveryPosition::Unexchanged);
    journal=prepareJournal(transaction,prepared);
    assert(journal.state==JournalState::NotCreated && journal.error==EEXIST);
    auto result=exchangeDirectories(a,"target.plugin",old,b,"previous.plugin",next);
    assert(result.state==ExchangeState::Exchanged && result.error==0);
    assert(id(a,"target.plugin").inode==next.inode && id(b,"previous.plugin").inode==old.inode);
    assert(locateExchange(loaded,id(a,"target.plugin"),id(b,"previous.plugin"))==RecoveryPosition::Exchanged);
    assert(locateExchange(loaded,next,next)==RecoveryPosition::Unknown);
    const auto backupFile=root/"backups"/"previous.plugin"/"original";
    struct stat kept{}; assert(!lstat(backupFile.c_str(),&kept));
    assert(kept.st_ino==oldFile.st_ino && kept.st_mode==oldFile.st_mode && kept.st_uid==oldFile.st_uid);
    char readAttribute[64]{};
    assert(getxattr(backupFile.c_str(),"com.fstr.installer-fixture",readAttribute,sizeof(readAttribute),0,0)==sizeof(attribute));
    assert(std::string(readAttribute)==attribute);
    auto fail=exchangeDirectories(a,"target.plugin",old,b,"previous.plugin",next);
    assert(fail.state==ExchangeState::NotExchanged && fail.error==EAGAIN);
    fail=exchangeDirectories(a,"../target.plugin",next,b,"previous.plugin",old);
    assert(fail.state==ExchangeState::NotExchanged && fail.error==EINVAL);
    fail=exchangeDirectories(a,"target.plugin",next,a,"target.plugin",next);
    assert(fail.state==ExchangeState::NotExchanged && fail.error==EINVAL);
    fail=exchangeDirectories(a,"missing.plugin",next,b,"previous.plugin",old);
    assert(fail.state==ExchangeState::NotExchanged && fail.error==ENOENT);
    std::ofstream(root/"active"/"file.plugin") << "not a bundle";
    fail=exchangeDirectories(a,"file.plugin",next,b,"previous.plugin",old);
    assert(fail.state==ExchangeState::NotExchanged && fail.error==ENOTDIR);
    assert(!symlink("target.plugin",(root/"active"/"link.plugin").c_str()));
    fail=exchangeDirectories(a,"link.plugin",next,b,"previous.plugin",old);
    assert(fail.state==ExchangeState::NotExchanged && fail.error==ELOOP);
    assert(!chmod((root/"backups").c_str(),0777));
    fail=exchangeDirectories(a,"target.plugin",next,b,"previous.plugin",old);
    assert(fail.state==ExchangeState::NotExchanged && fail.error==EACCES);
    assert(!chmod((root/"backups").c_str(),0700));
    assert(id(a,"target.plugin").inode==next.inode && id(b,"previous.plugin").inode==old.inode);
    auto restored=exchangeDirectories(a,"target.plugin",next,b,"previous.plugin",old);
    assert(restored.state==ExchangeState::Exchanged);
    assert(id(a,"target.plugin").inode==old.inode && id(b,"previous.plugin").inode==next.inode);
    assert(locateExchange(loaded,id(a,"target.plugin"),id(b,"previous.plugin"))==RecoveryPosition::Unexchanged);
    std::ofstream(root/"transaction"/"prepared-v1",std::ios::trunc)<<"partial";
    auto retained=loaded;
    assert(readPreparedJournal(transaction,loaded)==EINVAL);
    assert(loaded.previous.inode==retained.previous.inode && loaded.candidate.inode==retained.candidate.inode);
    close(transaction);
    // Abrupt process death at both durable checkpoints. No installer/Adobe paths.
    for (bool swapBeforeDeath : {false,true}) {
        const auto sub=root/(swapBeforeDeath ? "after-swap" : "before-swap");
        fs::create_directory(sub);
        assert(!chmod(sub.c_str(),0700));
        int tx=open(sub.c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
        assert(tx>=0);
        int ready[2]; assert(!pipe(ready));
        const auto child=fork(); assert(child>=0);
        if (!child) {
            close(ready[0]);
            const auto j=prepareJournal(tx,prepared);
            if (j.state!=JournalState::Durable) _exit(10);
            if (swapBeforeDeath) {
                const auto s=exchangeDirectories(a,"target.plugin",old,b,"previous.plugin",next);
                if (s.state!=ExchangeState::Exchanged) _exit(11);
            }
            const char marker='R';
            if (write(ready[1],&marker,1)!=1) _exit(12);
            for (;;) pause();
        }
        close(ready[1]);
        struct pollfd notify{ready[0],POLLIN,0};
        const int available=poll(&notify,1,5000);
        char marker=0;
        const bool received=available>0 && read(ready[0],&marker,1)==1 && marker=='R';
        close(ready[0]);
        assert(!kill(child,SIGKILL));
        int status=0; assert(waitpid(child,&status,0)==child);
        assert(received);
        assert(WIFSIGNALED(status) && WTERMSIG(status)==SIGKILL);
        PreparedExchange recovered{}; assert(!readPreparedJournal(tx,recovered));
        const auto position=locateExchange(recovered,id(a,"target.plugin"),id(b,"previous.plugin"));
        assert(position==(swapBeforeDeath ? RecoveryPosition::Exchanged : RecoveryPosition::Unexchanged));
        if (swapBeforeDeath) {
            const auto back=exchangeDirectories(a,"target.plugin",next,b,"previous.plugin",old);
            assert(back.state==ExchangeState::Exchanged);
        }
        assert(id(a,"target.plugin").inode==old.inode && id(b,"previous.plugin").inode==next.inode);
        close(tx);
    }
    close(a);close(b);
    fs::remove_all(root); // Only this freshly-created disposable fixture.
    std::cout << "PASS: native macOS exchange/restore, inode/mode/xattr preservation, negative guards\n";
}
