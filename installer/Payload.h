#pragma once
#include <cstddef>
namespace fstr::payload {
struct File { const char* name; const unsigned char* bytes; std::size_t size; const char* sha256; bool executable; };
extern const File files[];
extern const std::size_t count;
extern const char version[], buildId[], commit[], platform[];
}
