# PJ2.1 · Projekte in den plan-Tools

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** pj2/1-projekte-plan-tools
- **Abhängig von:** –
- **Tickets:** B-357
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

`plan_create`, `plan_get`, `plan_list`, `plan_section` und `plan_set` kennen Projekte: anlegen nach Vorlage, Rang lückenlos halten, Sprints und Tickets zuordnen; `docs/projekte/README.md` und die Sprint-Tabelle des Projekts ziehen sie selbst nach.

## Kontext

- Code: `tools/k3c-dev/internal/planning/` (eigenes Go-Modul). `store.go` (`resolve`, `ref`, `changeSet`: erst alles berechnen, dann schreiben), `create.go` (`Create`, `createTicket`/`createSprint`/`createSession`, `Get`, `Delete`), `edit.go` (`Set`, `follow*`, `Section`, `allowed`), `tables.go` (`findTable`, `placeRow`, `removeRow`, `syncIndex`, `syncRoadmap`), `list.go` (`List`, `Filter`), `planning.go` (`Load`, `Data`, `ParseSprint`, `ParseTicket`). MCP-Schicht: `tools/k3c-dev/internal/mcpsrv/tools_planning.go` (Beschreibungen und Parameter der Tools).
- Formate (PJ1, nicht ändern): Vorlage `docs/vorlagen/projekt.md` (Kopf `# XXX · Titel`, Felder `Status`, `Rang`, `Ziel-Tickets`; Abschnitte Ziel, Sprints mit Tabelle `| Sprint | Thema | Status |`, Nicht-Ziele, Notizen), Übersicht `docs/projekte/README.md` (Tabellen „Aktiv“ `| Rang | Projekt | Ziel | Datei |`, „Ruht“ `| Projekt | Grund | Datei |`, „Erledigt“ `| Projekt | Datei |`; heute leer mit Hinweissatz). Feld `Projekt` in Sprint- und Ticket-Vorlage (`–` oder Kürzel). Regeln: `docs/arbeitsweise.md` › Projekte und Rang; geprüft von `tests/planningProjects.test.ts` (Kürzel = drei Großbuchstaben = Dateianfang, Ränge lückenlos ab 1, `ruht`/`erledigt` mit `Rang: –`, Sprint ↔ Projekt-Tabelle beidseitig, Ticket-Projekt existiert, je Projekt höchstens ein aktiver Sprint).
- Beispiele und Fehlerfälle: B-357 › Beispiele, › Ausnahme- und Fehlerfälle (unbekanntes Kürzel, Rang außerhalb 1 … n oder auf ruhendem Projekt, `Sprints` unvollständig oder fremd, `plan_delete` auf Projekt mit Sprints oder Tickets → abgelehnt, nichts geändert).
- Budget: `edit.go` 231, `create.go` 204, `tables.go` 215 Zeilen. Projekt-Logik in eine neue Datei (Vorschlag `projects.go`, Tabellen-Pflege `projects_tables.go`), in den bestehenden Dateien nur Verzweigungen. Datei ≤ 400, Funktion ≤ 60 Zeilen.
- Tests: Muster `write_test.go` (`tempRepo` mit echten Vorlagen aus `docs/vorlagen/`, Prüfsummen „nichts geändert“ bei Ablehnung). `tempRepo` um `projekt.md` und eine leere `projekte/README.md` erweitern.
- Erkennen eines Projekts in `resolve`: ID aus genau drei Großbuchstaben (`^[A-Z]{3}$`); Sprint-IDs haben immer eine Zahl, es gibt keine Verwechslung.

## Erlaubte Dateien

- `tools/k3c-dev/internal/planning/` (Code und Tests), `tools/k3c-dev/internal/mcpsrv/tools_planning.go` (Beschreibungen, Filter `projekt`)
- `tools/k3c-dev/internal/planning/codemap.md`, `tools/k3c-dev/internal/mcpsrv/codemap.md` (falls vorhanden)
- `docs/sprints/aktiv/PJ2-projekte-k3c-dev/` (Status, Ergebnis), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Domäne je Session, `verworfen`, Sprint-Prio (PJ2.2); Oberfläche (PJ2.3); Vorlagen, Regeltexte und `tests/planning*.ts` (INF, PJ1); echte Projekte anlegen (PJ3).

