# MG3.3 · Review des Sprints MG3

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** truppen
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg3/3-review
- **Abhängig von:** MG3.1, MG3.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Sprint MG3 ist geprüft: alle sechs Truppen gültig, gerendert, mit Golden; Abnahme der README ausgefüllt.

## Kontext

Sprint-README und Ergebnisse MG3.1/MG3.2; Stil-Regeln aus `assets_guide {rig: figur}`; Proportion und Lesbarkeit der Silhouetten im Preview (Truppen müssen sich auf einen Blick unterscheiden).

## Erlaubte Dateien

- `public/grafik/truppen/**` (nur Korrekturen aus dem Review, über `assets_set`)
- `docs/sprints/aktiv/MG3-papier-truppen/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Assets, keine Sichtprüfung durch 🧑 (PS1).

## Schritte

1. `assets_list {gruppe: truppen}`, `assets_review` gegen den Start-Commit.
2. Je Asset jede Animation mit `assets_preview` ansehen (frischer Blick), Befunde beheben oder als D-Zeile.
3. `task test -- paper`, `task check`.
4. Abnahme (höchstens fünf Zeilen), Commit `chore(pap): Review MG3`.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis im Ergebnis.
- [ ] `task check` grün.

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
