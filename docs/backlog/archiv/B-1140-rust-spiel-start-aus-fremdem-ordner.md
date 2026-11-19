# B-1140 · Das Rust-Spiel findet Grafik und Schrift nicht, wenn es aus einem fremden Arbeitsordner startet

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU57
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Sichtprüfung des Pakets (RU57.3) startete `dist\k3c-game\k3c-game.exe` aus einem fremden Arbeitsordner: Das Spiel meldet „fehlende Grafik: Schrift public/schrift/k3c.ttf“ samt Hinweis „Erst task gen:paper ausführen“. `DEFAULT_GRAFIK_DIR`, `DEFAULT_RIG_DIR` und `DEFAULT_FONT_PATH` gelten relativ zum Arbeitsordner (wie im Go-Spiel). Doppelklick und Verknüpfungen mit „Ausführen in“ funktionieren, eine Konsole in einem anderen Ordner nicht; der Hinweis „gen:paper“ ist für Empfänger falsch.

## Ziel

Ohne Schalter gestartet wechselt das Spiel in den Ordner der EXE, wenn der Arbeitsordner kein `public/grafik` hat, der EXE-Ordner es aber hat.

## Beteiligte und Zielgruppen

Empfänger des Pakets; Entwickler.

## Anforderungen

- Nur beim Start ohne Schalter (oder nur `play`): relative Pfade in Schaltern dürfen sich nicht verschieben.
- Reine Entscheidungsfunktion mit Tests, kein Plattformcode.

## Nicht-Ziele

Pfade der Dateien auf die EXE verankern (größerer Umbau der Bibliothek-Konfiguration); Text der Fehlermeldung für Empfänger.

## Regeln und Einschränkungen

Neue Datei `game/workdir.rs` und eine Zeile in `game/mod.rs`; `main.rs`.

- **Bereich:** `rust/crates/k3c-game/src/game/workdir.rs`, `rust/crates/k3c-game/src/game/workdir_tests.rs`, `rust/crates/k3c-game/src/game/mod.rs`, `rust/crates/k3c-game/src/main.rs`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`cd C:\temp && D:\k3c-game\k3c-game.exe` startet mit Grafik; `k3c-game.exe --saves s` aus fremdem Ordner meldet wie bisher die fehlende Grafik.

## Ausnahme- und Fehlerfälle

Wechsel schlägt fehl: Arbeitsordner bleibt, das Spiel meldet die fehlende Grafik.

## Akzeptanzkriterien

- **AC-01** Entscheidungsfunktion `anchor_target` ist getestet (Wechsel nur ohne Schalter, nur ohne Spielordner im Arbeitsordner, nur mit Spielordner neben der EXE).
- **AC-02** Das gebaute Paket startet aus einem fremden Arbeitsordner (Aufnahme-Modus nicht nötig: Fenster-Start ohne Fehlermeldung).

## Offene Fragen

keine.

## Notizen

Gefunden bei der Sichtprüfung in RU57.3, 2026-11-19.
