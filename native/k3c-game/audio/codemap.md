# native/k3c-game/audio/

## Responsibility

Fachpaket Ton (B-801): spielt Klang-Effekte, Menü-Klänge und Musik zu Ereignissen, Cues und Lage des Frames; ohne Ton-Gerät, bei Treiberfehlern, in Tests und im Aufnahme-Modus bleibt das Spiel stumm.

## Design

- `audio.go`: `New(_ *art.Library)` liefert einen `view.Listener` (`Sound`, `Listen`); Paketkommentar.
- Planer (rein, getestet): `mix.go` (`Mix`, `Request`, `Loop`, `EffectVolume`, `MusicVolume`; Takt 30 Hz), `events.go` (Ereignis → Slot), `music.go` (Musikzustände, `MusicVoice`), `duck.go` (Ducking bei Warnungen).
- Daten: `register.go` (`Register`, `LoadRegister`, `SkillSlot` aus `data/sounds.json`), `atlas.go` (`Atlas`, `Sprite`, `Pick` aus `public/audio/atlas.json`), `bank.go` (Pfade und Laden der Ton-Dateien).
- Gerät: `output.go` (Wiedergabe mit 44,1 kHz), `sink.go` (oto im Spiel, Fake in Tests; `NewPlayer`, `Err`).

## Flow

1. `New` lädt Register und Atlas, öffnet das Ausgabegerät (Fehler → stumm).
2. Je Frame `Listen(frame)` → `events.go`/`mix.go` entscheiden Klänge, Lautstärke und Musik → `duck.go` senkt die Musik bei Warnungen.
3. `output.go` setzt die Anfragen über `sink` (oto) um.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `k3c/engine/sim`, `k3c/data`, `github.com/ebitengine/oto/v3`.
- Konsumenten: `native/k3c-game/game` (Eintrag in `Packages`).
