//! Port von `native/k3c-game/game/notice_test.go`.

use super::*;
use k3c_platform::NullCanvas;

#[test]
fn ascii_text_umlaute_und_emojis() {
    let cases = [
        ("Lädt …", "Laedt"),
        ("💥 Größe prüfen", "Groesse pruefen"),
        ("fehlende Grafik: public/grafik", "fehlende Grafik: public/grafik"),
        ("ÄÖÜ ß", "AeOeUe ss"),
    ];
    for (input, want) in cases {
        assert_eq!(ascii_text(input), want, "ascii_text({input:?})");
    }
}

#[test]
fn wrap_text_bricht_um() {
    let cases: &[(&str, usize, &[&str])] = &[
        ("", 10, &[""]),
        ("ein kurzer Satz", 20, &["ein kurzer Satz"]),
        ("ein kurzer Satz", 8, &["ein", "kurzer", "Satz"]),
        ("aaaa bbbb cccc", 9, &["aaaa bbbb", "cccc"]),
        ("abcdefghijkl x", 5, &["abcde", "fghij", "kl x"]),
    ];
    for &(input, cols, want) in cases {
        let got = wrap_text(input, cols);
        assert_eq!(got, want, "wrap_text({input:?}, {cols})");
        for l in &got {
            assert!(l.len() <= cols, "Zeile {l:?} zu lang");
        }
    }
}

/// Kein Panic bei Nicht-ASCII und cols 0 (Go schneidet Bytes und hängt bei cols 0).
#[test]
fn wrap_text_ohne_panic() {
    assert_eq!(wrap_text("äöü", 3).concat(), "äöü");
    assert_eq!(wrap_text("ab", 0), vec!["a", "b"]);
}

/// Ein langer Pfad bleibt vollständig lesbar: nach dem Umbruch ergeben die Teile wieder den Pfad.
#[test]
fn notice_lines_keep_path() {
    let path = "C:/Users/spieler/k3c/public/grafik/gebaeude/burg/sheet-idle-0123456789abcdef0123456789abcdef.png";
    let lines = notice_lines(&["Grafik fehlt:", path]);
    assert_eq!(lines[0], "Grafik fehlt:");
    assert_eq!(lines[1..].concat(), path);
    for l in &lines {
        assert!(l.len() <= NOTICE_COLS, "Zeile {l:?} breiter als {NOTICE_COLS}");
    }
}

#[test]
fn notice_scale_passt_ins_fenster() {
    let cases = [
        (1280, 720, 352, 150, 3),
        (1920, 1080, 352, 150, 4),
        (3840, 2160, 352, 150, 6), // Deckel
        (320, 180, 352, 150, 1),   // nie unter 1
        (1280, 720, 0, 0, 1),
        (1280, 200, 352, 150, 1), // die Höhe begrenzt
    ];
    for (w, h, iw, ih, want) in cases {
        assert_eq!(notice_scale(w, h, iw, ih), want, "notice_scale({w},{h},{iw},{ih})");
    }
}

#[test]
fn cap_lines_kuerzt_mit_punkten() {
    let l = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(cap_lines(l(&["a", "b"]), 2), l(&["a", "b"]));
    assert_eq!(cap_lines(l(&["a", "b", "c", "d"]), 3), l(&["a", "b", "..."]));
    assert!(cap_lines(l(&["a"]), 0).is_empty());
}

/// Zeichnen: erst die Fläche, dann eine Zeile je Textzeile; ohne Zeilen nur die Fläche.
#[test]
fn notice_draw_zeichnet_flaeche_und_zeilen() {
    let n = Notice::new(&["Lädt …", "", "Stufe: eins"]);
    assert_eq!(n.lines, vec!["Laedt", "", "Stufe: eins"]);
    let mut c = NullCanvas::default();
    n.draw(&mut c);
    assert_eq!((c.rects, c.texts), (1, 3));
    let mut c = NullCanvas::default();
    Notice::default().draw(&mut c);
    assert_eq!((c.rects, c.texts), (1, 0));
}
