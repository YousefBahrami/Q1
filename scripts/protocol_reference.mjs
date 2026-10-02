#!/usr/bin/env node
// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
// Independent M1.2 reference: Node/OpenSSL, no Rust imports or third-party packages.
// Seeds below are PUBLIC TEST FIXTURES, never operational keys or nonce generation.
import assert from 'node:assert/strict';
import { createHash, createPrivateKey, createPublicKey, sign, verify } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const fixture = fileURLToPath(new URL('../vectors/protocol_objects/v1/approved.tsv', import.meta.url));
const concat = (...parts) => Buffer.concat(parts);
const be = (value, width) => {
  let remaining = BigInt(value);
  assert(remaining >= 0n && remaining < 1n << BigInt(width * 8));
  const result = Buffer.alloc(width);
  for (let index = width - 1; index >= 0; --index) {
    result[index] = Number(remaining & 255n);
    remaining >>= 8n;
  }
  return result;
};
const head = (major, value) => {
  value = BigInt(value);
  if (value < 24n) return Buffer.from([major * 32 + Number(value)]);
  for (const [width, tag] of [[1, 24], [2, 25], [4, 26], [8, 27]]) {
    if (value < 1n << BigInt(width * 8)) return concat(Buffer.from([major * 32 + tag]), be(value, width));
  }
  throw new Error('CBOR integer exceeds u64');
};
function cbor(value) {
  if (value === null) return Buffer.from([0xf6]);
  if (Buffer.isBuffer(value)) return concat(head(2, value.length), value);
  if (Array.isArray(value)) return concat(head(4, value.length), ...value.map(cbor));
  assert(typeof value === 'bigint' || (Number.isSafeInteger(value) && value >= 0));
  return head(0, value);
}

// A separate strict decoder checks canonical primitives used by these objects.
function decode(bytes) {
  let offset = 0;
  const take = count => {
    assert(offset + count <= bytes.length, 'truncated CBOR');
    const result = bytes.subarray(offset, offset + count);
    offset += count;
    return result;
  };
  function item(depth) {
    assert(depth <= 16);
    const initial = take(1)[0];
    if (initial === 0xf6) return null;
    const major = initial >> 5, extra = initial & 31;
    assert([0, 2, 4].includes(major));
    let argument = BigInt(extra);
    if (extra >= 24) {
      assert(extra <= 27, 'indefinite/reserved encoding');
      const width = 2 ** (extra - 24);
      argument = BigInt(`0x${take(width).toString('hex')}`);
      assert(argument >= [24n, 256n, 65536n, 4294967296n][extra - 24], 'nonminimal encoding');
    }
    if (major === 0) return argument;
    assert(argument <= BigInt(major === 4 ? 65535 : 16777216));
    const length = Number(argument);
    if (major === 2) return take(length);
    const result = [];
    for (let index = 0; index < length; ++index) result.push(item(depth + 1));
    return result;
  }
  assert(bytes.length <= 16777216);
  const value = item(1);
  assert.equal(offset, bytes.length, 'trailing bytes');
  return value;
}

const frame = (domain, payload) => concat(Buffer.from('Q1DS'), be(1, 2), be(domain, 2), be(payload.length, 8), payload);
const hash = (domain, payload) => createHash('sha256').update(frame(domain, payload)).digest();
function key(seedByte) {
  return createPrivateKey({ key: concat(Buffer.from('302e020100300506032b657004220420', 'hex'), Buffer.alloc(32, seedByte)), format: 'der', type: 'pkcs8' });
}
const publicBytes = privateKey => createPublicKey(privateKey).export({ format: 'der', type: 'spki' }).subarray(-32);
function merkle(profile, items) {
  if (items.length === 0) return hash(9, concat(be(profile, 2), be(0, 8)));
  let level = items.map((bytes, index) => hash(9, concat(be(profile, 2), be(index, 8), be(bytes.length, 8), bytes)));
  while (level.length > 1) {
    const next = [];
    for (let i = 0; i < level.length; i += 2) {
      next.push(i + 1 < level.length ? hash(10, concat(be(profile, 2), level[i], level[i + 1])) : level[i]);
    }
    level = next;
  }
  return level[0];
}

