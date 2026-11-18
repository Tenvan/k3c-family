use crate::nature::neutral_nature;
use crate::planned_gen::Options;
use crate::testutil::{biome, biome_ids, count_entities, generate, generate_with, validate};
use crate::types::Entity;

#[test]
fn validate_500_seeds() {
    for id in biome_ids() {
        let b = biome(&id);
        for seed in 0..500 {
            let l = generate(&b, &seed.to_string());
            let errs = validate(&l, &b);
            assert!(errs.is_empty(), "Biom {id}, Seed {seed}: {errs:?}");
        }
    }
}

#[test]
fn validate_meldet_kaputtes_level() {
    let b = biome("forest");
    let mut l = generate(&b, "kaputt");
    for c in &mut l.chunks {
        if c.kind == "exit" {
            c.kind = "forest".into();
        }
    }
    l.entities.push(Entity {
        kind: "castle".into(),
        x: l.width_units + 1.0,
        ..Entity::default()
    });
    let want = [
        "Genau ein Tiefen-Eingang erwartet",
        "Genau eine Burg erwartet",
        "Entity außerhalb des Levels",
    ];
    assert_eq!(validate(&l, &b), want);
}

#[test]
fn hub_in_der_mitte() {
    for id in biome_ids() {
        let l = generate(&biome(&id), "7");
        let castle = l.entities.iter().find(|e| e.kind == "castle").map_or(f64::NAN, |e| e.x);
        assert!(
            l.hub_center_units == l.width_units / 2.0 && castle == l.hub_center_units,
            "Biom {id}: Hub nicht in der Mitte"
        );
    }
}

/// W2.1 (B-114/AC-01): Biome mit Adern haben über 500 Seeds genau so viele, gleicher Seed gleiche Adern; die Adern
/// kommen nach allen übrigen Würfen, alle anderen Objekte bleiben gleich.
#[test]
fn adern_je_stufe() {
    let mut with_veins = 0;
    for id in biome_ids() {
        let b = biome(&id);
        if b.veins.count == 0 {
            continue;
        }
        with_veins += 1;
        let mut bare = b.clone();
        bare.veins.count = 0;
        // Adern-Eigenschaft neutral (B-891): genau veins.count
        let neutral = Options {
            nature: neutral_nature(&b),
            ..Options::default()
        };
        for seed in 0..500 {
            let s = seed.to_string();
            let l = generate_with(&b, &s, &neutral).unwrap_or_else(|e| panic!("{e}"));
            let n = count_entities(&l, &b.veins.kind);
            assert_eq!(
                n, b.veins.count,
                "Biom {id}, Seed {seed}: {n} Adern, erwartet {}",
                b.veins.count
            );
            let again = generate_with(&b, &s, &neutral).unwrap_or_else(|e| panic!("{e}"));
            let without = generate_with(&bare, &s, &neutral).unwrap_or_else(|e| panic!("{e}"));
            let rest: Vec<Entity> = l.entities.iter().filter(|e| e.kind != b.veins.kind).cloned().collect();
            assert!(
                l.entities == again.entities && rest == without.entities,
                "Biom {id}, Seed {seed}: Adern nicht reproduzierbar oder verschieben andere Objekte"
            );
        }
    }
    assert!(
        with_veins >= 2 && biome("forest").veins.count == 0,
        "{with_veins} Biome mit Adern"
    );
}

/// Die endlichen Objekte (Bäume, Felsen, Erz, Truhen, Camps) je 100 Units, gemittelt über 100 Seeds, mit neutraler Natur
/// (B-891): Die Regel gilt für das Biom; gewürfelte Natur-Werte verschieben die Dichte je Welt.
fn density(id: &str) -> f64 {
    let finite = ["tree", "rock", "copperOre", "chest", "recruitCamp"];
    let (mut objects, mut width) = (0, 0.0);
    for seed in 0..100 {
        let b = biome(id);
        let o = Options {
            nature: neutral_nature(&b),
            ..Options::default()
        };
        let l = generate_with(&b, &seed.to_string(), &o).unwrap_or_else(|e| panic!("{e}"));
        objects += l.entities.iter().filter(|e| finite.contains(&e.kind.as_str())).count();
        width += l.width_units;
    }
    100.0 * objects as f64 / width
}

/// W2.2 (B-115/AC-02, Sprint W2 AC-04, Beschluss Q28): Breiten laut materialien-gebaeude.md § 1, Dichte unter Tage
/// nicht abnehmend und ab der Mine mindestens 2,8 endliche Objekte je 100 Units.
#[test]
fn breite_und_dichte() {
    // Kristallhöhle B-645: Platz für das Camp außerhalb der Linien
    let widths = [
        ("forest", 900, 1100),
        ("cave", 700, 900),
        ("mine", 550, 700),
        ("ironhold", 480, 560),
        ("crystal", 600, 700),
    ];
    for (id, min, max) in widths {
        let got = biome(id).length_units;
        assert!(
            got.min == min && got.max == max,
            "{id}: Breite {got:?}, laut Regel {min} bis {max}"
        );
    }
    let mut prev = 0.0;
    for (i, id) in ["cave", "mine", "ironhold", "crystal"].iter().enumerate() {
        let d = density(id);
        assert!(
            !(d < prev || (i > 0 && d < 2.8)),
            "{id}: Dichte {d:.2} (davor {prev:.2}), erwartet nicht abnehmend und ≥ 2,8"
        );
        prev = d;
    }
}

/// K2.1c (B-130/AC-02): Die Kristallhöhle hat über 500 Seeds genau einen Bau vor dem Ausgang; ohne Bau entsteht dasselbe
/// Level ohne die Entität (kein Würfel) für Biome ohne Bau. Mit Bau hängt die Lage des Camps vom Bau ab (B-645) und mit
/// ihr die Verteilung der freien Chunks. Alle übrigen Biome haben keinen Bau.
#[test]
fn lair_nur_in_der_kristallhoehle() {
    for id in biome_ids() {
        let b = biome(&id);
        assert_eq!(b.lair, id == "crystal", "Biom {id}: lair = {}", b.lair);
        let mut bare = b.clone();
        bare.lair = false;
        for seed in 0..500 {
            let s = seed.to_string();
            let l = generate(&b, &s);
            let without = generate(&bare, &s);
            assert!(
                b.lair || (l.entities == without.entities && l.chunks == without.chunks),
                "Biom {id}, Seed {seed}: ohne Bau muss dasselbe Level entstehen"
            );
            let want = i64::from(b.lair);
            let n = count_entities(&l, "lair");
            assert_eq!(n, want, "Biom {id}, Seed {seed}: {n} Bau, erwartet {want}");
        }
    }
}

#[test]
fn validate_meldet_falschen_bau() {
    let b = biome("crystal");
    let mut l = generate(&b, "bau");
    for e in &mut l.entities {
        if e.kind == "lair" {
            e.x = l.hub_center_units;
        }
    }
    assert_eq!(
        validate(&l, &b),
        [format!("Bau nicht vor dem Ausgang ({} Units)", l.hub_center_units)]
    );
    l.entities.retain(|e| e.kind != "lair");
    assert_eq!(validate(&l, &b), ["1 Bau erwartet, 0 gefunden"]);
}
