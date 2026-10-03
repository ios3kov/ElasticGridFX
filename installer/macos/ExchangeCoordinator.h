#pragma once
#include "PreparedJournal.h"

namespace fstr::installer {
struct ExchangeLocations {
    int activeParent;
    const char* activeName;
    int backupParent;
    const char* backupName;
    int transactionDirectory;
};
// Trusted frontend code, never supplied by a package. Recollects stopped hosts,
// complete scan, fixed-root safety and full old/new payload snapshots under lock.
// Zero means verified; nonzero refuses mutation. No exceptions may cross this API.
using Revalidate = int (*)(void*, RecoveryPosition) noexcept;
enum class TransactionState { Refused, Prepared, Installed, NeedsRecovery, Unchanged, Restored };
struct TransactionResult { TransactionState state; int error; };
enum class RecoveryAction { Inspect, Restore };
// Internal replacement only: frontend stages verified payload outside Adobe and
// retains authentic expected record/snapshots. Fresh installation is separate.
TransactionResult replaceVerified(ExchangeLocations paths, PreparedExchange expected,
                                  Revalidate verifier, void* context);
TransactionResult recoverVerified(ExchangeLocations paths, PreparedExchange expected,
                                  Revalidate verifier, void* context, RecoveryAction action);
}
