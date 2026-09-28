"""Real Apple sample/dlopen test on an OWNED child, explicitly not After Effects."""
import json
from pathlib import Path
import platform
import subprocess
import sys
import tempfile
import select
import unittest

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import live_identity as li


@unittest.skipUnless(platform.system()=='Darwin','Apple sampler requires macOS; not AE evidence')
class NativeImageObservation(unittest.TestCase):
    def test_real_sampler_reads_owned_child_image(self):
        with tempfile.TemporaryDirectory(prefix='egfx-native-probe-') as tmp:
            folder=Path(tmp).resolve()
            library=folder/'ElasticGrid-fixture.dylib'
            (folder/'library.c').write_text('int fixture(void){return 42;}\n')
            subprocess.run(['clang','-dynamiclib',str(folder/'library.c'),'-o',str(library)],check=True,timeout=30)
            (folder/'host.c').write_text('#include <dlfcn.h>\n#include <stdio.h>\n#include <unistd.h>\nint main(int n,char**v){if(n!=2)return 2;void*p=dlopen(v[1],RTLD_NOW);if(!p){fprintf(stderr,"%s",dlerror());return 3;}puts("READY");fflush(stdout);sleep(60);return 0;}\n')
            host=folder/'probe-host'
            subprocess.run(['clang',str(folder/'host.c'),'-o',str(host)],check=True,timeout=30)
            child=subprocess.Popen([str(host),str(library)],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
            try:
                ready, _, _ = select.select([child.stdout], [], [], 5)
                self.assertTrue(ready, 'Fixture startup timed out')
                self.assertEqual(child.stdout.readline().strip(), 'READY')
                try:
                    images,observation=li.capture(child.pid,host,folder)
                except Exception:
                    # Owned CI fixture only: retain header/images for diagnosis.
                    sample=folder/'sample-private.txt'
                    if sample.exists():
                        text=sample.read_text()
                        print('FIXTURE SAMPLE HEADER:', text[:2500])
                        print('FIXTURE IMAGE:', '\n'.join(line for line in text.splitlines() if 'ElasticGrid-fixture' in line))
                    raise
                uuidset=li.macho_uuids(library.read_bytes())
                observed=li.select_image(images,library,uuidset)
                self.assertIn(observed['uuid'],uuidset)
                record={'status':'PASS','scope':'owned fixture; NOT After Effects','observed':observed,'capture':observation}
                print(json.dumps(record))
                # Real negative: same process observation cannot identify a different UUID.
                with self.assertRaises(li.Blocked): li.select_image(images,library,{'00000000-0000-0000-0000-000000000001'})
            finally:
                # ONLY this Popen child belongs to the test. Production has no kill.
                child.terminate()
                try: child.communicate(timeout=5)
                except subprocess.TimeoutExpired: child.kill();child.communicate(timeout=5)


if __name__=='__main__':unittest.main()
