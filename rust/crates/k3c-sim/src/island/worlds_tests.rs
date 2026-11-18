//! Welt-Dateien laden und prüfen (Go: `island_worlds_test.go`, WZ18.2, B-488/AC-02; docs/rules/welten.md § 2 bis 4). Alle
//! Tests laufen bis auf `eingebettete_welten_gueltig`: Die Standard-Welt hat eine Geschichte (`story`), die
//! `standard_world` mit RU19 liest und prüft (`decode_world_story`, `check_story`); `own_*` prüft den Kopf der Standard-Welt
//! schon an den Daten. Hilfen (`test_island`, `test_world`, `parse_test_world`) stehen in `testutil.rs`.
use std::path::PathBuf;

use k3c_data::Source;

use super::switch_testutil::{test_island, test_world};
use super::testutil::*;
use super::worlds::{STANDARD_WORLD_ID, load_world, parse_world, standard_world};

/// Go: `TestWeltAusMapFSLaden`: Die Welt kommt aus dem Text (Go: `fstest.MapFS`), `load_world` liest `worlds/<id>.json`.
#[test]
fn welt_aus_map_fs_laden() {
    let raw = format!(
        r#"{{"id":"nordmeer","name":"Nordmeer","notes":"Entwurf","islands":[{},{}]}}"#,
        test_island("island1", r#","plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}"#),
        test_island(
            "eisinsel",
            r#","depths":[0,4],"chunkCounts":{"0":24},"plannedChunks":{"0":[{"slot":-3,"kind":"clearing","seed":"lichtung"}]}"#
        )
    );
    let w = parse_world_in(&raw, "nordmeer").expect("Welt laden");
    assert!(
        w.id == "nordmeer" && w.name == "Nordmeer" && w.notes == "Entwurf" && w.islands.len() == 2,
        "Welt {w:?}"
    );
    let d = &w.islands[1];
    assert!(
        d.id == "eisinsel" && d.count_at(0) == 24 && d.planned_at(0)[0].seed == "lichtung",
        "Insel 2: {d:?}"
    );
    let data = test_data();
    let leer = Source::Dir(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../.work/tmp")
            .join("k3c-ru10-5-ohne-welten"),
    );
    match load_world(&leer, "nordmeer", &data, &test_rules(&data)) {
        Err(e) if e.contains("nordmeer") => {}
        other => panic!("fehlende Datei: {other:?}"),
    }
}

/// Go: `TestEingebetteteWeltenGueltig` (AC-02): Jede eingebettete Welt-Datei (heute nur standard.json) besteht die Prüfung;
/// die Standard-Welt ist Voreinstellung.
#[test]
fn eingebettete_welten_gueltig() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../data/worlds");
    let names: Vec<String> = std::fs::read_dir(dir)
        .expect("data/worlds")
        .map(|e| e.expect("Eintrag").file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        names,
        ["standard.json"],
        "eingebettet: {names:?}, erwartet nur standard.json"
    );
    let data = test_data();
    let rules = test_rules(&data);
    for n in &names {
        let id = n.trim_end_matches(".json");
        if let Err(e) = load_world(&Source::Embedded, id, &data, &rules) {
            panic!("{n}: {e}");
        }
    }
    let w = standard_world(&data, &rules).expect("Standard-Welt");
    assert!(
        w.id == STANDARD_WORLD_ID && w.name == "Standard" && !w.islands.is_empty() && w.islands[0].id == "island1",
        "Standard-Welt {w:?}"
    );
}

/// Go: `TestStandardWeltGleichBisherigenInseln` (AC-02): Die Liste `islands` von standard.json ist unverändert gegenüber
/// data/islands.json vor WZ18.2 (Hash von damals, Zeilenenden auf LF normiert) plus dem Hafen auf Insel 1 Tiefe 0 (BS5.1,
/// B-637) und der Geschichte der Insel 1 (GS1.5, B-976).
#[test]
fn standard_welt_gleich_bisherigen_inseln() {
    let raw = Source::Embedded.read("worlds/standard.json").expect("standard.json");
    let islands = islands_bytes(raw.as_bytes()).replace(b"\r\n", b"\n");
    assert_eq!(
        sha256_hex(&islands),
        "ea0213804059ff85c0bea5f3c073c9cead03ca25395d2610e7fe5ba5ef31de57",
        "islands geändert"
    );
}

