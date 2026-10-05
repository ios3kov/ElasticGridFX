# U8 native installer continuation — 2026-10-04

Authority: user requested continuing with convenient Mac/Windows installation
and preservation of the previous version after accepting Windows Dev56. Rules
8.0.0 retained. Development/validation only; no main/merge/release authority.
Existing plugin bytes, animation and completed performance work remain unchanged.

## Product and acceptance

- U8.1: double-click native Mac app / Windows exe, Install and Restore actions;
  no Python or terminal required. Administrator authorization comes from the OS.
- U8.2: pin the exact ordinary clean payload/version/architecture in the installer;
  verify all payload bytes before staging and immediately before publication.
- U8.3: refuse running Adobe render hosts, incomplete scans, duplicate/unknown
  copies, links/reparse points and unsafe paths. Scan standard system, user and
  AE application plugin roots. Custom paths are outside automatic discovery;
  expose this limitation to the user, never claim whole-machine discovery.
- U8.4: fixed native roots only. Backups outside Adobe, same volume required.
  Journal before mutation, retain all interrupted transaction trees. Publication
  is exclusive for fresh install; replacement preserves the old version through
  the native atomic API, with no copy/delete fallback.
- U8.5: Restore verifies the protected old/current identities and bytes and
  refuses changed/newer installations. Fresh install has no previous version.
- U8.6: clear outcomes for cancellation, stopped-host requirement, conflicts,
  already installed, success and recovery. No secrets/network/downloader/service.

## Architecture / verification

Mac frontend and native fixed-root collector/stager reuse the existing snapshot,
receipt, journal and exchange/fresh-publication coordinators. Windows uses known
folders, Unicode Win32 UI, protected transactions and ReplaceFileW with backup;
its documented partial-failure layouts require explicit inspection/recovery.
Both executors accept only their action, never package-selected destinations.
Test roots and faults belong to separately compiled test executables, never the
shipping CLI. Isolated fixture tests cover fresh/update/restore, tampering,
conflicts, running hosts, interruption and concurrent locks. Native packaging
pins payload identity independently from installer source identity.

Stages: native Mac collector/engine → fixture checks → Mac UI/package → Windows
engine/UI/fixture CI/package → real administrator installation acceptance on
each platform. Missing Windows environment/admin acceptance stays NOT RUN.
No Developer ID/notarization requirement is added; OS warnings for unsigned/ad
hoc distributed installers must be documented without bypassing OS protection.

Primary API sources checked 2026-10-04:
[ReplaceFileW](https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-replacefilew),
[known folders](https://learn.microsoft.com/en-us/windows/win32/shell/knownfolderid),
[NSAppleScript error result](https://developer.apple.com/documentation/foundation/nsapplescript/executeandreturnerror(_:)).
Checksums bind bytes, not publisher authenticity. Trust is the authorized
distribution channel and the user's OS authorization of the selected installer.

Windows root correction discovered in native research: Adobe documents
`[Program Files]/Adobe/Common/Plug-ins/7.0/MediaCore`, not the Windows
`CommonProgramFiles` known folder. Resolve x64 `FOLDERID_ProgramFiles`, append
the fixed Adobe/Common path, and keep ProgramData for protected backups. This
supersedes the earlier U8 CommonProgramFiles note without changing the manual
Windows instructions already delivered to the user.
[Adobe shared plugin path](https://ae-plugins.docsforadobe.dev/print_page/),
[Adobe troubleshooting path](https://helpx.adobe.com/premiere/desktop/troubleshooting/crash-issues/premiere-freezes-on-the-splash-screen.html).


Windows native fixture research: ReplaceFileW freezes the backup's original
DACL (INHERITED_ACE cleared, protected DACL), instead of re-inheriting staged
permissions. Exact disposable-fixture security descriptors established this;
the earlier re-inheritance hypothesis was disproved. Compute that deterministic
protected original descriptor from prepared, require it exactly together with
original owner/group, file identity, bytes, attributes and timestamps. Bind the
exact retained full snapshot in its own durable receipt. Restore checks exact
retained/installed snapshots; ReplaceFileW's permission merge is replaced with
the exact prepared original permission set, protected against new inheritance,
then every original metadata field and the deterministic protected security
snapshot must match. No new principals/rights, arbitrary ACLs or silent masking.
A subsequent backup ACL change refuses Restore. A broad comparison proposal was
rejected by automatic review and never applied. All execution remains isolated
native test fixtures until separately accepted administrator installation.


SetNamedSecurityInfo may retain the AUTO_INHERITED bookkeeping flag after Restore;
accept only the two exact protected original descriptors (P or P+AI), with all
original ACE principal/mask/flags and owner/group exact. The backup publication
still requires its exact P form. Neither variant permits future inheritance or
new grants. Native user-profile root uses FOLDERID_UserProfiles; .AEX scans are
case-insensitive to avoid missing uppercase duplicates.

Already-protected originals keep their exact descriptor on repeated update; this
alternative is allowed only when prepared proves SE_DACL_PROTECTED. Unprotected
originals cannot use the unchanged-descriptor alternative. Repeated Install →
Restore → Install is an explicit fixture. Mac cancellation exits instead of
interpreting unexpected modal responses as Restore.

Windows path-case follow-up — 2026-10-05: the fresh fixture on a19f2b6
refused publication because raw CompareStringOrdinal saw directory_iterator
backslashes versus environment forward separators. Use fs::path::make_preferred
on both verified paths before ordinal case-insensitive comparison. This does
not resolve links, weaken duplicate/security checks or change the fixed target.
Fresh/mixed-separator, existing uppercase .AEX, duplicate and restore scenarios
remain required. CI37287630975/37287631033 are retained FAIL, not plugin acceptance.
