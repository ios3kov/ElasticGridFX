# Owned corner transaction —2026-10-07

Release0.9.4 still REQUIRES frozen image/live grid/live loupe during Surface and
Perspective corner drags, one saved Point commit on release, existing animation
and Undo, and selected-quality output afterward. Dev167 DRAW/AdjustCursor output
flags failed in the native Point loop; do not repeat that experiment. Dev170's
NO_ECW_UI priority experiment never delivered an identity-proven gesture and is
not product UI. The original hang cause remains UNKNOWN.

## One discriminating prerequisite

SDK25.6 AE_Effect.h PF_ParamUIFlags permits changing DISABLED during UI events.
Hypothesis: disabling standard Point controls while a custom corner is hovered
lets our existing custom CLICK/DRAG handle the gesture, instead of AE's native
Point loop. This effect on native hit priority is UNKNOWN until measured.
Dev171 / nondefault `corner-hit-probe` changes only that UI policy plus the
existing bounded passive route/resource journal. It keeps all four coordinate
rows, Point types/IDs and Timeline keys. No quality, sampler, saved Point,
render-worker override, frame request, native input hook or migration is added.

Claim scope is stable effect stream ID/window/time/time scale/mode. A pressed
button cannot acquire a different claim; leaving the corner, effect-panel cursor,
unknown context, close/deactivate/mouse-exit cancels the claim. DRAW synchronization
respects only a matching claim; Fit Layer is not disabled by a hover. New/closed
contexts retain no pointer and perform no parameter read. Ordinary builds have
the same UI policy as Dev168; this remains a disposable prerequisite probe.

## Bounded native procedure

Use a NEW320x240/32bpc one-effect generated Surface scene, Full/Final, selected
layer/effect, at most one12px/<=1second gesture. This answers input ownership only,
not rendering performance or original-scene stability. Unlike prior1920x1080
observations, it reduces irrelevant background-frame allocation without changing
AE settings, quality, memory caps or the renderer. Leave idle-cache preference
and other settings unchanged; no user-project opening or writes.

Read main-project UI BEFORE first DoScript; exact PID/path/birth and fresh UI
receipt required. Preserve/exclude only the same9 authorized Hot Loader test
bundles and restore them exactly after the owned process exits. Loaded FSTR
path/UUID/BID must match packaged Dev171; no AEHL image. Memory guard BEFORE scene
creation: loading/idle<=1.25GiB, total<=1.5GiB, armed growth<=256MiB, max45seconds.
No gesture until ARMED plus fresh screenshot/unchanged window geometry; only the
already-authorized helper in its composition region. Immediate Point AND layer
position readback is required; absence of a custom event without actual corner
delta is NOT RUN/UNKNOWN, not a priority failure. SIGTERM is prohibited after the
confirmed test-runner OnForceQuit crash path; exact-owned disposal uses the guard.

PASS requires custom CLICK/first DRAG/final DRAG, intended corner delta with no
layer-position change, visible coordinate rows and restored UI flags after exit.
Native supervision instead of delivered custom drag disproves the hypothesis.
Any identity, UI, guard or point-read failure stops this trial; no blind retry.
No diagnostic promotion, original-scene drag/RAM Preview, new-source push/release.

## After ownership PASS only

Implement tentative raw16.16 corner coordinates outside canonical streams;
only viewer geometry/loupe reads them. CLICK has no saved value writes. Owned
DRAG requests UI-only invalidation while canonical parameters remain identical;
last DRAG writes the one final Point with normal CHANGED_VALUE/host Undo and
selected-quality update. Context/time/owner/mode changes cancel, not commit.
Renderer workers never consult transient UI state or reuse changed-state outputs.
Review no-motion/repress/cancel/error/invalid-quad/text-projection paths and verify
both modes, old keys and Undo in a clean ordinary exact candidate on both OSes.
