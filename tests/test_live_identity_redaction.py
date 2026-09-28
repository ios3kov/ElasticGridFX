"""Regression for the masked paths observed in native macOS sample output."""
from pathlib import Path
import sys
import unittest

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


if __name__ == '__main__':
    unittest.main()
