//! Port von `native/k3c-game/hud/fonts.go`.
//!
//! Schriftrollen (Spiegel von src/scenes/render/fontRules.ts, Beschluss Q03, docs/rules/bedienung.md § 2): Größe in px
//! bezogen auf 1080 Bildpunkte Fensterhöhe, Art (Pflicht- oder Nebeninfo) und Farbe. Das native Fenster skaliert die
//! Rollen mit seiner Höhe (`scale_of`), wie der Browser mit FIT auf 1920 × 1080 skaliert.

use crate::view::cell::Cell;

/// Farbe als RGBA (Go: `color.RGBA`, vormultipliziert).
pub type Rgba = [u8; 4];

/// REF_HEIGHT ist die Bezugshöhe der Größen (Browser: GAME_HEIGHT).
pub const REF_HEIGHT: f64 = 1080.0;

/// FontArt: Pflicht-Info (Gold, Vorrat, HP, Warnungen, Meldungen) oder Nebeninfo (Hinweise, Tastenhilfen).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontArt {
    Info,
    Side,
}

/// FontRole ist eine Schriftrolle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FontRole {
    pub px: f64,
    pub art: FontArt,
    pub clr: Rgba,
}

pub const WHITE: Rgba = [255, 255, 255, 255];
pub const BLACK: Rgba = [0, 0, 0, 255];
pub const YELLOW: Rgba = [0xff, 0xd1, 0x66, 255];
pub const PINKISH: Rgba = [0xff, 0x8f, 0xa3, 255];
pub const CITIZEN_C: Rgba = [0xe9, 0xc4, 0x6a, 255];
pub const PAPER_C: Rgba = [0xff, 0xf1, 0xd0, 255];

// Rollen wie FONTS im Browser (nur die, die das HUD zeichnet).
pub const ROLE_PLAYER: FontRole = FontRole {
    px: 28.0,
    art: FontArt::Info,
    clr: WHITE,
};
pub const ROLE_SHARED: FontRole = FontRole {
    px: 28.0,
    art: FontArt::Info,
    clr: WHITE,
};
pub const ROLE_FIGHT: FontRole = FontRole {
    px: 28.0,
    art: FontArt::Info,
    clr: PINKISH,
};
pub const ROLE_BANNER: FontRole = FontRole {
    px: 28.0,
    art: FontArt::Info,
    clr: WHITE,
};
pub const ROLE_BOSS: FontRole = FontRole {
    px: 28.0,
    art: FontArt::Info,
    clr: YELLOW,
};
pub const ROLE_TRAVEL: FontRole = FontRole {
    px: 40.0,
    art: FontArt::Info,
    clr: YELLOW,
};
pub const ROLE_JOIN: FontRole = FontRole {
    px: 26.0,
    art: FontArt::Side,
    clr: WHITE,
};
pub const ROLE_HINT: FontRole = FontRole {
    px: 24.0,
    art: FontArt::Side,
    clr: WHITE,
};
pub const ROLE_ROOM_INFO: FontRole = FontRole {
    px: 24.0,
    art: FontArt::Side,
    clr: WHITE,
};
pub const ROLE_CITIZEN: FontRole = FontRole {
    px: 32.0,
    art: FontArt::Info,
    clr: CITIZEN_C,
};
/// Geschichtszeile (B-974)
pub const ROLE_STORY: FontRole = FontRole {
    px: 32.0,
    art: FontArt::Info,
    clr: PAPER_C,
};

/// min_font_px ist die Mindestgröße bei 1080 px Höhe: Pflicht-Info 28 px in Zellen über die ganze Breite (beide nativen
/// Layouts), 24 px sonst; Nebeninfo 24 px bei 1 und 2 Spielern, 20 px darüber.
pub fn min_font_px(seats: i64, full_width: bool, art: FontArt) -> f64 {
    if art == FontArt::Info {
        if full_width {
            return 28.0;
        }
        return 24.0;
    }
    if seats <= 2 {
        return 24.0;
    }
    20.0
}

/// scale_of ist der Maßstab des HUD in der Zelle c: Fensterhöhe / 1080. Die Zellen liegen übereinander (game/layout.go),
/// die Fensterhöhe ist also Zellenhöhe × Zahl der Zellen.
pub fn scale_of(c: &Cell, cells: i64) -> f64 {
    let h = (c.rect.dy() * cells.max(1)) as f64;
    if h <= 0.0 {
        return 1.0;
    }
    h / REF_HEIGHT
}

/// luminance ist die relative Helligkeit nach WCAG.
pub fn luminance(c: Rgba) -> f64 {
    let ch = |v: u8| -> f64 {
        let x = f64::from(v) / 255.0;
        if x <= 0.03928 {
            return x / 12.92;
        }
        ((x + 0.055) / 1.055).powf(2.4)
    };
    0.2126 * ch(c[0]) + 0.7152 * ch(c[1]) + 0.0722 * ch(c[2])
}

/// contrast_ratio ist das Kontrastverhältnis zweier Farben nach WCAG (1 bis 21).
pub fn contrast_ratio(a: Rgba, b: Rgba) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// rgb macht aus 0xRRGGBB eine Farbe mit Deckkraft alpha (0..1, vormultipliziert).
pub fn rgb(hex: u32, alpha: f64) -> Rgba {
    let a = alpha.clamp(0.0, 1.0);
    [
        (f64::from(hex >> 16 & 0xff) * a) as u8,
        (f64::from(hex >> 8 & 0xff) * a) as u8,
        (f64::from(hex & 0xff) * a) as u8,
        (255.0 * a) as u8,
    ]
}

/// font_px ist die Größe der Rolle bei 1080, nie unter der Mindestgröße ihrer Art (beide nativen Layouts: Zellen über
/// die ganze Breite).
pub fn font_px(r: FontRole, cells: i64) -> f64 {
    r.px.max(min_font_px(cells, true, r.art))
}
