"""Access evidence laboratory v1. No consensus, issuance or hardware claim."""
from __future__ import annotations
import hashlib
import json
import secrets
import time
from pathlib import Path

PROFILE = 'Q1_RESOURCE_V1_RESEARCH_ONLY'
CHUNK = 4096
MAX_BYTES = 16 * 1024 * 1024
MAX_PROOF = 2 * 1024 * 1024

class Rejected(ValueError):
    """Stable laboratory rejection code."""

def need(ok, reason):
    if not ok:
        raise Rejected(reason)

def canonical(obj):
    return json.dumps(obj, sort_keys=True, separators=(',', ':'), ensure_ascii=True, allow_nan=False).encode('ascii')

def digest(label, *parts):
    h = hashlib.sha256()
    for part in (PROFILE.encode(), label.encode(), *parts):
        h.update(len(part).to_bytes(8, 'big'))
        h.update(part)
    return h.digest()

def fields(obj, names):
    need(type(obj) is dict and set(obj) == set(names.split()), 'SCHEMA')

def number(n, low, high):
    need(type(n) is int and low <= n <= high, 'INTEGER')
    return n

def hx(s, length):
    need(type(s) is str and len(s) == 2 * length, 'HEX')
    try:
        b = bytes.fromhex(s)
    except ValueError:
        raise Rejected('HEX') from None
    need(b.hex() == s, 'HEX')
    return b

def decode(raw, limit=MAX_PROOF):
    need(type(raw) is bytes and len(raw) <= limit, 'OVERSIZE')
    def unique(pairs):
        d = {}
        for k, v in pairs:
            need(k not in d, 'DUPLICATE_FIELD')
            d[k] = v
        return d
    try:
        obj = json.loads(raw, object_pairs_hook=unique)
        def bounded(v, depth=0):
            need(depth <= 16, 'DEPTH')
            if type(v) is dict:
                need(len(v) <= 32, 'FIELDS')
                for k, x in v.items():
                    need(k.isascii(), 'ASCII')
                    bounded(x, depth+1)
            elif type(v) is list:
                need(len(v) <= 64, 'ARRAY')
                for x in v:
                    bounded(x, depth+1)
            elif type(v) is str:
                need(v.isascii(), 'ASCII')
            else:
                number(v, 0, 2**64-1)
        bounded(obj)
        need(canonical(obj) == raw, 'NONCANONICAL')
        return obj
    except (UnicodeError, json.JSONDecodeError, RecursionError, OverflowError):
        raise Rejected('ENCODING') from None

def leaf(i, data):
    need(len(data) == CHUNK, 'MISSING_DATA')
    return digest('leaf', i.to_bytes(8, 'big'), data)

