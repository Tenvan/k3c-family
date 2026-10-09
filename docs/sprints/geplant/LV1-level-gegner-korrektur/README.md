# LV1 · SIM · Level und Gegner: Lava, Camps, Adern-Takt, Orte und IDs

- **Status:** geplant
- **Projekt:** KMP
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-291, B-262, B-200, B-189, B-217, B-260
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

- B-262: Camp-Regel bei Portal-Nähe, entscheidet 🧑.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- LV1.1 Lava und Camps im Level-Generator (AC-01, AC-02).
- LV1.2 Adern-Takt und besiegte Gegner (AC-03, AC-04).
- LV1.3 Ereignisse mit Ort, Schutzplatz ohne Entity-ID (AC-05, AC-06).
- LV1.4 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
