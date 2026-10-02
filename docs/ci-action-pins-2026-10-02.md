# CI action identity hardening — 2026-10-02

## Scope and requirement

Keep the existing CI jobs, events, permissions, versions and validation gates,
while removing mutable third-party action references and unused persisted checkout
credentials. This is development hardening, not release certification.

## Verified upstream identities

Exact tag refs were read from each official upstream repository and the returned
commit identities were independently checked through the GitHub connector:

| Action | Existing major | Pinned commit |
| --- | --- | --- |
| actions/checkout | v5 | [fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09](https://github.com/actions/checkout/commit/fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09) |
| actions/upload-artifact | v4 | [ea165f8d65b6e75b540449e92b4886f43607fa02](https://github.com/actions/upload-artifact/commit/ea165f8d65b6e75b540449e92b4886f43607fa02) |
| actions/download-artifact | v4 | [d3f86a106a0bac45b974a628896c90dbdf5c8093](https://github.com/actions/download-artifact/commit/d3f86a106a0bac45b974a628896c90dbdf5c8093) |

All 23 references in eight workflows now use full commit identities. Five
checkout steps that previously used the default now set `persist-credentials:
false`; the six existing explicit settings remain. No workflow requires a Git
push using credentials left in its checkout.

## Verification

- PASS: all eight old/new workflow documents parsed using Ruby's standard YAML
  parser. After normalizing only these action identities and the checkout
  credential setting, the complete parsed structures are identical.
- PASS: separate diff review confirms no event/job/permission/gate change.
- Offline code audit: 29 initial review candidates -> one after hardening.
  The remaining `vibe.no_ratelimit_auth` candidate points to
  `tests/test_target_ae_acceptance.py:54`, a local package-verification mock
  assertion. There is no HTTP/authentication request handler at this location;
  manual disposition is false positive. No scanner rule or test was weakened.
- No credential findings. Scanner exit 1 still means review candidates and its
  `release_readiness` remains `not_assessed`.
- Actual execution with these pins is tracked by exact-head checks in
  [draft PR #20](https://github.com/ios3kov/ElasticGridFX/pull/20).

Private original/post-fix reports are retained as
`outputs/host-checkpoint-code-audit-2026-10-02.json` and
`outputs/ci-pins-code-audit-2026-10-02.json` in the task workspace. Pinned actions
can become stale; subsequent updates must verify the official replacement commit
and run the affected hosted checks again.