/// Go: `TestWeltOhneHafenAbgelehnt` (BS5.1, B-637): Eine Insel, die nicht die letzte der Welt ist, braucht den Hafen in
/// Tiefe 0.
#[test]
fn welt_ohne_hafen_abgelehnt() {
    let raw = test_world(&format!("{},{}", test_island("a", ""), test_island("b", "")));
    match parse_world_in(&raw, "test") {
        Err(e) if e.contains("Insel a: kein Hafen") => {}
        other => panic!("Welt ohne Hafen: {other:?}"),
    }
    let harbor = r#","plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}"#;
    let raw = test_world(&format!("{},{}", test_island("a", harbor), test_island("b", "")));
    if let Err(e) = parse_world_in(&raw, "test") {
        panic!("Welt mit Hafen: {e}");
    }
}

/// Go: `TestWeltIDMuster`: Die Welt-ID muss zum Muster passen, sonst wird nichts gelesen (kein Pfadzugriff über die ID).
#[test]
fn welt_id_muster() {
    let data = test_data();
    let rules = test_rules(&data);
    let long = "a".repeat(33);
    for id in ["", "Nord", "../x", "-a", "a_b", long.as_str()] {
        match load_world(&Source::Embedded, id, &data, &rules) {
            Err(e) if e.contains("Welt-ID") => {}
            other => panic!("{id:?}: Ergebnis {other:?}"),
        }
        let raw = test_world(&test_island("a", ""));
        assert!(
            parse_world(&raw, id, &data, &rules).is_err(),
            "parse_world {id:?} angenommen"
        );
    }
    if let Err(e) = parse_test_world("") {
        panic!("gültige Test-Welt abgelehnt: {e}");
    }
}

// Ab hier: Tests ohne Go-Gegenstück.

