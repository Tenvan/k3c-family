# LV1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Level und Gegner: Lava, Camps, Adern-Takt, Orte und IDs
- **Projekt:** KMP

## Ziel

Level und Gegner verhalten sich nach Regelwerk, Golden-Läufe ändern sich nur durch echte Wirkung. Am Ende sichtbar: Keine Lava auf Linien, Camps mit Abstand, besiegte Gegner lassen Gold fallen, Ereignisse mit Ort.

## Ergebnis

Review LV1.4 (autonom, Claude, 2026-10-24): AC-01 bis AC-06 haben je einen Go-Test als Nachweis. Kein schwerer Befund. Der Code lag schon in develop (fc14a8ef, #446), deshalb gab es den Review-Diff nur im Squash. Verschoben: B-200/AC-01, also die Messung über 100 Seeds (B-565), und der stabile Schutzplatz samt Golden-Nachweis (B-605). `task check` grün. `check:go` hatte nur Last-Flackern in k3c-load und devrun, einzeln grün. `-race` läuft nur in der CI, weil lokal kein C-Compiler da ist. KR6 bestätigt die Vorentscheidungen (Mensch).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Lava liegt nicht auf den Mauerlinien (B-291/AC-01). |
| AC-02 | geprüft | Camps liegen nach dem Abstand zu den Linien nicht zu nah an den Portalen (B-262/AC-01). |
| AC-03 | geprüft | Der Aggressionspool unter Tage bleibt auch mit Adern im Wellen-Korridor (B-200/AC-02, B-20 … |
| AC-04 | geprüft | Ein besiegter Gegner verschwindet nicht im Portal, sondern lässt sein Gold fallen (B-189/A … |
| AC-05 | geprüft | Die Ereignisse `built` und `playerDown` tragen ihren Ort (B-217/AC-01, B-217/AC-02). |
| AC-06 | geprüft | Der Schutzplatz einer Truppe hängt nicht an ihrer Entity-ID (B-260/AC-01). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-291 | erledigt | AC-01 geprüft | Lava liegt nicht auf den Mauerlinien |
| B-262 | erledigt | AC-02 geprüft | Camps liegen nach dem Abstand zu den Linien nicht zu nah an den Portalen |
| B-200 | erledigt | AC-03 geprüft | Der Aggressionspool unter Tage bleibt auch mit Adern im Wellen-Korridor |
| B-189 | erledigt | AC-04 geprüft | Ein besiegter Gegner verschwindet nicht im Portal, sondern lässt sein Gold falle … |
| B-217 | erledigt | AC-05 geprüft | Die Ereignisse `built` und `playerDown` tragen ihren Ort |
| B-260 | erledigt | AC-06 geprüft | Der Schutzplatz einer Truppe hängt nicht an ihrer Entity-ID |
