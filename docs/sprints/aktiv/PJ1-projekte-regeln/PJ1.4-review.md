# PJ1.4 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** pj1/4-review
- **Abhängig von:** PJ1.3
- **Tickets:** B-355, B-356, B-338
- **Kriterien:** alle

## Ziel

PJ1 ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, der Sprint ist erledigt und sein PR gegen `develop` offen.

## Kontext

- Ablauf und Befund-Regeln: `docs/arbeitsweise.md` › Review-Session.
- Schwerpunkt dieses Sprints: Kein Kriterium umformuliert; Feld-Migration aus PJ1.2 ändert je Datei genau eine Zeile (`git diff --stat origin/develop...origin/sprint/pj1 -- docs/backlog docs/sprints`); die Übergangsregel aus PJ1.1 steht wortgleich im Sinn in Arbeitsweise und Test (PJ1.3, Regel 8).
- B-338/AC-02 (`plan_set` mit `verworfen`) gehört zu PJ2, nicht zu diesem Sprint.

## Erlaubte Dateien

- Schwere Befunde in den Dateien des Sprints (`docs/arbeitsweise.md`, `docs/glossar.md`, `docs/vorlagen/`, `docs/projekte/`, `tests/planning*.test.ts`)
- `docs/sprints/aktiv/PJ1-projekte-regeln/`, `docs/sprints/erledigt/`, `docs/sprints/README.md`, `docs/backlog/`

## Nicht-Ziele

Stil, Benennung, Vereinfachungen; PJ2 vorbereiten.

## Schritte

1. Branch `sprint/pj1` holen, `git merge origin/develop`, `Status: in Arbeit`, committen, pushen.
2. `task check`, Diff lesen, nur schwere Befunde.
3. Abnahme (höchstens fünf Zeilen), Tickets B-355, B-356 auf `erledigt` (archivieren); B-338 bleibt `eingeplant` für PJ2 (AC-02 offen).
4. Sprint-Ordner nach `docs/sprints/erledigt/`, `Status: erledigt`, Fahrplan, Versionsvorschlag (Patch: Doku und Planung), PR des Sprints öffnen.

## Fertig, wenn

- [ ] Alle Kriterien AC-01 bis AC-09 haben einen Nachweis in PJ1.1–PJ1.3.
- [ ] `task check` grün, PR gegen `develop` offen.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
