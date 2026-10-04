#include "SnapshotReceipt.h"
#include <array>
#include <cerrno>
#include <charconv>
#include <fcntl.h>
#include <limits>
#include <string>
#include <string_view>
#include <sys/acl.h>
#include <sys/stat.h>
#include <unistd.h>

namespace fstr::installer {
namespace {
constexpr const char* name="snapshots-v1";
constexpr std::string_view magic="FSTR-SNAPSHOTS-1\n";
int aclError(int fd) {
    filesec_t security=filesec_init();
    if(!security) return ENOMEM;
    struct stat again{}; int hasACL=0,error=0;
    if(fstatx_np(fd,&again,security)) error=errno;
    else if(filesec_query_property(security,FILESEC_ACL,&hasACL)) error=errno;
    filesec_free(security);
    if(!error && hasACL) error=ENOTSUP;
    return error;
}
int parentError(int fd) {
    struct stat s{},again{};
    if(fstat(fd,&s)) return errno;
    if(!S_ISDIR(s.st_mode) || s.st_uid!=geteuid() || (s.st_mode&0777)!=0700) return EACCES;
    int error=aclError(fd);
    if(!error && fstat(fd,&again)) error=errno;
    if(!error && (s.st_dev!=again.st_dev || s.st_ino!=again.st_ino || s.st_mode!=again.st_mode || s.st_uid!=again.st_uid)) error=EAGAIN;
    return error;
}
bool valid(const PayloadSnapshot& s) {
    return s.root.device>=0 && s.root.inode && s.entries>0 && s.entries<=4096 &&
        s.fileBytes<=32ULL*1024*1024;
}
bool valid(const SnapshotReceipt& r) {
    if(!valid(r.candidate)) return false;
    if(!r.previousPresent) return r.previous.root.device==0 && r.previous.root.inode==0 &&
        r.previous.entries==0 && r.previous.fileBytes==0 && r.previous.sha256==std::array<unsigned char,32>{};
    return valid(r.previous) && r.previous.root.device==r.candidate.root.device &&
        r.previous.root.inode!=r.candidate.root.inode;
}
std::string encode(const PayloadSnapshot& s) {
    constexpr char hex[]="0123456789abcdef";
    std::string hash; hash.reserve(64);
    for(auto byte:s.sha256) { hash+=hex[byte>>4]; hash+=hex[byte&15]; }
    return std::to_string(s.root.device)+"\n"+std::to_string(s.root.inode)+"\n"+hash+"\n"+
        std::to_string(s.entries)+"\n"+std::to_string(s.fileBytes)+"\n";
}
bool token(std::string_view& text,std::string_view& out) {
    const auto end=text.find('\n');
    if(end==text.npos || !end) return false;
    out=text.substr(0,end); text.remove_prefix(end+1); return true;
}
bool number(std::string_view& text,uint64_t& out) {
    std::string_view t;
    if(!token(text,t) || (t.size()>1 && t.front()=='0')) return false;
    const auto parsed=std::from_chars(t.data(),t.data()+t.size(),out);
    return parsed.ec==std::errc{} && parsed.ptr==t.data()+t.size();
}
bool decode(std::string_view& text,PayloadSnapshot& out) {
    uint64_t device=0,inode=0; std::string_view hash;
    if(!number(text,device) || !number(text,inode) || !token(text,hash) || hash.size()!=64 ||
       device>static_cast<uint64_t>(std::numeric_limits<dev_t>::max()) || inode>std::numeric_limits<ino_t>::max()) return false;
    out.root={static_cast<dev_t>(device),static_cast<ino_t>(inode)};
    auto digit=[](char c)->int { return c>='0'&&c<='9' ? c-'0' : c>='a'&&c<='f' ? c-'a'+10 : -1; };
    for(size_t i=0;i<32;++i) {
        const int a=digit(hash[2*i]),b=digit(hash[2*i+1]);
        if(a<0 || b<0) return false;
        out.sha256[i]=static_cast<unsigned char>(16*a+b);
    }
    return number(text,out.entries) && number(text,out.fileBytes);
}
}
bool matchesSnapshot(const PayloadSnapshot& a,const PayloadSnapshot& b) noexcept {
    return a.root.device==b.root.device && a.root.inode==b.root.inode &&
        a.sha256==b.sha256 && a.entries==b.entries && a.fileBytes==b.fileBytes;
}
JournalResult prepareSnapshotReceipt(int parent,const SnapshotReceipt& r) {
    if(!valid(r)) return {JournalState::NotCreated,EINVAL};
    if(int e=parentError(parent)) return {JournalState::NotCreated,e};
    const std::string bytes=std::string(magic)+(r.previousPresent?"1\n":"0\n")+encode(r.previous)+encode(r.candidate);
    int fd=openat(parent,name,O_WRONLY|O_CREAT|O_EXCL|O_NOFOLLOW|O_CLOEXEC,0600);
    if(fd<0) return {JournalState::NotCreated,errno};
    int error=0;
    if(fchmod(fd,0600)) error=errno;
    for(size_t done=0;!error && done<bytes.size();) {
        const ssize_t n=write(fd,bytes.data()+done,bytes.size()-done);
        if(n<0 && errno==EINTR) continue;
        if(n<=0) { error=n<0?errno:EIO; break; }
        done+=static_cast<size_t>(n);
    }
    if(!error && fsync(fd)) error=errno;
    if(!error && fsync(parent)) error=errno;
    if(!error && fcntl(fd,F_FULLFSYNC)) error=errno;
    if(close(fd) && !error) error=errno;
    return {error?JournalState::CreatedNeedsRecovery:JournalState::Durable,error};
}
int readSnapshotReceipt(int parent,SnapshotReceipt& out) {
    if(int e=parentError(parent)) return e;
    int fd=openat(parent,name,O_RDONLY|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK);
    if(fd<0) return errno;
    struct stat s{},after{}; int error=0;
    if(fstat(fd,&s)) error=errno;
    else if(!S_ISREG(s.st_mode) || s.st_uid!=geteuid() || (s.st_mode&0777)!=0600 || s.st_nlink!=1 || s.st_size<1 || s.st_size>512) error=EINVAL;
    if(!error) error=aclError(fd);
    std::array<char,513> bytes{}; size_t used=0;
    while(!error && used<bytes.size()) {
        const ssize_t n=read(fd,bytes.data()+used,bytes.size()-used);
        if(n<0 && errno==EINTR) continue;
        if(n<0) { error=errno; break; }
        if(!n) break;
        used+=static_cast<size_t>(n);
    }
    if(!error && fstat(fd,&after)) error=errno;
    if(!error && (s.st_size!=after.st_size || s.st_mtimespec.tv_sec!=after.st_mtimespec.tv_sec ||
        s.st_mtimespec.tv_nsec!=after.st_mtimespec.tv_nsec || s.st_ctimespec.tv_sec!=after.st_ctimespec.tv_sec ||
        s.st_ctimespec.tv_nsec!=after.st_ctimespec.tv_nsec)) error=EAGAIN;
    if(close(fd) && !error) error=errno;
    if(error) return error;
    if(used!=static_cast<size_t>(s.st_size) || used>512) return EINVAL;
    std::string_view text(bytes.data(),used),present;
    if(!text.starts_with(magic)) return EINVAL;
    text.remove_prefix(magic.size());
    if(!token(text,present) || (present!="0" && present!="1")) return EINVAL;
    SnapshotReceipt r{}; r.previousPresent=present=="1";
    if(!decode(text,r.previous) || !decode(text,r.candidate) || !text.empty() || !valid(r)) return EINVAL;
    out=r; return 0;
}
}
