#pragma once
#include <sys/types.h>

namespace fstr::installer {
struct Identity { dev_t device; ino_t inode; };
enum class ExchangeState { NotExchanged, Exchanged, ExchangedNeedsRecovery };
struct ExchangeResult { ExchangeState state; int error; };

// Internal native primitive, not an installer/CLI or authorization mechanism.
// Caller owns verified directory FDs, a transaction lock, payload verification
// and a durable PREPARED journal. Names are single components, never package paths.
ExchangeResult exchangeDirectories(int parentA, const char* nameA, Identity expectedA,
                                   int parentB, const char* nameB, Identity expectedB);
}
