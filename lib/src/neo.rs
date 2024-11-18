use base64;
use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug)]
struct OpcodeData {
    name: &'static str,
    size: usize,
}

static OPCODETABLE: Lazy<HashMap<u8, OpcodeData>> = Lazy::new(|| {
    [
        (
            0x00,
            OpcodeData {
                name: "PUSHINT8",
                size: 1,
            },
        ),
        (
            0x01,
            OpcodeData {
                name: "PUSHINT16",
                size: 2,
            },
        ),
        (
            0x02,
            OpcodeData {
                name: "PUSHINT32",
                size: 4,
            },
        ),
        (
            0x03,
            OpcodeData {
                name: "PUSHINT64",
                size: 8,
            },
        ),
        (
            0x04,
            OpcodeData {
                name: "PUSHINT128",
                size: 16,
            },
        ),
        (
            0x05,
            OpcodeData {
                name: "PUSHINT256",
                size: 32,
            },
        ),
        (
            0x08,
            OpcodeData {
                name: "PUSHT",
                size: 0,
            },
        ),
        (
            0x09,
            OpcodeData {
                name: "PUSHF",
                size: 0,
            },
        ),
        (
            0x0a,
            OpcodeData {
                name: "PUSHA",
                size: 4,
            },
        ),
        (
            0x0b,
            OpcodeData {
                name: "PUSHNULL",
                size: 0,
            },
        ),
        (
            0x0c,
            OpcodeData {
                name: "PUSHDATA1",
                size: 1,
            },
        ),
        (
            0x0d,
            OpcodeData {
                name: "PUSHDATA2",
                size: 2,
            },
        ),
        (
            0x0e,
            OpcodeData {
                name: "PUSHDATA4",
                size: 4,
            },
        ),
        (
            0x0f,
            OpcodeData {
                name: "PUSHM1",
                size: 0,
            },
        ),
        (
            0x10,
            OpcodeData {
                name: "PUSH0",
                size: 0,
            },
        ),
        (
            0x11,
            OpcodeData {
                name: "PUSH1",
                size: 0,
            },
        ),
        (
            0x12,
            OpcodeData {
                name: "PUSH2",
                size: 0,
            },
        ),
        (
            0x13,
            OpcodeData {
                name: "PUSH3",
                size: 0,
            },
        ),
        (
            0x14,
            OpcodeData {
                name: "PUSH4",
                size: 0,
            },
        ),
        (
            0x15,
            OpcodeData {
                name: "PUSH5",
                size: 0,
            },
        ),
        (
            0x16,
            OpcodeData {
                name: "PUSH6",
                size: 0,
            },
        ),
        (
            0x17,
            OpcodeData {
                name: "PUSH7",
                size: 0,
            },
        ),
        (
            0x18,
            OpcodeData {
                name: "PUSH8",
                size: 0,
            },
        ),
        (
            0x19,
            OpcodeData {
                name: "PUSH9",
                size: 0,
            },
        ),
        (
            0x1a,
            OpcodeData {
                name: "PUSH10",
                size: 0,
            },
        ),
        (
            0x1b,
            OpcodeData {
                name: "PUSH11",
                size: 0,
            },
        ),
        (
            0x1c,
            OpcodeData {
                name: "PUSH12",
                size: 0,
            },
        ),
        (
            0x1d,
            OpcodeData {
                name: "PUSH13",
                size: 0,
            },
        ),
        (
            0x1e,
            OpcodeData {
                name: "PUSH14",
                size: 0,
            },
        ),
        (
            0x1f,
            OpcodeData {
                name: "PUSH15",
                size: 0,
            },
        ),
        (
            0x20,
            OpcodeData {
                name: "PUSH16",
                size: 0,
            },
        ),
        (
            0x21,
            OpcodeData {
                name: "NOP",
                size: 0,
            },
        ),
        (
            0x22,
            OpcodeData {
                name: "JMP",
                size: 1,
            },
        ),
        (
            0x23,
            OpcodeData {
                name: "JMP_L",
                size: 4,
            },
        ),
        (
            0x24,
            OpcodeData {
                name: "JMPIF",
                size: 1,
            },
        ),
        (
            0x25,
            OpcodeData {
                name: "JMPIF_L",
                size: 4,
            },
        ),
        (
            0x26,
            OpcodeData {
                name: "JMPIFNOT",
                size: 1,
            },
        ),
        (
            0x27,
            OpcodeData {
                name: "JMPIFNOT_L",
                size: 4,
            },
        ),
        (
            0x28,
            OpcodeData {
                name: "JMPEQ",
                size: 1,
            },
        ),
        (
            0x29,
            OpcodeData {
                name: "JMPEQ_L",
                size: 4,
            },
        ),
        (
            0x2a,
            OpcodeData {
                name: "JMPNE",
                size: 1,
            },
        ),
        (
            0x2b,
            OpcodeData {
                name: "JMPNE_L",
                size: 4,
            },
        ),
        (
            0x2c,
            OpcodeData {
                name: "JMPGT",
                size: 1,
            },
        ),
        (
            0x2d,
            OpcodeData {
                name: "JMPGT_L",
                size: 4,
            },
        ),
        (
            0x2e,
            OpcodeData {
                name: "JMPGE",
                size: 1,
            },
        ),
        (
            0x2f,
            OpcodeData {
                name: "JMPGE_L",
                size: 4,
            },
        ),
        (
            0x30,
            OpcodeData {
                name: "JMPLT",
                size: 1,
            },
        ),
        (
            0x31,
            OpcodeData {
                name: "JMPLT_L",
                size: 4,
            },
        ),
        (
            0x32,
            OpcodeData {
                name: "JMPLE",
                size: 1,
            },
        ),
        (
            0x33,
            OpcodeData {
                name: "JMPLE_L",
                size: 4,
            },
        ),
        (
            0x34,
            OpcodeData {
                name: "CALL",
                size: 1,
            },
        ),
        (
            0x35,
            OpcodeData {
                name: "CALL_L",
                size: 4,
            },
        ),
        (
            0x36,
            OpcodeData {
                name: "CALLA",
                size: 0,
            },
        ),
        (
            0x37,
            OpcodeData {
                name: "CALLT",
                size: 2,
            },
        ),
        (
            0x38,
            OpcodeData {
                name: "ABORT",
                size: 0,
            },
        ),
        (
            0x39,
            OpcodeData {
                name: "ASSERT",
                size: 0,
            },
        ),
        (
            0x3a,
            OpcodeData {
                name: "THROW",
                size: 0,
            },
        ),
        (
            0x3b,
            OpcodeData {
                name: "TRY",
                size: 2,
            },
        ),
        (
            0x3c,
            OpcodeData {
                name: "TRY_L",
                size: 8,
            },
        ),
        (
            0x3d,
            OpcodeData {
                name: "ENDTRY",
                size: 1,
            },
        ),
        (
            0x3e,
            OpcodeData {
                name: "ENDTRY_L",
                size: 4,
            },
        ),
        (
            0x3f,
            OpcodeData {
                name: "ENDFINALLY",
                size: 0,
            },
        ),
        (
            0x40,
            OpcodeData {
                name: "RET",
                size: 0,
            },
        ),
        (
            0x41,
            OpcodeData {
                name: "SYSCALL",
                size: 4,
            },
        ),
        (
            0x43,
            OpcodeData {
                name: "DEPTH",
                size: 0,
            },
        ),
        (
            0x45,
            OpcodeData {
                name: "DROP",
                size: 0,
            },
        ),
        (
            0x46,
            OpcodeData {
                name: "NIP",
                size: 0,
            },
        ),
        (
            0x48,
            OpcodeData {
                name: "XDROP",
                size: 0,
            },
        ),
        (
            0x49,
            OpcodeData {
                name: "CLEAR",
                size: 0,
            },
        ),
        (
            0x4a,
            OpcodeData {
                name: "DUP",
                size: 0,
            },
        ),
        (
            0x4b,
            OpcodeData {
                name: "OVER",
                size: 0,
            },
        ),
        (
            0x4d,
            OpcodeData {
                name: "PICK",
                size: 0,
            },
        ),
        (
            0x4e,
            OpcodeData {
                name: "TUCK",
                size: 0,
            },
        ),
        (
            0x50,
            OpcodeData {
                name: "SWAP",
                size: 0,
            },
        ),
        (
            0x51,
            OpcodeData {
                name: "ROT",
                size: 0,
            },
        ),
        (
            0x52,
            OpcodeData {
                name: "ROLL",
                size: 0,
            },
        ),
        (
            0x53,
            OpcodeData {
                name: "REVERSE3",
                size: 0,
            },
        ),
        (
            0x54,
            OpcodeData {
                name: "REVERSE4",
                size: 0,
            },
        ),
        (
            0x55,
            OpcodeData {
                name: "REVERSEN",
                size: 0,
            },
        ),
        (
            0x56,
            OpcodeData {
                name: "INITSSLOT",
                size: 1,
            },
        ),
        (
            0x57,
            OpcodeData {
                name: "INITSLOT",
                size: 2,
            },
        ),
    ]
    .into()
});

