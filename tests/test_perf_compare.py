"""Performance comparison guards for 0.9.4 tooling; no AE execution."""
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import perf_compare as pc


def report(build,median,p95,mfr='on',digest='same',geometry='grid',case='PERF094-RQ-001'):
    fixture=dict(fixture_id='v2',project_sha256='a'*64,composition='C',rqindex=1,width=1920,height=1080,
                 fps=30.0,frame_start=0,frame_end=29,bit_depth=32,quality='Final Bicubic',
                 output_format='PNG sequence',output_pattern='f_[####].png',
                 geometry=geometry,color_management='none-linearize-off')
    samples=[{'wall_seconds':median} for _ in range(5)]
    return dict(schema=1,status='MEASURED',test_case_id=case,candidate={'build_id':build},fixture=fixture,
                settings={'mfr':mfr},samples_data=samples,
                summary={'wall_median_seconds':median,'wall_p95_seconds':p95,
                         'peak_rss_max_bytes':100,'output_digest':digest})


class Compare(unittest.TestCase):
    def test_reports_delta_only_with_exact_encoded_output_match(self):
        r=pc.compare(report('EGFX-'+'a'*24,10,11),report('EGFX-'+'b'*24,9,10))
        self.assertAlmostEqual(r['delta_percent']['median'],-10.0)
        self.assertEqual(r['quality_equivalence'],'EXACT_ENCODED_OUTPUT_MATCH')
        self.assertTrue(r['encoded_output_exact_match'])
        self.assertFalse(r['performance_regression_gate'])
        self.assertEqual(r['release'],'BLOCKED')

    def test_output_mismatch_is_explicit_fail(self):
        r=pc.compare(report('A',10,11,digest='a'),report('B',9,10,digest='b'))
        self.assertEqual(r['status'],'FAIL')
        self.assertEqual(r['quality_equivalence'],'FAIL_ENCODED_OUTPUT_MISMATCH')
        self.assertFalse(r['encoded_output_exact_match'])

    def test_over_five_percent_slowdown_sets_regression_gate(self):
        r=pc.compare(report('A',10,11),report('B',10.6,11.7))
        self.assertTrue(r['performance_regression_gate'])
        self.assertTrue(r['meaningful_timing_change'])

    def test_mismatched_fixture_mfr_geometry_or_case_refuses(self):
        a=report('A',10,11);b=report('B',9,10,'off')
        with self.assertRaises(ValueError):pc.compare(a,b)
        b=report('B',9,10);b['fixture']['bit_depth']=16
        with self.assertRaises(ValueError):pc.compare(a,b)
        with self.assertRaises(ValueError):pc.compare(a,report('B',9,10,geometry='four_corners'))
        with self.assertRaises(ValueError):pc.compare(a,report('B',9,10,case='PERF094-RQ-002'))


if __name__=='__main__':unittest.main()
