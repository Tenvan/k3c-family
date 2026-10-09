# S5 · CLI · Optionen- und Pause-Szene

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-146, B-172
- **Start-Commit:** 9e6849e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-146, B-172

## Ausgangslage

Die Aktion `pause` existiert in `src/input/playerInput.ts`, aber keine Szene; es gibt keine getrennten Lautstärken, keinen Schalter für Screenshake oder Flash und keine Farbschwäche-Symbole. Voraussetzung: Pause-Regel aus B-135 (F1). Das Speichern beim Verlassen ist Serverarbeit (S2, B-147).

## Ziel

Eine Szene für Optionen und Pause ist mit Controller, Tastatur und Touch bedienbar und speichert Lautstärke Musik/SFX, Screenshake aus, Flash aus und Farbschwäche-Symbole je Gerät. Am Ende sichtbar: Einstellungen bleiben nach dem Neuladen erhalten.

## Beteiligte und Zielgruppen

Spieler am TV, Handy und PC; 🧑 nimmt am Gerät ab.

## Anforderungen

B-146 › Anforderungen.

## Nicht-Ziele

Audio-Mixer (B-011), Juice-Effekte (B-164), Controller-Glyphen (S6), Spielstand speichern (S2).

## Regeln und Einschränkungen

`CLAUDE.md` (View + Menu reserviert, B nie belegen, Home-Button oben, Client rechnet nichts). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`.

## Beispiele

Spieler öffnet mit Menu die Pause, stellt Musik auf 0 %, schaltet Screenshake aus → nach dem Neuladen gelten die Werte noch.

## Ausnahme- und Fehlerfälle

`localStorage` gesperrt → Standardwerte, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Die Einstellungen liefern Standardwerte, begrenzen und fallen bei kaputtem Speicher zurück (B-146/AC-01).
- **AC-02** Die Szene ist mit Controller, Tastatur und Touch bedienbar und stellt alle Optionen getrennt ein (B-146/AC-02).
- **AC-03** Die Einstellungen bleiben nach Neuladen erhalten (B-146/AC-03).
- **AC-04** View + Menu führt weiter zur Landingpage, B bleibt unbelegt (B-146/AC-04).
- **AC-05** 🧑 hat die Szene am TV und am Handy abgenommen (B-146/AC-05).
- **AC-06** Alle Texte kommen aus zentralen Textdateien (de, en), die Sprache ist in den Optionen wählbar und bleibt erhalten (B-172/AC-01, B-172/AC-02, B-172/AC-03, B-172/AC-04).
- **AC-07** 🧑 hat die englischen Texte abgenommen (B-172/AC-05).

## Offene Fragen

Pause im gemeinsamen Raum: 🧑, `docs/fragenkatalog.md Q01`.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S5.1 | `S5.1-einstellungen-speicher.md` | Umsetzung | autonom | fertig |
| S5.2 | `S5.2-szene-optionen-pause.md` | Umsetzung | autonom | fertig |
| S5.3 | `S5.3-texte-de-en.md` | Umsetzung | autonom | fertig |
| S5.4 | `S5.4-abnahme-geraet.md` | Workshop | Mensch | fertig |
| S5.5 | `S5.5-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-04, Review S5.5: AC-01 (S5.1: `settings.ts`, 11 Tests), AC-02 und AC-04 (S5.2: `optionsLogic`, `OptionsScene`, B unbelegt, `shell.ts` unverändert) und AC-06 (S5.3: `texts*.ts`, `textRule.test.ts`) sind umgesetzt und im Diff geprüft; AC-03 Speicher-Teil per Test (S5.1), Neuladen im Browser offen.
- AC-05, AC-07 und AC-03 (Neuladen): angenommen, Validierung offen (S5.4, 🧑 am Gerät; im Fahrplan unter „Offen am Gerät“). Nichts davon im Browser gesehen.
- Keine schweren Befunde: Menu-kurz (< 600 ms, View blockiert) kollidiert nicht mit der Home-Kombi (400 ms), mit 2 lokalen Spielern stehen alle Sitzplätze und jedes Gerät bedient die Szene, `localStorage` wirft nie; die Änderung an `loadLogic.ts` und `stageView.ts` (nur Texte, gleiche Domäne) ist nicht schwer. `task check` und `task check:go` grün. Neue Tickets: keine (B-214, B-215 aus S5.2/S5.3 offen).
- Version: v0.9.0 vorgeschlagen (Minor: neue Optionen- und Pause-Szene sowie Sprachwahl de/en; nach dem offenen Vorschlag S4 v0.8.0, bei gemeinsamem Setzen anpassen).

- 2026-10-07, S5.4: Touch am PC (`?touch=1`) geprüft, Mangel: Optionen per Touch nicht schließbar, Touch-Overlay verdeckt das Menü → B-336; AC-05 bleibt offen.
- 2026-10-07: Sprint auf Entscheidung 🧑 abgeschlossen. Die offene Abnahme am Gerät ist nicht durchgeführt (verworfen) und geht in die Gesamtprüfung B-337/AC-05 über; keine weiteren Anzeige- und Touch/Tasten-Abnahmen bis zur Umsetzung von B-337.
