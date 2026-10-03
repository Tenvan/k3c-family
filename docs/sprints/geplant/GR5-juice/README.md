# GR5 · CLI · Juice: Treffer, Screenshake, Münzen

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-164
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

- **AC-01** Jedes Feedback-Event aus der Liste (Treffer, Kill, Münze aufheben, Bau fertig) löst den Effekt aus (B-164/AC-01).
- **AC-02** Mit Screenshake und Blitz „aus“ treten beide nicht auf (B-164/AC-02).
- **AC-03** Die Effekt-Auslösung ändert keinen Spielzustand, `noSim.test.ts` bleibt grün (B-164/AC-03).
- **AC-04** Screenshake wirkt im Split-Screen nur in der Kamera des betroffenen Spielers (B-164/AC-04).
- **AC-05** Die Blitzfrequenz liegt bei höchstens 3 pro Sekunde (B-164/AC-05).
- **AC-06** Auf einem Controller ohne Vibration läuft das Spiel ohne Fehler weiter (B-164/AC-06).
- **AC-07** `task check` ist grün.

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GR5.1 Effekte für Treffer, Kill, Münze, Bauen, nur aus Events (AC-01, AC-03).
- GR5.2 Abschalten über Optionen, Split-Screen-Kamera, Blitzgrenze, Vibration (AC-02, AC-04, AC-05, AC-06).
- GR5.3 Review (AC-07).

## Abnahme

–
