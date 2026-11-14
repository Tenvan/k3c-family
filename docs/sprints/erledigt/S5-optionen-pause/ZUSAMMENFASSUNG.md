# S5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Optionen- und Pause-Szene
- **Projekt:** –

## Ziel

Eine Szene für Optionen und Pause ist mit Controller, Tastatur und Touch bedienbar und speichert Lautstärke Musik/SFX, Screenshake aus, Flash aus und Farbschwäche-Symbole je Gerät. Am Ende sichtbar: Einstellungen bleiben nach dem Neuladen erhalten.

## Ergebnis

- 2026-10-04, Review S5.5: AC-01 (S5.1: `settings.ts`, 11 Tests), AC-02 und AC-04 (S5.2: `optionsLogic`, `OptionsScene`, B unbelegt, `shell.ts` unverändert) und AC-06 (S5.3: `texts*.ts`, `textRule.test.ts`) sind umgesetzt und im Diff geprüft; AC-03 Speicher-Teil per Test (S5.1), Neuladen im Browser offen. - AC-05, AC-07 und AC-03 (Neuladen): angenommen, Validierung offen (S5.4, 🧑 am Gerät; im Fahrplan unter „Offen am Gerät“). Nichts davon im Browser gesehen. - Keine schweren Befunde: Menu-kurz (< 600 ms, View blockiert) kollidiert nicht mit der Home-Kombi (400 ms), mit 2 lokalen Spielern stehe …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die Einstellungen liefern Standardwerte, begrenzen und fallen bei kaputtem Speicher zurück … |
| AC-02 | – | Die Szene ist mit Controller, Tastatur und Touch bedienbar und stellt alle Optionen getren … |
| AC-03 | – | Die Einstellungen bleiben nach Neuladen erhalten (B-146/AC-03). |
| AC-04 | – | View + Menu führt weiter zur Landingpage, B bleibt unbelegt (B-146/AC-04). |
| AC-05 | – | 🧑 hat die Szene am TV und am Handy abgenommen (B-146/AC-05). |
| AC-06 | – | Alle Texte kommen aus zentralen Textdateien (de, en), die Sprache ist in den Optionen wähl … |
| AC-07 | – | 🧑 hat die englischen Texte abgenommen (B-172/AC-05). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-146 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 – | Der Client hat eine Optionen- und Pause-Szene mit getrennter Lautstärke und Barr … |
| B-172 | erledigt | AC-06 –, AC-07 – | Der Client hat Deutsch und Englisch mit Sprachauswahl in den Optionen |
