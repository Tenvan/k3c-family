# US1.3 · Tasks und CI kennen die Papierschnitt-Grafik

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** us1/3-tasks-ci
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-03

## Ziel

Tasks und CI bauen und prüfen das Spiel mit Papierschnitt; `task check:all` grün.

## Kontext

`Taskfile.yml`, `.github/workflows/ci.yml`; lastabhängige Tests (B-704) einzeln wiederholen.

## Erlaubte Dateien

- `Taskfile.yml`, `*/Taskfile.yml`
- `.github/workflows/ci.yml`

## Nicht-Ziele

`gen:atlas`/`gen:gebaeude` entfernen (CU1).

## Schritte

1. Abhängigkeiten der Build-Tasks prüfen (`gen:paper` vor `build` nicht nötig, PNGs committet).
2. `task check:all`, Flakes einzeln wiederholen.
3. Commit.

## Fertig, wenn

- [ ] AC-03: `task check:all` grün (Flakes nach B-704 einzeln wiederholt und genannt).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
