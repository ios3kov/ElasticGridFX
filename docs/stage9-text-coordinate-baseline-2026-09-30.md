# Native text coordinate baseline — Stage 9 OPEN

Run: b7c354d8976942c1ad5d9088d9222f30. AE 25.6.0 arm64, PID 36457.
Installed/live Build ID EGFX-25e03a7ae1a9304311095c8e, candidate
1eed79aad052f144668c2ebed9426a7934f35f6b. Live image UUID/path and installed
payload identity PASS (live-identity.json). This is not a renderer PASS.

## Isolation

Previously opened project was saved (no dirty marker). Opened a new empty
project through AE UI, then ran the guarded fixture. No prior file overwritten.
New outputs/text-plane-3d-restore-20260930/text-plane.aep contains the owned
640x480, single native text fixture with unique run marker. Unlike the old path,
this scene passes structural ownership checks. Scripts return 0.

## Observed failure

At 50% viewer zoom, enabling threeDLayer alone moves the grid by approximately
160x100 viewer pixels, equivalent to layer Position [320,200,0]. Text is fixed.
2D grid spans the composition canvas. 3D grid starts at the text origin and
extends beyond the composition. Screenshots inspected in the conversation.
Native 3D text overlay baseline: **FAIL**, reproduced on the current candidate.

Host sourcePointToComp probe (no text animators):

| Input layer point | 2D / 3D zero rotation | 3D Y rotation 30 degrees |
|---|---|---|
| 0,0 | 320,200 | 320,200 |
| 640,0 | 960,200 | 1186.025390625,177.5 |
| 640,480 | 960,680 | 1186.025390625,927.5 |
| 0,480 | 320,680 | 320,680 |

Probe restores rotation and 3D state in finally. Output coordinates.txt and
transport logs are retained beside the project. No user composition was probed.
The sourcePointToComp API has a documented first-character limitation; this
simple text fixture cannot establish animated-character or per-character-3D behavior.

## Interpretation and next implementation gate

The current ui_projection forward matrix consumes native layer coordinates,
while the grid domain derives from the effect canvas (in_data width/height).
The host probe confirms that native layer projection adds Position even in the
zero-rotation case. The observed offset is consistent with applying it to an
already transformed text effect canvas. This does not establish a general
canvas-to-layer inverse for continuously rasterized text.

Do not apply a fixed Position subtraction: rotation/camera require a projective
mapping, and rendering and inverse picking must share its semantics. Required
next evidence: identify the host-supported effect-input-to-layer coordinate
mapping, then test native text, raster control, rotation, camera, parent and
neutral identity. Do not call general AEGP Layer APIs from MFR/render callbacks
without a documented threading contract. UI-only state cannot drive renders.

Installed artifact remains unchanged. No fix or Stage 9 acceptance claimed.
