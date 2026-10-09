# DV1.2 · plan-Tools mit Domäne DEV und ohne Sprint-Prio, Werkzeug-Tickets auf DEV

- **Status:** fertig
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

- AC-03 geprüft: `TestDomaeneDEV` (Go) legt ein Ticket in `DEV` an, setzt ein SRV-Ticket per `Set` auf `DEV` (Index nachgezogen), `TOOL` wird mit der Liste `REG, SIM, SRV, CLI, PLAT, INF, DEV` abgelehnt; `TestPlanungsToolsRoundtrip` legt per `plan_create` ein DEV-Ticket an und findet es mit `plan_list domain: DEV`. `task test -- planning` grün mit sechs Tickets in `DEV`.
- AC-04 geprüft: `TestSprintOhnePrioUndEinschiebbar` (Go): `Create` sprint und `Set` Tickets schreiben weder `Prio` noch `Einschiebbar`, `Set` mit `Prio` oder `Einschiebbar` am Sprint wird abgelehnt (unbekanntes Feld, nichts geändert); `plan_set X1 {Prio}` im Roundtrip ebenfalls abgelehnt. `SprintPrio`, `Sprint.Prio`, `maxPrio`, `prioRank` und der Einschiebbar-Zweig in `roadmapMarker` entfallen; Sprints ohne Projekt stehen in Fahrplan-Reihenfolge hinter denen mit Projekt.
- AC-05 geprüft: `plan_list domain: DEV` listet B-284, B-341, B-360, B-362, B-363, B-366; DV1.3 und DV1.4 tragen `DEV`, DV1 damit `INF, SRV, DEV` (Feld, Überschrift, Fahrplan).
- Nicht umgestellt (Arbeit nicht ganz in DEV-Dateien): B-092, B-299 (Seiten, PLAT), B-298 (`data/assets.json`, `public/`), B-099 (`engine/sim`, `data/`), B-286 (Fehlerquelle evtl. im Server), B-352 (Protokoll, `engine/room`), B-368 (auch `tests/planning.test.ts`, `arbeitsweise.md`, `CLAUDE.md`), B-315, B-080 (Server). Geplante Sprints: keine Session mit Datei nur in DEV-Dateien (M10 nur Entwürfe).
- Prüfungen: `check_run dev:test` grün, `check_run task:check` grün, `golangci-lint run` in `tools/k3c-dev` 0 issues, `tsc --noEmit` im Frontend sauber.
- Abweichung: Die Umstellung der Tickets und Sessions auf `DEV` lief von Hand (Feld, Index-Zeile, Sprint-Domäne, Fahrplan-Zeile), weil k3c-dev noch mit altem Code lief und `Domäne: DEV` ablehnte; danach `task test -- planning` grün. k3c-dev nicht neu gestartet. Das abschließende `plan_set DV1.2 {Status: fertig}` setzte die Sprint-Domäne mit dem alten Code auf `INF, SRV` zurück (DEV unbekannt); von Hand wieder auf `INF, SRV, DEV` gestellt. Bis zum Neustart von k3c-dev tut das jeder `plan_set`/`plan_create` an einer DV1-Session erneut. Frontend: Sprint-Karte ohne Prio-Badge, Mock mit DEV-Ticket B-363 (Chip `DEV`), `projects.test.ts` erwartet es unter „Ohne Projekt“.
- Neue Tickets: keine.
