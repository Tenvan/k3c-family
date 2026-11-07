# US1.4 · Doku und Codemaps beschreiben Papierschnitt

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** haiku
- **Umgebung:** offline
- **Branch:** us1/4-doku-codemaps
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-04

## Ziel

CLAUDE.md, `docs/arbeitsweise.md` und Codemaps beschreiben Papierschnitt, Workbench-Bereiche und den Wegfall des Studios.

## Kontext

Entscheidung 005, WB1.

## Erlaubte Dateien

- `CLAUDE.md`, `docs/arbeitsweise.md`
- `codemap.md`, `src/scenes/codemap.md`, `src/scenes/render/codemap.md`

## Nicht-Ziele

Code.

## Schritte

1. Stellen mit Atlas, Pixel-Gebäuden und Studio suchen.
2. Nachziehen.
3. Commit.

## Fertig, wenn

- [ ] AC-04: `grep` findet keine Beschreibung des Atlas als Lade-Weg und kein Asset-Studio mehr in den genannten Dateien.

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
