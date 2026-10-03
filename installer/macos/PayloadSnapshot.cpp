#include "PayloadSnapshot.h"
#include <CommonCrypto/CommonDigest.h>
#include <algorithm>
#include <cerrno>
#include <cstring>
#include <dirent.h>
#include <fcntl.h>
#include <new>
#include <string>
#include <sys/acl.h>
#include <sys/stat.h>
#include <sys/xattr.h>
#include <unistd.h>
#include <vector>

namespace fstr::installer {
namespace {
struct FD { int value; ~FD(){ if(value>=0) close(value); } };
struct Directory { DIR* value; ~Directory(){ if(value) closedir(value); } };
struct Hash {
    CC_SHA256_CTX context{};
    Hash(){ CC_SHA256_Init(&context); }
    void bytes(const void* p,size_t n){ CC_SHA256_Update(&context,p,static_cast<CC_LONG>(n)); }
    void number(uint64_t n){
        unsigned char b[8]; for(unsigned i=0;i<8;++i) b[i]=static_cast<unsigned char>(n>>(i*8));
        bytes(b,8);
    }
    void text(const std::string& s){ number(s.size()); bytes(s.data(),s.size()); }
};
bool stable(const struct stat& a,const struct stat& b){
    return a.st_dev==b.st_dev && a.st_ino==b.st_ino && a.st_mode==b.st_mode &&
        a.st_uid==b.st_uid && a.st_gid==b.st_gid && a.st_flags==b.st_flags &&
        a.st_size==b.st_size && a.st_nlink==b.st_nlink &&
        a.st_mtimespec.tv_sec==b.st_mtimespec.tv_sec && a.st_mtimespec.tv_nsec==b.st_mtimespec.tv_nsec &&
        a.st_ctimespec.tv_sec==b.st_ctimespec.tv_sec && a.st_ctimespec.tv_nsec==b.st_ctimespec.tv_nsec;
}
struct Collector {
    Hash hash;
    dev_t device;
    uint64_t entries=0, fileBytes=0, attributeBytes=0;
    int metadata(int fd,const struct stat& s){
        filesec_t security=filesec_init();
        if(!security) return errno;
        struct stat permissions{};
        int hasACL=0,error=0;
        if(fstatx_np(fd,&permissions,security)) error=errno;
        else if(filesec_query_property(security,FILESEC_ACL,&hasACL)) error=errno;
        filesec_free(security);
        if(error) return error;
        if(!stable(s,permissions)) return EAGAIN;
        // Refuse extended ACLs until their canonical serialization is supported.
        if(hasACL) return ENOTSUP;
        hash.number(s.st_mode); hash.number(s.st_uid); hash.number(s.st_gid); hash.number(s.st_flags);
        ssize_t size=flistxattr(fd,nullptr,0,0);
        if(size<0) return errno;
        if(size>65536) return EFBIG;
        std::vector<char> names(static_cast<size_t>(size));
        if(size && flistxattr(fd,names.data(),names.size(),0)!=size) return EAGAIN;
        std::vector<std::string> attributes;
        for(size_t start=0;start<names.size();){
            const size_t length=strnlen(names.data()+start,names.size()-start);
            if(!length || length==names.size()-start) return EINVAL;
            attributes.emplace_back(names.data()+start,length); start+=length+1;
        }
        std::sort(attributes.begin(),attributes.end());
        if(std::adjacent_find(attributes.begin(),attributes.end())!=attributes.end()) return EINVAL;
        hash.number(attributes.size());
        for(const auto& name:attributes){
            ssize_t count=fgetxattr(fd,name.c_str(),nullptr,0,0,0);
            if(count<0) return errno;
            if(count>1024*1024 || attributeBytes+static_cast<uint64_t>(count)>32*1024*1024) return EFBIG;
            std::vector<char> value(static_cast<size_t>(count));
            if(count && fgetxattr(fd,name.c_str(),value.data(),value.size(),0,0)!=count) return EAGAIN;
            attributeBytes+=static_cast<uint64_t>(count);
            hash.text(name); hash.number(value.size()); hash.bytes(value.data(),value.size());
        }
        return 0;
    }
    int visit(int fd,const std::string& relative,unsigned depth){
        if(depth>32 || ++entries>4096) return EFBIG;
        struct stat before{},after{};
        if(fstat(fd,&before)) return errno;
        if(before.st_dev!=device) return EXDEV;
        if(!S_ISREG(before.st_mode) && !S_ISDIR(before.st_mode)) return ENOTSUP;
        if(S_ISREG(before.st_mode) && before.st_nlink!=1) return ENOTSUP;
        hash.text(relative);
        if(int e=metadata(fd,before)) return e;
        if(S_ISREG(before.st_mode)){
            if(before.st_size<0 || before.st_size>256*1024*1024 ||
                fileBytes+static_cast<uint64_t>(before.st_size)>512*1024*1024) return EFBIG;
            hash.number(static_cast<uint64_t>(before.st_size));
            std::vector<char> buffer(65536);
            uint64_t readBytes=0;
            for(;;){
                ssize_t count=read(fd,buffer.data(),buffer.size());
                if(count<0 && errno==EINTR) continue;
                if(count<0) return errno;
                if(!count) break;
                readBytes+=static_cast<uint64_t>(count);
                if(readBytes>static_cast<uint64_t>(before.st_size)) return EAGAIN;
                hash.bytes(buffer.data(),static_cast<size_t>(count));
            }
            if(readBytes!=static_cast<uint64_t>(before.st_size)) return EAGAIN;
            fileBytes+=readBytes;
        }else{
            // Separate open file description; traversal never consumes caller FD offsets.
            FD scan{openat(fd,".",O_RDONLY|O_DIRECTORY|O_NOFOLLOW|O_CLOEXEC)};
            if(scan.value<0) return errno;
            Directory directory{fdopendir(scan.value)};
            if(!directory.value) return errno;
            scan.value=-1;
            std::vector<std::string> children;
            int error=0;
            for(;;){
                errno=0; auto* item=readdir(directory.value);
                if(!item){error=errno;break;}
                if(!std::strcmp(item->d_name,".") || !std::strcmp(item->d_name,"..")) continue;
                children.emplace_back(item->d_name);
                if(children.size()>4096){error=EFBIG;break;}
            }
            if(error) return error;
            std::sort(children.begin(),children.end());
            hash.number(children.size());
            for(const auto& child:children){
                struct stat entry{};
                if(fstatat(fd,child.c_str(),&entry,AT_SYMLINK_NOFOLLOW)) return errno;
                if(S_ISLNK(entry.st_mode)) return ELOOP;
                if(!S_ISREG(entry.st_mode) && !S_ISDIR(entry.st_mode)) return ENOTSUP;
                FD next{openat(fd,child.c_str(),O_RDONLY|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK)};
                if(next.value<0) return errno;
                struct stat opened{}; if(fstat(next.value,&opened)) return errno;
                if(!stable(entry,opened)) return EAGAIN;
                if(int e=visit(next.value,relative.empty() ? child : relative+"/"+child,depth+1)) return e;
            }
        }
        if(fstat(fd,&after)) return errno;
        return stable(before,after) ? 0 : EAGAIN;
    }
};
}
int snapshotPayload(int parent,const char* name,PayloadSnapshot& output) noexcept {
    if(!name || !*name || std::strlen(name)>255 || std::strchr(name,'/') ||
       !std::strcmp(name,".") || !std::strcmp(name,"..")) return EINVAL;
    try{
        FD root{openat(parent,name,O_RDONLY|O_DIRECTORY|O_NOFOLLOW|O_CLOEXEC)};
        if(root.value<0) return errno;
        struct stat initial{}; if(fstat(root.value,&initial)) return errno;
        Collector collector{{},initial.st_dev};
        collector.hash.text("FSTR-PAYLOAD-SNAPSHOT-1");
        if(int e=collector.visit(root.value,"",0)) return e;
        struct stat current{};
        if(fstatat(parent,name,&current,AT_SYMLINK_NOFOLLOW)) return errno;
        if(!stable(initial,current)) return EAGAIN;
        PayloadSnapshot result{{initial.st_dev,initial.st_ino},{},collector.entries,collector.fileBytes};
        CC_SHA256_Final(result.sha256.data(),&collector.hash.context);
        output=result; return 0;
    }catch(const std::bad_alloc&){return ENOMEM;}
    catch(...){return EIO;}
}
}
