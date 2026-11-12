# NG16 · SIM, SRV · Raum: Absturzbehandlung nach außen, Position nach dem Laden

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** SIM, SRV
- **Reife:** bereit
- **Tickets:** B-814, B-803
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Zwei Befunde am Raum aus dem nativen Spiel: Nach dem Laden stehen Monarchen an der Burg statt an ihrer Stelle (NG9.1; Vorprüfung: `IslandPlayerSave` hat keine Position). Die Absturzbehandlung des Raums (`safeTick`, `Manager.crash` in `engine/room/run.go`) ist nicht exportiert, und `Learn`/`Respec` liefern nur `ErrBadRequest` ohne Grund; `native/k3c-game/local` fängt Panics selbst (`local/step.go:55`). Einzelheiten: B-814, B-803.

## Ziel

Geklärt und belegt ist die Position der Monarchen nach dem Laden; der Raum bietet seine Absturzbehandlung und den Grund abgelehnter Skill-Befehle nach außen an, `local` nutzt beides. Am Ende sichtbar: Tests in `engine/room` und `native/k3c-game/local` grün, Antwort mit Fundstellen in B-814 › Notizen.

## Beteiligte und Zielgruppen

Claude (Prüfung, Umsetzung, Review); Spieler des nativen Spiels (Meldungen); 🧑 Ralf (Nachmeldung der Entscheidung zu B-814).

## Anforderungen

`B-814 › Anforderungen` (Revision 2: prüfen, dann dokumentieren oder beheben), `B-803 › Anforderungen`.

## Nicht-Ziele

Änderungen am Protokoll oder am Verhalten des Servers (B-803), Änderung des Spielstand-Formats, wenn die Prüfung „gewollt“ ergibt (B-814), Sim- und Balance-Läufe.

## Regeln und Einschränkungen

Schichtgrenzen: `engine/room` kennt kein Netz, `engine/` importiert nichts aus `native/`. Sim deterministisch. Golden-Dateien bleiben unverändert, außer eine Behebung zu B-814 verlangt es (dann nach Regel). Keine Rücksicht auf alte Spielstände. Sperre `spielstand` vorsorglich für eine mögliche Behebung zu B-814.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `engine/sim/**`, `engine/room/**`, `native/k3c-game/local/actions.go`, `native/k3c-game/local/actions_test.go`, `native/k3c-game/local/step.go`, `native/k3c-game/local/local_test.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** spielstand

## Beispiele

`B-814 › Beispiele` (Monarch bei x=40 speichern und laden), `B-803 › Beispiele` (Skill ohne Lernpunkt → „kein Lernpunkt“).

## Ausnahme- und Fehlerfälle

`B-803 › Ausnahme- und Fehlerfälle` (Panic im Takt → Raum geschlossen, kein späteres Speichern eines alten Stands). Spielstand ohne Position → Burg als Rückfall.

## Akzeptanzkriterien

- **AC-01** Belegte Antwort zur Position nach dem Laden in B-814 › Notizen (`B-814/AC-01`).
- **AC-02** Gewollt: dokumentiert und Ticket verworfen; Fehler: behoben mit Test im Raum-Paket (`B-814/AC-02`).
- **AC-03** Panic im Takt über den exportierten Weg schließt den Raum, danach keine Speicherung (`B-803/AC-01`).
- **AC-04** Abgelehntes `Learn` nennt den Grund (`B-803/AC-02`).

## Offene Fragen

keine (entschieden autonom, Claude, 2026-11-12: Default „gewollt“ bei fehlender Position im Spielstand, siehe `B-814 › Offene Fragen`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG16.1 | `NG16.1-position-nach-laden.md` | Umsetzung | autonom | offen |
| NG16.2 | `NG16.2-absturz-skill-grund.md` | Umsetzung | autonom | offen |
| NG16.3 | `NG16.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
