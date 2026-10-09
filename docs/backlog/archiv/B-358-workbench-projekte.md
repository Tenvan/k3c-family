# B-358 · Die Planungsseite der Workbench zeigt Projekte nach Rang mit ihren Sprints

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** PJ2
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-08, Chat, durch 🧑, Revision 1

## Ausgangslage

Die Planungsseite der Workbench (`tools/k3c-dev/frontend/src/planning/`: `PlanningPage.tsx`, `SprintsBacklog.tsx`, `SprintCard.tsx`, `Backlog.tsx`, `prompts.ts`) zeigt Sprints nach Status und Prio und den Ticket-Backlog. Themen sind dort nicht sichtbar, die Reihenfolge der Arbeit ergibt sich nur aus der Prio.

## Ziel

🧑 sieht auf einen Blick, welche Projekte in welcher Reihenfolge laufen, wie weit jedes ist und was als Nächstes kommt, und ändert die Rangfolge direkt auf der Seite.

## Beteiligte und Zielgruppen

🧑 beim Priorisieren und Freigeben; Agenten nutzen weiter die MCP-Tools (B-357).

## Anforderungen

- Hauptansicht: aktive Projekte nach Rang; je Projekt Kürzel, Titel, Sprints in Reihenfolge mit Status und Fortschritt (Sessions fertig/gesamt), der laufende Sprint hervorgehoben, die nächste offene autonome Session genannt.
- Rang per Knopf hoch/runter ändern; geschrieben wird über dieselbe Logik wie `plan_set` (B-357), danach lädt die Seite neu.
- Ruhende und erledigte Projekte eingeklappt am Ende; `ABN` (Abnahmen am Gerät) als eigener Bereich mit den offenen Mensch-Sessions.
- Je Projekt aufklappbar: Tickets ohne Sprint. Tickets ohne Projekt als eigener Bereich „Ohne Projekt“.
- Die Prompt-Vorschläge (`prompts.ts`, „nächste Session“) folgen der Rang-Reihenfolge statt der Prio.
- Bestehende Sprint-Karte und Ticket-Ansicht bleiben, nur gruppiert nach Projekt.

## Nicht-Ziele

Ziehen und Ablegen (Drag & Drop) von Sprints zwischen Projekten, Projekte auf der Seite anlegen oder löschen (geht über MCP), Zeitachsen.

## Regeln und Einschränkungen

- Domäne SRV: `tools/k3c-dev/` (Frontend und Wails-Bindung `app_planning.go`). Hängt von B-357 ab.
- Tests mit Mock-Daten (`api/mockPlanning.ts`), offline über `dev:test`.
- Komplexitäts-Budget: Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

- Seite öffnen → oben `1 LST Leistung & Stabilität`, Sprints `PM1 ▸ PF1 · NT1 · ST1`, „Nächste Session: PM1.1“.
- Bei GRA „hoch“ klicken → GRA Rang 1, LST Rang 2, Liste neu sortiert, `docs/projekte/README.md` geändert.

## Ausnahme- und Fehlerfälle

- „hoch“ beim Rang 1 bzw. „runter“ beim letzten Rang → Knopf deaktiviert.
- Schreiben scheitert (Tool lehnt ab) → Meldung mit Grund, Anzeige unverändert.
- Noch keine Projekte vorhanden (vor B-359) → bisherige Ansicht nach Sprints.

## Akzeptanzkriterien

- **AC-01** Die Planungsseite zeigt aktive Projekte nach Rang mit ihren Sprints, Fortschritt und nächster Session (Komponententest mit Mock-Daten, `dev:test` grün).
- **AC-02** Rang hoch/runter schreibt über die Planungs-Logik von B-357 und sortiert neu; an den Rändern deaktiviert (Test).
- **AC-03** Ruhende und erledigte Projekte sind eingeklappt, `ABN` und „Ohne Projekt“ sind eigene Bereiche (Test).
- **AC-04** Die Prompt-Vorschläge folgen dem Rang (`prompts.test.ts`).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
