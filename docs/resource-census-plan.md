# Corner-drag resource diagnosis — Dev169

Rules8.0.0, Critical native / Development. AI_ENTRYPOINT first; controlled state,
documented API contracts, memory/resource lifetime and the debugging stop criterion
apply. User authorizes the local diagnosis/fix/verification block, AE closure and
FSTR replacement. No push, release, foreign plug-in changes or other-app closure.

## Question and acceptance

Dev167 failed to freeze native Point dragging and coincided with a confirmed
system memory-pressure event. Its allocation/cache cause remains UNKNOWN. Do not
repeat its DRAW flags experiment. Dev169 is a passive, nondefault diagnostic on
ordinary Dev168 behavior, not a repair or a release candidate. No speed benchmark.

Compare process physical footprint with retained parameter snapshots, copied
loupe pixels, borrowed SmartRender worlds and actual checkin results. A positive
signal must identify which category grows; stable FSTR counters do not prove an
AE cache cause or exclude transient core allocations. Borrowed world scope is
not allocation ownership. Only then design a causal repair; separately validate
the approved frozen-image/live-grid/live-loupe interaction and release commit.

## Instrumentation contract

Feature `resource-census-probe` selects Dev169. Default remains ordinary Dev168.
No render-quality setter, additional frame request, event invalidation, saved
parameter change, host call on workers or retained host pointer is added.
Workers only update scalar atomics. Existing UI callbacks append at most64 records
per UI thread to a fresh0600 private temp file; unchanged samples are omitted and
nonforced records are separated by at least200ms. Log exhaustion is PARTIAL.
Unix millisecond timestamps bind the census and external observer.

Schema1 `resources` rows: snapshot, loupe copy, borrowed input, borrowed output.
Each row: created tokens, dropped tokens, live bytes, peak live bytes, largest
token, last width, height, bpc. Snapshot bytes estimate struct plus grid vector
capacity, not allocator overhead. A cloned snapshot starts another estimated
token. Loupe bytes describe the retained copied pixel vector, limited by the
existing64MiB bound. Input/output bytes are row stride times height while in
callback scope; token drop does not prove that AE freed or evicted those pixels.
`checkins`: pixel success/error, loupe success/error, async polls/nonnull receipts.
Individual relaxed atomics are not a coherent concurrent snapshot; interpret
balances at idle boundaries, never infer a leak from a single racing record.
AE owns pre_render_data and its deletion callback; the census cannot delete it.

## Native execution gate

1. Read current process inventory without starting AE. No concurrent AE validation.
   Inventory foreign test plug-ins and preserve exact files/hashes. If an isolated
   FSTR-only baseline requires temporarily excluding them, obtain that additional
   authorization with a concrete restore plan; none is implied by FSTR installation.
2. Build/package exact clean Dev169, preserve ordinary Dev168 and prior artifacts.
   Validate identity, PiPL, exports, signature, manifest and installer before use.
3. Use a saved disposable copy of the retained32bpc Surface/Full/Final one-effect
   scene. No original project writes, RAM Preview, long drag or adaptive-quality
   substitution. Bind loaded path/UUID/BID and owned PID before a gesture.
4. Check global memory headroom first. External guard validates ordinary AE path
   and process birth. Idle baseline must be<=1GiB; absolute limit1.5GiB or growth
   +256MiB. Gate failure means NO GESTURE, not a relaxed threshold.
5. Arm the guard before any action. One<=1-second corner gesture only after READY;
   maximum observation45seconds. On a limit the owned AE is stopped, sampled for
   one second and killed; it is also killed at the deadline. The saved fixture
   makes this authorized disposal safe. Recheck process identity before each
   signal and verify exit afterward. No Photoshop/Telegram or other-app signals.
6. The guard is best effort, not a guarantee against system stalls, allocations
   between200ms polls or OS/permission failure. A failed identity/read terminates
   observation without signaling an unverified process. Stop interaction at once.
   Never leave a human gesture running after observer expiry. Preserve any guard
   exception, check host state without launching it, and do not retry blindly.
7. Retain private logs/sample/project hashes, classify counters/footprint/checkins
   and stop on a limit/error. No raw project/log upload. A subsequent test needs a
   new discriminating hypothesis, not repetition of the failed Dev167 experiment.

## Offline evidence

Ordinary Rust120, probe Rust122, strict probe Clippy,16 host contracts and6 external
guard tests PASS. Initial probe test compilation failed for a missing cfg field;
repaired and rerun, original failure log retained. Guard tests cover independent
limits, PID/path/birth mismatch, target system struct layout, sample/log failure
cleanup and armed versus observe-only deadline. Tests do not prove native memory
growth, host frame validity or recovery. C++ renderer unchanged; no new C++ verdict.
Rust/C++ are outside the general static scanner's supported scope: NOT ASSESSED
by that scanner; manual ownership/worker/error-path review is recorded above.
Native observer execution and exact Dev169 host behavior: NOT RUN.

Private local evidence: outputs/resource-census-*.log,
outputs/resource-guard-tests.log, outputs/resource-census-foreign-inventory.json.
