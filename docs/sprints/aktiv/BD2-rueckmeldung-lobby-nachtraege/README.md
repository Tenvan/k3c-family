# BD2 · SIM, SRV, CLI · Rückmeldungs-Ereignisse gedrosselt und geprüft, Lobby startet nie still leer

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** SIM, SRV, CLI
- **Reife:** bereit
- **Tickets:** B-378, B-377, B-382
- **Start-Commit:** 658d5301
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-22, Ralf im Chat (Revision 1)

## Ausgangslage

Drei Nachträge aus den Reviews S9.4 und LB1.2: Eine gehaltene Skill-Taste ohne Ziel erzeugt `castFailed` in jedem Tick (B-378, `engine/sim/skills.go`); kein Server-Test prüft das Beispiel `testdata/protocol/s2c-snapshot-rueckmeldung.json` gegen die Simulation (B-377); die Lobby startet nach `save_not_found` still ein leeres Spiel (B-382, `src/scenes/lobbyLogic.ts`).

## Ziel

Rückmeldungs-Ereignisse belasten das Netz nicht mehr je Tick, ihr Beispiel ist gegen die Simulation geprüft, und die Lobby meldet einen fehlenden Spielstand, statt still neu zu starten. Am Ende sichtbar: `task check:go` und `task check` grün, Hinweis in der Lobby.

## Beteiligte und Zielgruppen

Spielende (weniger Netzlast, kein stiller Datenverlust); Agent in SIM, SRV und CLI nacheinander.

## Anforderungen

B-378, B-377, B-382 › Anforderungen (Ticket-Specs freigegeben, Revision 1).

## Nicht-Ziele

Neue Rückmeldungs-Ereignisse; Änderungen am Protokoll-Format; Lobby-Umbau über B-382 hinaus.

## Regeln und Einschränkungen

Sessions nacheinander SIM → SRV → CLI, je Session nur die Dateien ihrer Domäne. Simulation deterministisch (`engine/rng`), Netz-Budget der Ereignisse (Q08, siehe B-386) darf nicht wachsen.

## Beispiele

Skill-Taste 2 s gehalten ohne Ziel → genau ein `castFailed`. Spielstand zwischen Liste und Auswahl gelöscht → Hinweis „Spielstand nicht gefunden“ statt leerem Spiel.

## Ausnahme- und Fehlerfälle

Taste losgelassen und neu gedrückt → neues `castFailed` erlaubt. Spielstand-Name `familie` (Standard) wie bei der Auswahl → ebenfalls Hinweis, kein stilles `fresh`.

## Akzeptanzkriterien

- **AC-01** Eine gehaltene Skill-Taste ohne Ziel erzeugt höchstens ein `castFailed` je Tastendruck (B-378).
- **AC-02** Ein Server-Test vergleicht die Schlüssel von `s2c-snapshot-rueckmeldung.json` mit den Ereignissen der Simulation (B-377).
- **AC-03** Ein gewählter, nicht mehr vorhandener Spielstand startet kein leeres Spiel, sondern zeigt einen Hinweis (B-382).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD2.1 | `BD2.1-castfailed-drosseln.md` | Umsetzung | autonom | fertig |
| BD2.2 | `BD2.2-rueckmeldung-beispiel-test.md` | Umsetzung | autonom | fertig |
| BD2.3 | `BD2.3-lobby-save-fallback.md` | Umsetzung | autonom | fertig |
| BD2.4 | `BD2.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
