import importlib.util
from pathlib import Path
import unittest
spec=importlib.util.spec_from_file_location('useful_run',Path(__file__).with_name('run.py'))
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)

class UsefulTests(unittest.TestCase):
    def test_usable_redistributable_content_and_padding(self):
        for name,f in [('objects',r.object_data),('dataset',r.dataset_data),('archive',r.archive_data)]:
            raw=f();self.assertEqual(raw,f());self.assertTrue(r.utility(name,raw))
            packed=r.pad(raw);self.assertEqual(packed[:len(raw)],raw)
            self.assertEqual(len(packed)%r.CHUNK,0)
    def test_public_reconstruction_and_duplicate_storage_pass(self):
        raw=r.pad(r.archive_data());chunks=[raw[i:i+r.CHUNK] for i in range(0,len(raw),r.CHUNK)]
        levels=r.tree(chunks);m=r.manifest('11'*32,levels[-1][0].hex(),len(chunks),'22'*32)
        a=r.trial('reconstructed',m,levels,lambda i:r.pad(r.archive_data())[i*r.CHUNK:(i+1)*r.CHUNK],min(16,len(chunks)))
        self.assertEqual(a['passed'],10)
        self.assertTrue(a['full_recovery'])
    def test_missing_data_is_not_an_available_service(self):
        chunks=[b'x'*r.CHUNK]*32;levels=r.tree(chunks)
        m=r.manifest('11'*32,levels[-1][0].hex(),32,'22'*32)
        a=r.trial('missing',m,levels,lambda i:b'',16)
        self.assertEqual(a['passed'],0);self.assertFalse(a['full_recovery'])

if __name__=='__main__':unittest.main()