## Schritte

1. Branch `sprint/pj2` holen (die erste Session legt ihn von `origin/develop` an und setzt `Start-Commit`), `Status: in Arbeit`, committen, pushen.
2. Lesen: `Load` liest `docs/projekte/*.md` in `Data.Projects` (Kürzel, Titel, Status, Rang, Sprints aus der Tabelle); fehlt der Ordner, leere Liste. `plan_get XXX` liefert die Datei, `plan_section XXX` ersetzt Abschnitte.
3. `plan_create {kind: projekt}`: Kopie der Vorlage, `id` = Kürzel, Beispielzeile der Sprint-Tabelle raus, Eintrag in `docs/projekte/README.md` (aktiv: Rang = letzter + 1; Hinweissatz „Noch keine Projekte …“ entfällt beim ersten Projekt).
4. `plan_set` auf Projekt: `Status` (zu `ruht`/`erledigt`: Rang `–`, Zeile in die passende Tabelle, die übrigen rücken lückenlos nach; zurück zu `aktiv`: Rang = letzter + 1), `Rang` (1 … n, verschiebt die anderen lückenlos, schreibt alle betroffenen Projektdateien und die Übersicht), `Sprints` (kommagetrennte neue Reihenfolge; genau die vorhandenen Sprints).
5. `plan_set` mit `Projekt` an Sprint oder Ticket: Kürzel muss existieren; Sprint wird aus der Tabelle des alten Projekts entfernt und ans Ende der Tabelle des neuen angehängt (Spalten Sprint, Thema aus der Überschrift, Status); `–` trägt nur aus. Ändert sich der Status eines Sprints, zieht die Zeile in der Projekt-Tabelle mit.
6. `plan_list {kind: projekt}`: je Projekt eine Zeile `Rang Kürzel Status Sprints fertig/gesamt · Titel`, nach Rang, dann ruhend, erledigt; Filter `projekt` für Sprints und Tickets.
7. `plan_delete` auf einem Projekt ohne Sprints und Tickets löscht Datei und Übersichtszeile, sonst Ablehnung.
8. Tests je Schritt (Go, `tempRepo`), darunter: Ränge nach Statuswechsel und Rang-Verschiebung lückenlos; jede Ablehnung aus B-357 lässt alle Dateien unverändert. Ein Test legt in einer Kopie von `docs/` (nur die Planungsordner) ein Projekt an, ordnet einen Sprint zu und prüft danach Sprint-Tabelle und Übersicht gegen die Regeln aus `tests/planningProjects.test.ts` (dieselben Regeln in Go nachgebildet, nicht über Node aufgerufen).
9. Probelauf gegen den echten Planungstest (er liest das Repo, keine Kopie): bei sauberem Arbeitsstand mit einem `go run`-Schnipsel aus dem Scratchpad (nicht einchecken) über `planning.Create`/`planning.Set` im eigenen Checkout ein Projekt `TST` anlegen und einen geplanten Sprint zuordnen, `check_run task:test` mit Muster `planning`, danach `git checkout -- docs/` und `git clean -fd docs/projekte/`. Ergebnis nennen.
10. Prüfen, Ergebnis je Kriterium, `Status: fertig`, Commit `feat(srv): Projekte in den plan-Tools (PJ2.1)`.

## Fertig, wenn

- [ ] AC-01: Go-Tests für `plan_create`, `plan_get`, `plan_list` und `plan_section` mit Projekten grün.
- [ ] AC-02: Tests für Rang-Verschiebung und Statuswechsel: Ränge der aktiven Projekte lückenlos ab 1, ruhende und erledigte mit `–`.
- [ ] AC-03: Test für `Projekt` und `Sprints`: Sprint-Tabelle und `docs/projekte/README.md` gepflegt; Probelauf mit `task test -- planning` grün (Schritt 9).
- [ ] Jede Ablehnung aus B-357 › Ausnahme- und Fehlerfälle getestet, nichts geändert.
- [ ] `check_run dev:test` und `check_run task:check` grün.

## Prüfen

```bash
task check:dev
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
