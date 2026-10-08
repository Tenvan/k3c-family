# PJ2.4 · Review

- **Status:** in Arbeit
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

–
