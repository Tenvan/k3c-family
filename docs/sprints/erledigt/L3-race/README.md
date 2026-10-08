# L3 · INF · Race-Detector für die nebenläufigen Go-Pakete

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** INF
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-077
- **Start-Commit:** 657951e
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat („B-077 umsetzen“, Revision 1)

## Ausgangslage

Seit SP07 laufen Räume in eigenen Goroutinen (`engine/room`, `engine/net`). `go test -race` braucht cgo und einen
C-Compiler; unter Windows fehlt `gcc`, und die CI ruft kein `-race` auf (B-077).

## Ziel

Datenrennen in `engine/` fallen in der CI auf. Am Ende sichtbar: Der Go-Job der CI hat einen Schritt mit
`go test -race ./engine/...`, und `task check:go` führt `-race` aus oder meldet, warum nicht.

## Beteiligte und Zielgruppen

Entwickler und Agenten (SRV-Sessions); Review-Sessions. 🧑 gibt frei.

## Anforderungen

B-077 › Anforderungen.

## Nicht-Ziele

`-race` für `tools/k3c-dev`; einen C-Compiler zur Pflicht machen.

## Regeln und Einschränkungen

Befehle nur über `task`; Domäne INF (`Taskfile.yml`, `.github/`, `requirements.md`). Findet `-race` ein Datenrennen
in `engine/`, wird es als Ticket (SRV) erfasst, nicht hier behoben.

## Beispiele

B-077 › Beispiele.

## Ausnahme- und Fehlerfälle

B-077 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Die CI führt `go test -race ./engine/...` aus (B-077/AC-01).
- **AC-02** Lokal ohne C-Compiler meldet die Task den Verzicht auf `-race` und scheitert nicht (B-077/AC-02).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| L3.1 | `L3.1-race.md` | Umsetzung | autonom | fertig |
| L3.2 | `L3.2-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-01 · Review L3.2, `task check` und `task check:go` grün.
- AC-01 umgesetzt (CI-Schritt im Job `go` auf ubuntu-latest, nach `go test ./...`); erster grüner CI-Lauf steht aus (🧑).
- AC-02 geprüft (L3.1: ohne Compiler Hinweis und Exit 0, mit scheiterndem `gcc` Exit 1).
- Befunde: keine. Neue Tickets: keine.
