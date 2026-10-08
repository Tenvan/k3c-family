# PJ2.3 · Planungsseite nach Projekten

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** pj2/3-planungsseite-projekte
- **Abhängig von:** PJ2.2
- **Tickets:** B-358
- **Kriterien:** AC-06, AC-07, AC-08, AC-09

## Ziel

Die Planungsseite der Workbench zeigt aktive Projekte nach Rang mit Sprints, Fortschritt und nächster Session, ändert den Rang per Knopf und ordnet die Prompt-Vorschläge nach Rang.

## Kontext

- Stand nach PJ2.2: `planning.Data` enthält `Projects` (Kürzel, Titel, Status, Rang, Sprints); Sessions tragen `domain`; `plan_set` auf Projekt mit `Rang` hält die Ränge lückenlos.
- Frontend: `tools/k3c-dev/frontend/src/planning/` (`PlanningPage.tsx` 57, `SprintsBacklog.tsx` 131, `SprintCard.tsx` 107, `Backlog.tsx` 120, `planning.ts` 114 mit `sortSprints`/`isNext`/`filterSprints`, `prompts.ts` 75 mit `promptSprint`/`promptSessions`, Tests `planning.test.ts`, `prompts.test.ts`). API: `frontend/src/api/types.ts`, `mockPlanning.ts`, `wails.ts`, `index.ts`. Wails-Bindung `tools/k3c-dev/app_planning.go` (`PlanningData`, `PlanningSet(id, field, value)` = derselbe Weg wie `plan_set`).
- Anzeige laut B-358 › Anforderungen und › Beispiele: Hauptansicht aktive Projekte nach Rang (Kürzel, Titel, Sprints in Reihenfolge mit Status und Fortschritt fertig/gesamt, laufender Sprint hervorgehoben, „Nächste Session: …“ = erste Session mit `offen`, `autonom` und erledigten Abhängigkeiten im ersten nicht erledigten Sprint); Knöpfe „hoch“/„runter“ (am Rand deaktiviert) rufen `PlanningSet(<Kürzel>, "Rang", n±1)`, danach neu laden, Ablehnung als Meldung mit Grund, Anzeige unverändert. Ruhende und erledigte eingeklappt am Ende; `ABN` als eigener Bereich mit den offenen Mensch-Sessions; je Projekt aufklappbar die Tickets ohne Sprint; Bereich „Ohne Projekt“ für Sprints und Tickets mit `Projekt: –`. Ohne Projekte (heute, vor PJ3): bisherige Ansicht nach Sprints.
- Sprint-Karte und Ticket-Ansicht bleiben, nur gruppiert. Neue Komponenten in eigene Dateien (Vorschlag `ProjectsView.tsx`, `ProjectCard.tsx`, Logik in `projects.ts` mit Test `projects.test.ts`); Datei ≤ 400, Funktion ≤ 60 Zeilen.
- Prompts: `promptSessions`/Gruppen und „nächste Session“ in Rang-Reihenfolge (Projekt-Rang, dann Tabellenplatz), Sprints ohne Projekt danach wie bisher.

## Erlaubte Dateien

- `tools/k3c-dev/frontend/src/planning/`, `tools/k3c-dev/frontend/src/api/` (Typen, Mock, Bindung)
- `tools/k3c-dev/app_planning.go` (nur falls eine Bindung fehlt), `tools/k3c-dev/frontend/src/planning/codemap.md`
- `docs/sprints/aktiv/PJ2-projekte-k3c-dev/` (Status, Ergebnis), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Drag & Drop, Projekte auf der Seite anlegen oder löschen, Zeitachsen (B-358 › Nicht-Ziele); Änderungen an `internal/planning/` außer einer fehlenden Bindung; Browser-Abnahme durch 🧑 (nur Komponententests mit Mock-Daten).

## Schritte

1. Branch `sprint/pj2` holen, `git merge origin/develop`, `Status: in Arbeit`, committen, pushen.
2. Typen und Mock: `PlanProject` in `types.ts`, `mockPlanning.ts` mit drei aktiven Projekten (eines mit laufendem Sprint), einem ruhenden, einem erledigten, `ABN` mit offener Mensch-Session und Sprints/Tickets ohne Projekt.
3. Reine Logik in `projects.ts` (Sortieren nach Rang, Fortschritt, nächste Session, Gruppen ABN und „Ohne Projekt“) mit Tests zuerst.
4. Komponenten und Rang-Knöpfe; Fehler von `PlanningSet` als Meldung.
5. `prompts.ts` nach Rang, `prompts.test.ts` ergänzen.
6. Prüfen, Ergebnis je Kriterium, `Status: fertig`, Commit `feat(srv): Planungsseite nach Projekten (PJ2.3)`.

## Fertig, wenn

- [ ] AC-06: Komponententest mit Mock-Daten: Projekte nach Rang mit Sprints, Fortschritt und nächster Session; ohne Projekte die bisherige Ansicht.
- [ ] AC-07: Test: „hoch“/„runter“ ruft `PlanningSet` mit dem neuen Rang und lädt neu, an den Rändern deaktiviert, Ablehnung zeigt den Grund.
- [ ] AC-08: Test: ruhende und erledigte eingeklappt, `ABN` und „Ohne Projekt“ als eigene Bereiche.
- [ ] AC-09: `prompts.test.ts`: Vorschläge in Rang-Reihenfolge.
- [ ] `check_run dev:test` und `check_run task:check` grün.

## Prüfen

```bash
task check:dev
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
