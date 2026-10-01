from pathlib import Path
import sys
import unittest
import struct
import tempfile

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
from aerender_series import mapped_candidate, schedule, progress_record, verify_png_geometry

class SeriesContract(unittest.TestCase):
    def test_schedule(self):
        self.assertEqual(schedule(False), [('OFF','warmup')]+[('OFF','measured')]*5)
        paired=schedule(True)
        self.assertEqual(paired[:2],[('OFF','warmup'),('ON','warmup')])
        self.assertEqual(paired[2:],[('OFF','measured'),('ON','measured')]*5)

    def test_coarse_progress(self):
        log='PROGRESS:  0:00:00:01 (2): 0 Seconds\nPROGRESS:  0:00:00:00 (1): 1 Seconds\nPROGRESS:  Total Time Elapsed: 2 Seconds\n'
        result=progress_record(log,2)
        self.assertEqual(result['total_reported_seconds'],2)
        self.assertEqual(result['frame_reports'][0],dict(frame=2,reported_seconds=0))
        for bad in ('',log.replace('(2)','(1)'),log+log,log.replace('Total Time Elapsed','Unknown')):
            with self.assertRaises(ValueError): progress_record(bad,2)

class MappedIdentity(unittest.TestCase):
    def test_exact_process_and_single_candidate(self):
        path=Path('/Library/ElasticGrid.plugin/Contents/MacOS/ElasticGrid')
        self.assertTrue(mapped_candidate('p123\nn'+str(path)+'\n',123,path))
        with self.assertRaises(ValueError): mapped_candidate('p124\nn'+str(path),123,path)
        self.assertFalse(mapped_candidate('p123\nn/old/ElasticGrid\n',123,path))
        self.assertFalse(mapped_candidate('p123\nn'+str(path)+'\nn/old/ElasticGrid\n',123,path))
        self.assertFalse(mapped_candidate('p123\nn/Library/Other.plugin\n',123,path))

class OutputPrecision(unittest.TestCase):
    def test_encoded_header_must_match_the_declared_output(self):
        fixture=dict(schema=2,width=1920,height=1080,output_bit_depth=16,output_channels='RGBA')
        with tempfile.TemporaryDirectory() as folder:
            path=Path(folder)/'frame.png'
            def write(w=1920,h=1080,depth=16,color=6,interlace=0):
                path.write_bytes(b'\x89PNG\r\n\x1a\n'+struct.pack('>I',13)+b'IHDR'+
                    struct.pack('>IIBBBBB',w,h,depth,color,0,0,interlace)+b'\x00'*4)
            write();verify_png_geometry(path,fixture)
            for values in (dict(depth=8),dict(color=2),dict(w=960),dict(interlace=1)):
                write(**values)
                with self.subTest(values=values),self.assertRaises(ValueError):verify_png_geometry(path,fixture)
            path.write_bytes(b'\x89PNG\r\n\x1a\n')
            with self.assertRaises(ValueError):verify_png_geometry(path,fixture)

if __name__=='__main__': unittest.main()
