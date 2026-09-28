# ElasticGrid Build / Artifact Identity

Every testable native artifact carries a generated identity derived from Git at build time.

Identity chain:

`Git commit + clean/dirty -> Build ID -> dylib/bundle -> package SHA-256 -> installed identity -> loaded identity`

Generated fields:
- version;
- full Git commit;
- Git state: clean / dirty / unknown;
- Build ID: `elasticgrid-v<version>-<12-char-commit>-<state>`;
- artifact type: `ae-native-plugin`.

Evidence surfaces:
- `PF_Cmd_ABOUT`;
- `Contents/Resources/BuildIdentity.txt`;
- custom Info.plist keys;
- implementation hot-reload exports;
- first line of `/tmp/elasticgrid-fx.log`;
- shell reload log/status;
- package `.sha256` generated after the final ZIP is created.

CI requires a clean Git state and verifies that the packaged commit equals `GITHUB_SHA`.

Dirty builds remain available for internal development but are not valid manual-test/release evidence.


## CI source-state correction — 2026-09-28

Baseline evidence: Hot Loader Shell CI #77 reached bundle assembly but rejected the candidate because the build script reported `git_state=dirty` even though the GitHub checkout was the intended clean source. The gate therefore remained **FAIL**, as required.

The source-state contract is now split deliberately:

- CI first proves the checkout is clean before any build output exists;
- CI passes the verified commit/state into the build as authoritative build metadata;
- build.rs refuses a commit override that disagrees with Git HEAD;
- local/internal builds without CI metadata still auto-detect clean/dirty state;
- target triple and Rust toolchain are now included in the build record.

This avoids confusing build-generated files with source dirtiness while preserving fail-closed identity for release candidates.
