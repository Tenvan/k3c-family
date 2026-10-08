# B-357 · Die plan-Tools von k3c-dev legen Projekte an, ordnen Sprints und Tickets zu und setzen den Rang

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** PJ2
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Planungs-Tools `plan_list`, `plan_get`, `plan_create`, `plan_set`, `plan_section`, `plan_delete` (`tools/k3c-dev/internal/planning/`) kennen Ticket, Sprint und Session. Sie leiten die Sprint-Prio aus den Tickets ab, pflegen Index, Session-Tabelle und Fahrplan und lehnen `verworfen` für Sessions ab (B-338). Projekte (B-355) und Domäne je Session (B-356) kennen sie nicht.

## Ziel

Projekte werden wie Tickets und Sprints nur über MCP angelegt und gepflegt; Rang, Zuordnung und Übersichten bleiben dabei ohne Handarbeit stimmig.

## Beteiligte und Zielgruppen

Agenten beim Planen und Abschließen, die Planungsseite der Workbench (B-358) als zweiter Nutzer derselben Logik, 🧑 beim Setzen des Rangs.

## Anforderungen

- `plan_create` mit `kind: projekt` legt `docs/projekte/<KÜRZEL>-slug.md` nach Vorlage an und trägt es in `docs/projekte/README.md` ein (neues aktives Projekt: Rang = letzter + 1).
- `plan_get`, `plan_list` (`kind: projekt`, Filter `projekt`) und `plan_section` kennen Projekte; `plan_list` zeigt je Projekt Rang, Status und Fortschritt seiner Sprints.
- `plan_set` auf einem Projekt: `Status` (bei `ruht`/`erledigt` Rang `–`, Ränge rücken lückenlos nach), `Rang` (verschiebt die anderen lückenlos), `Sprints` (neue Reihenfolge der Sprint-Tabelle).
- `plan_set` mit `Projekt` an Sprint oder Ticket trägt in die Sprint-Tabelle des Projekts ein bzw. aus und pflegt die Übersichten.
- Sessions: Feld `Domäne` (B-356), Status `verworfen` (B-338/AC-02); Sprint-Feld `Domäne` wird aus den Sessions abgeleitet.
- Die Ableitung der Sprint-Prio entfällt, das Feld `Einschiebbar` wird nicht mehr geschrieben.
- Ein Ergebnis, das `tests/planning.test.ts` verletzen würde, lehnt das Tool mit Grund ab.

## Nicht-Ziele

Oberfläche (B-358), Regeltexte (B-355, B-356), Umzug der Daten (B-359).

## Regeln und Einschränkungen

- Domäne SRV: `tools/k3c-dev/` (eigenes Go-Modul). Formate der Dateien kommen aus B-355 und B-356 (Vorlagen), das Tool erfindet keine eigenen.
- Hängt von B-355 und B-356 ab (Vorlagen und Regeln müssen auf `develop` liegen).
- Komplexitäts-Budget: Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

- `plan_create {kind: projekt, id: GRA, slug: grafik, title: Grafik}` → `docs/projekte/GRA-grafik.md`, Rang = n + 1.
- `plan_set GRA {"Rang": "1"}` → GRA Rang 1, bisherige 1 … n rücken um eins.
- `plan_set GR7 {"Projekt": "GRA"}` → GR7 steht in der Sprint-Tabelle von GRA.
- `plan_set S5.4 {"Status": "verworfen"}` → angenommen.

## Ausnahme- und Fehlerfälle

- Unbekanntes Projekt-Kürzel bei `Projekt` → abgelehnt, nichts geändert.
- `Rang` außerhalb 1 … n oder auf ruhendem Projekt → abgelehnt.
- `Sprints` nennt einen Sprint, der nicht zum Projekt gehört, oder lässt einen weg → abgelehnt.
- `plan_delete` auf einem Projekt mit Sprints oder Tickets → abgelehnt.

## Akzeptanzkriterien

- **AC-01** `plan_create`, `plan_get`, `plan_list` und `plan_section` arbeiten mit Projekten (Go-Tests in `tools/k3c-dev`, `dev:test` grün).
- **AC-02** `plan_set` hält die Ränge der aktiven Projekte lückenlos, auch bei Statuswechsel (Test).
- **AC-03** `plan_set` mit `Projekt` und `Sprints` pflegt Sprint-Tabelle und `docs/projekte/README.md`; danach ist `task test -- planning` grün (Test auf Kopie von `docs/`).
- **AC-04** Sessions mit Feld `Domäne` und Status `verworfen` werden gelesen und geschrieben, die Sprint-Domänen abgeleitet; die Sprint-Prio wird nicht mehr abgeleitet (Test).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
