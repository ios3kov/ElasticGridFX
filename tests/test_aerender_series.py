from pathlib import Path
import sys
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
from aerender_series import mapped_candidate

class MappedIdentity(unittest.TestCase):
    def test_exact_process_and_single_candidate(self):
        path=Path('/Library/ElasticGrid.plugin/Contents/MacOS/ElasticGrid')
        self.assertTrue(mapped_candidate('p123\nn'+str(path)+'\n',123,path))
        with self.assertRaises(ValueError): mapped_candidate('p124\nn'+str(path),123,path)
        self.assertFalse(mapped_candidate('p123\nn/old/ElasticGrid\n',123,path))
        self.assertFalse(mapped_candidate('p123\nn'+str(path)+'\nn/old/ElasticGrid\n',123,path))
        self.assertFalse(mapped_candidate('p123\nn/Library/Other.plugin\n',123,path))

if __name__=='__main__': unittest.main()
