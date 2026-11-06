# MG7.2 · Hintergründe Mine, Eisenstollen und Kristall

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hintergruende
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg7/2-mine-eisen-kristall
- **Abhängig von:** MG7.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Hintergründe Mine (Schachtwand mit Gerüst), Eisenstollen (Schmiedehalle mit Glut) und Kristall (Kristallhalle) als Rig `hintergrund`, je fern, mitte, nah in v1 und v2, gerendert, Golden eingetragen.

## Kontext

Sprint-README und Ergebnis MG7.1 (Generator, Rahmen). Mine: `leder`, `stein`, `holz`, Laternen `gold`; Eisenstollen: `stein`, Glut `koenigsrot` und `gold`; Kristall: `koenigsblau`, `purpur`, Glanz `edelstein`.

## Erlaubte Dateien

- `public/grafik/hintergruende/**`
- `docs/sprints/*/MG7-papier-hintergruende/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine Änderung an Wald und Höhle außer Befund-Fixes.

## Schritte

1. Generator aus MG7.1 erweitern, Probe-Render.
2. `assets_create` und `assets_set` je Asset.
3. `assets_preview` je Asset, `task gen:paper`, fremde Gruppen zurücksetzen, `assets_review`.

## Fertig, wenn

- [ ] 18 Assets gültig und gerendert, jedes per Preview geprüft.
- [ ] Golden und Index der Gruppe aktuell.
- [ ] Ergebnis mit Nachweis je Kriterium.

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
