# MG8.6 · Review des Sprints MG8

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** objekte
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg8/6-review
- **Abhängig von:** MG8.1, MG8.2, MG8.3, MG8.4, MG8.5
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG8 geprüft: alle Assets in `objekte` und `icons` gültig, gerendert, mit Golden, lesbar unterscheidbar; Abnahme ausgefüllt.

## Kontext

Sprint-README, Ergebnisse MG8.1–MG8.5; Review mit frischem Blick (Opus) auf alle Sheets, wie MG4.5.

## Erlaubte Dateien

- `public/grafik/objekte/**`, `public/grafik/icons/**` (Korrekturen über `assets_set`)
- `docs/sprints/aktiv/MG8-papier-natur-objekte-icons/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Assets; Sichtprüfung durch 🧑 in PS1.

## Schritte

1. `assets_list`, `assets_review` gegen den Start-Commit.
2. Review aller Sheets (frischer Kontext), wichtige Befunde über `assets_set` beheben, kleine als D-Zeile.
3. `task gen:paper`, `tools/paper`-Tests, `task check`.
4. Abnahme, Commit `chore(pap): Review MG8`.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis.
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
