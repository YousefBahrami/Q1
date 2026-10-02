// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
"use strict";

// Non-protocol pre-M1 conformance experiment. No external npm dependencies.
const assert = require("node:assert/strict");
const crypto = require("node:crypto");

const ALPHABET = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
const GENERATORS = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
const VALID_CBOR = [
  "00", "17", "1818", "190100", "40", "4200ff", "60", "627131",
  "82014200ff", "8301f4f6",
];
const INVALID_CBOR = [
  "1801", "1817", "190018", "5f4100ff", "7f6171ff", "9f01ff", "a0",
  "a201000101", "c001", "f90000", "8201", "0102", "63713100", "ff",
];
const VALID_AMOUNTS = new Map([
  [0n, "5000000000000000000000000000000000"],
  [1n, "5000000000000000000000000000000001"],
  [255n, "50000000000000000000000000000000ff"],
  [256n, "5000000000000000000000000000000100"],
  [4294967295n, "50000000000000000000000000ffffffff"],
  [18446744073709551615n, "500000000000000000ffffffffffffffff"],
  [18446744073709551616n, "5000000000000000010000000000000000"],
  [1n << 127n, "5080000000000000000000000000000000"],
  [(1n << 128n) - 1n, "50ffffffffffffffffffffffffffffffff"],
]);
const INVALID_AMOUNTS = [
  "40", "4100", "480000000000000000", "4f000000000000000000000000000000",
  "510000000000000000000000000000000000", "00", "1bffffffffffffffff",
  "5f5000000000000000000000000000000000ff", "d84050000000000000000000000000000000",
  "20", "f90000",
];

function encodeAmount(value) {
  if (value < 0n || value >= (1n << 128n)) throw new Error("amount range");
  return Buffer.concat([Buffer.from([0x50]), Buffer.from(value.toString(16).padStart(32, "0"), "hex")]);
}

function decodeAmount(bytes) {
  if (bytes.length !== 17 || bytes[0] !== 0x50) throw new Error("amount encoding");
  return BigInt(`0x${bytes.subarray(1).toString("hex")}`);
}

function domainFrame(domainId, payload) {
  const header = Buffer.alloc(16);
  header.write("Q1DS", 0, "ascii");
  header.writeUInt16BE(1, 4);
  header.writeUInt16BE(domainId, 6);
  header.writeBigUInt64BE(BigInt(payload.length), 8);
  return Buffer.concat([header, payload]);
}

function sha256(value) {
  return crypto.createHash("sha256").update(value).digest();
}

function polymod(values) {
  let checksum = 1;
  for (const value of values) {
    const top = checksum >>> 25;
    checksum = (((checksum & 0x01ffffff) << 5) ^ value) >>> 0;
    for (let index = 0; index < 5; index += 1) {
      if (((top >>> index) & 1) !== 0) checksum = (checksum ^ GENERATORS[index]) >>> 0;
    }
  }
  return checksum >>> 0;
}

function hrpExpand(hrp) {
  return [
    ...Buffer.from(hrp, "ascii").map((byte) => byte >>> 5),
    0,
    ...Buffer.from(hrp, "ascii").map((byte) => byte & 31),
  ];
}

function convert8to5(bytes) {
  let accumulator = 0;
  let bits = 0;
  const output = [];
  for (const byte of bytes) {
    accumulator = ((accumulator << 8) | byte) >>> 0;
    bits += 8;
    while (bits >= 5) {
      bits -= 5;
      output.push((accumulator >>> bits) & 31);
    }
  }
  if (bits !== 0) output.push((accumulator << (5 - bits)) & 31);
  return output;
}

function convert5to8(values) {
  let accumulator = 0;
  let bits = 0;
  const output = [];
  for (const value of values) {
    if (value < 0 || value > 31) throw new Error("data range");
    accumulator = ((accumulator << 5) | value) >>> 0;
    bits += 5;
    while (bits >= 8) {
      bits -= 8;
      output.push((accumulator >>> bits) & 255);
    }
  }
  if (bits > 4 || ((accumulator << (8 - bits)) & 255) !== 0) throw new Error("padding");
  return Buffer.from(output);
}

