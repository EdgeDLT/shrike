use base64;
use sha2::{Digest, Sha256};

pub const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn base64_to_scripthash(encoded: &str) -> String {
    let bytes = base64::decode(encoded).unwrap();
    hex::encode(bytes)
}

pub fn scripthash_to_address(script_hash: &str) -> String {
    let script_hash = hex::decode(script_hash).unwrap();

    let mut addr = [0u8; 25];
    addr[0] = 53;
    addr[1..21].copy_from_slice(&script_hash);

    let sum = &checksum(&addr[0..21])[0..4];
    addr[21..25].copy_from_slice(sum);

    bytes_to_base58(&addr)
}

pub fn base64_to_address(encoded: &str) -> String {
    let script_hash = base64_to_scripthash(encoded);
    scripthash_to_address(&script_hash)
}

pub fn checksum(data: &[u8]) -> Vec<u8> {
    Sha256::digest(Sha256::digest(data)).to_vec()
}

#[allow(dead_code)]
pub fn reverse_hex(hex: &str) -> String {
    let mut value = hex::decode(hex).unwrap();
    value.reverse();

    hex::encode(value)
}

pub fn bytes_to_base58(bytes: &[u8]) -> String {
    let zcount = bytes.iter().take_while(|x| **x == 0).count();
    let size = (bytes.len() - zcount) * 138 / 100 + 1;
    let mut buffer = vec![0u8; size];

    let mut i = zcount;
    let mut high = size - 1;

    while i < bytes.len() {
        let mut carry = bytes[i] as u32;
        let mut j = size - 1;

        while j > high || carry != 0 {
            carry += 256 * buffer[j] as u32;
            buffer[j] = (carry % 58) as u8;
            carry /= 58;

            if j > 0 {
                j = j.saturating_sub(1);
            }
        }

        i += 1;
        high = j;
    }

    let mut j = buffer.iter().take_while(|x| **x == 0).count();

    let mut result = String::new();
    for _ in 0..zcount {
        result.push('1');
    }

    while j < size {
        result.push(ALPHABET[buffer[j] as usize] as char);
        j += 1;
    }

    result
}

// added this then realized I didn't need it... oh well, one day maybe
#[allow(clippy::same_item_push)]
pub fn base58_to_bytes(base58: &str) -> Vec<u8> {
    let zcount = base58.chars().take_while(|x| *x == '1').count();
    let size = (base58.len() - zcount) * 733 / 1000 + 1;
    let mut buffer = vec![0u8; size];

    let mut i = zcount;
    let mut high = size - 1;

    while i < base58.len() {
        let mut carry = ALPHABET
            .iter()
            .position(|&x| x == base58.as_bytes()[i])
            .unwrap() as u32;
        let mut j = size - 1;

        while j > high || carry != 0 {
            carry += 58 * buffer[j] as u32;
            buffer[j] = (carry % 256) as u8;
            carry /= 256;

            if j > 0 {
                j = j.saturating_sub(1);
            }
        }

        i += 1;
        high = j;
    }

    let mut j = buffer.iter().take_while(|x| **x == 0).count();

    let mut result = Vec::new();
    for _ in 0..zcount {
        result.push(0);
    }

    while j < size {
        result.push(buffer[j]);
        j += 1;
    }

    result
}

pub fn address_to_base64(address: &str) -> String {
    let bytes = base58_to_bytes(address);
    base64::encode(&bytes[1..21])
}
