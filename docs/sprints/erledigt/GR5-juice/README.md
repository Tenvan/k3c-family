# GR5 · CLI · Juice: Treffer, Screenshake, Münzen

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** CLI
- **Prio:** mittel
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-164
- **Start-Commit:** 1fa9529
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, Revision 1; mit Änderungen aus der Spec-Prüfung

## Ausgangslage

Treffer, Kills und Münzen haben keine Rückmeldung; Feedback-Events liefert erst F3 (B-139) über das Protokoll aus F4 (B-140); die Optionen-Szene mit Abschaltern kommt mit S5 (B-146). Details in B-164.

## Ziel

Treffer, Münzen, Tod und Bauen haben sichtbare Effekte, Screenshake und Blitz sind abschaltbar. Am Ende sichtbar: Effekte am TV und eine Option, die sie ausschaltet.

## Beteiligte und Zielgruppen

Spieler am TV und Handy, auch Lichtempfindliche; Umsetzung durch Agent; 🧑 nimmt am TV ab.

## Anforderungen

B-164 › Anforderungen.

## Nicht-Ziele

Feedback-Events und Protokoll (F3, F4), Optionen-Szene (S5), Sound (SO2).

## Regeln und Einschränkungen

`src/scenes` rechnet nichts; höchstens 3 Blitze pro Sekunde; 2 Spieler im Split-Screen; B nicht belegen. Voraussetzung: F3, F4, S5.

## Beispiele

Gegner getroffen → Blitz; Screenshake „aus“ → Kamera ruhig.

## Ausnahme- und Fehlerfälle

Controller ohne Vibration → keine Vibration, kein Fehler. Unbekanntes Event → ignoriert.

## Akzeptanzkriterien

- **AC-01** Jedes Feedback-Event aus der Liste (Treffer, Kill, Münze aufheben, Münze geben, Bau fertig, Tod; Q08) löst den Effekt aus (B-164/AC-01).
- **AC-02** Mit Screenshake und Blitz „aus“ treten beide nicht auf (B-164/AC-02).
- **AC-03** Die Effekt-Auslösung ändert keinen Spielzustand, `noSim.test.ts` bleibt grün (B-164/AC-03).
- **AC-04** Screenshake wirkt im Split-Screen nur in der Kamera des betroffenen Spielers (B-164/AC-04).
- **AC-05** Die Blitzfrequenz liegt bei höchstens 3 pro Sekunde (B-164/AC-05).
- **AC-06** Auf einem Controller ohne Vibration läuft das Spiel ohne Fehler weiter (B-164/AC-06).
- **AC-07** `task check` ist grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR5.1 | `GR5.1-effekte-aus-events.md` | Umsetzung | autonom | fertig |
| GR5.2 | `GR5.2-abschalten-kamera-vibration.md` | Umsetzung | autonom | fertig |
| GR5.3 | `GR5.3-review.md` | Review | autonom | fertig |
| GR5.4 | `GR5.4-abnahme-tv.md` | Workshop | Mensch | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-04, Review GR5.3: AC-01 (GR5.1: `effects.test.ts`, 7 Event-Arten), AC-03 (`effectFor` rein, `noSim.test.ts` grün), AC-04, AC-05, AC-06, AC-07 (GR5.2: Tests `shakeCell`, `flashAllowed` im 16-ms-Takt, `rumblePad`; `task check` und `task check:go` grün) sind umgesetzt und im Diff geprüft.
- AC-02 (Schalter per Test; Aussehen und Blitz-Eindruck) sowie die Sicht auf AC-01 und AC-04: angenommen, Validierung offen (🧑, TV; im Fahrplan unter „Offen am Gerät“); Vibration auf der Xbox angenommen. Nichts davon im Browser gesehen.
- Keine schweren Befunde: Effekte nutzen `frame.state.events` (nicht `pendingEvents`), Einstellungen je Frame gelesen, Shake nur an der Zelle des getroffenen lokalen Spielers, kein `Math.random()`, `src/input/` und `settings.ts` unverändert, B nicht belegt. B-164 archiviert; Ticket B-217 (SIM, `built`/`playerDown` ohne Ort, Behelf im Client) bleibt offen.
- Version: v0.10.0 vorgeschlagen (Minor: sichtbare Effekte im Spiel; nach dem offenen Vorschlag S5 v0.9.0, bei gemeinsamem Setzen anpassen).
- 2026-10-07: Sprint auf Entscheidung 🧑 abgeschlossen. Die offene Abnahme am Gerät ist nicht durchgeführt (verworfen) und geht in die Gesamtprüfung B-337/AC-05 über; keine weiteren Anzeige- und Touch/Tasten-Abnahmen bis zur Umsetzung von B-337.
