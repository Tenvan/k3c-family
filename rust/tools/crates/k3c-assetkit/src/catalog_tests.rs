use super::*;
use crate::testutil::{repo, write_file, write_png};

/// Katalog des Mini-Repos mit einem Pferd (NC-SA), Gruppen und Credits.
fn sample() -> Vec<Asset> {
    let dir = repo();
    let public = dir.path().join("public");
    write_png(&public.join("sprites").join("horse").join("idle.png"), 100, 65);
    write_file(
        &public.join("grafik").join("index.json"),
        r#"[{"pack":"town","group":"props","file":"well.png","w":64,"h":48}]"#,
    );
    write_file(
        &public.join("grafik").join("CREDITS.md"),
        "# Grafik\n\n| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n\
         | town, `city` | GothicVania Town | Luis Zuno | CC0 1.0 | https://example.org/town |\n",
    );
    write_file(
        &public.join("sprites").join("CREDITS.md"),
        "| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n\
         | horse | Horse | Jemand | CC BY-NC-SA 4.0 | https://example.org/horse |\n",
    );
    catalog(dir.path()).expect("Katalog")
}

fn by_file<'a>(list: &'a [Asset], file: &str) -> &'a Asset {
    list.iter().find(|a| a.file == file).expect(file)
}

#[test]
fn katalog_mit_credits() {
    let list = sample();
    assert_eq!(list.len(), 4, "Katalog: {list:?}");
    let house = by_file(&list, "grafik/town/well.png");
    assert_eq!(
        (house.pack.as_str(), house.group.as_str(), house.w, house.h),
        ("town", "props", 64, 48)
    );
    assert_eq!((house.artist.as_str(), house.flag.as_str()), ("Luis Zuno", ""));
    let horse = by_file(&list, "sprites/horse/idle.png");
    assert_eq!((horse.flag.as_str(), horse.w), ("NC", 100));
    let coin = by_file(&list, "audio/coin.ogg");
    assert_eq!((coin.kind.as_str(), coin.license.as_str()), (KIND_SOUND, SELF_MADE));
}

/// B-437: eigene Gebäude-Grafik ohne Credit-Zeile hat eine eigene Lizenz, fremde Packs ohne Credit bleiben „unbekannt“.
#[test]
fn katalog_eigene_gebaeude() {
    let dir = repo();
    let graphics = dir.path().join("public").join("grafik");
    write_png(&graphics.join("gebaeude").join("taverne-32.png"), 192, 128);
    write_png(&graphics.join("fremd").join("x.png"), 16, 16);
    let list = catalog(dir.path()).expect("Katalog");
    let own = by_file(&list, "grafik/gebaeude/taverne-32.png");
    assert_eq!((own.license.as_str(), own.flag.as_str()), (OWN_BUILDING, ""));
    assert_eq!(by_file(&list, "grafik/fremd/x.png").license, "unbekannt");
}

#[test]
fn katalog_suche() {
    let list = sample();
    let got = search(&list, "TOWN props", "", 0);
    assert!(
        got.len() == 1 && got[0].file == "grafik/town/well.png",
        "Suche town props: {got:?}"
    );
    assert_eq!(search(&list, "nc-sa", "", 0).len(), 1, "Suche Lizenz");
    assert_eq!(search(&list, "", KIND_SOUND, 0).len(), 1, "Filter Art");
    assert_eq!(search(&list, "", "", 2).len(), 2, "limit");
}

/// Q70: ND nicht übernehmbar vor NC vor SA; Wörter wie „Sample“ zählen nicht.
#[test]
fn license_flag() {
    let cases = [
        ("CC0 1.0", ""),
        ("CC BY 3.0", ""),
        ("OGA-BY 3.0", ""),
        ("CC BY-SA 4.0", "SA"),
        ("CC-BY-SA 3.0", "SA"),
        ("CC BY-NC 4.0", "NC"),
        ("CC BY-NC-SA 4.0", "NC"),
        ("CC BY-NC-ND 4.0", "ND"),
        ("Sample Pack", ""),
        (SELF_MADE, ""),
        (OWN_BUILDING, ""),
    ];
    for (license, want) in cases {
        assert_eq!(super::license_flag(license), want, "{license:?}");
    }
}
