//! Port von `native/k3c-game/audio/audio_test.go`.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use super::Sound;
use super::mix::{Mix, Request};
use super::music::MusicVoice;
use super::output::{Output, audible, new_device};
use super::register::Register;
use super::testutil::temp_dir;
use crate::view::frame::Frame;
use crate::view::layer::Listener;
use crate::view::settings::default_settings;

/// Zählt die ausgegebenen Mixe (Go: `recorder`).
struct Recorder(Rc<Cell<u32>>);

impl Output for Recorder {
    fn apply(&mut self, _m: &Mix) {
        self.0.set(self.0.get() + 1);
    }
}

fn frame(recording: bool) -> Frame {
    Frame {
        settings: default_settings(),
        recording,
        ..Frame::default()
    }
}

// In Tests ist der Ton stumm (kein Gerät); Listen läuft je Frame genau einmal durch die Wiedergabe.
#[test]
fn new_is_silent_in_tests_and_listens() {
    let mut s = Sound::new();
    assert!(s.out.is_silent(), "Wiedergabe in Tests nicht stumm");
    let mixes = Rc::new(Cell::new(0));
    s.out = Box::new(Recorder(Rc::clone(&mixes)));
    s.listen(&frame(false));
    s.listen(&frame(false));
    assert_eq!(mixes.get(), 2, "Mixe");
}

// Aufnahme-Modus: Der Mix wird gerechnet, aber nicht ausgegeben.
#[test]
fn recording_stays_silent() {
    let mut s = Sound::new();
    let mixes = Rc::new(Cell::new(0));
    s.out = Box::new(Recorder(Rc::clone(&mixes)));
    s.listen(&frame(true));
    assert!(
        mixes.get() == 0 && s.plan.tick == 1,
        "Aufnahme: {} Mixe ausgegeben, {} gerechnet",
        mixes.get(),
        s.plan.tick
    );
}

// Ohne hörbaren Mix entsteht kein Gerät; ein abgeschaltetes Device bleibt still.
#[test]
fn device_stays_off_without_sound() {
    let mut d = new_device(temp_dir("aus"), Arc::new(Register::default()));
    d.apply(&Mix {
        effect_bus: 1.0,
        music_bus: 1.0,
        ..Mix::default()
    });
    assert!(d.ctx.is_none(), "Gerät ohne hörbaren Klang angelegt");
    let coin = || {
        vec![Request {
            slot: "sfx:coin".into(),
            gain: 1.0,
        }]
    };
    assert!(
        !audible(&Mix {
            effect_bus: 0.0,
            effects: coin(),
            ..Mix::default()
        }),
        "Effekte bei Lautstärke 0 sind nicht hörbar"
    );
    let music = Mix {
        music_bus: 1.0,
        music: [
            MusicVoice {
                slot: "music:tag",
                gain: 0.1,
            },
            MusicVoice::default(),
        ],
        ..Mix::default()
    };
    assert!(audible(&music), "Musik ist hörbar");
    d.off = true;
    d.apply(&Mix {
        effect_bus: 1.0,
        effects: coin(),
        ..Mix::default()
    });
    assert!(d.ctx.is_none(), "abgeschaltet: kein Gerät");
}
