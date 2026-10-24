# BD9.3 · Review BD9

- **Status:** in Arbeit
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** bd9/3-review
- **Abhängig von:** BD9.1, BD9.2
- **Tickets:** B-604, B-581
- **Kriterien:** alle

## Ziel

Der Sprint ist geprüft, Abnahme-Abschnitt ausgefüllt, PR auf `develop`.

## Kontext

Sessions BD9.1 und BD9.2; Review-Ablauf `docs/arbeitsweise.md` › Review-Session.

## Erlaubte Dateien

- Tests in `engine/**`, `src/**`
- kleine Fixes (< 30 Zeilen) in den Dateien der Sessions
- `docs/sprints/**/BD9-*/**`, `docs/backlog/B-604*`, `B-581*`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Kriterien AC-01 bis AC-03 prüfen.
2. `task check`, `task check:go`, `task check:race` nacheinander.
3. Lobby und Pausenmenü im Browser-Pane mit zwei Geräten (zwei Tabs) durchgehen.
4. Abnahme-Abschnitt, PR.

## Fertig, wenn

- [ ] Prüfläufe grün, Abnahme ausgefüllt, PR offen.

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
