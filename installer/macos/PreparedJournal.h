#pragma once
#include "AtomicExchange.h"

namespace fstr::installer {
struct PreparedExchange { Identity previous; Identity candidate; };
enum class JournalState { NotCreated, Durable, CreatedNeedsRecovery };
struct JournalResult { JournalState state; int error; };
// Fixed filename in a freshly allocated, locked, verified transaction directory.
// This immutable record precedes exchange; it never supplies destination paths.
JournalResult prepareJournal(int transactionDirectory, PreparedExchange record);
int readPreparedJournal(int transactionDirectory, PreparedExchange& record);
enum class RecoveryPosition { Unexchanged, Exchanged, Unknown };
// Identity classification only. Caller must also revalidate full payload bytes,
// trusted package metadata, fixed paths and stopped hosts before any mutation.
RecoveryPosition locateExchange(PreparedExchange record, Identity active, Identity backup);
}
