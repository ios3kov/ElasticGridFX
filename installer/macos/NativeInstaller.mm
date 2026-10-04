#import <Foundation/Foundation.h>
#import <Security/Security.h>
#include "NativeInstaller.h"
#include "ExchangeCoordinator.h"
#include "FreshPublication.h"
#include "Payload.h"
#include <CommonCrypto/CommonDigest.h>
#include <algorithm>
#include <cerrno>
#include <fcntl.h>
#include <fstream>
#include <libproc.h>
#include <regex>
#include <stdexcept>
#include <sys/stat.h>
#include <sys/acl.h>
#include <unistd.h>

namespace fs=std::filesystem;
namespace fstr::installer {
namespace {
constexpr const char* target="FSTR Stretch.plugin";
constexpr const char* retained="previous.plugin";
struct FD { int value=-1; explicit FD(int n):value(n){} ~FD(){if(value>=0)close(value);} FD(const FD&)=delete; };
void require(bool ok,const char* message){if(!ok)throw std::runtime_error(message);}
void checked(int error,const char* message){if(error)throw std::runtime_error(std::string(message)+" ("+std::to_string(error)+")");}
bool absent(const fs::path& p){struct stat s{};if(!lstat(p.c_str(),&s))return false;require(errno==ENOENT,"Cannot inspect path.");return true;}
void noLinks(const fs::path& p){
    fs::path current;
    for(const auto& part:p){current/=part;if(absent(current))break;
        struct stat s{};require(!lstat(current.c_str(),&s) && !S_ISLNK(s.st_mode),"Linked paths are not supported.");}
}
FD directory(const fs::path& p){noLinks(p);int fd=open(p.c_str(),O_RDONLY|O_DIRECTORY|O_NOFOLLOW|O_CLOEXEC);require(fd>=0,"Cannot open installation folder.");return FD(fd);}
void noACL(int fd){
    filesec_t security=filesec_init();require(security!=nullptr,"Cannot inspect folder access rules.");
    struct stat s{};int present=0;
    int result=fstatx_np(fd,&s,security);if(!result)result=filesec_query_property(security,FILESEC_ACL,&present);
    filesec_free(security);require(!result && !present,"Extended folder access rules are unsupported. Nothing replaced.");
}
void protectedAncestors(const fs::path& p){
    fs::path current;
    for(const auto& part:p){current/=part;if(absent(current))break;FD fd=directory(current);struct stat s{};
        require(!fstat(fd.value,&s) && s.st_uid==0 && !(s.st_mode&0002) && (!(s.st_mode&0020) || s.st_gid==80),"Unsafe system ancestor. Nothing replaced.");noACL(fd.value);}
}
void secure(const fs::path& p,mode_t mode){
    noLinks(p);
    if(absent(p)){secure(p.parent_path(),0755);require(!mkdir(p.c_str(),mode),"Cannot create installation folder.");}
    struct stat s{};require(!lstat(p.c_str(),&s) && S_ISDIR(s.st_mode) && s.st_uid==geteuid() && !(s.st_mode&0002),"Unsafe folder ownership or permissions. Nothing replaced.");
    // System Adobe ancestors may allow administrator-group writes. Our own
    // active/transaction directories are checked more strictly by the core.
    require(!(s.st_mode&0020) || s.st_gid==80,"Unsafe group-writable folder.");FD fd=directory(p);noACL(fd.value);
}
std::string fileBytes(const fs::path& p){
    noLinks(p);FD fd(open(p.c_str(),O_RDONLY|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK));
    struct stat before{},after{};require(fd.value>=0 && !fstat(fd.value,&before) && S_ISREG(before.st_mode) && before.st_nlink==1 && before.st_size>=0 && before.st_size<=32*1024*1024,"Unknown or unreadable plugin file.");
    std::string bytes(static_cast<std::size_t>(before.st_size),'\0');std::size_t done=0;
    while(done<bytes.size()){auto n=read(fd.value,bytes.data()+done,bytes.size()-done);if(n<0 && errno==EINTR)continue;require(n>0,"Cannot read plugin.");done+=static_cast<std::size_t>(n);}
    require(!fstat(fd.value,&after) && before.st_ino==after.st_ino && before.st_size==after.st_size && before.st_mtimespec.tv_sec==after.st_mtimespec.tv_sec && before.st_mtimespec.tv_nsec==after.st_mtimespec.tv_nsec && before.st_ctimespec.tv_sec==after.st_ctimespec.tv_sec && before.st_ctimespec.tv_nsec==after.st_ctimespec.tv_nsec,"Plugin changed during inspection.");
    return bytes;
}
std::string digest(const void* p,std::size_t size){unsigned char bytes[32];CC_SHA256(p,static_cast<CC_LONG>(size),bytes);std::string out;constexpr char h[]="0123456789abcdef";for(auto b:bytes){out+=h[b>>4];out+=h[b&15];}return out;}
void signature(const fs::path& p){
    NSURL* url=[NSURL fileURLWithPath:[NSString stringWithUTF8String:p.c_str()]];
    SecStaticCodeRef code=nullptr;
    require(SecStaticCodeCreateWithPath((__bridge CFURLRef)url,kSecCSDefaultFlags,&code)==errSecSuccess,"Cannot inspect plugin signature.");
    auto result=SecStaticCodeCheckValidity(code,kSecCSStrictValidate|kSecCSCheckAllArchitectures,nullptr);CFRelease(code);
    require(result==errSecSuccess,"Plugin signature is invalid. Nothing replaced.");
}
std::string identity(const fs::path& p){
    auto bytes=fileBytes(p/"Contents/Resources/BuildIdentity.json");
    NSData* data=[NSData dataWithBytes:bytes.data() length:bytes.size()];
    id m=[NSJSONSerialization JSONObjectWithData:data options:0 error:nullptr];
    require([m isKindOfClass:[NSDictionary class]],"Unknown installed plugin preserved.");
    NSDictionary* b=m;NSString* idString=b[@"build_id"];
    require([b[@"source_state"] isEqual:@"clean"] && [b[@"target"] isEqual:@"aarch64-apple-darwin"] && [b[@"profile"] isEqual:@"release"] && [idString isKindOfClass:[NSString class]],"Unknown installed build preserved.");
    std::string idValue=[idString UTF8String];require(std::regex_match(idValue,std::regex("EGFX-[0-9a-f]{24}")),"Unknown installed identity.");
    require(fileBytes(p/"Contents/MacOS/ElasticGrid").find("ElasticGridBuildID="+idValue)!=std::string::npos,"Installed identity differs from binary.");
    signature(p);return idValue;
}
bool isPlugin(const fs::path& p){
    auto name=p.filename().string();std::transform(name.begin(),name.end(),name.begin(),[](unsigned char c){return static_cast<char>(std::tolower(c));});
    bool named=name=="fstr stretch.plugin" || name.find("elasticgrid")!=std::string::npos;
    const auto plist=p/"Contents/Info.plist";
    if(absent(plist))return named;
    auto bytes=fileBytes(plist);NSData* data=[NSData dataWithBytes:bytes.data() length:bytes.size()];
    id metadata=[NSPropertyListSerialization propertyListWithData:data options:NSPropertyListImmutable format:nullptr error:nullptr];
    require([metadata isKindOfClass:[NSDictionary class]],"Cannot read a plugin identity; scan incomplete.");
    return named || [metadata[@"CFBundleIdentifier"] isEqual:@"com.elasticgrid.fx"];
}
void scan(const fs::path& root,std::vector<fs::path>& found,unsigned depth=0){
    noLinks(root);if(absent(root))return;require(depth<=12,"Plugin scan depth exceeded.");
    for(const auto& entry:fs::directory_iterator(root)){
        const auto p=entry.path();noLinks(p);
        if(!entry.is_directory())continue;
        auto ext=p.extension().string();
        if(ext==".plugin" || ext==".bundle"){if(isPlugin(p))found.push_back(p);continue;}
        auto name=p.filename().string();if(name.starts_with('(') && name.ends_with(')'))continue;
        scan(p,found,depth+1);
    }
}
void environment(const InstallEnvironment& e,bool present){
    e.checkHosts();std::vector<fs::path> found;
    for(const auto& root:e.scanRoots)scan(root,found);
    std::sort(found.begin(),found.end());found.erase(std::unique(found.begin(),found.end()),found.end());
    require(found.size()==(present?1u:0u) && (!present || found.front()==e.active/target),"Another or unknown copy of FSTR Stretch was found. Nothing replaced. Remove the conflict first.");
    noLinks(e.active);noLinks(e.backups);
}
void syncTree(const fs::path& p){
    noLinks(p);
    if(fs::is_directory(p)){for(const auto& item:fs::directory_iterator(p))syncTree(item.path());FD fd=directory(p);require(!fsync(fd.value),"Cannot flush staged directory.");}
    else{FD fd(open(p.c_str(),O_RDWR|O_NOFOLLOW|O_CLOEXEC));require(fd.value>=0 && !fsync(fd.value) && !fcntl(fd.value,F_FULLFSYNC),"Cannot durably flush staged file.");}
}
void stage(const fs::path& p){
    require(absent(p),"Staging folder already exists.");require(!mkdir(p.c_str(),0755),"Cannot stage plugin.");
    for(std::size_t i=0;i<payload::count;++i){const auto& f=payload::files[i];require(digest(f.bytes,f.size)==f.sha256,"Embedded payload checksum failed.");
        const fs::path out=p/f.name;secure(out.parent_path(),0755);FD fd(open(out.c_str(),O_WRONLY|O_CREAT|O_EXCL|O_NOFOLLOW|O_CLOEXEC,f.executable?0755:0644));require(fd.value>=0,"Cannot stage payload file.");
        std::size_t done=0;while(done<f.size){auto n=write(fd.value,f.bytes+done,f.size-done);if(n<0 && errno==EINTR)continue;require(n>0,"Cannot write staged file.");done+=static_cast<std::size_t>(n);}require(!fchmod(fd.value,f.executable?0755:0644),"Cannot set payload permissions.");
    }
    syncTree(p);signature(p);
}
void verifyCandidate(const fs::path& p){
    std::size_t count=0;for(const auto& item:fs::recursive_directory_iterator(p)){noLinks(item.path());if(item.is_regular_file())++count;else require(item.is_directory(),"Unknown staged object.");}
    require(count==payload::count,"Unexpected payload files.");
    for(std::size_t i=0;i<payload::count;++i){const auto& f=payload::files[i];auto bytes=fileBytes(p/f.name);require(bytes.size()==f.size && digest(bytes.data(),bytes.size())==f.sha256,"Staged payload changed.");struct stat s{};require(!lstat((p/f.name).c_str(),&s) && (s.st_mode&0777)==(f.executable?0755:0644),"Staged permissions changed.");}
    require(identity(p)==payload::buildId,"Wrong staged build.");
}
void mark(int tx,const char* name){FD fd(openat(tx,name,O_WRONLY|O_CREAT|O_EXCL|O_NOFOLLOW|O_CLOEXEC,0600));require(fd.value>=0,"Cannot record completed transaction.");require(!fsync(fd.value) && !fcntl(fd.value,F_FULLFSYNC) && !fsync(tx),"Cannot flush completed transaction.");}
bool marked(const fs::path& tx){return !absent(tx/"installed") || !absent(tx/"restored");}
struct Verify {
    const InstallEnvironment* env;fs::path tx;SnapshotReceipt saved;
    static int exchange(void* opaque,RecoveryPosition position) noexcept {try{auto& c=*static_cast<Verify*>(opaque);environment(*c.env,true);FD a=directory(c.env->active),b=directory(c.tx);PayloadSnapshot active{},backup{};checked(snapshotPayload(a.value,target,active),"Active snapshot failed.");checked(snapshotPayload(b.value,retained,backup),"Backup snapshot failed.");bool swapped=position==RecoveryPosition::Exchanged;require(matchesSnapshot(active,swapped?c.saved.candidate:c.saved.previous) && matchesSnapshot(backup,swapped?c.saved.previous:c.saved.candidate),"Payload changed. Restore refused.");return 0;}catch(...){return EAGAIN;}}
    static int fresh(void* opaque,bool published) noexcept {try{auto& c=*static_cast<Verify*>(opaque);environment(*c.env,published);return 0;}catch(...){return EAGAIN;}}
};
void hosts(){
    int bytes=proc_listpids(PROC_ALL_PIDS,0,nullptr,0);require(bytes>0,"Cannot inspect running Adobe hosts.");std::vector<pid_t> ids(static_cast<std::size_t>(bytes)/sizeof(pid_t)+1024);int used=proc_listpids(PROC_ALL_PIDS,0,ids.data(),static_cast<int>(ids.size()*sizeof(pid_t)));require(used>0 && used<static_cast<int>(ids.size()*sizeof(pid_t)),"Process scan incomplete.");
    for(int i=0;i<used/static_cast<int>(sizeof(pid_t));++i){if(ids[static_cast<std::size_t>(i)]<=0)continue;char path[PROC_PIDPATHINFO_MAXSIZE]{};errno=0;int n=proc_pidpath(ids[static_cast<std::size_t>(i)],path,sizeof(path));if(n<=0){if(errno==ESRCH || errno==ENOENT)continue;throw std::runtime_error("Cannot inspect a process. Nothing replaced.");}std::string s(path);std::transform(s.begin(),s.end(),s.begin(),[](unsigned char c){return static_cast<char>(std::tolower(c));});if(fs::path(s).filename()=="crashpad_handler")continue;for(const auto* needle:{"after effects","aerender","premiere pro","adobe media encoder","dynamiclinkmanager"})require(s.find(needle)==std::string::npos,"Close After Effects and other Adobe render applications, then try again.");}
}
}
InstallEnvironment systemEnvironment(){
    InstallEnvironment e{"/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX","/Library/Application Support/FSTR FX/Backups",{},hosts};
    e.scanRoots.push_back(e.active.parent_path());
    for(const auto& p:fs::directory_iterator("/Applications")){if(p.path().filename().string().starts_with("Adobe After Effects")){e.scanRoots.push_back(p.path()/"Plug-ins");e.scanRoots.push_back(p.path()/"Contents/Plug-ins");}}
    for(const auto& p:fs::directory_iterator("/Users")){if(p.is_directory() && !p.is_symlink())e.scanRoots.push_back(p.path()/"Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore");}
    return e;
}
std::string runNative(const InstallEnvironment& e,bool restore){
    @autoreleasepool {
    e.checkHosts();noLinks(e.active);noLinks(e.backups);
    if(geteuid()==0){protectedAncestors(e.active.parent_path());protectedAncestors(e.backups);}
    bool present=!absent(e.active/target);environment(e,present);
    if(present)identity(e.active/target);
    // Legacy manual installations can have a user-owned FSTR FX folder. Adopt
    // only the exact verified single-plugin directory, never a shared/unknown
    // directory or payload tree. Plugin metadata/bytes remain unchanged.
    if(!absent(e.active)){
        struct stat old{};require(!lstat(e.active.c_str(),&old),"Cannot inspect active folder.");
        if(old.st_uid!=geteuid()){
            require(geteuid()==0 && present && S_ISDIR(old.st_mode) && !(old.st_mode&0022),"Unsafe existing installation folder preserved.");
            for(const auto& item:fs::directory_iterator(e.active)){
                if(item.path().filename()==".DS_Store"){fileBytes(item.path());continue;} // Preserve Finder metadata.
                require(item.path().filename()==target,"Shared installation folder preserved. Use manual installation.");
            }
            secure(e.active.parent_path(),0755);environment(e,true);FD folder=directory(e.active);noACL(folder.value);
            require(!fchown(folder.value,0,0) && !fchmod(folder.value,0755),"Cannot protect the verified legacy installation folder.");
        }
    }
    secure(e.active,0755);secure(e.backups,0755);
    FD active=directory(e.active);
    FD lock(openat(active.value,".fstr-frontend-lock-v1",O_RDWR|O_CREAT|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK,0600));struct stat l{};
    require(lock.value>=0 && !fstat(lock.value,&l) && S_ISREG(l.st_mode) && l.st_nlink==1 && l.st_uid==geteuid() && (l.st_mode&0777)==0600 && l.st_size==0 && !flock(lock.value,LOCK_EX|LOCK_NB),"Another installer is running or the lock is unsafe.");
    std::vector<fs::path> records;for(const auto& p:fs::directory_iterator(e.backups)){noLinks(p.path());require(p.is_directory(),"Unknown backup entry preserved.");records.push_back(p.path());}std::sort(records.rbegin(),records.rend());
    // A missing completion marker means interrupted work. No new transaction
    // may conceal it; Restore inspects the authentic pre-mutation receipt.
    fs::path pending;for(const auto& p:records)if(!marked(p)){require(pending.empty(),"Multiple incomplete transactions need manual inspection.");pending=p;}
    if(restore){
        fs::path tx=pending;
        if(tx.empty())for(const auto& p:records)if(!absent(p/"installed") && absent(p/"restored")){tx=p;break;}
        require(!tx.empty(),"No previous installation is available.");FD backup=directory(tx);SnapshotReceipt saved{};checked(readSnapshotReceipt(backup.value,saved),"Recovery receipt is missing or unsafe. Both trees retained.");Verify context{&e,tx,saved};
        if(!saved.previousPresent){auto r=inspectFreshVerified({active.value,target,backup.value,retained,backup.value},Verify::fresh,&context);require(r.error==0,"Fresh installation requires manual recovery; files retained.");if(pending==tx)mark(backup.value,r.state==TransactionState::Installed?"installed":"restored");return r.state==TransactionState::Installed?"No previous version exists. Fresh installation verified; no files removed.":"Interrupted fresh installation was not published. Staged files retained; you can try Install again.";}
        PreparedExchange expected{saved.previous.root,saved.candidate.root};auto r=recoverVerified({active.value,target,backup.value,retained,backup.value},expected,Verify::exchange,&context,RecoveryAction::Restore);require(!r.error && (r.state==TransactionState::Restored || r.state==TransactionState::Unchanged),"Restore refused: files changed or recovery is incomplete. Both versions retained.");mark(backup.value,"restored");return "Previous version restored. Both versions are retained in Backups.";
    }
    require(pending.empty(),"An interrupted installation was found. Choose Restore before another installation.");
    if(present && identity(e.active/target)==payload::buildId){verifyCandidate(e.active/target);return "This version is already installed.";}
    CFUUIDRef uuid=CFUUIDCreate(nullptr);NSString* identifier=CFBridgingRelease(CFUUIDCreateString(nullptr,uuid));CFRelease(uuid);
    auto stamp=std::to_string(static_cast<unsigned long long>([NSDate date].timeIntervalSince1970*1000000));fs::path tx=e.backups/(stamp+"-"+[identifier UTF8String]);secure(tx,0700);FD backup=directory(tx);
    stage(tx/retained);verifyCandidate(tx/retained);SnapshotReceipt saved{};saved.previousPresent=present;checked(snapshotPayload(backup.value,retained,saved.candidate),"Cannot snapshot candidate.");Verify context{&e,tx,saved};
    TransactionResult result{};
    if(present){checked(snapshotPayload(active.value,target,context.saved.previous),"Cannot snapshot previous version.");auto receipt=prepareSnapshotReceipt(backup.value,context.saved);require(receipt.state==JournalState::Durable && !receipt.error,"Cannot prepare recovery receipt; staged files retained.");result=replaceVerified({active.value,target,backup.value,retained,backup.value},{context.saved.previous.root,context.saved.candidate.root},Verify::exchange,&context);}
    else result=publishFreshVerified({active.value,target,backup.value,retained,backup.value},saved.candidate,Verify::fresh,&context);
    require(result.state==TransactionState::Installed && !result.error,"Installation incomplete. Files retained; choose Restore to inspect/recover.");
    mark(backup.value,"installed");return present?"Installed. Previous version saved in Backups.":"Installed. No previous version was present.";
    }
}
}
