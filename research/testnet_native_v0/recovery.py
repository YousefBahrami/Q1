"""Native-only atomic outcome receipts. No new consensus validity rules."""
import hashlib
import json
import node
from faults import barrier


def identity(request):
    return hashlib.sha256(b'Q1_NATIVE_APPLICATION_REQUEST\0'+node.canonical(request)).hexdigest()


class RecoverableWorker(node.Worker):
    storage_limit = 1024*1024
    def __init__(self,*args,**kwargs):
        self.receipts={}
        super().__init__(*args,**kwargs)
    def decode_state(self, raw):
        node.need(len(raw)<=self.storage_limit,'ARCHIVE_LIMIT')
        obj=json.loads(raw)
        node.need(node.canonical(obj)==raw,'NONCANONICAL_JOURNAL')
        node.fields(obj,'native_receipts state')
        node.need(type(obj['native_receipts']) is dict and len(obj['native_receipts'])<=6,'RECEIPT_LIMIT')
        for who,entry in obj['native_receipts'].items():
            node.need(who in tuple(map(str,range(6))),'RECEIPT_SIGNER')
            node.fields(entry,'digest seq stage result')
            node.need(type(entry['digest']) is str and len(entry['digest'])==64,'RECEIPT_DIGEST')
            node.integer(entry['seq'],1)
            node.need(entry['stage'] in ('started','complete'),'RECEIPT_STAGE')
            if entry['stage']=='complete':
                node.need(len(node.canonical(entry['result']))<=node.MAX_FRAME,'RECEIPT_RESULT_LIMIT')
                if 'error' not in entry['result']:
                    node.authentic(entry['result'])
                    node.need(entry['result']['signer']==self.who,'RECEIPT_RESPONSE_SIGNER')
        self.receipts=obj['native_receipts']
        return node.decode(node.canonical(obj['state']))
    def encode_state(self):
        node.need(len(node.canonical(self.state))<=node.MAX_FRAME,'ARCHIVE_LIMIT')
        return node.canonical(dict(native_receipts=self.receipts,state=self.state))
    def commit_response(self, request, response):
        self.receipts[str(request['signer'])]=dict(digest=identity(request),seq=request['body']['seq'],stage='complete',result=response)
        self.persist()
        barrier('after_'+request['body']['kind']+'_commit')
    def apply(self, request):
        b=node.authentic(request);who=str(request['signer']);seq=node.integer(b.get('seq'),1)
        node.need(seq>self.state['seen'].get(who,0),'REPLAYED_MESSAGE')
        old=self.receipts.get(who)
        node.need(old is None or old['stage']!='started','APPLICATION_RECONCILIATION_REQUIRED')
        self.receipts[who]=dict(digest=identity(request),seq=seq,stage='started',result=None)
        self.persist()
        barrier('before_'+b['kind']+'_dispatch')
        try:return super().apply(request)
        except node.Rejected as e:
            # A failed coordinator can have durable intermediate work. Keep it
            # unresolved; never treat it as an unexecuted transfer.
            if b['kind']!='run':self.commit_response(request,dict(error=str(e)))
            raise
    def reconcile(self, request):
        node.need(not self.poisoned,"POISONED")
        b=node.authentic(request);who=str(request['signer']);seq=node.integer(b.get('seq'),1)
        entry=self.receipts.get(who)
        if entry is None or seq>entry['seq']:
            node.need(seq>self.state['seen'].get(who,0),'RETIRED_APPLICATION_REQUEST')
            # This serialized worker has never started this exact request.
            return self.apply(request)
        node.need(entry['seq']==seq and entry['digest']==identity(request),'APPLICATION_IDENTITY_CONFLICT')
        if entry['stage']=='complete':return entry['result']
        return dict(error='STOP_RECONCILIATION_REQUIRED',request_id=entry['digest'],stage='started',ledger=node.backend('status',history=self.state['history']))
