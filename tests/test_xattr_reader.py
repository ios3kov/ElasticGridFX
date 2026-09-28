"""Portable ABI-boundary regressions; real Darwin xattr roundtrip is separate."""
import ctypes
import errno
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import xattr_reader as xr


class XattrReader(unittest.TestCase):
    def test_binary_and_empty_attribute(self):
        for payload in (b'', b'one\x00two\xff'):
            def native(buffer, size):
                if buffer is not None and payload:
                    ctypes.memmove(buffer, payload, len(payload))
                return len(payload)
            self.assertEqual(xr.bounded_read(native), payload)

    def test_error_is_not_empty_metadata(self):
        def denied(buffer, size):
            ctypes.set_errno(errno.EACCES)
            return -1
        with self.assertRaises(PermissionError):
            xr.bounded_read(denied)

    def test_size_growth_or_shrink_is_rejected(self):
        for sizes in ([0,1], [8,7]):
            with self.subTest(sizes=sizes):
                with self.assertRaises(ValueError):
                    xr.bounded_read(lambda buffer,size: sizes.pop(0))

    def test_oversized_attribute_is_rejected_before_allocation(self):
        with patch.object(xr.ctypes,'create_string_buffer') as allocation:
            with self.assertRaises(ValueError):
                xr.bounded_read(lambda b,s:xr.LIMIT+1)
            allocation.assert_not_called()

    def test_darwin_read_uses_native_api_and_nofollow(self):
        def listing(path, buffer, size, flags):
            self.assertEqual(flags,1)
            if buffer is not None: ctypes.memmove(buffer,b'test\0',5)
            return 5
        def reading(path,name,buffer,size,position,flags):
            self.assertEqual((name,position,flags),(b'test',0,1))
            if buffer is not None: ctypes.memmove(buffer,b'raw\0value',9)
            return 9
        with patch.object(xr.platform,'system',return_value='Darwin'), patch.object(xr,'darwin_api',return_value=(listing,reading)):
            result=xr.attribute_hashes(Path('/owned/fixture'))
        self.assertEqual(result,{'test':xr.hashlib.sha256(b'raw\0value').hexdigest()})

    def test_malformed_native_list_is_rejected(self):
        for payload in (b'no-terminator', b'a\0a\0', b'\0'):
            def listing(path,buffer,size,flags):
                if buffer is not None: ctypes.memmove(buffer,payload,len(payload))
                return len(payload)
            with patch.object(xr.platform,'system',return_value='Darwin'), patch.object(xr,'darwin_api',return_value=(listing,None)):
                with self.assertRaises(ValueError): xr.attribute_hashes(Path('/owned/fixture'))


if __name__=='__main__': unittest.main()
