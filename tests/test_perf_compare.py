"""Timing comparison tests; quality equivalence deliberately remains NOT RUN."""
import json
from pathlib import Path
import sys
import unittest

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import perf_compare as pc


def report(build,median,p95,mfr='on'):
    fixture=dict(fixture_id='v1',project_sha256='a'*64,composition='C',rqindex=1,width=1920,height=1080,
                 fps=30.0,frame_start=0,frame_end=29,bit_depth=32,quality='Final Bicubic',
                 output_format='PNG sequence',output_pattern='f_[####].png')
    samples=[{'wall_seconds':median} for _ in range(5)]
    return dict(schema=1,status='MEASURED',candidate={'build_id':build},fixture=fixture,
                settings={'mfr':mfr},samples_data=samples,
                summary={'wall_median_seconds':median,'wall_p95_seconds':p95,'peak_rss_max_bytes':100})


class Compare(unittest.TestCase):
    def test_reports_delta_without_quality_claim(self):
        r=pc.compare(report('EGFX-'+'a'*24,10,11),report('EGFX-'+'b'*24,9,10))
        self.assertAlmostEqual(r['delta_percent']['median'],-10.0)
        self.assertEqual(r['quality_equivalence'],'NOT RUN')
        self.assertFalse(r['release']!='BLOCKED')

    def test_over_five_percent_sets_investigation_gate(self):
        r=pc.compare(report('A',10,11),report('B',10.6,11.7))
        self.assertTrue(r['investigation_gate'])

    def test_mismatched_fixture_or_mfr_refuses(self):
        a=report('A',10,11);b=report('B',9,10,'off')
        with self.assertRaises(ValueError):pc.compare(a,b)
        b=report('B',9,10);b['fixture']['bit_depth']=16
        with self.assertRaises(ValueError):pc.compare(a,b)


if __name__=='__main__':unittest.main()
