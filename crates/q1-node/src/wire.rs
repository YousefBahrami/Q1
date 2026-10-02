//! Loopback transport, separate from signed consensus objects.

use crate::Result;
use q1_primitives::cbor::{self, Value};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

pub const STATUS: u64 = 0;
pub const SUBMIT: u64 = 1;
pub const PROPOSE: u64 = 2;
pub const COMMIT: u64 = 3;
pub const SYNC: u64 = 4;
pub const ERROR: u64 = 255;
// Local transport bounds, not production-network protocol limits.
const MAX_FRAME: usize = cbor::MAX_OBJECT_SIZE;
const TIMEOUT: Duration = Duration::from_secs(15);

pub fn address(value: &str) -> Result<SocketAddr> {
    let address: SocketAddr = value.parse()?;
    if !address.ip().is_loopback() || address.port() == 0 {
        return Err("LOCALNET requires an explicit nonzero loopback address".into());
    }
    Ok(address)
}

pub fn configure(stream: &TcpStream) -> Result<()> {
    stream.set_read_timeout(Some(TIMEOUT))?;
    stream.set_write_timeout(Some(TIMEOUT))?;
    Ok(())
}

pub fn read(stream: &mut TcpStream, genesis: &[u8; 32]) -> Result<(u64, Vec<u8>)> {
    let mut size = [0; 4];
    stream.read_exact(&mut size)?;
    let size = u32::from_be_bytes(size) as usize;
    if size > MAX_FRAME {
        return Err("transport frame exceeds local limit".into());
    }
    let mut bytes = vec![0; size];
    stream.read_exact(&mut bytes)?;
    let Value::Array(values) = cbor::decode(&bytes)? else {
        return Err("invalid transport envelope".into());
    };
    match values.as_slice() {
        [
            Value::Unsigned(1),
            Value::Bytes(id),
            Value::Unsigned(op),
            Value::Bytes(payload),
        ] if id == genesis => Ok((*op, payload.clone())),
        _ => Err("invalid version, genesis binding, or transport fields".into()),
    }
}

pub fn write(stream: &mut TcpStream, genesis: &[u8; 32], op: u64, payload: Vec<u8>) -> Result<()> {
    let bytes = cbor::encode(&Value::Array(vec![
        Value::Unsigned(1),
        Value::Bytes(genesis.to_vec()),
        Value::Unsigned(op),
        Value::Bytes(payload),
    ]))?;
    if bytes.len() > MAX_FRAME {
        return Err("transport frame exceeds local limit".into());
    }
    stream.write_all(&(bytes.len() as u32).to_be_bytes())?;
    stream.write_all(&bytes)?;
    Ok(())
}

pub fn request(peer: SocketAddr, genesis: &[u8; 32], op: u64, payload: Vec<u8>) -> Result<Vec<u8>> {
    let mut stream = TcpStream::connect_timeout(&peer, TIMEOUT)?;
    configure(&stream)?;
    write(&mut stream, genesis, op, payload)?;
    let (response_op, response) = read(&mut stream, genesis)?;
    if response_op == ERROR {
        return Err(String::from_utf8_lossy(&response).into_owned().into());
    }
    if response_op != op {
        return Err("unexpected response opcode".into());
    }
    Ok(response)
}
