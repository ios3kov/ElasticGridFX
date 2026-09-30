# AE project roundtrip safety — 2026-09-28

## Reproduced defect

The original tests/ae_project_roundtrip.jsx guard returns when a saved, occupied or dirty project is open. JavaScript still executes finally after that return. Its unconditional cleanup closed the current project with DO_NOT_SAVE_CHANGES, created another project and removed fixed /tmp files. Thus the guard itself did not protect user work.

A Node VM executes the actual JSX with host operations replaced by counters. On the old source it fails: saved project must not close user project; expected 0 close calls, actual 1. No user project or real After Effects process was used for reproduction.

## Fix and required behavior

- Establish ownership only after confirming an empty, unsaved, non-dirty project and reserving a fresh test workspace.
- Unknown/failing dirty-state queries refuse execution instead of assuming safety.
- Refusal cannot close/replace projects, change bit depth or remove files.
- Cleanup can close only the tracked test project while it remains current; a changed project is left untouched.
- Failed close cannot proceed to newProject. Failure to restore an empty project is not PASS.
- Use a unique Folder.temp/ElasticGridFX-roundtrip-* workspace. Never remove fixed old paths. Retain the AEP/PNG as evidence.
- Bound the AppleScript wait to 120 seconds. Timeout is not PASS and does not authorize killing AE or closing its project; the host may still be executing.

## Checks

PASS: node tests/test_ae_project_safety.js — 11 control-flow cases: saved/occupied/dirty/absent/unknown state, throwing host getter, workspace collision, unexpected payload, fixture failure, failed close and changed current project. Bash syntax check PASS. Final-validation CI runs and retains this test separately from C++ tests.

BLOCKED: real After Effects save/open/close/render and object-identity behavior on the target Mac. The VM test proves the guard/control-flow regression only; it is not host integration or release approval. The flat-color roundtrip frame still checks file production, not correct deformation; it must not be used to close the image-deformation acceptance gate.

## API references

- https://ae-scripting.docsforadobe.dev/general/application/ (app.open/newProject, exitCode)
- https://ae-scripting.docsforadobe.dev/general/project/ (Project.close)
- https://extendscript.docsforadobe.dev/file-system-access/folder-object/ (Folder.temp, exists, create)

The test deliberately refuses when ownership cannot be proven. Actual project testing must use an authorized isolated AE environment, not the user's unsaved work.
