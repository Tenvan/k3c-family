//! Port von `native/k3c-game/menu/licenses.go`: Lizenzen und Version (B-808, Vorbild `src/scenes/menuPages.ts › licenses`
//! und `lizenzen.html`). Eine Liste ist die Quelle für die Lizenz-Seite im Menü und die Lizenzdatei im Paket
//! ([`license_text`], Go: `LicenseText`). Die Schrift liegt mit ihrer Lizenzdatei unter `public/schrift/` (B-815).
//!
//! Seit RU58.6 (B-1139) nennt die Liste die Rust-Bibliotheken des Spiels statt der Go-Bibliotheken der Vorlage (Ralf
//! 2026-11-19). Die Versionszeile ([`version_text`]) liegt in
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

/// Die Werke mit Lizenzhinweis: die direkten Abhängigkeiten des Rust-Spiels (Lizenzen aus `cargo metadata`, B-1139) und die
/// Werke aus `public/schrift` und `public/audio/README.md`. `work` nennt Crates mit ihrem Namen aus `Cargo.toml`
/// (`table_tests.rs` prüft, dass jeder in den `Cargo.toml` der Spiel-Crates steht). Eingabe (gilrs) steht unter Engine.
static LICENSES: [License; 10] = [
    lic(
        TextKey::LicEngine,
        "wgpu",
        "gfx-rs developers",
        "MIT OR Apache-2.0",
        "github.com/gfx-rs/wgpu",
    ),
    lic(
        TextKey::LicEngine,
        "winit",
        "The winit contributors",
        "Apache-2.0",
        "github.com/rust-windowing/winit",
    ),
    lic(
        TextKey::LicEngine,
        "pollster",
        "Joshua Barretto",
        "Apache-2.0 OR MIT",
        "github.com/zesterer/pollster",
    ),
    lic(
        TextKey::LicEngine,
        "gilrs",
        "Arvamer",
        "Apache-2.0 OR MIT",
        "gitlab.com/gilrs-project/gilrs",
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
        "fontdue",
        "Joe Cumbo",
        "MIT OR Apache-2.0 OR Zlib",
        "github.com/mooman219/fontdue",
    ),
    lic(
        TextKey::LicGraphics,
        "png",
        "The image-rs Developers",
        "MIT OR Apache-2.0",
        "github.com/image-rs/image-png",
    ),
    lic(
        TextKey::LicGraphics,
        "flate2",
        "Alex Crichton, Josh Triplett",
        "MIT OR Apache-2.0",
        "github.com/rust-lang/flate2-rs",
    ),
    lic(
        TextKey::LicAudio,
        "rodio",
        "The RustAudio contributors",
        "MIT OR Apache-2.0",
        "github.com/RustAudio/rodio",
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
