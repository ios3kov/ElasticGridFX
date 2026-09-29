from pathlib import Path
import sys
import subprocess
import unittest
from unittest.mock import patch

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
import aerender_identity_probe as probe

class RenderProcessDiscovery(unittest.TestCase):
    def test_leaf_histogram_is_thresholded_not_recursive_stack_sum(self):
        text='Call graph:\n 999 recursive\nSort by top of stack, same collapsed (when >= 5):\n        longest_match  (in PNGIO)        52\n\nBinary Images:\nprivate paths\n'
        self.assertEqual(probe.leaf_summary(text),[dict(symbol='longest_match',image='PNGIO',observations=52)])
        for bad in ('',text+text,text.replace('52','4'),text.replace('(in PNGIO)','unknown')):
            with self.assertRaises(ValueError): probe.leaf_summary(bad)

    def test_detached_process_requires_parent_token_and_output(self):
        path=Path('/test/aerendercore'); output=Path('/owned/frame_[#####].png')
        for token,want in (('10',True),('100',False)):
            args=f'{path} -output {output} -mfr OFF 100 -aerenderpid {token}\n'
            with patch.object(probe.subprocess,'check_output',return_value=f'12 {path}\n'), patch.object(probe.subprocess,'run',return_value=subprocess.CompletedProcess([],0,args,'')):
                self.assertEqual(bool(probe.detached_renderers(10,path,output)),want)
                self.assertEqual(probe.detached_renderers(10,path,Path('/other/frame.png')),[])

    def test_only_owned_descendants_with_exact_executable(self):
        path=Path('/Applications/Adobe After Effects 2025/aerender')
        table=f'10 1 {path}\n11 10 /helper\n12 11 {path}\n13 1 {path}\n14 10 {path}-other\n'
        with patch.object(probe.subprocess,'check_output',return_value=table):
            self.assertEqual(probe.render_children(10,path),[dict(pid=12,executable=str(path))])

    def test_no_renderer_does_not_select_launcher(self):
        path=Path('/test/aerender')
        with patch.object(probe.subprocess,'check_output',return_value=f'10 1 {path}\n'):
            self.assertEqual(probe.render_children(10,path),[])

if __name__=='__main__': unittest.main()
