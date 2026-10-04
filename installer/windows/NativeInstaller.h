#pragma once
#define WIN32_LEAN_AND_MEAN
#define NOMINMAX
#ifndef UNICODE
#define UNICODE
#endif
#ifndef _UNICODE
#define _UNICODE
#endif
#include <windows.h>
#include <objbase.h>
#include <filesystem>
#include <functional>
#include <string>
#include <vector>
namespace fstr::installer {
struct InstallEnvironment {
    std::filesystem::path active, backups;
    std::vector<std::filesystem::path> scanRoots;
    std::function<void()> checkHosts;
};
InstallEnvironment systemEnvironment();
std::wstring runNative(const InstallEnvironment&,bool restore);
bool elevated();
}