pub const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn base64_to_hex(encoded: &str) -> String {
    let bytes = base64::decode(encoded).unwrap();
    hex::encode(bytes)
}

pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    base64::encode(&bytes)
}

pub fn hex_decode(encoded: &str) -> Vec<u8> {
    hex::decode(encoded).unwrap()
}

pub fn base64_to_script_hash(encoded: &str) -> String {
    let hex = base64_to_hex(encoded);
    let mut value = hex::decode(hex).unwrap();
    value.reverse();
    format!("0x{}", hex::encode(value))
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
    let script_hash = base64_to_hex(encoded);
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

pub fn neo3_disassemble(base64_encoded_script: &str) -> String {
    let mut out = String::new();
    let script = base64::decode(base64_encoded_script).expect("Invalid base64 encoding");

    let interopmethod: HashMap<u32, String> = HashMap::new();

    let mut ip = 0;

    while ip < script.len() {
        let opcode = script[ip];
        if let Some(opcodedata) = OPCODETABLE.get(&opcode) {
            let inst = &opcodedata.name;

            if inst == &"SYSCALL" {
                let hash = u32::from_le_bytes([
                    script[ip + 1],
                    script[ip + 2],
                    script[ip + 3],
                    script[ip + 4],
                ]);
                let interop_name = if let Some(name) = interopmethod.get(&hash) {
                    name.clone()
                } else {
                    hash.to_string()
                };
                out.push_str(&format!("{} {}\n", inst, interop_name));
                ip += 4;
            } else if opcodedata.size == 0 {
                out.push_str(&format!("{}\n", inst));
            } else {
                if inst == &"PUSHDATA1" || inst == &"PUSHDATA2" || inst == &"PUSHDATA4" {
                    let data_size = match opcodedata.size {
                        1 => script[ip + 1] as usize,
                        2 => u16::from_le_bytes([script[ip + 1], script[ip + 2]]) as usize,
                        4 => u32::from_le_bytes([
                            script[ip + 1],
                            script[ip + 2],
                            script[ip + 3],
                            script[ip + 4],
                        ]) as usize,
                        _ => {
                            out.push_str(&format!(
                                "SOMEBODY MESSED UP THE PUSHDATA SIZE for {} at index {} (size {})\n",
                                opcodedata.name, ip, opcodedata.size
                            ));
                            return out;
                        }
                    };

                    let data_start_idx = ip + opcodedata.size + 1;
                    let data = &script[data_start_idx..data_start_idx + data_size];
                    out.push_str(&format!("{} {}\n", inst, hex::encode(data)));
                    ip += opcodedata.size + data_size;
                } else {
                    let data = &script[ip + 1..ip + 1 + opcodedata.size];
                    out.push_str(&format!("{} {}\n", inst, hex::encode(data)));
                    ip += opcodedata.size;
                }
            }
        } else {
            out.push_str(&format!("INVALID OPCODE {}\n", opcode));
        }
        ip += 1;
    }
    out
}
