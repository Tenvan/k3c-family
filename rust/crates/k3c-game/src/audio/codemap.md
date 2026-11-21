# rust/crates/k3c-game/src/audio/

## Responsibility

Ton im nativen Spiel: Klang-Effekte, Menü-Klänge, Wetter- und Reit-Loops und Musik zu Ereignissen und Lage des Frames.
Ohne Gerät, bei Treiberfehler, ohne Ton-Dateien, in Tests und im Aufnahme-Modus bleibt das Spiel stumm, nie ein Absturz.

## Design

Zwei Schichten. Planer (reine Logik, getestet): `register.rs` (Slot-Register `Register`, `load_register`, Skill-Slots),
`atlas.rs` (Ton-Atlas `public/audio/atlas.json`, `Sprite`, `pick`), `events.rs` (Ereignis → `SoundCue`, Lautstärke je
Spielerhälfte, `Ear`, `attenuation`, `night_gain`), `duck.rs` (Ducking), `music.rs` (`MusicState`, Übergangstabelle,
`Fader`, Crossfade), `mix.rs` (`Planner`, `Request`, `Loop`, `Mix`). Gerät: `bank.rs` (`Bank`, `load_bank`, Klänge aus dem
Atlas), `output.rs` (`Output`-Trait, `Silent`, `new_device` über `k3c_platform::AudioOut`, `MAX_VOICES`, `SAMPLE_RATE`).
`mod.rs`: `Sound { plan: Planner, out: Box<dyn Output> }`, `Sound::new` (stumm in Tests oder bei fehlendem Register),
`new_silent`, `new_sound`. Es gibt kein `sink.rs`; Tests nehmen `NullAudio` oder `testutil::FakeAudio`.

## Flow

`Sound` ist `view::layer::Listener`: je Frame einmal `listen`, `Planner::plan` leitet aus dem Frame (Ereignisse, Cues,
Zellen, Phase) den `Mix` ab (Lautstärken, Musik, Ducking, Hub), `Output` spielt ihn über das Gerät. Im Aufnahme-Modus
(`Frame::recording`) wird gerechnet, aber nicht ausgegeben.

## Integration

- Abhängigkeiten: `k3c_platform` (`AudioOut`, Log), `view::frame::Frame`, `view::layer::Listener`; Dateien unter `public/audio/`.
- Konsument: `game` (Paketbildung).
