//! Port von `native/k3c-game/audio/output_test.go`.
//!
//! Abweichungen: `fakeSink`/`fakePlayer` → `testutil::FakeAudio`; statt einer Log-Umleitung prüfen die Tests
//! `Device::notes`. `awaitBank` entfällt (die Bank lädt synchron beim ersten hörbaren Mix). Den Lesefehler eines
//! Stücks stellt `FakePlayer::err` nach (Stimme klingt nicht mehr); das Schließen der Datei aus Go entfällt, die
//! Plattform liest Stücke ganz. `writeWAV` entfällt: `FakeAudio` dekodiert nicht.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

use k3c_platform::SoundId;

use super::events::SLOT_RIDE;
use super::mix::{Loop, Mix, Request};
use super::music::MusicVoice;
use super::new_sound;
use super::output::{Output, new_device};
use super::register::Register;
use super::testutil::{FakeState, fake_device, temp_dir};
use crate::view::frame::Frame;
use crate::view::layer::Listener;
use crate::view::settings::default_settings;

fn req(slot: &'static str, gain: f64) -> Request {
    Request {
        slot: slot.into(),
        gain,
    }
}

fn count(notes: &[String], part: &str) -> usize {
    notes.iter().filter(|n| n.contains(part)).count()
}

// AC-01 (B-805): Das Gerät lässt sich nicht anlegen → Log-Zeile, stumm, kein zweiter Versuch, kein Fehler nach außen.
#[test]
fn device_open_failure_stays_silent() {
    let mut d = new_device(temp_dir("open"), Arc::new(Register::default()));
    let opens = Rc::new(Cell::new(0));
    let o = Rc::clone(&opens);
    d.open = Box::new(move || {
        o.set(o.get() + 1);
        Err("oto: context is already created".to_string())
    });
    let m = Mix {
        effect_bus: 1.0,
        effects: vec![req("sfx:coin", 1.0)],
        ..Mix::default()
    };
    d.apply(&m);
    d.apply(&m);
    assert!(
        d.off && d.ctx.is_none() && opens.get() == 1,
        "off={} Versuche={}",
        d.off,
        opens.get()
    );
    assert_eq!(count(&d.notes, "❌ Ton-Gerät nicht startbar"), 1, "{:?}", d.notes);
}

// AC-01 (B-805): Der Treiber fällt im Lauf aus → eine Log-Zeile, alle Stimmen enden, keine neuen, Listen läuft weiter.
// Davor: Lautstärken wie im Mix (Pegel mal Bus), auch nach einer Änderung des Busses.
#[test]
fn device_driver_failure_mutes_and_game_goes_on() {
    let reg = Register::default();
    let st = Rc::new(RefCell::new(FakeState::default()));
    let opens = Rc::new(Cell::new(0));
    let mut d = fake_device("treiber", reg.clone(), &st, &opens);
    let mut m = Mix {
        effect_bus: 0.5,
        music_bus: 1.0,
        effects: vec![req("sfx:coin", 0.8)],
        loops: vec![Loop { key: 0, gain: 1.0 }],
        ..Mix::default()
    };
    d.apply(&m);
    let bank = d.bank.as_mut().expect("Bank geladen");
    bank.singles.insert("sfx:coin".to_string(), SoundId(90));
    bank.singles.insert(SLOT_RIDE.to_string(), SoundId(91));
    d.apply(&m);
    {
        let s = st.borrow();
        assert_eq!(s.players.len(), 2, "Stimmen");
        assert!(s.players[0].vol == 0.4_f32 && s.players[1].vol == 0.5_f32 && s.players[0].is_playing());
    }
    m.effects.clear();
    m.effect_bus = 1.0;
    d.apply(&m);
    {
        let s = st.borrow();
        assert!(
            s.players[0].vol == 0.8_f32 && s.players[1].vol == 1.0_f32,
            "Bus nicht nachgezogen"
        );
    }

    st.borrow_mut().err = Some("oto: initialization failed: WASAPI: device invalidated".to_string());
    d.apply(&m);
    for (i, p) in st.borrow().players.iter().enumerate() {
        assert!(p.stopped, "Stimme {i} läuft nach dem Ausfall weiter");
    }
    assert!(d.off);
    assert_eq!(count(&d.notes, "💥 Ton-Gerät ausgefallen"), 1, "{:?}", d.notes);
    m.effects = vec![req("sfx:coin", 1.0)];
    d.apply(&m);
    let notes_before = d.notes.clone();
    let mut s = new_sound(Arc::new(reg), Box::new(d));
    let f = Frame {
        settings: default_settings(),
        ..Frame::default()
    };
    s.listen(&f);
    s.listen(&f);
    assert!(st.borrow().players.len() == 2 && opens.get() == 1 && s.plan.tick == 2);
    assert_eq!(count(&notes_before, "💥"), 1, "Ausfall mehrfach gemeldet");
}

