# MG8.2 · Felsen, Erzbrocken und Adern als requisite

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** objekte
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg8/2-fels-erz
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Felsen (`felsen`, `felsen-b`, `felsen-c`), Erzbrocken (`erzbrocken`) und Adern (`ader-stein`, `ader-kupfer`, `ader-eisen`, `ader-kristall`) liegen als statische `requisite` unter `public/grafik/objekte/`.

## Kontext

`docs/assets/zuordnung-welt.md` › Materialien und Adern, Ressourcen; alte Grafiken `public/grafik/gebaeude/` (`felsen*`, `erzbrocken-32.png`, `ader-<n>-32.png`). Kupfer und Kristall ohne eigenes Material (siehe Sprint › Ausnahmefälle).

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `docs/sprints/aktiv/MG8-papier-natur-objekte-icons/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG8-Zeilen)

## Nicht-Ziele

Kein `assets_style_set`, keine Biom-Looks je Fels (ein Look), keine anderen Gruppen.

## Schritte

1. Je Asset `assets_create {gruppe: objekte}` und `assets_set`.
2. Jedes Asset mit `assets_preview` prüfen.
3. `task gen:paper`, `assets_review`, `bunx vitest run tools/paper`.
4. Ergebnis, Commit `feat(pap): Felsen, Erz und Adern (MG8.2)`.

## Fertig, wenn

- [ ] AC-01: Assets gültig.
- [ ] AC-02: Golden eingetragen, Tests grün.
- [ ] AC-03: alle mit `assets_preview` geprüft; Review ohne roten Eintrag.

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
