# MG8.5 · Material-Icons als requisite

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** objekte
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg8/5-material-icons
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Material-Icons `icon-holz`, `icon-stein`, `icon-kupfer`, `icon-eisen`, `icon-kristall` liegen als statische `requisite` unter `public/grafik/icons/`.

## Kontext

`docs/assets/zuordnung-welt.md` › Materialien (`material:*`). Kupfer und Kristall ohne eigenes Material (Sprint › Ausnahmefälle); die fünf müssen sich in Form und Farbe unterscheiden.

## Erlaubte Dateien

- `public/grafik/icons/**`
- `docs/sprints/aktiv/MG8-papier-natur-objekte-icons/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG8-Zeilen)

## Nicht-Ziele

Keine Skill-Icons (fehlen in `data/`), kein `assets_style_set`.

## Schritte

1. Je Asset `assets_create {gruppe: icons}` und `assets_set`.
2. Jedes Asset mit `assets_preview` prüfen.
3. `task gen:paper`, `assets_review`, `bunx vitest run tools/paper`.
4. Ergebnis, Commit `feat(pap): Material-Icons (MG8.5)`.

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

- AC-01 geprüft: icon-holz, icon-stein, icon-kupfer, icon-eisen, icon-kristall gültig.
- AC-02 geprüft: `task gen:paper`, `icons/golden.json` ergänzt, fremde Gruppen zurückgesetzt; `tools/paper` 187/187 grün.
- AC-03 geprüft: alle mit `assets_preview` angesehen. Befund `icon-holz` ragte links über den Rand, verschoben und erneut geprüft. Eisen als Barren in `stein` Schatten mit Rostkante (D-MG8-02).
- Keine neuen Tickets.
