# WZ2.7 · Planungsseite: gemerkter Splitter und getrennte Filter

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** sprint/wz2
- **Abhängig von:** WZ2.3
- **Tickets:** B-400, B-401, B-402
- **Kriterien:** AC-08, AC-09, AC-10

## Ziel

Die Planungsseite hat einen gemerkten Splitter zwischen Backlog und Details, getrennte Filter für Backlog und Projekte und zeigt in jeder Session-Zeile den Titel.

## Kontext

- Spec: `docs/backlog/B-400-planung-splitter-merken.md` (AC-01, AC-02), `docs/backlog/B-401-planung-filter-getrennt.md` (AC-01 bis AC-03) und `docs/backlog/B-402-planung-session-titel.md` (AC-01, AC-02); Entscheidungen 🧑 2026-10-11 stehen in der Sprint-README unter „Offene Fragen“.
- Vorgaben: Splitter nur in der Sprint-/Backlog-Ansicht mit **einem** gemeinsamen, gemerkten Anteil (20–80 %). Backlog-Filter: Prio, Umgebung, Projekt, Suche; er wirkt auf Backlog- **und Sprint-Liste**. Projekt-Filter: Status, Domäne; er wirkt nur auf die Projekte. Titel: nur Session-Titel.
- `SprintsBacklog.tsx` in `tools/k3c-dev/frontend/src/planning/` hält einen gemeinsamen Filter (`planning.filter`, `FilterBar`, `filterSprints`, `filterTickets`, `parseFilter` in `planning.ts`) und gibt die gefilterten Sprints und Tickets an `BacklogList` und `ProjectsView` weiter.
- Das Splitter-Bauteil `Splitter` (Ziehen, Doppelklick = Standard, 20–80 %) liegt in `tools/k3c-dev/frontend/src/tasks/TaskBars.tsx`.
- `ParseSprint` in `tools/k3c-dev/internal/planning/planning.go` füllt `Titel` nur für Stichpunkt-Entwürfe; für Sessions mit Datei kommt der Titel aus der Überschrift der Session-Datei (`# WZ2.1 · Titel`).
- Baut auf WZ2.3 auf (dieselben Dateien in `planning/`).

## Erlaubte Dateien

- `tools/k3c-dev/frontend/src/planning/SprintsBacklog.tsx`, `SprintCard.tsx`, `Backlog.tsx`, `ProjectsView.tsx`, `planning.ts`, `projects.ts` und zugehörige `*.test.ts(x)`
- `tools/k3c-dev/frontend/src/tasks/TaskBars.tsx` nur, um `Splitter` an eine gemeinsame Stelle zu heben (ohne Verhaltensänderung)
- `tools/k3c-dev/frontend/src/styles/planning.css`
- `tools/k3c-dev/internal/planning/planning.go`, `list.go` und zugehörige `*_test.go` (Session-Titel aus der Datei, B-402)

## Nicht-Ziele

Neue Filter-Felder; Änderungen an der Tasks-Seite außer dem Verschieben von `Splitter`; Optik der Karten.

## Schritte

1. Test zuerst (rot): Splitter-Anteil wird gespeichert, wiederhergestellt und auf 20–80 % begrenzt; in der Projekt-Ansicht gibt es keinen Splitter (B-400/AC-01, AC-02).
2. Splitter zwischen Backlog und Details der Sprint-/Backlog-Ansicht einbauen, `Splitter` wiederverwenden.
3. Test zuerst (rot): Backlog-Filter (Prio, Umgebung, Projekt, Suche) ändert Backlog- und Sprint-Liste, nicht die Projekte; Projekt-Filter (Status, Domäne) ändert nur die Projekte (B-401/AC-01, AC-02).
4. Filter in zwei Zustände trennen, je Bereich gemerkt (B-401/AC-03); Trefferzahl je Bereich.
5. Test zuerst (rot): Laden eines Sprints liefert für Sessions mit Datei den Titel aus der Überschrift (B-402/AC-01); `SessionRow` zeigt Titel, sonst `Typ · Agent` (B-402/AC-02).
6. Titel aus der Session-Datei lesen und in `Session.Titel` liefern; Zeile zeigt Titel mit Tooltip.
7. `task check:dev` ausführen, Ergebnis eintragen. Abnahme am Bildschirm gehört 🧑 (WZ2.8).

## Fertig, wenn

- [ ] AC-08: Tests für Splitter (Ziehen, Zurücksetzen, gemerkter Anteil, Grenzen) grün (`B-400/AC-01`, `AC-02`).
- [ ] AC-09: Tests für getrennte Filter und deren Merken grün (`B-401/AC-01`–`AC-03`).
- [ ] AC-10: Go-Test (Titel aus Datei) und Komponententest (Zeile zeigt Titel) grün (`B-402/AC-01`, `AC-02`).
- [ ] `task check:dev` grün; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Prüfen

```bash
task check:dev
```

Bei laufendem k3c-dev stattdessen `check_run` mit `task:check:dev`. Abnahme am Bildschirm nur durch 🧑.

## Ergebnis

- AC-08 umgesetzt, geprüft: `Splitter.test.tsx` und `filters.test.ts › Splitter-Anteil` (B-400/AC-01, AC-02) grün: waagerechter Balken zwischen Backlog und Details der Sprint-/Backlog-Ansicht, ein gemeinsamer Anteil (20–80 %, `planning.split`, Vorgabe 48 %), Doppelklick setzt zurück; die Projekt-Ansicht hat keinen Splitter. `Splitter` aus `tasks/TaskBars.tsx` wiederverwendet (neue Eigenschaft `horizontal`).
- AC-09 umgesetzt, geprüft: `filters.test.ts` (B-401/AC-01–AC-03) grün: Backlog-Filter (Suche, Schnellfilter, Prio, Umgebung, Projekt, Erledigte) wirkt auf Backlog- und Sprint-Liste, der Projekt-Filter (Status, Domäne) nur auf die Projekte; beide werden getrennt als `planning.filter` gemerkt. Die Domäne gehört jetzt zum Projekt-Filter (Domäne eines Sprints oder Tickets des Projekts); die alte Domänen-Auswahl im gemeinsamen Filter entfällt.
- AC-10 umgesetzt, geprüft: `TestSessionTitelAusDatei` (B-402/AC-01, vor der Änderung rot) und `SessionRow.test.tsx` (B-402/AC-02) grün; die Session-Zeile zeigt den Titel aus der Überschrift der Session-Datei, Typ und Agent stehen im Tooltip.
- `task check:dev` grün (Frontend, go test, golangci-lint 0 Issues).
- Nicht geprüft: Aussehen, Ziehen des Splitters und die neuen Filterleisten am Bildschirm (Abnahme gehört 🧑, WZ2.8). Die Komponententests für Splitter und Session-Zeile entstanden nach dem Code, nicht davor. Abweichungen vom Plan: die Zeile `Tickets: …` der Sprint-Karte zeigt keine Ticket-Titel (B-402: nur Session-Titel, 🧑). Neue Tickets: keine.