function encodeBech32mData(hrp, data) {
  const residue = (polymod([...hrpExpand(hrp), ...data, 0, 0, 0, 0, 0, 0]) ^ 0x2bc830a3) >>> 0;
  const checksum = [];
  for (let index = 0; index < 6; index += 1) {
    checksum.push((residue >>> (5 * (5 - index))) & 31);
  }
  return `${hrp}1${[...data, ...checksum].map((value) => ALPHABET[value]).join("")}`;
}

function encodeBech32m(hrp, payload) {
  return encodeBech32mData(hrp, convert8to5(payload));
}

function decodeAddress(text, expectedHrp) {
  if (!/^[\x21-\x7e]+$/.test(text) || text !== text.toLowerCase() || text.length > 90) {
    throw new Error("text policy");
  }
  const separator = text.lastIndexOf("1");
  if (separator < 1 || separator + 7 > text.length) throw new Error("separator");
  const hrp = text.slice(0, separator);
  if (!["q1l", "q1p", "q1r"].includes(hrp) || hrp !== expectedHrp) throw new Error("network");
  const data = [...text.slice(separator + 1)].map((character) => {
    const value = ALPHABET.indexOf(character);
    if (value < 0) throw new Error("alphabet");
    return value;
  });
  if (polymod([...hrpExpand(hrp), ...data]) !== 0x2bc830a3) throw new Error("checksum");
  const payload = convert5to8(data.slice(0, -6));
  if (payload.length !== 36) throw new Error("length");
  if (payload[0] !== 1 || payload[1] !== 1 || payload.readUInt16BE(2) !== 1) {
    throw new Error("unsupported fields");
  }
  return payload;
}

