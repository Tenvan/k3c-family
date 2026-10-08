# B-368 · Die Planung liegt in der ErpApi-Ablage und k3c-dev bedient sie mit den Planungs-Tools der Workbench-Spec

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die gemeinsame Spec `docs/standards/workbench-seiten.md` (v1.2) schreibt für alle Workbenches dieselben Planungsbegriffe
und Tools vor: Ablage `TODOs/2-projekte-aktiv/PJ-NNN-<name>/`, Sprints als Datei `NN-<name>.md`, Sessions als Zeile mit
ID `PJ-054#22`, Tickets `TODOs/1-backlog/<PRÄFIX>-NNN-<thema>.md`, Tools `plan_project`, `plan_sprint`, `plan_session`,
`plan_ticket` mit `action`. k3c plant heute in `docs/projekte`, `docs/sprints` (Session je Datei) und `docs/backlog`
(`B-NNN`) mit `plan_create`, `plan_set`, `plan_section`, `plan_delete`. Beschluss 🧑 2026-10-08 (Chat): k3c zieht ganz auf
die ErpApi-Ablage um, aber erst nach den übrigen Seiten der Spec und als eigenes Projekt mit Migrationsskript und Probelauf.
Die übrigen Seiten (Dienste & Logs, MCP, Tasks) und Tool-Regeln setzt der Branch `feat/workbench-seiten` um.

## Ziel

Planung, Werkzeuge und Regeln von k3c folgen der Spec wortgleich wie ErpApi, ohne dass Inhalte, IDs oder Nachweise verloren gehen.

## Beteiligte und Zielgruppen

🧑 entscheidet und gibt frei; Agenten planen und arbeiten danach über die neuen Tools.

## Anforderungen

- Migrationsskript mit Trockenlauf und Diff: 13 Projekte, 133 Sprints (570 Dateien), 324 Tickets samt Archiv.
- Die ausführlichen Session-Dateien (Kontext, Erlaubte Dateien, Schritte, Fertig wenn, Ergebnis) bleiben vollständig erhalten.
- Planungsparser, `plan_*`-Tools, Planungsseite (§ 1 der Spec), `tests/planning.test.ts`, `docs/arbeitsweise.md`, Glossar
  und `CLAUDE.md` folgen im selben Projekt; bis zum Umstieg bleibt die heutige Planung voll nutzbar.

## Nicht-Ziele

Die Seiten Dienste & Logs, MCP und Tasks (Branch `feat/workbench-seiten`).

## Regeln und Einschränkungen

Spec `docs/standards/workbench-seiten.md` ist führend; eine Änderung dort zuerst (Version hochzählen). Komplexitäts-Budget.

## Beispiele

`plan_session {action: done, project: PJ-012, id: 3}` verschiebt die Session nach Done und zählt den Index nach.

## Ausnahme- und Fehlerfälle

Inhalt einer Session passt nicht in eine Tabellenzeile → Detailabschnitt im Sprint, nie Verlust.

## Akzeptanzkriterien

- **AC-01** Ein Trockenlauf des Migrationsskripts zeigt für jede alte Datei ihr Ziel; kein Abschnitt fehlt (Test).
- **AC-02** Nach dem Umstieg liefern `plan_project`, `plan_sprint`, `plan_session` und `plan_ticket` dieselben Projekte,
  Sprints, Sessions und Tickets wie vorher `plan_list` (Test gegen den Bestand).
- **AC-03** Die Planungsseite zeigt Aufbau, Reihenfolge, Ebenen und Einklappen nach § 1 der Spec (🧑 am Bildschirm).

## Offene Fragen

- Wie werden die Session-Details im Sprint abgelegt (Abschnitt je Session oder eigene Datei je Session neben dem Sprint)? 🧑
- Welche Ticket-Präfixe ersetzen `B-NNN` (je Domäne wie `SIM-001`, oder ein Präfix für alles)? 🧑

## Notizen

Vorlage ErpApi: `C:\WORKSPACE\ErpApi\TODOs\` (README, `0-vorlagen/projekt-*.template.md`). Skill `todo-planner` lokal erst
in 2.0.0/2.1.0; die Spec verweist auf orga-planning ≥ 3.0.0.
