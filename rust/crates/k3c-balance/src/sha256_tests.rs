//! Tests zu `sha256.rs`: die Testvektoren aus FIPS 180-4.

use crate::sha256::{Sha256, sha256_hex};

#[test]
fn leerer_text() {
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn abc() {
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn zwei_bloecke() {
    assert_eq!(
        sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
}

/// Eine Million „a“, in ungleichen Stücken geschrieben (der Puffer läuft über Blockgrenzen).
#[test]
fn eine_million_a() {
    let mut h = Sha256::default();
    let chunk = [b'a'; 1000];
    for i in 0..1000 {
        h.write(&chunk[..if i % 2 == 0 { 1000 } else { 999 }]);
        if i % 2 == 1 {
            h.write(b"a");
        }
    }
    assert_eq!(
        h.finish_hex(),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}
