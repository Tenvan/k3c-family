# DV1.1 · Domäne DEV in Arbeitsweise, Glossar und Planungstest, Sprints ohne Prio und Einschiebbar

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** dv1/1-domaene-dev-regeln
- **Abhängig von:** –
- **Tickets:** B-365, B-361
- **Kriterien:** AC-01, AC-02

## Ziel

Regeln, Glossar, Vorlagen und Planungstest kennen die Domäne `DEV`, und Sprints haben weder `Prio` noch `Einschiebbar`; `task test -- planning` ist grün.

## Kontext

- Domänen-Tabelle: `docs/arbeitsweise.md` › Domänen. Heute nennt SRV `cmd/` und `tools/k3c-dev/`, PLAT `src/tools/`. Beschluss 🧑 2026-10-08: `DEV` = `tools/k3c-dev/`, `cmd/k3c-load/`, `cmd/k3c-tui/`, `src/tools/`; SRV behält `cmd/k3c-server/`.
- Planungstest: `tests/planningDocs.ts` (`DOMAINS`, `ALLOWED.sprint` mit `Prio`/`Einschiebbar`) und `tests/planning.test.ts` (Zeilen 32–35 `sprintPrio`, Zeile 93 Prüfung „Prio = höchste Prio der Tickets“). `checkTemplate` verlangt, dass **jede** Sprint-README (geplant, aktiv, erledigt) genau die Felder der Vorlage hat; darum verlieren alle Sprint-READMEs beide Zeilen im selben Commit.
- Vorlagen: `docs/vorlagen/sprint.md` (Felder `Prio`, `Einschiebbar`), `docs/vorlagen/session.md` und `docs/vorlagen/ticket.md` (Auswahlliste `Domäne`).
- Fahrplan `docs/sprints/README.md` Zeile 21: Hinweis-Zeile `**Einschiebbar** …` entfällt. Achtung: Die alten plan-Tools suchen diese Zeile für Sprints mit `Einschiebbar: ja` (`tables.go` `roadmapMarker`); ohne das Feld in den Dateien greift sie nicht mehr.
- Glossar `docs/glossar.md`: Zeile `Domäne` (47) um DEV ergänzen, Zeile `Einschiebbar` (50) als entfallen markieren (mit B-361).
- `docs/arbeitsweise.md` Zeile 192 nennt „entfallen mit B-361“ – auf „entfallen (B-361)“ umstellen.
- Die plan-Tools kennen `DEV` erst nach DV1.2. In dieser Session darum kein Dokument auf `DEV` setzen (der Go-Validator würde es bei späteren `plan_set`-Aufrufen ablehnen).

## Erlaubte Dateien

- `docs/arbeitsweise.md`, `docs/glossar.md`
- `docs/vorlagen/sprint.md`, `docs/vorlagen/session.md`, `docs/vorlagen/ticket.md`
- `tests/planningDocs.ts`, `tests/planning.test.ts`
- `docs/sprints/**/README.md` (nur die Kopf-Zeilen `Prio` und `Einschiebbar` entfernen), `docs/sprints/README.md`
- Planungs-Dateien für Status (über plan-Tools)

## Nicht-Ziele

Kein Go-Code in `tools/k3c-dev/` (DV1.2). Keine Tickets oder Sessions auf `DEV` umstellen (DV1.2). Ticket-Prio bleibt.

## Schritte

1. Glossar: `DEV` im Eintrag `Domäne` ergänzen; `Einschiebbar` als entfallen (B-361) beschreiben.
2. `docs/arbeitsweise.md` › Domänen: Zeile `DEV` (Entwickler-Werkzeug) mit `tools/k3c-dev/`, `cmd/k3c-load/`, `cmd/k3c-tui/`, `src/tools/`; SRV auf `cmd/k3c-server/` ohne k3c-dev; PLAT ohne `src/tools/`. Zeile 192 anpassen.
3. Vorlagen: `DEV` in die Auswahlliste `Domäne` von `session.md`, `ticket.md`, `sprint.md`; in `sprint.md` die Felder `Prio` und `Einschiebbar` streichen.
4. `tests/planningDocs.ts`: `DEV` in `DOMAINS`, `Prio`/`Einschiebbar` aus `ALLOWED.sprint`. `tests/planning.test.ts`: `sprintPrio`/`ticketPrio` und die Prio-Prüfung entfernen (Ticket-Prio-Prüfung bleibt).
5. Aus allen `docs/sprints/{geplant,aktiv,erledigt}/*/README.md` die Zeilen `- **Prio:** …` und `- **Einschiebbar:** …` im Kopf entfernen (skriptbar, nur Kopf vor dem ersten `## `). Fahrplan: Zeile `**Einschiebbar** …` löschen.
6. `task test -- planning`, dann `task check`.

## Fertig, wenn

- [ ] AC-01: Domänen-Tabelle nennt `DEV` mit `tools/k3c-dev/` (und `cmd/k3c-load/`, `cmd/k3c-tui/`, `src/tools/`), SRV nennt k3c-dev nicht mehr, Glossar-Eintrag `Domäne` nennt `DEV`.
- [ ] AC-02: `docs/vorlagen/sprint.md` ohne `Prio`/`Einschiebbar`; `grep -rn "\*\*Prio:\*\*\|\*\*Einschiebbar:\*\*" docs/sprints` ist leer; `task test -- planning` grün.
- [ ] `task check` grün.

## Prüfen

```bash
task test -- planning
task check
```

Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
