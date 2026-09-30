"""Safety/source guard for the Stage 9 Mac launcher. Does not execute AE."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
SOURCE = (ROOT/'RUN_ELASTICGRID_PLANE_AE_TEST_MAC.command').read_text()

class PlaneLauncherSafety(unittest.TestCase):
    def test_launcher_is_pinned_and_non_destructive(self):
        self.assertIn('plane-candidate-de31498/ElasticGrid.artifact.json', SOURCE)
        self.assertIn('plane-candidate-de31498/ElasticGrid.plugin.zip', SOURCE)
        self.assertIn('tools/ae_plane_acceptance.py', SOURCE)
        for forbidden in ('sudo ', 'kill ', 'killall ', 'rm -rf', 'UPDATE_ELASTICGRID_MAC',
                          'install_candidate.py', 'osascript -e'):
            self.assertNotIn(forbidden, SOURCE)

    def test_launcher_requires_mac_arm64_and_logs(self):
        self.assertIn('uname -s', SOURCE)
        self.assertIn('uname -m', SOURCE)
        self.assertIn('arm64', SOURCE)
        self.assertIn('terminal.log', SOURCE)
        self.assertIn('PYTHONDONTWRITEBYTECODE=1', SOURCE)

if __name__ == '__main__':
    unittest.main()
