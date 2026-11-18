//! Port von `native/k3c-game/audio/duck.go`: Ducking wie src/audio/ducking.ts (SO4.3, Beschluss Q16).
//!
//! Eine Warnung senkt die Musik kurz ab und stellt sie danach wieder her; überlappende Warnungen verlängern die
//! Absenkung. Zeit in Sekunden aus dem Takt (Listen-Zähler / tps). Die Form kommt aus data/sounds.json › mischung
//! (B-935); `DEFAULT_DUCK` gilt ohne Eintrag. Getestet wird in RU17.4 (`TestDuck` aus music_test.go).

/// Form einer Absenkung: Pegel der Musik während der Warnung, Absenken, Halten nach der letzten Warnung und
/// Zurückkehren auf vollen Pegel in Sekunden.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DuckShape {
    pub level: f64,
    pub attack: f64,
    pub hold: f64,
    pub release: f64,
}

/// Die Form ohne Mischpegel in sounds.json (Stand vor B-935).
pub const DEFAULT_DUCK: DuckShape = DuckShape {
    level: 0.35,
    attack: 0.15,
    hold: 1.2,
    release: 0.8,
};

/// Eine Absenkung: Rampe ab `start` hinab, gesenkt bis `end`, danach Rampe hinauf. Null-Wert = keine.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DuckWindow {
    pub start: f64,
    pub end: f64,
    pub shape: DuckShape,
    pub set: bool,
}

/// Pegelfaktor der Musik zum Zeitpunkt t (ducking.ts › duckGain).
pub fn duck_gain(t: f64, w: &DuckWindow) -> f64 {
    let s = w.shape;
    if !w.set || t <= w.start {
        1.0
    } else if t < w.start + s.attack {
        1.0 - (1.0 - s.level) * (t - w.start) / s.attack
    } else if t <= w.end {
        s.level
    } else if t < w.end + s.release {
        s.level + (1.0 - s.level) * (t - w.end) / s.release
    } else {
        1.0
    }
}

/// Meldet eine neue Warnung zum Zeitpunkt t mit Form s (ducking.ts › extendDuck): eine laufende Absenkung wird
/// verlängert statt gestapelt; in der Rampe hinauf beginnt die neue Rampe hinab beim aktuellen Pegel, ohne Sprung.
pub fn extend_duck(w: &DuckWindow, t: f64, s: DuckShape) -> DuckWindow {
    let g = duck_gain(t, w);
    let mut end = t + s.hold;
    if w.set && t <= w.end {
        end = end.max(w.end);
    }
    if w.set && t > w.start && t <= w.end {
        return DuckWindow {
            start: w.start,
            end,
            shape: w.shape,
            set: true,
        };
    }
    DuckWindow {
        start: t - s.attack * (1.0 - g) / (1.0 - s.level),
        end,
        shape: s,
        set: true,
    }
}
