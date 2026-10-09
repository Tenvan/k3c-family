# B-274 · TestRestore läuft unter Windows auch in task check:all stabil grün

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NT1
- **Projekt:** LST
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Im Probelauf der Release-Checkliste (RL1.2, 2026-10-04) schlug `task check:all` unter Windows einmal fehl: `engine/store/backups_test.go:81: bisheriger Stand nicht gesichert: {"version":1,"campaignId":"a","n":2}` (erwartet `n:3`). Einzeln lief der Test danach 53-mal grün (`go test -count=50 -run TestRestore ./engine/store`), die CI (Linux amd64/arm64) ist grün. Ungeprüft vermutet: Dateizugriff unter Last (Rename in `writeAtomic`, Virenscanner) oder Reihenfolge der Sicherungsnamen in `engine/store/backups.go` (`rotate`, `backups` sortiert nach Name; `-1`-Suffix sortiert vor dem Namen ohne Suffix).

## Ziel

`task check:all` ist unter Windows reproduzierbar grün, der Release-Punkt „Alles grün“ hängt nicht vom Zufall ab; falls die Ursache im Code liegt, verliert `Restore` am Pi keinen bisherigen Stand.

## Beteiligte und Zielgruppen

Entwickler und Agenten unter Windows (führen `task check:all` aus), 🧑 (Release-Freigabe).

## Anforderungen

- Ursache des Fehlschlags benannt (Test oder Code).
- Behebung so, dass der Test unter paralleler Last stabil ist.

## Nicht-Ziele

Windows-Runner in der CI.

## Regeln und Einschränkungen

Spielstände überleben jedes Update (`docs/arbeitsweise.md` › Spielstand); kein `time.Sleep` als Behebung.

## Beispiele

`task check:all` unter Windows, dreimal hintereinander → jedes Mal Exit 0.

## Ausnahme- und Fehlerfälle

nicht relevant (Fehlerbehebung).

## Akzeptanzkriterien

- **AC-01** Ursache im Ticket festgehalten.
- **AC-02** `go test -count=200 -cpu 1,4 -run TestRestore ./engine/store` und dreimal `task check:all` unter Windows grün.

## Offene Fragen

keine

## Notizen

Quelle: RL1.2, Probelauf 2026-10-04.
