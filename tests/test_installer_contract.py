import dataclasses
import sys
import unittest
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'tools'))
from installer_contract import Candidate, Installed, Observation, TARGETS, TRIPLES, plan


class InstallerContractTests(unittest.TestCase):
    def fixture(self, platform):
        c = Candidate(platform, TRIPLES[platform], '0.9.4', 'EGFX-'+'a'*24, 'b'*64,
                      True, True, False)
        o = Observation(platform, True, True, True, True, ())
        return c, o

    def run_plan(self, c, o):
        return plan(c, o, expected_version='0.9.4', expected_build_id='EGFX-'+'a'*24)

    def test_fresh_update_and_idempotence_on_both_platforms(self):
        for platform in TARGETS:
            c, o = self.fixture(platform)
            self.assertEqual(self.run_plan(c, o).operation, 'CREATE_ONLY')
            old = Installed(TARGETS[platform], 'EGFX-'+'c'*24, 'd'*64, True)
            o = dataclasses.replace(o, copies=(old,))
            p = self.run_plan(c, o)
            self.assertEqual(p.operation, 'BACKUP_THEN_REPLACE')
            self.assertEqual(p.expected_old, old)
            self.assertEqual(p.execution, 'NOT RUN')
            self.assertNotIn('Adobe/', p.backup_root_relative)
            same = dataclasses.replace(old, build_id=c.build_id, payload_sha256=c.payload_sha256)
            self.assertEqual(self.run_plan(c, dataclasses.replace(o, copies=(same,))).operation,
                             'ALREADY_INSTALLED')

    def test_unknown_duplicate_or_modified_copies_are_never_overwritten(self):
        for platform in TARGETS:
            c, o = self.fixture(platform)
            old = Installed(TARGETS[platform], c.build_id, c.payload_sha256, True)
            cases = [(old, old), (dataclasses.replace(old, relative_location='../foreign'),),
                     (dataclasses.replace(old, verified=False),),
                     (dataclasses.replace(old, payload_sha256='c'*64),),
                     (dataclasses.replace(old, build_id='unknown'),)]
            for copies in cases:
                with self.subTest(platform=platform, copies=copies), self.assertRaises(ValueError):
                    self.run_plan(c, dataclasses.replace(o, copies=copies))

    def test_incomplete_or_unsafe_environment_blocks_all_actions(self):
        c, o = self.fixture('macos-arm64')
        for field in ['scan_complete', 'hosts_stopped', 'safe_destination', 'safe_backup_root']:
            with self.subTest(field=field), self.assertRaises(ValueError):
                self.run_plan(c, dataclasses.replace(o, **{field: False}))
        with self.assertRaises(ValueError):
            self.run_plan(c, dataclasses.replace(o, pending_transaction=True))
        for value in ['false', 1, None]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                self.run_plan(c, dataclasses.replace(o, scan_complete=value))

    def test_dirty_diagnostic_wrong_platform_and_wrong_version_are_rejected(self):
        c, o = self.fixture('windows-x64')
        changes = [dict(platform='macos-arm64'), dict(target_triple='i686-pc-windows-msvc'),
                   dict(version='0.9.3'), dict(version='00.9.4'), dict(build_id='EGFX-'+'c'*24),
                   dict(payload_sha256='x'*64), dict(clean_source=False),
                   dict(payload_verified=False), dict(diagnostic=True)]
        for change in changes:
            with self.subTest(change=change), self.assertRaises(ValueError):
                self.run_plan(dataclasses.replace(c, **change), o)


if __name__ == '__main__':
    unittest.main()
