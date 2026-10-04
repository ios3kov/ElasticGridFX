# Commercial licensing integration — 2026-10-04

## Authority and current scope

User authorized implementing U11 and selected **aescripts and Plugin Play**.
Local-only work; Mac first, Windows deferred. No account registration, vendor
messages, uploads, agreements, product publication or customer-key use authorized
by this research. Existing installer/U0–U10 obligations remain open.
This document is a research/design checkpoint, not working activation.

## Native entry: supported API found

SDK25.6 `Examples/Headers/AE_EffectSuites.h:632–642` defines
PF_EffectUISuite1 / PF_SetOptionsButtonName(effect_ref, nameZ).
The [SDK utility guide](https://ae-plugins.docsforadobe.dev/effect-details/useful-utility-functions/)
requires calling it during PF_Cmd_PARAM_SETUP. The name is bounded to 31 A_char
storage; use a short ASCII caption (`Activate...` / `License...`) or verify native
encoding before using an ellipsis. No custom parameter row/header overlay needed.

The pinned after-effects0.4.0 already exposes
`ae::pf::suites::EffectUISuite::set_options_button_name`; its wrapper checks length
and acquires/releases the suite. IDoDialog must be declared consistently in PiPL
(build.rs) and GlobalSetup. The pipl0.1.1 generator sets cfg(does_dialog), which
adds AdobePluginInstance::do_dialog and its dispatcher. Our Instance currently
has no such method; merely enabling the flag is incomplete.
PF_Cmd_DO_DIALOG is the UI entry, not a render callback. Avoid SEND_DO_DIALOG
unless initial activation prompting is explicitly approved.

SDK/code support is VERIFIED. Placement, click behavior, narrow-panel layout
and caption refresh following activation remain NOT RUN in AE. The documented
caption setter is ParamSetup-only: do not assume it can safely be called from
render, an arbitrary event or activation completion to change a live caption.
Investigate rebuilding UI through a supported lifecycle; otherwise propose a
stable `License...` entry rather than silently violate callback contracts.

## Marketplace evidence and unresolved dependencies

[aescripts author page](https://aescripts.com/faq/article/view/faq/become-an-author/)
confirms integrated licensing and distribution/manager support. It does not
publish the native SDK ABI, product identity or sample integration contract.
An author-provided SDK, versioned examples, permitted packaging instructions and
test entitlements are needed before implementing its adapter.

[Plugin Play pricing](https://www.pluginplay.app/pricing) describes subscription
access and a developer FAQ. The public page text inspected does not supply a
native licensing SDK or API contract. Its
[legacy account page](https://app.pluginplay.app/account) describes an access key
for tools; this does not prove a compatible native AE SDK, current protocol or
that an aescripts key can activate a Plugin Play purchase.
User confirmed on 2026-10-04 that author access is not available yet.
No vendor SDK was found in tracked project sources or local output inventory.
Do not infer a protocol by inspecting third-party proprietary licensing code.

Before integrating each provider obtain:

1. Native macOS arm64 / Windows x64 SDK or documented API, version and examples.
2. Product/application identifier and vendor-issued sandbox/test licenses.
3. Permitted dual-market distribution: one binary vs separate channel builds;
   whether either framework supports keys sold by the other marketplace.
4. Offline entitlement contract, subscription expiration/revocation semantics,
   signed local state verification and AE restart behavior.
5. Activation limits, deactivation/transfer, lost-device recovery and render-node
   policy, including aerender/MFR and headless UI restrictions.
6. Trial/unlicensed output rules, cancellation, network timeouts, logging/privacy,
   package dependencies and author obligations.

These are integration dependencies, not reasons to replace either provider with
our own guessed key generator. User keys, passwords and signing secrets must
not be requested in chat or stored in project streams/logs.

## Integration sequence and acceptance

L1: Finish bounded SDK/header research (this checkpoint); native prototype next.
L2: Obtain both provider contracts; reconcile dual-market/offline policies with
user before dependent implementation. No provider adapter or runtime PASS yet.
L3: Use existing architecture: UI/main-thread activation and provider integration
outside rendering; trusted cached entitlement consumed by rendering only under
approved failure policy. Keys belong to provider-owned protected activation
storage, never Grid Positions/sequence data or shared project files.
L4: Implement complete native UI, provider validation and failure/retry/cancel
flows. Avoid a public Activate button with an inert or fake backend. About and
support remain reachable independently of licensing failure.
L5: Test genuine vendor-issued valid/invalid keys, restart, offline operation,
transfer, unavailable service, expiry/revocation per approved policy and exact
saved-project/output/performance regression; no network or dialogs from MFR or
headless rendering. Mac acceptance first; Windows remains NOT RUN.
L6: Integrate installer/update retention; reconcile U11 with U8/U10 before a
commercial Release gate. Development proof does not authorize publication.

Installed ordinary Mac Dev48 remains unchanged by this research. It has no
commercial activation yet. No activated/trial/revoked status is fabricated.

## Independent Mac UI implementation — 2026-10-04

User separately authorized building the independent window before vendor SDKs.
Mac now declares IDoDialog consistently in PiPL and GlobalSetup, sets the native
`License...` caption only in ParamsSetup and handles Instance::do_dialog. The
caption is deliberately stable while the provider is absent; no fake Activate
button or input that discards keys. Native AppKit window states that licensing
is not connected and the development build works without activation. About
shows the Cargo product version; Support opens the public project issues page
only on a user click, with a fallback URL if the browser cannot be opened.
Close performs no project/parameter/license write. About returns to License.
The native bridge requires main thread/NSApp, contains Objective-C exceptions,
and the Rust entry skips render-engine UI. No new third-party dependency, HTTP
request, credential storage or render callback work. Windows dialog remains
deferred and does not gain the Mac-only flag. Native host validation follows
compilation; neither source support nor a build is host acceptance.


## Exact Mac UI acceptance checkpoint

Ordinary Dev52 source740dbaa, build EGFX-01242dd7b423e5aa1384abae installed and
its loaded UUID/path verified in AE25.6. Native License... appears next to global
Reset. Clicking it opens the independent window; About shows0.9.4 and correct
copyright, OK returns to License, Close returns to AE. Support click was processed
without an openURL-failure alert; browser destination verification remains
incomplete due to browser policy-loader failure and an unrelated later Chrome
page observation. Do not report full Support destination verification PASS.
AE marks the project dirty after DO_DIALOG; no project stream mutation was
introduced. Recorded keys/density/spacing and exact frame match Dev48 before and
after UI and after saving/reopening a separate test copy. Rust86, Clippy with
warnings denied, host contract15, About/build identity23 and signed payload
checks PASS. Evidence: outputs/update-094-license-window-mac/verification.json.
This closes the bounded Mac independent-window implementation/checks; provider
activation, caption switching, subscription/offline/transfer policy and Windows
remain unimplemented/unverified. Installer obligations are not closed by U11.


## Windows UI source implementation — superseding deferral

User explicitly resumed Windows feature parity. Shared header registration,
PiPL/GlobalSetup IDoDialog and dialog dispatcher now include Windows. Native
Win32 DialogBoxIndirectParamW provides License/About/Support/Close, with Unicode
controls, keyboard close/cancel, owned-thread/window checks and exception
containment. Provider SDK integration is still blocked; native Windows MSVC
compile/runtime remains NOT RUN here. Older statements about a Mac-only flag or
Windows deferral above describe their historical checkpoints. See
windows-update-parity-2026-10-04.md; no speed tests are requested by latest user.
