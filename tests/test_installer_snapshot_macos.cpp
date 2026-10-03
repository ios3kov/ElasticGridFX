#include "PayloadSnapshot.h"
#include <cassert>
#include <cerrno>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/xattr.h>
#include <sys/acl.h>
#include <uuid/uuid.h>
#include <unistd.h>
namespace fs=std::filesystem;
using namespace fstr::installer;
int main(){
    char temp[]="/private/tmp/egfx-installer-snapshot-XXXXXX";
    const char* created=mkdtemp(temp); assert(created);
    const fs::path root(created);
    fs::create_directory(root/"plugin");
    fs::create_directory(root/"plugin"/"Contents");
    const auto file=root/"plugin"/"Contents"/"binary";
    std::ofstream(file)<<"bytes";
    const int parent=open(root.c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW);
    assert(parent>=0);
    PayloadSnapshot original{},same{},different{};
    const int initial=snapshotPayload(parent,"plugin",original);
    if(initial) std::cerr<<"snapshot error "<<initial<<"\n";
    assert(!initial && original.entries==3 && original.fileBytes==5);
    assert(!snapshotPayload(parent,"plugin",same) && original.sha256==same.sha256);
    std::ofstream(file,std::ios::trunc)<<"other";
    assert(!snapshotPayload(parent,"plugin",different) && original.sha256!=different.sha256);
    std::ofstream(file,std::ios::trunc)<<"bytes";
    assert(!snapshotPayload(parent,"plugin",same) && original.sha256==same.sha256);
    struct stat s{}; assert(!lstat(file.c_str(),&s));
    assert(!chmod(file.c_str(),0751));
    assert(!snapshotPayload(parent,"plugin",different) && original.sha256!=different.sha256);
    assert(!chmod(file.c_str(),s.st_mode&0777));
    const char attribute[]="snapshot-metadata";
    assert(!setxattr(file.c_str(),"com.fstr.snapshot-test",attribute,sizeof(attribute),0,0));
    assert(!snapshotPayload(parent,"plugin",different) && original.sha256!=different.sha256);
    assert(!removexattr(file.c_str(),"com.fstr.snapshot-test",0));
    assert(!snapshotPayload(parent,"plugin",same) && original.sha256==same.sha256);
    assert(!rename((root/"plugin").c_str(),(root/"renamed").c_str()));
    assert(!snapshotPayload(parent,"renamed",same) && original.sha256==same.sha256 && original.root.inode==same.root.inode);
    assert(!rename((root/"renamed").c_str(),(root/"plugin").c_str()));
    assert(!symlink("Contents/binary",(root/"plugin"/"link").c_str()));
    different=original;
    assert(snapshotPayload(parent,"plugin",different)==ELOOP && different.sha256==original.sha256);
    assert(!unlink((root/"plugin"/"link").c_str()));
    assert(!link(file.c_str(),(root/"plugin"/"hardlink").c_str()));
    assert(snapshotPayload(parent,"plugin",different)==ENOTSUP);
    assert(!unlink((root/"plugin"/"hardlink").c_str()));
    assert(!mkfifo((root/"plugin"/"fifo").c_str(),0600));
    assert(snapshotPayload(parent,"plugin",different)==ENOTSUP);
    assert(!unlink((root/"plugin"/"fifo").c_str()));
    assert(snapshotPayload(parent,"../plugin",different)==EINVAL);
    assert(!snapshotPayload(parent,"plugin",same) && original.sha256==same.sha256);
    const auto huge=root/"plugin"/"huge";
    const int oversized=open(huge.c_str(),O_CREAT|O_EXCL|O_WRONLY,0600);
    assert(oversized>=0 && !ftruncate(oversized,257*1024*1024)); close(oversized);
    assert(snapshotPayload(parent,"plugin",different)==EFBIG);
    assert(!unlink(huge.c_str()));
    auto deep=root/"plugin"/"deep";
    for(int i=0;i<34;++i){ fs::create_directory(deep); deep/="child"; }
    assert(snapshotPayload(parent,"plugin",different)==EFBIG);
    fs::remove_all(root/"plugin"/"deep");
    const int aclFile=open(file.c_str(),O_RDONLY|O_NOFOLLOW);
    assert(aclFile>=0);
    acl_t acl=acl_init(1); assert(acl);
    acl_entry_t ace{}; assert(!acl_create_entry(&acl,&ace));
    assert(!acl_set_tag_type(ace,ACL_EXTENDED_ALLOW));
    uuid_t principal{}; assert(!acl_set_qualifier(ace,principal));
    acl_permset_t perms{}; assert(!acl_get_permset(ace,&perms));
    assert(!acl_add_perm(perms,ACL_READ_DATA));
    assert(!acl_set_fd_np(aclFile,acl,ACL_TYPE_EXTENDED)); acl_free(acl);
    assert(snapshotPayload(parent,"plugin",different)==ENOTSUP);
    close(aclFile);
    close(parent);fs::remove_all(root);
    std::cout<<"PASS: bounded payload snapshot detects bytes/modes/xattrs, retains rename identity and rejects links/special files\n";
}
