import importlib.util
from pathlib import Path
import unittest
import signal
import ctypes
import tempfile
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('guard', Path(__file__).parents[1]/'tools/ae_memory_guard.py')
guard = importlib.util.module_from_spec(spec); spec.loader.exec_module(guard)

class FakeProcess:
    pid = 123
    def __init__(self, path=guard.HOST, birth=99): self.path=path; self.birth=birth
    def read(self): return self.path,self.birth,100

class MemoryGuardTests(unittest.TestCase):
    def test_absolute_and_growth_limits_are_independent(self):
        self.assertIsNone(guard.limit_reason(512*guard.MIB,600*guard.MIB))
        self.assertEqual(guard.limit_reason(512*guard.MIB,800*guard.MIB),'growth-from-baseline')
        self.assertEqual(guard.limit_reason(1400*guard.MIB,1600*guard.MIB),'absolute-footprint')
    def test_changed_pid_birth_or_wrong_app_never_receives_signal(self):
        sent=[]
        for p in [FakeProcess(birth=100),FakeProcess('/Applications/Other.app/Other'),FakeProcess(birth=0)]:
            with self.assertRaises(RuntimeError): guard.signal_owned(p,99,signal.SIGKILL,lambda *a:sent.append(a))
        self.assertEqual(sent,[])
        guard.signal_owned(FakeProcess(),99,signal.SIGSTOP,lambda *a:sent.append(a))
        self.assertEqual(sent,[(123,signal.SIGSTOP)])
    def test_system_header_layout_matches_target(self):
        self.assertEqual(ctypes.sizeof(guard.Usage),160)
        self.assertEqual(guard.Usage.phys_footprint.offset,72)
        self.assertEqual(guard.Usage.proc_start_abstime.offset,80)
    def test_sample_failure_still_closes_suspended_owned_host(self):
        sent=[]
        def fail(*args,**kwargs): raise TimeoutError('sample unavailable')
        with self.assertRaises(TimeoutError):
            guard.stop_on_limit(FakeProcess(),99,Path('/tmp'),lambda row:None,
                                fail,lambda *args:sent.append(args))
        self.assertEqual(sent,[(123,signal.SIGSTOP),(123,signal.SIGKILL)])
    def test_log_failure_still_closes_suspended_owned_host(self):
        sent=[]
        def fail(row): raise OSError('disk unavailable')
        with self.assertRaises(OSError):
            guard.stop_on_limit(FakeProcess(),99,Path('/tmp'),fail,
                                send=lambda *args:sent.append(args))
        self.assertEqual(sent,[(123,signal.SIGSTOP),(123,signal.SIGKILL)])
    def test_deadline_closes_only_armed_saved_host(self):
        for armed in (False,True):
            with tempfile.TemporaryDirectory() as temporary:
                with patch.object(guard,'Process',return_value=FakeProcess()), \
                     patch.object(guard.time,'monotonic',side_effect=[0,2]), \
                     patch.object(guard,'signal_owned') as send:
                    guard.monitor(123,Path(temporary)/'fresh',1,armed)
                if armed: send.assert_called_once_with(send.call_args.args[0],99,signal.SIGKILL)
                else: send.assert_not_called()
    def test_loading_uses_tighter_idle_cap_before_gesture(self):
        gate=guard.PhaseGate(500*guard.MIB,True)
        self.assertEqual(gate.observe(0,900*guard.MIB), (None,False))
        self.assertTrue(gate.loading)
        self.assertEqual(gate.observe(1,1025*guard.MIB),('loading-idle-cap',False))
    def test_setup_marker_alone_cannot_arm_unstable_memory(self):
        gate=guard.PhaseGate(500*guard.MIB,True)
        for t,b in [(0,600),(0.5,650),(1,700),(1.5,650)]:
            self.assertEqual(gate.observe(t,b*guard.MIB,True),(None,False))
        self.assertTrue(gate.loading)
    def test_loaded_idle_baseline_keeps_gesture_growth_limit(self):
        gate=guard.PhaseGate(500*guard.MIB,True)
        self.assertEqual(gate.observe(0,900*guard.MIB,True),(None,False))
        self.assertEqual(gate.observe(1,900*guard.MIB,True),(None,True))
        self.assertFalse(gate.loading)
        self.assertEqual(gate.observe(2,1157*guard.MIB),('growth-from-baseline',False))
    def test_no_setup_marker_cannot_arm_even_stable_memory(self):
        gate=guard.PhaseGate(500*guard.MIB,True)
        for t in range(5):self.assertEqual(gate.observe(t,900*guard.MIB),(None,False))
        self.assertTrue(gate.loading)

if __name__ == '__main__': unittest.main()
