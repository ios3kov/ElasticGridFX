#include "AtomicExchange.h"
#include <cerrno>
#include <cstring>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/stdio.h>
#include <unistd.h>

namespace fstr::installer {
namespace {
bool component(const char* name) {
    return name && *name && std::strlen(name) <= 255 &&
           !std::strchr(name, '/') && std::strcmp(name, ".") && std::strcmp(name, "..");
}
int parentError(int fd, struct stat& out) {
    if (fstat(fd, &out)) return errno;
    // A privileged frontend must not use parents controlled by another user.
    // Fixed roots and ancestor/reparse checks remain collector responsibilities.
    if (!S_ISDIR(out.st_mode) || out.st_uid != geteuid() || (out.st_mode & 0022)) return EACCES;
    return 0;
}
int leafError(int fd, const char* name, Identity expected, struct stat& out) {
    if (fstatat(fd, name, &out, AT_SYMLINK_NOFOLLOW)) return errno;
    if (S_ISLNK(out.st_mode)) return ELOOP;
    if (!S_ISDIR(out.st_mode)) return ENOTDIR;
    if (out.st_dev != expected.device || out.st_ino != expected.inode) return EAGAIN;
    return 0;
}
}
ExchangeResult exchangeDirectories(int parentA, const char* nameA, Identity expectedA,
                                   int parentB, const char* nameB, Identity expectedB) {
    auto fail = [](int error) { return ExchangeResult{ExchangeState::NotExchanged, error}; };
    if (!component(nameA) || !component(nameB)) return fail(EINVAL);
    struct stat pa{}, pb{}, a{}, b{};
    if (int e = parentError(parentA, pa)) return fail(e);
    if (int e = parentError(parentB, pb)) return fail(e);
    if (int e = leafError(parentA, nameA, expectedA, a)) return fail(e);
    if (int e = leafError(parentB, nameB, expectedB, b)) return fail(e);
    if (a.st_dev != b.st_dev || a.st_dev != pa.st_dev || b.st_dev != pb.st_dev) return fail(EXDEV);
    if (a.st_dev == b.st_dev && a.st_ino == b.st_ino) return fail(EINVAL);
    // Public Darwin10.12+ API. Directory FDs bind parents; NOFOLLOW_ANY prevents
    // following substituted leaf links. No copy/delete/two-rename fallback.
    if (renameatx_np(parentA, nameA, parentB, nameB, RENAME_SWAP | RENAME_NOFOLLOW_ANY))
        return fail(errno);
    // Never report an already-committed exchange as if nothing happened.
    struct stat nowA{}, nowB{};
    if (int e = leafError(parentA, nameA, expectedB, nowA))
        return {ExchangeState::ExchangedNeedsRecovery, e};
    if (int e = leafError(parentB, nameB, expectedA, nowB))
        return {ExchangeState::ExchangedNeedsRecovery, e};
    if (fsync(parentA)) return {ExchangeState::ExchangedNeedsRecovery, errno};
    if (fsync(parentB)) return {ExchangeState::ExchangedNeedsRecovery, errno};
    return {ExchangeState::Exchanged, 0};
}
}
