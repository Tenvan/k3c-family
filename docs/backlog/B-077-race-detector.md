# B-077 · Die nebenläufigen Go-Pakete werden mit dem Race-Detector geprüft

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** L3
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat („B-077 umsetzen“, Revision 1)

## Ausgangslage

Ab SP07 laufen Räume in eigenen Goroutinen (`engine/room`, `engine/net`). Die Sessions verlangen
`go test -race ./engine/...`, aber `-race` braucht cgo und einen C-Compiler. Unter Windows fehlt `gcc` (SP07.1:
„cgo: C compiler gcc not found“), und die CI (`.github/workflows/ci.yml`) ruft nur `go test ./...` ohne `-race` auf.
Datenrennen fallen so nirgends automatisch auf.

## Ziel

Datenrennen in den nebenläufigen Paketen fallen in der CI auf, auch wenn lokal kein C-Compiler da ist.

## Beteiligte und Zielgruppen

Entwickler und Agenten (SRV-Sessions ab SP07); Review-Sessions.

## Anforderungen

- Der Linux-Job der CI führt `go test -race` für `./engine/...` aus.
- Eine Task (`task check:race` oder Teil von `task check:go`) führt `-race` aus, wenn cgo verfügbar ist, und meldet
  sonst klar, dass es übersprungen wird.

## Nicht-Ziele

`gcc` auf jedem Entwickler-Rechner zur Pflicht machen; `-race` für `tools/k3c-dev`.

## Regeln und Einschränkungen

Befehle nur über `task` (CLAUDE.md); `requirements.md` nennt ggf. die optionale Voraussetzung.

## Beispiele

PR mit einem Datenrennen in `engine/room` → der CI-Job scheitert mit dem Bericht des Race-Detectors.

## Ausnahme- und Fehlerfälle

Rechner ohne C-Compiler → die Task überspringt `-race` mit Hinweis, der Rest von `task check:go` läuft.

## Akzeptanzkriterien

- **AC-01** Die CI führt `go test -race ./engine/...` aus (Job-Log).
- **AC-02** Lokal ohne C-Compiler meldet die Task den Verzicht auf `-race` und scheitert nicht daran.

## Offene Fragen

keine

## Notizen

Entdeckt in SP07.1 (2026-10-01).
