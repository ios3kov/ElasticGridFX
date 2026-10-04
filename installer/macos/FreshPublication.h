#pragma once
#include "SnapshotReceipt.h"
#include "ExchangeCoordinator.h"

namespace fstr::installer {
struct FreshLocations {
    int activeParent;
    const char* activeName;
    int stagingParent;
    const char* stagingName;
    int transactionDirectory;
};
// Native frontend recollects stopped hosts, duplicate scan and fixed root safety
// while the destination lock is held. Core independently checks the full tree.
// The trusted frontend must durably flush staged files/directories beforehand;
// snapshot verification alone does not establish staging durability.
using FreshRevalidate=int (*)(void*,bool published) noexcept;
TransactionResult publishFreshVerified(FreshLocations,const PayloadSnapshot&,
                                      FreshRevalidate,void*);
// Read-only classification against the protected PRE-mutation receipt. No
// uninstall/cleanup is inferred from a missing previous version.
TransactionResult inspectFreshVerified(FreshLocations,FreshRevalidate,void*);
}
