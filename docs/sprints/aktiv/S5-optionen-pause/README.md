# S5 · CLI · Optionen- und Pause-Szene

- **Status:** aktiv
- **Domäne:** CLI
- **Reife:** bereit
- **Einschiebbar:** nein
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
| S5.4 | `S5.4-abnahme-geraet.md` | Workshop | Mensch | offen |
| S5.5 | `S5.5-review.md` | Review | autonom | offen |

## Abnahme

–
