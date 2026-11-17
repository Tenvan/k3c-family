# RU4.8 · Ton: AudioOut, Gerät, NullAudio

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.1
- **Tickets:** B-1014
- **Kriterien:** AC-04

## Ziel

`k3c-platform` spielt OGG, MP3 und WAV über `trait AudioOut`; ohne Gerät oder headless läuft alles stumm ohne Fehler.

## Kontext

Braucht die Freigabe der Crate `rodio` durch 🧑 (RU4-README › Braucht 🧑); ohne Freigabe Status `blockiert`. Vorgaben: RU4-README › Vorentscheidungen › Ton. Go-Vorbild für das Verhalten am Gerät: `native/k3c-game/audio/sink.go` (oto-Player), `output.go` (Gerät starten, bei Fehler stumm bis zum Neustart, Warnung einmal je Slot, B-805), Rate 44 100 Hz. Testdateien: `public/audio/probe-a.ogg`, `probe-a.mp3`. Den Port des Go-Pakets `audio` (Mischen, Bank, Atlas, Ducking) macht RU17 auf dieser Schnittstelle.

## Erlaubte Dateien

- `rust/crates/k3c-platform/**`
- `rust/Cargo.toml`
- `rust/Cargo.lock`

## Nicht-Ziele

Port von `native/k3c-game/audio` (RU17). Ton an Ereignissen der Sim.

## Schritte

1. `rodio` mit fester Version, ohne Standard-Features, nur Wiedergabe sowie Vorbis, MP3, WAV, in `rust/Cargo.toml`; in `k3c-platform/Cargo.toml` einbinden.
2. `src/audio.rs`: `trait AudioOut`, `SoundId`, `VoiceId`, `NullAudio` (zählt Aufrufe, `playing` immer falsch); `open_audio() -> Box<dyn AudioOut>`.
3. `src/backend/sound.rs`: Gerät öffnen, Klänge beim `load` dekodieren und als Samples halten, Stimmen mit Lautstärke, Schleife, Stopp; Gerätefehler → `failed()` und stumm, Log-Warnung einmal.
4. `tests/layering.rs`: `rodio::` und `cpal::` nur in `src/backend/`.
5. Tests: `src/audio_tests.rs` (`NullAudio`; Dekodieren von `probe-a.ogg` und `.mp3` liefert Samples mit 44,1 kHz, ohne Gerät).
6. Probe: Taste T spielt `probe-a.ogg` (Beobachtung im Ergebnis).

## Fertig, wenn

- [ ] AC-04: `cargo test -p k3c-platform audio` grün (Dekodieren OGG und MP3, `NullAudio`).
- [ ] AC-04: `open_audio` ohne Gerät liefert `NullAudio` mit Warnung (Test mit erzwungenem Fehlerpfad).
- [ ] `task rust:lint` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
