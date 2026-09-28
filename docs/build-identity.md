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
