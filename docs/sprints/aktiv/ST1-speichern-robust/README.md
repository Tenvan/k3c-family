# ST1 · SRV · Speichern alle 60 s, unter Windows robust, Rotation der Spielmetrik

- **Status:** aktiv
- **Projekt:** LST
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-186, B-187, B-272
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt)

## Ausgangslage

Der Server speichert beim Verlassen, nicht periodisch (B-186); unter Windows scheitert Speichern an kurz gesperrten Dateien (B-187); Spielmetrik-Reports fallen aus der Rotation (B-272).

## Ziel

Ein Absturz kostet höchstens 60 s Spiel, kein Spielstand oder Bericht geht durch eine gesperrte Datei verloren. Am Ende sichtbar: HUD zeigt „gesichert“, Spielstände überstehen gesperrte Dateien, `reports/` bleibt begrenzt.

## Beteiligte und Zielgruppen

Spielende am TV; 🧑 betreibt den Server am PC oder Pi.

## Anforderungen

B-186 › Anforderungen; B-187 › Anforderungen; B-272 › Anforderungen.

## Nicht-Ziele

Cloud-Speicher, Spielstand-Migration (B-288).

## Regeln und Einschränkungen

SRV; die HUD-Anzeige „gesichert“ läuft als eigene Session (Protokoll-Grenzfall).

## Beispiele

Server speichert um 21:00:00 und 21:01:00 → zwei 💾-Logzeilen, HUD blinkt „gesichert“.

## Ausnahme- und Fehlerfälle

Zieldatei 2 s gesperrt → Wiederholung mit Frist ⏳, danach ❌ im Log, alter Stand bleibt.

## Akzeptanzkriterien

- **AC-01** Der Server speichert alle 60 s und bei Tagesanbruch, das HUD zeigt „gesichert“ (B-186/AC-01, B-186/AC-02, B-186/AC-03).
- **AC-02** Speichern übersteht unter Windows eine kurz gesperrte Zieldatei (B-187/AC-01, B-187/AC-02).
- **AC-03** Die Rotation in reports/ erfasst auch die Spielmetrik-Reports (B-272/AC-01).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| ST1.1 | `ST1.1-speichern-60s.md` | Umsetzung | autonom | fertig |
| ST1.2 | `ST1.2-sperre-rotation.md` | Umsetzung | autonom | fertig |
| ST1.3 | `ST1.3-review.md` | Review | autonom | fertig |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- ST1.1 Speichern alle 60 s und bei Tagesanbruch, Anzeige „gesichert“ (AC-01).
- ST1.2 Wiederholung bei gesperrter Datei, Rotation für Spielmetrik (AC-02, AC-03).
- ST1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

2026-10-25, Claude (ST1.3): `task check` und `task check:go` grün; `check:race` lokal übersprungen (kein C-Compiler), die CI prüft es.
AC-01 erfüllt durch ST1.1 (Autosave 60 s und Tagesanbruch, Ereignis `saved`, HUD), AC-02 und AC-03 durch ST1.2 (Rename-Wiederholung mit Test, Rotation der Session-Reports).
Review: Der MEDIUM-Befund („gesichert“ nach Game Over) ist behoben, zwei LOW-Punkte stehen im Ergebnis von ST1.1.
Offen: Ralf sieht die Anzeige „gesichert“ am TV (B-186/AC-03). Keine neuen Tickets.
