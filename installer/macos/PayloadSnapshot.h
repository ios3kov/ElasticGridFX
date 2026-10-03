#pragma once
#include "AtomicExchange.h"
#include <array>
#include <cstdint>

namespace fstr::installer {
struct PayloadSnapshot {
    Identity root;
    std::array<unsigned char,32> sha256;
    uint64_t entries;
    uint64_t fileBytes;
};
// Read-only snapshot v1: sorted relative names, type, mode, uid/gid, flags,
// extended attributes and regular file bytes. Rejects links/special files,
// multiple hardlinks, any extended ACL, mount crossings and excessive payloads.
// Does not authenticate publisher or enforce fixed installer roots.
int snapshotPayload(int parent,const char* name,PayloadSnapshot& output) noexcept;
}
