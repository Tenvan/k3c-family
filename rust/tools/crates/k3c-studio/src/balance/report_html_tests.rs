//! Ohne Go-Gegenstück: die kopierten Vorlagen und der eigene PNG-Zuschnitt. Die Go-Tests zum HTML-Bericht stehen wie in
//! Go bei den Richtwerten (`hints_tests.rs`).

use std::fs;

use super::super::report_inflate::{zlib, zlib_stored};
use super::super::report_png::{Rgba, cropped_data_uri, decode, encode};
use super::super::test_helpers::repo_root;
use super::fill;

/// Die eingebetteten Vorlagen sind Byte-Kopien der Go-Dateien, solange es sie gibt.
#[test]
fn vorlagen_wie_go() {
    let go = repo_root().join("tools/k3c-studio/internal/balance/report");
    if !go.exists() {
        return;
    }
    let own = repo_root().join("rust/tools/crates/k3c-studio/src/balance/report");
    let mut names = 0;
    for entry in fs::read_dir(&go).expect("Go-Vorlagen") {
        let name = entry.expect("Eintrag").file_name();
        assert_eq!(
            fs::read(own.join(&name)).expect("Kopie"),
            fs::read(go.join(&name)).expect("Go"),
            "{name:?}"
        );
        names += 1;
    }
    assert_eq!(names, 6);
}

#[test]
fn platzhalter_nur_einmal_ersetzt() {
    let out = fill("a __X__ b __Y__ c", &[("__X__", "__Y__"), ("__Y__", "2")]);
    assert_eq!(out, "a __Y__ b 2 c");
}

fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex"))
        .collect()
}

/// Feste und dynamische Huffman-Blöcke (erzeugt mit zlib), dazu der eigene unkomprimierte Strom.
#[test]
fn inflate_wie_zlib() {
    let fixed = hex("789ccb48cdc9c957c84090003a2e067d");
    assert_eq!(zlib(&fixed, 17).expect("fest"), b"hello hello hello");
    let dynamic = hex(concat!(
        "789c358f810903300cc36e951cffffc270b7512805cb6ad22aa58078821e2a5a081c3762cf07715952aee0fd024306a27f9464c454abbb",
        "2e0c924bb2a4fb20933df533cd1bbe73918d35ce9bfd41ae99eedea8873da4bfe2db64fac4dab743d1ec7c004d285c07"
    ));
    // dieselbe Folge wie beim Erzeugen des Stroms
    let want: Vec<u8> = std::iter::successors(Some(1u32), |x| Some((x * 75 + 74) % 65537))
        .skip(1)
        .take(240)
        .map(|x| b"aaaabbbcde"[(x % 10) as usize])
        .collect();
    assert_eq!(zlib(&dynamic, want.len()), Some(want));
    let long = vec![7u8; 70_000];
    assert_eq!(zlib(&zlib_stored(&long), long.len()), Some(long));
    assert_eq!(zlib(&fixed, 16), None, "falsche Länge");
    assert_eq!(zlib(&fixed[..8], 17), None, "abgeschnitten");
}

/// Zuschnitt: erst das Feld oben links, dann der transparente Rand weg.
#[test]
fn zuschnitt_auf_sichtbare_pixel() {
    let mut pixels = vec![0u8; 4 * 3 * 4];
    for (x, y) in [(1, 1), (3, 2)] {
        pixels[(y * 4 + x) * 4..][..4].copy_from_slice(&[9, 8, 7, 255]);
    }
    let png = encode(&Rgba {
        width: 4,
        height: 3,
        pixels,
    });
    let uri = cropped_data_uri(&png, Some((2, 3))).expect("data-URI");
    let b64 = uri.strip_prefix("data:image/png;base64,").expect("Präfix");
    let raw = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64).expect("base64");
    let out = decode(&raw).expect("PNG");
    assert_eq!((out.width, out.height, out.pixels), (1, 1, vec![9, 8, 7, 255]));
    assert_eq!(cropped_data_uri(b"kein PNG", None), None);
    assert_eq!(cropped_data_uri(&png[..30], None), None);
}
