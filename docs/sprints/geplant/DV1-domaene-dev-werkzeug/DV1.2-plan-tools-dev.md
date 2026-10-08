# DV1.2 · plan-Tools mit Domäne DEV und ohne Sprint-Prio, Werkzeug-Tickets auf DEV

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** dv1/2-plan-tools-dev
- **Abhängig von:** DV1.1
- **Tickets:** B-365, B-361
- **Kriterien:** AC-03, AC-04, AC-05

## Ziel

Die plan-Tools kennen die Domäne `DEV` und schreiben an Sprints weder `Prio` noch `Einschiebbar`; offene Werkzeug-Tickets und die Sessions DV1.3/DV1.4 tragen `DEV`.

## Kontext

- Domänenliste und Feld-Schema: `tools/k3c-dev/internal/planning/edit.go` (Zeile 13 `domains`, Zeile 23 Sprint-Schema mit `Prio`/`Einschiebbar`, Zeilen 150–151 `Prio` folgt den Tickets).
- `create.go` `createSprint` (Zeilen 115–119): setzt `Prio` aus `SprintPrio` und Default `Einschiebbar: nein`.
- `list.go`: `SprintPrio` (Zeile 131) und Sprint-Zeile mit `Prio` (Zeile 105); `planning.go` `Sprint.Prio` (Zeile 35, JSON `prio`); `priority.go` ordnet Sprints ohne Projekt nach Prio (Übergang, seit PJ3 haben alle offenen Sprints ein Projekt).
- `tables.go` `roadmapMarker`: Zweig `einschiebbar == "ja"` → `**Einschiebbar**`-Tabelle entfällt (DV1.1 hat die Zeile aus dem Fahrplan gelöscht).
- Tests: `write_test.go` (`TestSprintPrioFolgtTickets`, Einschiebbar-Fall Zeile 130, Fixture `indexMD`/Fahrplan Zeile 21), `domains_test.go` `TestProjektSprintVorPrio`, `priority_test.go`, `mcpsrv/roundtrip_test.go`, `mcpsrv/tools_planning.go` (Beschreibungstexte).
- Frontend: Chips kommen aus den Daten (`frontend/src/planning/planning.ts` `domains()`); `api/types.ts` Zeile 318 Kommentar, `api/mockPlanning.ts` (Glossar-Zeile `Einschiebbar`, Mock-Tickets) nachziehen, damit die Mock-Ansicht einen `DEV`-Chip zeigt.
- Ticket-Umstellung: Kandidaten (Dateien in `tools/k3c-dev/`, `cmd/k3c-load/`, `cmd/k3c-tui/`, `src/tools/`) aus `plan_list status: offen` und `eingeplant`, je Ticket an Ausgangslage/Anforderungen prüfen, z. B. B-341, B-360, B-362, B-363, B-366, B-368, B-092, B-099, B-298, B-299, B-284, B-286. Nur Tickets, deren Arbeit ganz in DEV-Dateien liegt. Geplante Sprints: Sessions mit Dateien nur dort auf `DEV` (Sprint-Domäne folgt).

## Erlaubte Dateien

- `tools/k3c-dev/internal/planning/`, `tools/k3c-dev/internal/mcpsrv/tools_planning.go`, `tools/k3c-dev/internal/mcpsrv/roundtrip_test.go`
- `tools/k3c-dev/frontend/src/api/types.ts`, `tools/k3c-dev/frontend/src/api/mockPlanning.ts`, `tools/k3c-dev/frontend/src/planning/`
- Planungs-Dateien (nur über plan-Tools: Domäne von Tickets und Sessions, Status)

## Nicht-Ziele

B-360 (fehlendes Feld ablehnen allgemein), B-363, B-366 (Spalte Projekt im Fahrplan). Erledigte Tickets/Sprints bleiben bei ihrer Domäne. Ticket-Prio bleibt.

## Schritte

1. Failing Go-Tests: Ticket mit `Domäne: DEV` anlegen geht; `Domäne: TOOL` wird mit der Liste der gültigen Domänen abgelehnt; `plan_create {kind: sprint}` schreibt weder `Prio` noch `Einschiebbar`; `plan_set` an einem Sprint mit `Prio` wird abgelehnt.
2. `edit.go`: `DEV` in `domains`, `Prio`/`Einschiebbar` aus dem Sprint-Schema, Prio-Ableitung in `Set` entfernen.
3. `create.go`: Prio-Ableitung und Default `Einschiebbar` raus. `tables.go`: `roadmapMarker` ohne Einschiebbar-Zweig.
4. `SprintPrio`, `Sprint.Prio` und die Prio-Ordnung für Sprints ohne Projekt entfernen bzw. auf Rang reduzieren (`list.go`, `planning.go`, `priority.go`); Frontend-Typen und Mocks nachziehen. Alte Tests anpassen.
5. `check_run dev:test` und `go:test` grün; k3c-dev neu starten lassen (🧑) oder Tests als Nachweis nehmen.
6. Mit den neuen plan-Tools: offene Werkzeug-Tickets (Kontext) und die Sessions DV1.3, DV1.4 auf `Domäne: DEV` setzen.
7. `task check` grün.

## Fertig, wenn

- [ ] AC-03: Go-Test legt Ticket mit `Domäne: DEV` an und setzt es per `Set`; `TOOL` wird mit Liste abgelehnt; `task test -- planning` grün mit einem Ticket in `DEV`.
- [ ] AC-04: Go-Test: `Create` sprint ohne `Prio`/`Einschiebbar`, `Set` schreibt sie nicht (Prio an Sprint abgelehnt).
- [ ] AC-05: `plan_list domain: DEV` listet die umgestellten offenen Werkzeug-Tickets; Liste im Ergebnis.
- [ ] `task check` und `task check:dev` grün.

## Prüfen

```bash
task check
task check:dev
```

Keine manuellen Prüfungen. Die neuen plan-Tools wirken erst nach einem Neustart von k3c-dev; ohne Neustart Schritt 6 mit den Tests belegen und die Umstellung von Hand im Rückfall-Weg (arbeitsweise.md) machen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