// Ein Musikstück mit Lesefehler endet, der Slot bleibt stumm und wird nicht neu geöffnet; Lautstärke wie im Mix.
#[test]
fn music_read_error_drops_track() {
    let st = Rc::new(RefCell::new(FakeState::default()));
    let opens = Rc::new(Cell::new(0));
    let mut reg = Register::default();
    reg.music.insert("music:tag".to_string(), "tag.wav".to_string());
    let mut d = fake_device("musik", reg, &st, &opens);
    std::fs::write(d.dir.join("tag.wav"), b"RIFF still").expect("tag.wav");
    let m = Mix {
        music_bus: 0.5,
        music: [
            MusicVoice {
                slot: "music:tag",
                gain: 0.6,
            },
            MusicVoice::default(),
        ],
        ..Mix::default()
    };
    d.apply(&m);
    {
        let s = st.borrow();
        assert!(
            d.tracks.len() == 1 && s.players[0].vol == 0.3_f32 && s.players[0].is_playing(),
            "{s:?}"
        );
    }
    st.borrow_mut().players[0].err = true;
    d.apply(&m);
    d.apply(&m);
    let s = st.borrow();
    // Abweichung (Review RU17.5): Die verstummte Stimme endet und das Stück startet einmal neu aus dem geladenen
    // Klang (kein zweites Laden), ohne Meldung; Stimmen-Grenze der Plattform darf keinen Slot dauerhaft stumm machen.
    assert!(
        d.tracks.len() == 1 && s.players.len() == 2 && s.players[0].stopped && s.players[1].is_playing(),
        "{s:?}"
    );
    assert!(s.loads == 1 && s.players[1].vol == 0.3_f32, "{s:?}");
    assert!(count(&d.notes, "Ton nicht abspielbar") == 0 && !d.off, "{:?}", d.notes);
}

// Ergänzung zu Go (Review RU17.5): Eine Hufschlag-Schleife, die nicht klingt, startet neu; je Slot höchstens
// MAX_VOICES Effekte gleichzeitig.
#[test]
fn silent_loop_restarts_and_voices_capped() {
    let st = Rc::new(RefCell::new(FakeState::default()));
    let opens = Rc::new(Cell::new(0));
    let mut d = fake_device("grenze", Register::default(), &st, &opens);
    let m = Mix {
        effect_bus: 1.0,
        loops: vec![Loop { key: 1, gain: 0.5 }],
        ..Mix::default()
    };
    d.apply(&m);
    let bank = d.bank.as_mut().expect("Bank geladen");
    bank.singles.insert(SLOT_RIDE.to_string(), SoundId(1));
    bank.singles.insert("sfx:coin".to_string(), SoundId(2));
    d.apply(&m);
    st.borrow_mut().players[0].err = true;
    d.apply(&m);
    {
        let s = st.borrow();
        assert!(
            s.players.len() == 2 && s.players[0].stopped && s.players[1].is_playing(),
            "{s:?}"
        );
    }
    let coin = Mix {
        effects: vec![req("sfx:coin", 1.0)],
        ..m.clone()
    };
    for _ in 0..6 {
        d.apply(&coin);
    }
    let s = st.borrow();
    let coins = s.players.iter().filter(|p| p.sound == 2).count();
    assert_eq!(coins, super::output::MAX_VOICES, "Effekte je Slot begrenzt");
}

// Ergänzung zu Go: Ein Stück, das die Plattform nicht laden kann, bleibt stumm, mit genau einer Meldung.
#[test]
fn music_load_error_is_silent_once() {
    let st = Rc::new(RefCell::new(FakeState::default()));
    let opens = Rc::new(Cell::new(0));
    let mut reg = Register::default();
    reg.music.insert("music:tag".to_string(), "tag.ogg".to_string());
    let mut d = fake_device("kaputt", reg, &st, &opens);
    std::fs::write(d.dir.join("tag.ogg"), b"kaputt").expect("tag.ogg");
    let m = Mix {
        music_bus: 1.0,
        music: [
            MusicVoice {
                slot: "music:tag",
                gain: 1.0,
            },
            MusicVoice::default(),
        ],
        ..Mix::default()
    };
    d.apply(&m);
    d.apply(&m);
    assert!(d.tracks.is_empty() && st.borrow().players.is_empty());
    assert_eq!(count(&d.notes, "Ton nicht abspielbar"), 1, "{:?}", d.notes);
}
