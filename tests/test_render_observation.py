import csv
import io
from pathlib import Path
import sys
import tempfile
import unittest
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
import render_observation as ro

BUILD='EGFX-'+'a'*24
class Observation(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.addCleanup(self.temp.cleanup)
        self.path=Path(self.temp.name)/'render.csv'
    def write(self,rows,build=BUILD):
        stream=io.StringIO();stream.write('# schema=1,build_id='+build+'\n')
        writer=csv.writer(stream,lineterminator='\n');writer.writerow(ro.HEADER);writer.writerows(rows)
        self.path.write_text(stream.getvalue())
    def row(self):return [1,'smart',3,30,1920,1080,32,'plane_region',1,1,2,5,8,18,20,21,0]
    def test_successful_callback_reports_only_observed_phases(self):
        self.write([self.row()]);r=ro.inspect(self.path,BUILD)
        self.assertEqual(r['status'],'DECODED');self.assertEqual(r['summary']['plane_region']['observed_render_callbacks'],1)
        self.assertEqual(r['summary']['plane_region']['median_sampling_ms'],10/1e6)
        self.assertIn('not runtime identity',r['scope'])
    def test_overlapping_callbacks_keep_out_of_order_completion(self):
        late=self.row();late[-1]=5;early=self.row();early[0]=2
        self.write([late,early]);r=ro.inspect(self.path,BUILD)
        self.assertEqual(r['observed_peak_sampling_interval_overlap'],2)
    def test_fixture_rejects_wrong_route_and_missing_frames(self):
        self.write([self.row()]);record=ro.inspect(self.path,BUILD)
        fixture=dict(schema=3,expected_render_path='plane_region',width=1920,height=1080,
                     bit_depth=32,fps=30.0,frame_start=3,frame_end=3)
        self.assertEqual(ro.validate_fixture(record,fixture)['status'],'ROUTE_AND_FRAME_COVERAGE_PASS')
        for key,value in [('schema',2),('expected_render_path','legacy_cpu'),('width',128),
                          ('bit_depth',16),('frame_end',4),('fps',24.0),('frame_end',10**100)]:
            with self.subTest(key=key),self.assertRaises(ValueError):ro.validate_fixture(record,dict(fixture,**{key:value}))
        record['status']='PARTIAL'
        with self.assertRaises(ValueError):ro.validate_fixture(record,fixture)

    def test_marker_sequence_and_phase_fail_closed(self):
        for index,value in [(0,2),(1,'gpu'),(3,-1),(8,2),(12,''),(13,3),(15,1)]:
            row=self.row();row[index]=value;self.write([row])
            with self.subTest(index=index),self.assertRaises(ValueError):ro.inspect(self.path,BUILD)
        self.write([self.row()],build='EGFX-'+'b'*24)
        with self.assertRaises(ValueError):ro.inspect(self.path,BUILD)
    def test_incomplete_and_empty_are_not_decoded_success(self):
        row=self.row();row[9]=0;row[12:15]=['','',''];self.write([row])
        self.assertEqual(ro.inspect(self.path,BUILD)['status'],'PARTIAL')
        self.write([]);self.assertEqual(ro.inspect(self.path,BUILD)['status'],'NO_RECORDS')
        self.path.write_bytes(self.path.read_bytes().rstrip(b'\n'))
        with self.assertRaises(ValueError):ro.inspect(self.path,BUILD)
    def test_full_cap_is_explicitly_truncated(self):
        rows=[]
        for sequence in range(1,ro.MAX_RECORDS+1):
            row=self.row();row[0]=sequence;rows.append(row)
        self.write(rows);self.assertEqual(ro.inspect(self.path,BUILD)['status'],'TRUNCATED')
        rows.append(self.row());self.write(rows)
        with self.assertRaises(ValueError):ro.inspect(self.path,BUILD)

if __name__=='__main__':unittest.main()
