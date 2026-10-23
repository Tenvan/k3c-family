# LV1 · SIM · Level und Gegner: Lava, Camps, Adern-Takt, Orte und IDs

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-291, B-262, B-200, B-189, B-217, B-260
- **Start-Commit:** a7e95e1d
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Lava liegt teils auf Mauerlinien (B-291), Camps nah an Portalen (B-262), der Aggressionspool unter Tage reißt mit Adern den Korridor (B-200), besiegte Gegner verschwinden im Portal (B-189), `built`/`playerDown` haben keinen Ort (B-217), Schutzplätze hängen an Entity-IDs (B-260).

## Ziel

Level und Gegner verhalten sich nach Regelwerk, Golden-Läufe ändern sich nur durch echte Wirkung. Am Ende sichtbar: Keine Lava auf Linien, Camps mit Abstand, besiegte Gegner lassen Gold fallen, Ereignisse mit Ort.

## Beteiligte und Zielgruppen

🧑 entscheidet die Camp-Regel (B-262); Agent baut in `engine/sim/` und `engine/level/`.

## Anforderungen

B-291 › Anforderungen; B-262 › Anforderungen; B-200 › Anforderungen; B-189 › Anforderungen; B-217 › Anforderungen; B-260 › Anforderungen.

## Nicht-Ziele

Neue Gegner und Traits (K1).

## Regeln und Einschränkungen

SIM; Werte für den Adern-Takt beschließt REG (RG1, B-289) vorher.

## Beispiele

Seed mit Lava am Rand → Mauerlinie verschoben, kein Platz auf Lava.

## Ausnahme- und Fehlerfälle

Kein Platz außerhalb der Lava → Linie entfällt mit Log-Warnung, Level bleibt spielbar.

## Akzeptanzkriterien

- **AC-01** Lava liegt nicht auf den Mauerlinien (B-291/AC-01).
- **AC-02** Camps liegen nach dem Abstand zu den Linien nicht zu nah an den Portalen (B-262/AC-01).
- **AC-03** Der Aggressionspool unter Tage bleibt auch mit Adern im Wellen-Korridor (B-200/AC-01, B-200/AC-02, B-200/AC-03).
- **AC-04** Ein besiegter Gegner verschwindet nicht im Portal, sondern lässt sein Gold fallen (B-189/AC-01).
- **AC-05** Die Ereignisse `built` und `playerDown` tragen ihren Ort (B-217/AC-01, B-217/AC-02).
- **AC-06** Der Schutzplatz einer Truppe hängt nicht an ihrer Entity-ID (B-260/AC-01).

## Offene Fragen

Vorentschieden (autonom, Claude, 2026-10-23), Bestätigung durch 🧑 über KR6:

- **B-262:** „so lassen“ (Beschluss 🧑 2026-10-04 im Ticket); AC-02 ist damit der Vermerk plus grüner Camp-Test, keine Codeänderung.
- **B-291:** Generator-Regel: Ein Lava-Chunk, dessen Streifen einen Linien-Platz überlappt, wird zu einem gewöhnlichen Chunk (kein zusätzlicher Würfel); ändert nur den Eisenstollen-Golden.
- **B-200:** keine Wertänderung, solange der Go-Test zeigt, dass der Pool mit 2 besetzten Adern ohne Kills nicht vor 4 min voll ist (Rechnung: 7,7 min). Die Messung (AC-01) läuft über `sim_test`; geht das nicht, Kriterium „nicht gemessen“ mit Ticket.
- **B-217:** `x` an `built` und `playerDown` ist zusätzlich (keine neue Protokoll-Version); der Client nutzt das Feld und behält den Rückfall für ältere Server.
- Golden-Änderungen (B-291, B-260, ggf. B-217) je in eigenem Commit mit Begründung (`task go:golden`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LV1.1 | `LV1.1-lava-camps.md` | Umsetzung | autonom | fertig |
| LV1.2 | `LV1.2-adern-takt-besiegte-gegner.md` | Umsetzung | autonom | fertig |
| LV1.3 | `LV1.3-ereignis-ort-schutzplatz.md` | Umsetzung | autonom | offen |
| LV1.4 | `LV1.4-review.md` | Review | autonom | offen |

## Abnahme

–