function readArgument(input, state, additional) {
  if (additional <= 23) return additional;
  const width = ({ 24: 1, 25: 2, 26: 4, 27: 8 })[additional];
  if (!width || state.cursor + width > input.length) throw new Error("argument");
  let value = 0n;
  for (let index = 0; index < width; index += 1) {
    value = (value << 8n) | BigInt(input[state.cursor + index]);
  }
  state.cursor += width;
  const minimum = ({ 1: 24n, 2: 256n, 4: 65536n, 8: 4294967296n })[width];
  if (value < minimum) throw new Error("non-shortest");
  if (value > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("research numeric ceiling");
  return Number(value);
}

function parseCbor(input, state, depth) {
  if (depth > 16 || state.cursor >= input.length) throw new Error("depth/truncated");
  const initial = input[state.cursor++];
  const major = initial >>> 5;
  const additional = initial & 31;
  if (major === 0) {
    readArgument(input, state, additional);
  } else if (major === 2 || major === 3) {
    const length = readArgument(input, state, additional);
    if (length > (major === 2 ? 16777216 : 128) || state.cursor + length > input.length) {
      throw new Error("string");
    }
    const bytes = input.subarray(state.cursor, state.cursor + length);
    state.cursor += length;
    if (major === 3 && ![...bytes].every((byte) => byte >= 0x21 && byte <= 0x7e)) {
      throw new Error("text");
    }
  } else if (major === 4) {
    const length = readArgument(input, state, additional);
    if (length > 65535) throw new Error("array");
    for (let index = 0; index < length; index += 1) parseCbor(input, state, depth + 1);
  } else if (major === 7 && [20, 21, 22].includes(additional)) {
    // Schema-position checks are outside this primitive corpus.
  } else {
    throw new Error("prohibited type");
  }
}

function validProfileItem(bytes) {
  try {
    if (bytes.length > 16777216) return false;
    const state = { cursor: 0 };
    parseCbor(bytes, state, 1);
    return state.cursor === bytes.length;
  } catch {
    return false;
  }
}

const seed = Buffer.from("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f", "hex");
const pkcs8 = Buffer.concat([Buffer.from("302e020100300506032b657004220420", "hex"), seed]);
const privateKey = crypto.createPrivateKey({ key: pkcs8, format: "der", type: "pkcs8" });
const publicKey = crypto.createPublicKey(privateKey).export({ format: "der", type: "spki" }).subarray(-32);
const message = domainFrame(1, Buffer.from("82014200ff", "hex"));
const signature = crypto.sign(null, message, privateKey);
assert.equal(publicKey.toString("hex"), "03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8");
assert.equal(signature.toString("hex"), "355e9ab16419b61d545e9d0e61402352d85ea87d0335eafbf9ec0c1c9a9bcb9e7d22645a93042a8632973d83e8fb5a83945dd53f09e0b288723513bdcb32e908");
assert(crypto.verify(null, message, crypto.createPublicKey(privateKey), signature));
const mutatedMessage = Buffer.from(message);
mutatedMessage[mutatedMessage.length - 1] ^= 1;
assert(!crypto.verify(null, mutatedMessage, crypto.createPublicKey(privateKey), signature));
const mutatedSignature = Buffer.from(signature);
mutatedSignature[0] ^= 1;
assert(!crypto.verify(null, message, crypto.createPublicKey(privateKey), mutatedSignature));

for (const vector of VALID_CBOR) assert(validProfileItem(Buffer.from(vector, "hex")), `valid ${vector}`);
for (const vector of INVALID_CBOR) assert(!validProfileItem(Buffer.from(vector, "hex")), `invalid ${vector}`);
for (const [value, vector] of VALID_AMOUNTS) {
  assert.equal(encodeAmount(value).toString("hex"), vector);
  assert.equal(decodeAmount(Buffer.from(vector, "hex")), value);
}
for (const vector of INVALID_AMOUNTS) assert.throws(() => decodeAmount(Buffer.from(vector, "hex")));

const accountId = sha256(domainFrame(15, publicKey));
const binaryPayload = Buffer.concat([Buffer.from([1, 1, 0, 1]), accountId]);
const addresses = Object.fromEntries(["q1l", "q1p", "q1r"].map((hrp) => [hrp, encodeBech32m(hrp, binaryPayload)]));
for (const hrp of Object.keys(addresses)) assert.deepEqual(decodeAddress(addresses[hrp], hrp), binaryPayload);
assert.throws(() => decodeAddress(addresses.q1l, "q1p"));
assert.throws(() => decodeAddress(addresses.q1l.slice(0, -1) + "q", "q1l"));
assert.throws(() => decodeAddress(addresses.q1l.toUpperCase(), "q1l"));
assert.throws(() => decodeAddress(` ${addresses.q1l}`, "q1l"));
const wrongVersion = Buffer.from(binaryPayload);
wrongVersion[0] = 2;
const wrongVersionAddress = encodeBech32m("q1l", wrongVersion);
assert.throws(() => decodeAddress(wrongVersionAddress, "q1l"));
const wrongAlgorithm = Buffer.from(binaryPayload);
wrongAlgorithm.writeUInt16BE(2, 2);
const wrongAlgorithmAddress = encodeBech32m("q1l", wrongAlgorithm);
const wrongLengthAddress = encodeBech32m("q1l", binaryPayload.subarray(0, 35));
const invalidPaddingAddress = encodeBech32mData("q1l", [...convert8to5(binaryPayload), 1]);
assert.throws(() => decodeAddress(wrongAlgorithmAddress, "q1l"));
assert.throws(() => decodeAddress(wrongLengthAddress, "q1l"));
assert.throws(() => decodeAddress(invalidPaddingAddress, "q1l"));

console.log(`node=${process.version}`);
console.log(`node_openssl=${process.versions.openssl}`);
console.log("node_crypto_impl=OpenSSL-backed node:crypto Ed25519");
console.log(`message=${message.toString("hex")}`);
console.log(`public_key=${publicKey.toString("hex")}`);
console.log(`signature=${signature.toString("hex")}`);
console.log(`account_id=${accountId.toString("hex")}`);
for (const [hrp, address] of Object.entries(addresses)) console.log(`${hrp}=${address}`);
console.log(`invalid_checksum=${addresses.q1l.slice(0, -1)}q`);
console.log(`invalid_mixed_case=Q${addresses.q1l.slice(1)}`);
console.log(`invalid_padding=${invalidPaddingAddress}`);
console.log(`unsupported_version=${wrongVersionAddress}`);
console.log(`unsupported_algorithm=${wrongAlgorithmAddress}`);
console.log(`incorrect_length=${wrongLengthAddress}`);
console.log(`cbor_valid=${VALID_CBOR.length}`);
console.log(`cbor_rejected=${INVALID_CBOR.length}`);
console.log(`amount_valid=${VALID_AMOUNTS.size}`);
console.log(`amount_rejected=${INVALID_AMOUNTS.length}`);
