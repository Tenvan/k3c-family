//! Port von `native/k3c-game/menu/licenses.go`: Lizenzen und Version (B-808, Vorbild `src/scenes/menuPages.ts › licenses`
//! und `lizenzen.html`). Eine Liste ist die Quelle für die Lizenz-Seite im Menü und die Lizenzdatei im Paket
//! ([`license_text`], Go: `LicenseText`). Die Schrift liegt mit ihrer Lizenzdatei unter `public/schrift/` (B-815).
//!
//! Die Einträge stehen wörtlich wie in Go (Go-Bibliotheken wie Ebitengine): Port ohne Wertänderung. Welche Bibliotheken
//! das Rust-Spiel wirklich nutzt, klärt der Paket-Sprint RU35. Die Versionszeile ([`version_text`]) liegt in
//! `version.rs`; die Hilfs-EXE `menu/licensefile` des Go-Pakets gibt es nicht (Paketbau, RU35).

use super::texts::{TextKey, tr};

pub use super::version::version_text;

/// License ist ein Werk, das das Spiel nutzt: Teil (Engine, Schrift, Grafik, Audio), Name, Urheber, Lizenz, Quelle.
#[derive(Clone, Copy, Debug)]
pub struct License {
    pub part: TextKey,
    pub work: &'static str,
    pub holder: &'static str,
    pub license: &'static str,
    pub place: &'static str,
}

const fn lic(
    part: TextKey,
    work: &'static str,
    holder: &'static str,
    license: &'static str,
    place: &'static str,
) -> License {
    License {
        part,
        work,
        holder,
        license,
        place,
    }
}

/// Die Werke mit Lizenzhinweis (Lizenzen aus den LICENSE-Dateien der Module in `go.mod` bzw. aus `public/schrift` und
/// `public/audio/README.md`).
static LICENSES: [License; 8] = [
    lic(
        TextKey::LicEngine,
        "Ebitengine",
        "Hajime Hoshi",
        "Apache-2.0",
        "github.com/hajimehoshi/ebiten",
    ),
    lic(
        TextKey::LicEngine,
        "Oto, purego",
        "The Ebitengine Authors",
        "Apache-2.0",
        "github.com/ebitengine",
    ),
    lic(
        TextKey::LicFont,
        "Go Regular",
        "Bigelow & Holmes, Google",
        "BSD-3-Clause",
        "public/schrift/LICENSE",
    ),
    lic(
        TextKey::LicFont,
        "go-text/typesetting",
        "The go-text Authors",
        "Unlicense / BSD-3-Clause",
        "github.com/go-text/typesetting",
    ),
    lic(
        TextKey::LicGraphics,
        "x/image, rasterx",
        "The Go Authors, Steven R. Wiley",
        "BSD-3-Clause",
        "golang.org/x/image",
    ),
    lic(
        TextKey::LicAudio,
        "go-mp3",
        "Hajime Hoshi",
        "Apache-2.0",
        "github.com/hajimehoshi/go-mp3",
    ),
    lic(
        TextKey::LicAudio,
        "oggvorbis, vorbis",
        "Johann Freymuth",
        "MIT",
        "github.com/jfreymuth/oggvorbis",
    ),
    lic(
        TextKey::LicAudio,
        "Klänge und Musik",
        "Kenney, OpenGameArt, Freesound (colorsCrimsonTears)",
        "CC0",
        "public/audio/README.md",
    ),
];

/// Liefert die Liste der Werke (Go: `licenses`).
pub fn licenses() -> &'static [License] {
    &LICENSES
}

/// Der Inhalt der Lizenzdatei im Paket (`LIZENZEN.txt`): Version und je Werk eine Zeile, deutsch und englisch aus
/// derselben Liste wie die Lizenz-Seite (Go: `LicenseText`).
pub fn license_text() -> String {
    let mut b = String::new();
    b.push_str("K3C – Family Three Crowns, natives Spiel · native game\n");
    b.push_str(&format!("{} · {}\n\n", version_text("de"), version_text("en")));
    for lang in ["de", "en"] {
        b.push_str(&tr(lang, TextKey::Licenses));
        b.push('\n');
        for l in &LICENSES {
            b.push_str(&format!(
                "- {}: {} – {} – {} – {}\n",
                tr(lang, l.part),
                l.work,
                l.holder,
                l.license,
                l.place
            ));
        }
        b.push('\n');
    }
    b.push_str("Lizenztext der Schrift / font license: public/schrift/LICENSE\n");
    b
}