const senderKey = key(7), sender = publicBytes(senderKey), recipient = publicBytes(key(8));
const chain = [1, 1, Buffer.alloc(32, 9)];
const chainId = hash(0x10, cbor(chain));
const identity = [1, sender, null];
const participantId = hash(0x13, cbor(identity));
const participant = [1, participantId, sender, null, 0, null];
const otherId = hash(0x13, cbor([1, null, recipient]));
const participants = [participant, [1, otherId, null, recipient, 0, null]].sort((a, b) => Buffer.compare(a[1], b[1]));
const address = concat(Buffer.from([1, 1, 0, 1]), hash(15, recipient));
function transfer(nonce) {
  const body = [1, chainId, sender, address, be(123, 16), be(4, 16), nonce, 6, 100];
  const payload = frame(1, cbor(body));
  const signature = sign(null, payload, senderKey);
  assert(verify(null, payload, createPublicKey(senderKey), signature));
  const mutated = Buffer.from(payload);
  mutated[mutated.length - 1] ^= 1;
  assert(!verify(null, mutated, createPublicKey(senderKey), signature));
  return { body, payload, signature, signed: [body, 1, signature] };
}
const tx = transfer(5), tx2 = transfer(6);
const transfers = [tx.signed, tx2.signed, tx.signed];
const results = {
  chain_preimage: cbor(chain), chain_frame: frame(0x10, cbor(chain)), chain_id: chainId,
  participant_identity: cbor(identity), participant_id: participantId,
  participant_record: cbor(participant), participant_record_hash: hash(8, cbor(participant)),
  participant_set: cbor([1, 1, participants]), participant_root: merkle(3, participants.map(cbor)),
  transfer_body: cbor(tx.body), transfer_signing_payload: tx.payload,
  transfer_signature: tx.signature, signed_transfer: cbor(tx.signed), transfer_id: hash(2, cbor(tx.signed)),
  transfer2_signature: tx2.signature, transfer2_id: hash(2, cbor(tx2.signed)),
  block_body: cbor([1, transfers]), transaction_root: merkle(1, transfers.map(cbor)),
  empty_block_body: cbor([1, []]), empty_transaction_root: merkle(1, []),
  delay_none: cbor([1, 0, 0, Buffer.alloc(0), Buffer.alloc(0)]),
  delay_none_frame: frame(0x14, cbor([1, 0, 0, Buffer.alloc(0), Buffer.alloc(0)])),
  delay_none_hash: hash(0x14, cbor([1, 0, 0, Buffer.alloc(0), Buffer.alloc(0)])),
};
for (const name of ['chain_preimage', 'participant_identity', 'participant_record', 'participant_set', 'transfer_body', 'signed_transfer', 'block_body', 'empty_block_body']) {
  assert.deepEqual(cbor(decode(results[name])), results[name], name);
}
for (const hex of ['1801', '9fff', '82018000', '59000100', '82', 'a0', 'd800', '830119ffff']) {
  assert.throws(() => decode(Buffer.from(hex, 'hex')), hex);
}
const output = Object.keys(results).sort().map(name => `${name}\t${results[name].toString('hex')}\n`).join('');
const args = process.argv.slice(2);
if (args.length === 1 && args[0] === '--emit') {
  process.stdout.write(output);
} else {
  assert(args.length <= 1, 'usage: node scripts/protocol_reference.mjs [--emit|rust-output.tsv]');
  for (const path of [fixture, ...args]) assert.equal(output, readFileSync(path, 'utf8'), `reference mismatch: ${path}`);
  console.log(`Node/OpenSSL: ${Object.keys(results).length} protocol vectors, signatures and canonical rejection checks passed`);
}
