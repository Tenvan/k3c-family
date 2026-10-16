# WZ5.1 · Planungsseite im erp-qs-Look umsetzen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz5/1-planung-erpqs-look
- **Abhängig von:** –
- **Tickets:** B-439
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Die Planungsseite von k3c-dev sieht aus und bedient sich wie die erp-qs-Workbench (B-439), mit k3c-Daten.

## Kontext

Vorlage (nur lesen): `C:\WORKSPACE\erp-qs-automation\tools\dev-workbench\frontend\src` (views/PlanningView, components/planning, components/backlog, lib/filterGroups, lib/ticketDoc). Ziel: `tools/k3c-dev/frontend/src/planning/`, `styles/planning.css`, `api/mock*.ts`. Volltext für den Lesemodus aus `planning.Get` (wie `plan_get`).

## Erlaubte Dateien

- `tools/k3c-dev/frontend/src/planning/…`, `tools/k3c-dev/frontend/src/styles/planning.css`
- `tools/k3c-dev/frontend/src/api/types.ts`, `api/wails.ts`, `api/mockPlanning.ts`, `api/mockProjects.ts`
- `tools/k3c-dev/app_planning.go` (+ Test)

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Binding `PlanningGet(id)` und Mock-Volltexte.
2. Filterleiste mit Gruppen, Chips und Trefferzahl; Filterlogik in reinen Funktionen mit Tests.
3. Spalten mit Splitter, Detail nur bei Auswahl.
4. Pills, Projektkarte, Sprint-Panel, Session-Zeilen, Backlog je Domäne.
5. Lesemodus (Dialog) für Ticket, Sprint, Session.
6. Mock hell/dunkel prüfen, `task check` und `dev:test`.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
