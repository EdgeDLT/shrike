use shrike_lib::neo::ALPHABET;

pub fn is_neo_address(string: &str) -> bool {
    string.chars().count() == 34 && string.starts_with('N') && string.chars().all(|c| ALPHABET.contains(&(c as u8)))
}

#[allow(dead_code)]
pub fn is_neo_script_hash(string: &str) -> bool {
    string.chars().count() == 42 && string.starts_with("0x") && string.chars().skip(2).take(40).all(|c| c.is_ascii_hexdigit())
}

pub fn is_neo_txid_hash(string: &str) -> bool {
    string.chars().count() == 66 && string.starts_with("0x") && string.chars().skip(2).take(64).all(|c| c.is_ascii_hexdigit())
}

#[test]
fn test_is_neo_address() {
    assert!(is_neo_address("NSTSntFPK36QXsjEK6oAhnPzSyfgfVA2GQ"));
    assert!(!is_neo_address("NSTSntFPK36QXsjEK6oAhnPzSyfgfVA2GQ1"));
    assert!(!is_neo_address("NSTSntFPK36QXsjEK6oAhnPzSyfgfVA2G"));
    assert!(!is_neo_address("NSTSntFPK36QXsjEK6OAhnPzSyfgfVA2GQ"));
}

#[test]
fn test_is_neo_script_hash() {
    assert!(is_neo_script_hash("0x6250481ec87ae2052f90ec7cb46d757b8db1c447"));
    assert!(!is_neo_script_hash("0x6250481ec/7ae2052f90ec7cb46#757b8db1c447"));
    assert!(!is_neo_script_hash("NSTSntFPK36QXsjEK6oAhnPzSyfgfVA2GQ"));
    assert!(!is_neo_script_hash("0000000000000000000000000000000000000000"));
    assert!(!is_neo_script_hash("0x3184522d88fe2a5e0ac35324d119e6e82b3df7d7c95c927c247155467d49e8ba"));
}

#[test]
fn test_is_neo_txid_hash() {
    assert!(is_neo_txid_hash("0x3184522d88fe2a5e0ac35324d119e6e82b3df7d7c95c927c247155467d49e8ba"));
    assert!(!is_neo_txid_hash("0x31*4522d88fe2a5e0ac35324d119e6e82b3df7d7c95@927c247155467d49e8ba"));
    assert!(!is_neo_txid_hash("NSTSntFPK36QXsjEK6oAhnPzSyfgfVA2GQ"));
    assert!(!is_neo_txid_hash("0x00000000000000000000000000000000000000000"));
    assert!(!is_neo_txid_hash("0x6250481ec87ae2052f90ec7cb46d757b8db1c447"));
}
