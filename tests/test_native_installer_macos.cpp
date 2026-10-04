#include "NativeInstaller.h"
#include "PayloadSnapshot.h"
#include <cassert>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <fcntl.h>
#include <unistd.h>
#include <sys/file.h>
namespace fs=std::filesystem;
using namespace fstr::installer;
PayloadSnapshot snap(fs::path p){int fd=open(p.parent_path().c_str(),O_RDONLY|O_DIRECTORY);assert(fd>=0);PayloadSnapshot s{};assert(!snapshotPayload(fd,p.filename().c_str(),s));close(fd);return s;}
template<class F> void refuses(F f){bool failed=false;try{f();}catch(const std::exception&){failed=true;}assert(failed);}
int main(int argc,char* argv[]){
    assert(argc==2);fs::path previous=fs::absolute(argv[1]);
    char temp[]="/private/tmp/egfx-native-frontend-XXXXXX";assert(mkdtemp(temp));fs::path root(temp);
    auto make=[&](const char* name){auto p=root/name;fs::create_directories(p/"scan");return InstallEnvironment{p/"scan/FSTR FX",p/"Backups",{p/"scan"},[]{}};};
    {
        auto e=make("fresh");auto result=runNative(e,false);assert(result.starts_with("Installed."));
        assert(runNative(e,false)=="This version is already installed.");auto s=snap(e.active/"FSTR Stretch.plugin");
        assert(runNative(e,true).starts_with("No previous version"));assert(snap(e.active/"FSTR Stretch.plugin").sha256==s.sha256);
    }
    {
        auto e=make("update");fs::create_directory(e.active);fs::copy(previous,e.active/"FSTR Stretch.plugin",fs::copy_options::recursive);
        auto old=snap(e.active/"FSTR Stretch.plugin");assert(runNative(e,false).starts_with("Installed."));auto installed=snap(e.active/"FSTR Stretch.plugin");assert(old.sha256!=installed.sha256);
        assert(runNative(e,true).starts_with("Previous version restored"));assert(snap(e.active/"FSTR Stretch.plugin").sha256==old.sha256);
        assert(runNative(e,false).starts_with("Installed."));
        std::ofstream(e.active/"FSTR Stretch.plugin/Contents/PkgInfo",std::ios::app)<<"tamper";
        refuses([&]{runNative(e,true);});
    }
    {
        auto e=make("blocked-host");e.checkHosts=[]{throw std::runtime_error("host running");};refuses([&]{runNative(e,false);});assert(!fs::exists(e.active));
    }
    {
        auto e=make("duplicate");fs::copy(previous,e.scanRoots[0]/"ElasticGrid.plugin",fs::copy_options::recursive);refuses([&]{runNative(e,false);});assert(!fs::exists(e.active));
    }
    {
        auto e=make("link");fs::create_directory(root/"outside");fs::create_directory_symlink(root/"outside",e.active);refuses([&]{runNative(e,false);});assert(fs::is_empty(root/"outside"));
    }
    {
        auto e=make("pending");fs::create_directories(e.backups/"interrupted");refuses([&]{runNative(e,false);});assert(!fs::exists(e.active/"FSTR Stretch.plugin"));
    }
    {
        auto e=make("interrupted-post-publication");fs::create_directory(e.active);fs::copy(previous,e.active/"FSTR Stretch.plugin",fs::copy_options::recursive);
        auto old=snap(e.active/"FSTR Stretch.plugin");assert(runNative(e,false).starts_with("Installed."));
        auto tx=fs::directory_iterator(e.backups)->path();assert(fs::remove(tx/"installed"));refuses([&]{runNative(e,false);});
        assert(runNative(e,true).starts_with("Previous version restored"));assert(snap(e.active/"FSTR Stretch.plugin").sha256==old.sha256);
    }
    {
        auto e=make("lock");assert(runNative(e,false).starts_with("Installed."));int fd=open((e.active/".fstr-frontend-lock-v1").c_str(),O_RDWR);assert(fd>=0);assert(!flock(fd,LOCK_EX|LOCK_NB));
        refuses([&]{runNative(e,false);});assert(!flock(fd,LOCK_UN));close(fd);
    }
    std::cout<<"Fresh, repeat, update, restore, tamper, host, duplicate, link, pending, interrupted, lock PASS\n";
    // Keep the disposable directory as evidence; never clean user/system roots.
    std::cout<<root<<'\n';
}
