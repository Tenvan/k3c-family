# B-1235 · Spieltest: Aufnahme beginnt mit dem Spiel im Vollbild, Pegel statt Datenrate beim Ton, Whisper 30 s

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Auftrag im Chat
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Beim Spieltest begann die Aufnahme (Ton und Bild) vor dem Spiel, also schon während des Ladens. Das Spiel lief im Fenster. Das Ton-Diagramm zeigte die Datenrate, bei PCM konstant und damit kein Ausschlag. Whisper erkannte 20 s am Stück, lange Sätze wurden abgeschnitten.

## Ziel

Ton und Bild beginnen mit dem ersten Bild des Spiels, das Spiel läuft immer im Vollbild, das Ton-Diagramm zeigt den echten Pegel, und die Transkription nutzt das Maximum, das sinnvoll ist.

## Beteiligte und Zielgruppen

Ralf (🧑) als Tester.

## Anforderungen

- Das Spiel startet zuerst und schreibt `<ordner>/ready`, sobald sein erstes richtiges Bild steht; erst dann startet ffmpeg (Frist 30 s, danach trotzdem).
- Der Status meldet `starting`, Stopp beendet das Spiel schon beim Laden; scheitert ffmpeg, endet das Spiel wieder.
- Mit `--playtest` startet das Spiel immer im Vollbild (rahmenlos, aktueller Monitor).
- Das Ton-Diagramm zeigt den Pegel der letzten Sekunde in Prozent (RMS, −60 bis 0 dBFS).
- Whisper `queue=30`; mehr bringt nichts, whisper.cpp arbeitet intern in 30-s-Fenstern.

## Nicht-Ziele

Überlappende Transkriptionsfenster oder ein VAD-Modell; Monitorwahl im Spiel.

## Regeln und Einschränkungen

Schreiben nur im Repo; Dateien ≤ 400 Zeilen.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/frontend/src/playtest/**`, `rust/crates/k3c-game/src/playtest/**`, `rust/crates/k3c-game/src/game/run.rs`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Start: Status „Spiel startet …", nach dem Menübild beginnen Ton und Bild. Wellen im Ton-Diagramm folgen der Stimme.

## Ausnahme- und Fehlerfälle

Spiel ohne `ready` (alte EXE): Aufnahme beginnt nach 30 s. Spiel endet vor dem ersten Bild: Start scheitert, Ordner weg. ffmpeg scheitert: Spiel wird beendet.

## Akzeptanzkriterien

- **AC-01** Go-Tests: Aufnahme beginnt erst nach `ready`, Stopp während des Ladens, ffmpeg-Fehler beendet das Spiel, Spielstart-Fehler startet kein ffmpeg.
- **AC-02** Rust-Test: `announce_ready` schreibt `ready`; Smoke mit echtem Spiel: Datei erscheint nach dem ersten Bild.
- **AC-03** Pegel-Berechnung getestet (Stille, Vollaussteuerung, Ende der Datei).
- **AC-04** `WhisperArgs` enthält `queue=30`.
- **AC-05** Abnahme durch Ralf: Vollbild auf dem gewählten Monitor, Aufnahme beginnt mit dem Spiel, Pegel folgt dem Mikro.

## Offene Fragen

Das Spiel geht im Vollbild auf den Monitor, auf dem das Fenster startet; dieser kann vom aufgenommenen Monitor abweichen.

## Notizen

Messung an Session 2026-11-22-5: queue 30 und 60 geben bis auf Einzelwörter denselben Text; ab etwa 30 s entscheidet das Modellfenster.