#[test]
fn own_sha256_stimmt_mit_dem_bekannten_wert_ueberein() {
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn own_standard_welt_kopf_aus_den_daten() {
    let data = test_data();
    let f = &data.standard_world;
    assert_eq!(f.id.as_deref(), Some(STANDARD_WORLD_ID));
    assert_eq!(f.name.as_deref(), Some("Standard"));
    let islands = f.islands.as_ref().expect("Inseln");
    assert_eq!(islands[0]["id"], "island1");
}

#[test]
fn own_welt_kopf_wird_uebernommen() {
    let raw = format!(
        r#"{{"id":"test","name":"Test","notes":"N","extends":"standard","combat":{{"mode":"general","groupSize":3}},"overrides":{{"waves":{{"a":1}}}},"islands":[{}]}}"#,
        test_island("a", "")
    );
    let w = parse_world_in(&raw, "test").expect("Welt");
    assert_eq!((w.notes.as_str(), w.extends.as_str()), ("N", "standard"));
    assert_eq!((w.combat.mode.as_str(), w.combat.group_size), ("general", 3));
    assert!(
        w.overrides.contains_key("waves"),
        "Überschreibung fehlt: {:?}",
        w.overrides
    );
    let plain = parse_test_world("").expect("Welt");
    assert_eq!((plain.combat.mode.as_str(), plain.combat.group_size), ("", 0));
    assert!(plain.extends.is_empty() && plain.overrides.is_empty() && plain.skins.is_empty());
}

#[test]
fn own_null_felder_der_insel_gelten_wie_in_go() {
    let w = parse_test_world(
        r#","plannedChunks":null,"chunkCounts":null,"endBoss":null,"nature":null,"natureByDepth":null,"etappe":null"#,
    )
    .expect("null-Felder");
    let d = &w.islands[0];
    assert!(d.planned_chunks.is_empty() && d.chunk_counts.is_empty() && d.end_boss.is_empty());
    assert!(d.nature.is_empty() && d.nature_by_depth.is_empty() && d.etappe == 1);
    // `depthScaling: null` ist wie in Go ein vorhandenes Feld ohne Faktoren.
    let err = parse_test_world(r#","depthScaling":null"#).expect_err("null");
    assert!(err.contains("Insel a: depthScaling.hp fehlt (Pflichtfeld)"), "{err}");
}

#[test]
fn own_nature_und_tiefen_werden_streng_gelesen() {
    let cases = [
        (
            r#","natureByDepth":{"0":{"dichte":1}}"#,
            r#"Insel a: natureByDepth.0: unbekanntes Feld "dichte""#,
        ),
        (r#","depths":"x""#, "Insel a: kein gültiges JSON"),
    ];
    for (fields, want) in cases {
        let err = parse_test_world(fields).expect_err(fields);
        assert!(err.contains(want), "{fields}: Fehler {err:?} nennt {want:?} nicht");
    }
    // Doppelte Schlüssel kann `test_island` nicht bauen (es geht über eine Map): der Text kommt von Hand.
    let raw = test_world(
        r#"{"id":"a","name":"Insel","depths":[4],"depthScaling":{"hp":1,"damage":1,"speed":1},"nature":{"traits":{},"traits":{}}}"#,
    );
    let err = parse_world_in(&raw, "test").expect_err("doppelt");
    assert!(err.contains(r#"Insel a: nature: doppeltes Feld "traits""#), "{err}");
    let w = parse_test_world(r#","natureByDepth":{"0":{"traits":{"forestDensity":5}}}"#).expect("Natur je Tiefe");
    assert_eq!(w.islands[0].nature_by_depth["0"].traits["forestDensity"], 5);
}

#[test]
fn own_insel_ohne_objekt_wird_mit_ihrem_platz_gemeldet() {
    let raw = r#"{"id":"test","name":"Test","islands":[5]}"#;
    let err = parse_world_in(raw, "test").expect_err("keine Insel");
    assert!(
        err.starts_with("data/worlds/test.json: Insel Nr. 1: kein gültiges JSON"),
        "{err}"
    );
}

/// Die Bytes der Liste `islands` einer Welt-Datei (Go: `json.RawMessage` des Feldes): von `[` bis zur passenden `]`.
fn islands_bytes(raw: &[u8]) -> Vec<u8> {
    let key = b"\"islands\"";
    let at = raw.windows(key.len()).position(|w| w == key).expect("Feld islands");
    let start = at + raw[at..].iter().position(|&b| b == b'[').expect("Liste");
    let (mut depth, mut in_str, mut esc) = (0, false, false);
    for (i, &b) in raw.iter().enumerate().skip(start) {
        match (in_str, b) {
            (true, _) if esc => esc = false,
            (true, b'\\') => esc = true,
            (true, b'"') => in_str = false,
            (true, _) => {}
            (false, b'"') => in_str = true,
            (false, b'[') => depth += 1,
            (false, b']') => {
                depth -= 1;
                if depth == 0 {
                    return raw[start..=i].to_vec();
                }
            }
            _ => {}
        }
    }
    panic!("Liste islands nicht geschlossen");
}

trait ReplaceBytes {
    fn replace(&self, from: &[u8], to: &[u8]) -> Vec<u8>;
}

impl ReplaceBytes for Vec<u8> {
    fn replace(&self, from: &[u8], to: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.len());
        let mut i = 0;
        while i < self.len() {
            if self[i..].starts_with(from) {
                out.extend_from_slice(to);
                i += from.len();
            } else {
                out.push(self[i]);
                i += 1;
            }
        }
        out
    }
}

/// SHA-256 als Hex-Text (nur für den Hash-Test; die Konstanten kommen aus Primzahlen: Wurzeln und Kubikwurzeln).
fn sha256_hex(msg: &[u8]) -> String {
    let mut primes: Vec<u64> = Vec::new();
    let mut n = 2;
    while primes.len() < 64 {
        if primes.iter().all(|p| n % p != 0) {
            primes.push(n);
        }
        n += 1;
    }
    let frac = |x: f64| ((x - x.floor()) * 4_294_967_296.0) as u32;
    let k: Vec<u32> = primes.iter().map(|&p| frac((p as f64).cbrt())).collect();
    let mut h: Vec<u32> = primes[..8].iter().map(|&p| frac((p as f64).sqrt())).collect();
    let mut data = msg.to_vec();
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&(msg.len() as u64 * 8).to_be_bytes());
    for block in data.chunks(64) {
        let mut w = [0u32; 64];
        for (slot, b) in w.iter_mut().zip(block.chunks(4)) {
            *slot = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = [h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]];
        for (ki, wi) in k.iter().zip(&w) {
            let [a, b, c, d, e, f, g, hh] = v;
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add((e & f) ^ (!e & g))
                .wrapping_add(*ki)
                .wrapping_add(*wi);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let t2 = s0.wrapping_add((a & b) ^ (a & c) ^ (b & c));
            v = [t1.wrapping_add(t2), a, b, c, d.wrapping_add(t1), e, f, g];
        }
        for (hi, vi) in h.iter_mut().zip(v) {
            *hi = hi.wrapping_add(vi);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}
