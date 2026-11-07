# MG8.1 · Bäume, Wurzeln, Büsche und Setzling als requisite

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** objekte
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg8/1-baeume
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Tanne (`tanne`), Pappel (`pappel`), Birke (`birke`), Höhlenwurzel (`wurzel`), Busch (`busch`), Höhlenbusch (`hoehlenbusch`) und Setzling (`setzling`) liegen als `requisite` unter `public/grafik/objekte/`, nur über die Asset-MCP-Tools angelegt und mit `assets_preview` geprüft.

## Kontext

`docs/assets/zuordnung-welt.md` › Ressourcen; alte Grafiken `public/grafik/gebaeude/` (`tanne-*`, `wurzel-*`, `busch-32.png`, `setzling-32.png`). Bäume mit `idle` (Krone wiegt), Rest statisch. Nach `task gen:paper` fremde Gruppen zurücksetzen (D-MG3-06).

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `docs/sprints/aktiv/MG8-papier-natur-objekte-icons/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG8-Zeilen)

## Nicht-Ziele

Kein `assets_style_set`, keine weiteren Formvarianten (`tanne-2` …), keine Änderung an Renderer, Workbench oder anderen Gruppen.

## Schritte

1. `assets_guide {rig: requisite}`.
2. Je Asset `assets_create {gruppe: objekte}` und `assets_set`.
3. Jede Animation mit `assets_preview` prüfen.
4. `task gen:paper`, `assets_review`, `bunx vitest run tools/paper`.
5. Ergebnis, Commit `feat(pap): Natur Bäume und Büsche (MG8.1)`.

## Fertig, wenn

- [ ] AC-01: `assets_list {gruppe: objekte}` meldet die Assets gültig.
- [ ] AC-02: `golden.json` eingetragen, `tools/paper`-Tests grün.
- [ ] AC-03: jede Animation mit `assets_preview` geprüft; `assets_review` ohne roten Eintrag.

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
