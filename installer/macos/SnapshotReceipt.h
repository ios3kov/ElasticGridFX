#pragma once
#include "PayloadSnapshot.h"
#include "PreparedJournal.h"

namespace fstr::installer {
// Protected, immutable pre-mutation expectations. Never contains destinations.
// The frontend must resolve/authenticate fixed roots before passing this FD.
struct SnapshotReceipt {
    bool previousPresent;
    PayloadSnapshot previous;
    PayloadSnapshot candidate;
};
JournalResult prepareSnapshotReceipt(int transactionDirectory, const SnapshotReceipt&);
int readSnapshotReceipt(int transactionDirectory, SnapshotReceipt&);
bool matchesSnapshot(const PayloadSnapshot&, const PayloadSnapshot&) noexcept;
}
