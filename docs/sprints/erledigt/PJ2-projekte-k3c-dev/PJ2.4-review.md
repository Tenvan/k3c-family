# PJ2.4 · Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** pj2/4-review
- **Abhängig von:** PJ2.3
- **Tickets:** B-357, B-358, B-338
- **Kriterien:** alle

## Ziel

PJ2 ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, der Sprint ist erledigt und sein PR gegen `develop` offen.

## Kontext

- Ablauf und Befund-Regeln: `docs/arbeitsweise.md` › Review-Session (leicht, nur schwere Befunde im Diff).
- Schwerpunkt: Schreibwege bleiben unter `docs/` (Pfade nur aus geprüften IDs, auch das Projekt-Kürzel); jede Ablehnung ändert nichts; Ränge lückenlos; das Tool schreibt nichts, was `tests/planning*.ts` verletzt; die Übergangsregel zur Sprint-Prio (PJ2.2) passt zu Arbeitsweise und Planungstest; kein Kriterium umformuliert.
- Danach ist B-338 vollständig (AC-01 aus PJ1, AC-02 aus PJ2.2).

## Erlaubte Dateien

- Schwere Befunde in `tools/k3c-dev/` (Dateien des Sprints)
- `docs/sprints/aktiv/PJ2-projekte-k3c-dev/`, `docs/sprints/erledigt/`, `docs/sprints/README.md`, `docs/backlog/`

## Nicht-Ziele

Stil, Benennung, Vereinfachungen; PJ3 vorbereiten.

## Schritte

1. Branch `sprint/pj2` holen, `git merge origin/develop`, `Status: in Arbeit`, committen, pushen.
2. `task check`, `task check:dev`, Diff `git diff origin/develop...origin/sprint/pj2` lesen, nur schwere Befunde.
3. Abnahme (höchstens fünf Zeilen), Tickets B-357, B-358, B-338 auf `erledigt` (archivieren).
4. Sprint-Ordner nach `docs/sprints/erledigt/`, `Status: erledigt`, Fahrplan, Versionsvorschlag (Minor: Wirkung im Werkzeug), PR des Sprints öffnen.

## Fertig, wenn

- [ ] Alle Kriterien AC-01 bis AC-09 haben einen Nachweis in PJ2.1–PJ2.3.
- [ ] `task check` und `task check:dev` grün, PR gegen `develop` offen.

## Prüfen

```bash
task check
task check:dev
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Alle Kriterien haben einen Nachweis: AC-01 bis AC-03 in PJ2.1, AC-04 und AC-05 in PJ2.2, AC-06 bis AC-09 in PJ2.3. AC-04 mit der von 🧑 beschlossenen Übergangsregel: Das Feld `Prio` wird weiter geschrieben, geordnet wird nach Rang. Den Wegfall der Felder übernimmt B-361.
- `task check` und `task check:dev` grün. Gelesen: `git diff origin/develop...sprint/pj2` (PJ2.2, PJ2.3; PJ2.1 ist schon mit #220 auf `develop`).
- Schwerer Befund behoben: `syncSprintDomain` griff bei einer Sprint-README ohne `# `-Überschrift auf Index −1 zu (Panic im MCP-Server). Jetzt gibt es eine Ablehnung ohne Änderung, mit Test in `TestSessionDomaeneUndVerworfen`.
- Schreibwege geprüft: Pfade nur aus `resolve` (geprüfte IDs) und Dateinamen der Session-Tabelle ohne `/` oder `\`; jede Ablehnung schreibt nichts (changeSet); Ränge lückenlos (PJ2.1); das Tool schreibt `Domäne` und `Prio` passend zu `tests/planning*.ts`.
- Hinweis: Das Review lief auf Anweisung von 🧑 in derselben Unterhaltung wie PJ2.2/PJ2.3.
