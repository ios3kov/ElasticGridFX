#include "PreparedJournal.h"
#include <array>
#include <cerrno>
#include <charconv>
#include <fcntl.h>
#include <limits>
#include <string>
#include <string_view>
#include <sys/stat.h>
#include <unistd.h>

namespace fstr::installer {
namespace {
constexpr const char* filename = "prepared-v1";
bool same(Identity a, Identity b) { return a.device == b.device && a.inode == b.inode; }
int parentError(int fd) {
    struct stat s{};
    if (fstat(fd, &s)) return errno;
    if (!S_ISDIR(s.st_mode) || s.st_uid != geteuid() || (s.st_mode & 0022)) return EACCES;
    return 0;
}
bool valid(PreparedExchange r) {
    return r.previous.device >= 0 && r.previous.device == r.candidate.device &&
           r.previous.inode && r.candidate.inode && !same(r.previous, r.candidate);
}
}
JournalResult prepareJournal(int parent, PreparedExchange r) {
    if (!valid(r)) return {JournalState::NotCreated, EINVAL};
    if (int e = parentError(parent)) return {JournalState::NotCreated, e};
    const std::string bytes = "FSTR-PREPARED-1\n" +
        std::to_string(r.previous.device) + "\n" + std::to_string(r.previous.inode) + "\n" +
        std::to_string(r.candidate.device) + "\n" + std::to_string(r.candidate.inode) + "\n";
    int fd = openat(parent, filename, O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC, 0600);
    if (fd < 0) return {JournalState::NotCreated, errno};
    int error = 0;
    if (fchmod(fd, 0600)) error = errno;
    for (size_t done = 0; !error && done < bytes.size();) {
        ssize_t n = write(fd, bytes.data() + done, bytes.size() - done);
        if (n < 0 && errno == EINTR) continue;
        if (n <= 0) { error = n < 0 ? errno : EIO; break; }
        done += static_cast<size_t>(n);
    }
    if (!error && fsync(fd)) error = errno;
    if (!error && fsync(parent)) error = errno;
    // Darwin's full sync requests the device cache flush, not only kernel sync.
    // Unsupported full sync refuses publication rather than silently weakening it.
    if (!error && fcntl(fd, F_FULLFSYNC)) error = errno;
    if (close(fd) && !error) error = errno;
    // Retain even a partial record; never delete evidence after creation failure.
    return {error ? JournalState::CreatedNeedsRecovery : JournalState::Durable, error};
}
int readPreparedJournal(int parent, PreparedExchange& record) {
    if (int e = parentError(parent)) return e;
    int fd = openat(parent, filename, O_RDONLY | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK);
    if (fd < 0) return errno;
    struct stat s{};
    int error = 0;
    if (fstat(fd, &s)) error = errno;
    else if (!S_ISREG(s.st_mode) || s.st_uid != geteuid() || (s.st_mode & 0777) != 0600 ||
             s.st_nlink != 1 || s.st_size < 1 || s.st_size > 128) error = EINVAL;
    std::array<char, 129> bytes{};
    size_t used = 0;
    while (!error && used < bytes.size()) {
        ssize_t n = read(fd, bytes.data() + used, bytes.size() - used);
        if (n < 0 && errno == EINTR) continue;
        if (n < 0) { error = errno; break; }
        if (!n) break;
        used += static_cast<size_t>(n);
    }
    if (close(fd) && !error) error = errno;
    if (error) return error;
    if (used != static_cast<size_t>(s.st_size) || used > 128) return EINVAL;
    std::string_view text(bytes.data(), used);
    constexpr std::string_view header = "FSTR-PREPARED-1\n";
    if (!text.starts_with(header)) return EINVAL;
    text.remove_prefix(header.size());
    std::array<unsigned long long, 4> values{};
    for (auto& value : values) {
        const auto end = text.find('\n');
        if (end == text.npos || !end) return EINVAL;
        auto token = text.substr(0, end);
        if (token.size() > 1 && token.front() == '0') return EINVAL;
        auto parsed = std::from_chars(token.data(), token.data() + token.size(), value);
        if (parsed.ec != std::errc{} || parsed.ptr != token.data() + token.size()) return EINVAL;
        text.remove_prefix(end + 1);
    }
    if (!text.empty() || values[0] > static_cast<unsigned long long>(std::numeric_limits<dev_t>::max()) ||
        values[2] > static_cast<unsigned long long>(std::numeric_limits<dev_t>::max()) ||
        values[1] > std::numeric_limits<ino_t>::max() || values[3] > std::numeric_limits<ino_t>::max()) return EINVAL;
    PreparedExchange parsed{{static_cast<dev_t>(values[0]), static_cast<ino_t>(values[1])},
                            {static_cast<dev_t>(values[2]), static_cast<ino_t>(values[3])}};
    if (!valid(parsed)) return EINVAL;
    record = parsed;
    return 0;
}
RecoveryPosition locateExchange(PreparedExchange r, Identity active, Identity backup) {
    if (!valid(r)) return RecoveryPosition::Unknown;
    if (same(active, r.previous) && same(backup, r.candidate)) return RecoveryPosition::Unexchanged;
    if (same(active, r.candidate) && same(backup, r.previous)) return RecoveryPosition::Exchanged;
    return RecoveryPosition::Unknown;
}
}
