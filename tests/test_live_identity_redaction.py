"""Regression for the masked paths observed in native macOS sample output."""
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import live_identity as li


class NativeRedaction(unittest.TestCase):
    def test_masked_paths_cannot_be_accepted_as_exact(self):
        executable = '/private/var/folders/example/probe-host'
        binary = Path('/private/var/folders/example/ElasticGrid-fixture.dylib')
        value = 'F4ACC72B-ACE2-3DDB-9D08-1B2175787564'
        # Format retained from failed native run36465410962, with its PID.
        masked = ('Process:         probe-host [1250]\n'
                  'Path:            /private/var/folders/*/probe-host\n'
                  'Binary Images:\n'
                  '  0x10281c000 - 0x10281ffff +ElasticGrid-fixture.dylib (0) '
                  '<' + value + '> /var/folders/*/ElasticGrid-fixture.dylib\n')
        with self.assertRaises(li.Blocked):
            li.parse_sample(masked, 1250, executable)
        exact_header = masked.replace('/private/var/folders/*/probe-host', executable)
        images = li.parse_sample(exact_header, 1250, executable)
        with self.assertRaises(li.Blocked):
            li.select_image(images, binary, {value})
        def trusted(address):
            return executable if address is None else str(binary)
        images = li.parse_sample(masked, 1250, executable, path_lookup=trusted)
        found = li.select_image(images, binary, {value})
        self.assertEqual(found['reported_path'], '/var/folders/*/ElasticGrid-fixture.dylib')
        self.assertIn('libproc', found['path_source'])
        with self.assertRaises(li.Blocked):
            li.parse_sample(masked, 1250, executable, path_lookup=lambda _: '/wrong/program')
        with self.assertRaises(li.Blocked):
            li.parse_sample(masked, 1250, executable, path_lookup=lambda a: executable if a is None else '/wrong/library')
        with self.assertRaises(li.Blocked):
            li.parse_sample(masked.replace('[1250]', '[1251]'), 1250, executable, path_lookup=trusted)

    def test_missing_native_interface_blocks_without_escalation(self):
        for error in (OSError('unavailable'), AttributeError('missing symbol')):
            with patch.object(li.ctypes, 'CDLL', side_effect=error):
                with self.assertRaises(li.Blocked):
                    li.system_path(1250)
        with self.assertRaises(li.Blocked):
            li.system_path(-1)

    def test_mask_is_not_an_independent_path_observation(self):
        self.assertFalse(li.masked_path_consistent('/Users/*/program', '/Users/*/program'))
        self.assertFalse(li.masked_path_consistent('/Users/*/*/program', '/Users/one/two/program'))
        self.assertFalse(li.masked_path_consistent('/Users/*/program', '/elsewhere/program'))
        self.assertTrue(li.masked_path_consistent('/var/folders/*/program', '/private/var/folders/test/program'))


if __name__ == '__main__':
    unittest.main()
