# WZ18 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Welten im Spiel: Welt-Dateien, Welt-Wahl im Raum und Welten-Editor
- **Projekt:** WZG

## Ziel

Welten liegen als Dateien unter `data/worlds/`; der Raum wählt beim Anlegen (Dev-Mode) eine Welt, der Spielstand speichert Welt-ID und Insel-ID (alte Stände werden migriert), mehrere Inseln sind spielbar, und der Welten Editor legt Welten und Inseln an und bearbeitet sie (`B-488 › Ziel`, ohne Chat). **Am Ende sichtbar:** im Asset-Studio eine Welt mit zwei Inseln als Entwurf, im Spiel im Dev-Mode g …

## Ergebnis

- 2026-10-19, Review WZ18.7 (Claude, getrennter Reviewer): AC-01–AC-05 und AC-07 geprüft (Nachweis in den Ergebnissen WZ18.1–WZ18.7), AC-06 offen (WZ18.8, 🧑 am PC). - Prüfungen: `task check` (Lint ohne ungetracktes `wailsjs/`), `task build`, `task check:go`, `task check:studio` grün; Golden byte-gleich; nur `standard.json` eingecheckt. - Behoben: veraltete `islands.json`-Pfade in Kommentaren/Notizen, Protokoll-Version 7 in Codemaps und Glossar. Keine schweren Befunde. - Neue Tickets: B-499 (Studio „Im Spiel öffnen“ zweimal in eigener Welt), B-500 (`fullsave.go` Tiefen aus der Welt). Hinweis zu …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-488/AC-01` Regeltext und Glossar („Welt“, „Welt-Datei“), Format `data/worlds/<id>.json` … |
| AC-02 | geprüft | `B-488/AC-02` Prüfung jeder Welt-Datei; `standard.json` gleich der bisherigen `islands.jso … |
| AC-03 | geprüft | `B-488/AC-03` Raum mit gewählter Welt (Dev-Mode), Spielstand mit Welt-ID und Insel-ID, Mig … |
| AC-04 | geprüft | `B-488/AC-04` Protokoll mit Welt-ID, Beispiel in `docs/protocol.md`, Versionen erhöht. |
| AC-05 | geprüft | `B-488/AC-05` Welten Editor legt Welten und Inseln an und bearbeitet sie; Speichern nur Gü … |
| AC-06 | offen | `B-488/AC-09` (Teil Hand) 🧑 nimmt eine Welt mit zwei Inseln am PC als Entwurf ab und start … |
| AC-07 | geprüft | `B-488/AC-10` Das Review weist jedes Kriterium nach; `task check:all` und `task check:go` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-488 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 offen, AC-07 geprüft | Welten und Inseln im Asset-Studio anlegen und bearbeiten, Welt per Prompt füllen |
