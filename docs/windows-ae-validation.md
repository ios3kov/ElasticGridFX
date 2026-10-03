# Windows AE validation

This packet validates an **already installed** Windows candidate. It never copies,
moves, replaces or deletes a plug-in.

## Prerequisites

- Use the exact `FSTR Stretch.aex` and `FSTR Stretch.windows-artifact.json`
  from the same GitHub Actions artifact.
- Place that candidate in the chosen After Effects installation only by an
  explicit manual/test-machine action.
- Start the exact After Effects build to test and leave one empty, unsaved project open.
- Close other After Effects instances.

## Automated validation

From the extracted validation artifact root:

```powershell
python tools/windows_ae_validation.py `
  --candidate "dist\windows\FSTR Stretch.aex" `
  --manifest "dist\windows\FSTR Stretch.windows-artifact.json" `
  --installed-aex "C:\path\to\the\installed\FSTR Stretch.aex" `
  --afterfx "C:\Program Files\Adobe\Adobe After Effects 2025\Support Files\AfterFX.exe"
```

The runner fails closed unless:

- the candidate and installed file match the manifest SHA-256;
- the installed binary contains the exact Build ID from the manifest;
- exactly one selected After Effects process is running;
- exactly one loaded `.aex` with the ElasticGridFX match name exists and it is
  the selected installed candidate;
- arm/disarm ownership guards confirm a disposable empty project;
- the render smoke produces and passes the existing pixel checks;
- save/reopen preserves the tested project state and produces a frame.

The runner retains its unique workspace and evidence. A successful automated run
returns **BLOCKED**, not final PASS, because the following checks still require
separate validation:

1. cold-start first-application / deferred auto-binding;
2. interactive guide drag and cursor behavior;
3. real UI Undo/Redo;
4. MFR, Render Queue and `aerender` on the exact Windows artifact.

## Evidence boundary

Windows compile, PE/export and PiPL checks are build/static evidence. They do not
become After Effects runtime evidence until this runner (and the remaining manual
checks) are executed against the exact candidate.