def tree(chunks):
    row = [leaf(i, b) for i, b in enumerate(chunks)]
    need(1 <= len(row) <= MAX_BYTES // CHUNK and len(row) & (len(row)-1) == 0, 'SIZE')
    levels = [row]
    while len(row) > 1:
        row = [digest('node', row[i], row[i+1]) for i in range(0, len(row), 2)]
        levels.append(row)
    return levels

def manifest(unit, root, count, session):
    number(count, 1, MAX_BYTES // CHUNK)
    need(count & (count-1) == 0, 'SIZE')
    hx(root, 32); hx(unit, 32); hx(session, 32)
    return dict(profile=PROFILE, unit=unit, session=session, root=root, count=count, chunk=CHUNK)

def validate(m):
    fields(m, 'profile unit session root count chunk')
    need(m == manifest(m['unit'], m['root'], m['count'], m['session']), 'MANIFEST')

def mid(m):
    validate(m)
    return digest('manifest', canonical(m)).hex()

def select(m, nonce, counter, samples):
    number(samples, 1, min(64, m['count']))
    number(counter, 1, 2**64-1)
    seed = digest('selection', hx(mid(m), 32), hx(nonce, 32), counter.to_bytes(8, 'big'))
    # Power-of-two corpus size makes reduction unbiased. Repeated indices
    # are discarded; the complete framing is fixed by this code and tests.
    chosen, j = set(), 0
    while len(chosen) < samples:
        chosen.add(int.from_bytes(digest('index', seed, j.to_bytes(8, 'big')), 'big') % m['count'])
        j += 1
    return sorted(chosen)

def challenge(m, nonce, counter, samples=64):
    return dict(profile=PROFILE, manifest=mid(m), nonce=nonce, counter=counter,
                indices=select(m, nonce, counter, samples))

def check_challenge(m, q):
    fields(q, 'profile manifest nonce counter indices')
    need(type(q['indices']) is list, 'INDICES')
    for i in q['indices']:
        number(i, 0, m['count']-1)
    need(q == challenge(m, q['nonce'], q['counter'], len(q['indices'])), 'CHALLENGE')

def proof(m, q, levels, read):
    check_challenge(m, q)
    rows = []
    for i in q['indices']:
        data = read(i)
        need(type(data) is bytes and len(data) == CHUNK, 'MISSING_DATA')
        cursor, siblings = i, []
        for level in levels[:-1]:
            siblings.append(level[cursor ^ 1].hex())
            cursor //= 2
        rows.append(dict(index=i, data=data.hex(), path=siblings))
    return canonical(dict(profile=PROFILE, challenge=digest('challenge', canonical(q)).hex(), rows=rows))

def verify(raw, m, q):
    check_challenge(m, q)
    p = decode(raw)
    fields(p, 'profile challenge rows')
    need(p['profile'] == PROFILE and p['challenge'] == digest('challenge', canonical(q)).hex(), 'CONTEXT')
    need(type(p['rows']) is list and len(p['rows']) == len(q['indices']), 'ROWS')
    for row, index in zip(p['rows'], q['indices']):
        fields(row, 'index data path')
        number(row['index'], 0, m['count']-1)
        need(row['index'] == index, 'INDEX')
        node = leaf(index, hx(row['data'], CHUNK))
        need(type(row['path']) is list and len(row['path']) == m['count'].bit_length()-1, 'PATH')
        cursor = index
        for sib in row['path']:
            sibling = hx(sib, 32)
            node = digest('node', sibling, node) if cursor & 1 else digest('node', node, sibling)
            cursor //= 2
        need(node.hex() == m['root'], 'MEMBERSHIP')
    return digest('proof', raw).hex()

def full_recovery(m, read):
    validate(m)
    # Bounded leaf hashes only; no second full corpus in verifier memory.
    leaves = [leaf(i, read(i)) for i in range(m['count'])]
    while len(leaves) > 1:
        leaves = [digest('node', leaves[i], leaves[i+1]) for i in range(0, len(leaves), 2)]
    need(leaves[0].hex() == m['root'], 'FULL_ROOT')
    return m['root']

class Verifier:
    """One experiment verifier; monotonic deadlines, no self-authorized root."""
    def __init__(self, assigned, window_ns=1_000_000_000, clock=time.monotonic_ns):
        validate(assigned)
        number(window_ns, 1, 60_000_000_000)
        self.assigned = dict(assigned)
        self.window, self.clock = window_ns, clock
        self.committed = False
        self.counter = 0
        self.pending = None

    def commit(self, m):
        need(not self.committed and m == self.assigned, 'ASSIGNMENT')
        self.committed = True

    def issue(self, samples=64):
        need(self.committed, 'NOT_COMMITTED')
        need(self.pending is None, 'OUTSTANDING')
        self.counter += 1
        q = challenge(self.assigned, secrets.token_hex(32), self.counter, samples)
        self.pending = (q, self.clock())
        return json.loads(canonical(q))

    def finish(self, raw):
        need(self.pending is not None, 'REPLAY_OR_UNISSUED')
        q, start = self.pending
        self.pending = None  # failed responses also consume the challenge
        elapsed = self.clock() - start
        need(0 <= elapsed <= self.window, 'TIMEOUT')
        need(raw is not None, 'MISSING_RESPONSE')
        return verify(raw, self.assigned, q)

class FileReader:
    def __init__(self, path):
        self.path = Path(path)
    def __call__(self, index):
        try:
            with self.path.open('rb') as f:
                f.seek(index * CHUNK)
                return f.read(CHUNK)
        except FileNotFoundError:
            return b''
