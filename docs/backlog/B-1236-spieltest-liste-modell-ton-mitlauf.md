# B-1236 · Spieltest: large-v3 als Standard, Liste nach Uhrzeit, Modelle bei Neu laden, Ton läuft mit dem Bild

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WN1
- **Projekt:** WZN
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Auftrag im Chat
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das Standardmodell war das deutsche Turbo-Modell, das am Ende einer Aufnahme etwa 20 s verlor. Die Session-Liste sortierte die Kennungen als Text („-10“ unter „-9“) und zeigte einen Zähler. Eine neue Modelldatei erschien erst nach einem Neustart. video.mkv hat keinen Ton, der Ton-Player stand über dem Video.

## Ziel

Bessere Auswertung als Standard, eine Liste nach Zeit mit Uhrzeit, Modelle ohne Neustart und ein Video, das sich mit Ton ansehen lässt.

## Beteiligte und Zielgruppen

Ralf (🧑) als Tester.

## Anforderungen

- Standardmodell `ggml-large-v3.bin` (Hugging Face ggerganov/whisper.cpp, SHA-256 geprüft); ohne diese Datei gilt irgendeine Datei aus models/ oder models/whisper/.
- Sessions nach Startzeit absteigend, Anzeige „Datum · Uhrzeit“.
- „Neu laden“ liest Modelle und Voraussetzungen der Auswertung neu.
- Bild-Player oben, Ton-Player darunter; der Ton läuft mit dem Bild (Start, Pause, Springen).

## Nicht-Ziele

Ton in die Videodatei muxen; Offset-Korrektur zwischen Ton und Bild.

## Regeln und Einschränkungen

Dateien ≤ 400 Zeilen.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/frontend/src/playtest/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Neue Datei in models/ ablegen, „Neu laden“ drücken: sie steht in der Modellauswahl. Video abspielen: der Ton läuft mit.

## Ausnahme- und Fehlerfälle

Sessions ohne session.json sortieren nach Kennung ans Ende. Blockiert der Browser das Abspielen, bleibt das Bild ohne Ton.

## Akzeptanzkriterien

- **AC-01** Go-Test: Kennung „-10“ steht vor „-9“, Sortierung nach Startzeit.
- **AC-02** Vitest: Uhrzeit in der Liste, Bild-Player vor Ton-Player, Neu laden ruft die Modelle erneut ab.
- **AC-03** Standardmodell large-v3 mit Größe und Prüfsumme.
- **AC-04** Abnahme durch Ralf: Ton und Bild laufen im Fenster synchron genug zum Prüfen.

## Offene Fragen

Ob der Ton im Fenster ohne Versatz mitläuft (beide Dateien starten mit demselben ffmpeg).

## Notizen

Vergleich an Session 2026-11-22-10: large-v3 (22 s) liefert den ganzen Text bis 120 s, das deutsche Turbo-Modell (7 s) endet bei 102 s.
