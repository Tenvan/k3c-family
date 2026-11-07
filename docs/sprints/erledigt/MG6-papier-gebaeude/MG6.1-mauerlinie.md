# MG6.1 · Mauerlinie: Mauer, Tor, Turm, Hintergrundmauer

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gebaeude
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg6/1-mauerlinie
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Mauer, Tor, Turm und Hintergrundmauer liegen als Paper-Specs (Rig `gebaeude`) unter `public/grafik/gebaeude-paper/` mit den Stufen 1–5 als Delta (Holz → Stein → Kupfer → Eisen → Kristall). Die Session legt dabei die Materialkette und das Maß fest, die MG6.2–MG6.5 übernehmen.

## Kontext

- Vorbild der alten Grafik: `public/grafik/gebaeude/<name>.json` (`vorlage`, `stufen[].aenderung`), Maße `docs/standards/gebaeude-grafik.md` › 1. Mauer 1 × 5, Tor 2 × 4, Turm 3 × 8, Hintergrundmauer 1 × 4 Kacheln (kachelbar).
- Muster der Spec: `tools/paper/fixtures/specs/mauer.paper.json` (Quader, Falz als Licht, Auflage als Schatten, Stufe als `zusaetze`).
- Stufen sind kumulativ: Stufe n = Basis + `stufen[0..n-2]`, je Stufe erst Zusätze, dann `tausch` (`tools/paper/render/stufen.ts`).
- Materialkette (README › D-MG6-02): Baumaterial `holz` → `stein` → `fell-fuchs` (Kupfer) → `stoff` (Eisen) → `koenigsblau` (Kristall) per `tausch`; bleibende Teile nie in einem Material der Kette.
- `assets_preview` kennt keine Stufe (D-MG6-03): Stufen ≥ 2 aus der gerenderten Ausgabe `<asset>/stand.s<n>.png` ansehen.

## Erlaubte Dateien

- `public/grafik/gebaeude-paper/{mauer,tor,turm,hintergrundmauer}.paper.json`, ihre Ausgabe-Ordner, `public/grafik/gebaeude-paper/index.paper.json`, `public/grafik/gebaeude-paper/golden.json`
- `docs/sprints/aktiv/MG6-papier-gebaeude/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG6-Zeilen)

## Nicht-Ziele

Die alten Gebäude-JSONs (`gen:gebaeude`) bleiben bis CU1; kein Code in Renderer oder Workbench; kein `assets_style_set`.

## Schritte

1. `assets_guide {rig: gebaeude}` lesen.
2. Je Gebäude `assets_create` (Gruppe `gebaeude-paper`), dann `assets_set` mit Basis-Teilen und `stufen` (4 Deltas).
3. Jedes Gebäude mit `assets_preview` (Stufe 1) und jede weitere Stufe über die gerenderte Ausgabe ansehen; Befunde notieren und per `assets_set` beheben.
4. `task gen:paper` (Golden, D-MG1-02), `assets_list`, `assets_review` gegen `HEAD`.
5. `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: `assets_list --gruppe gebaeude-paper` meldet alle vier gültig.
- [ ] AC-02: `golden.json` der Gruppe passt zur Ausgabe (`task test -- paper` grün).
- [ ] AC-03: alle vier in allen Stufen angesehen, Befunde im Ergebnis; `assets_review` ohne roten Eintrag.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-01 geprüft:** `assets_list` meldet mauer, tor, turm, hintergrundmauer gültig (0 ungültig).
- **AC-02 geprüft:** `task gen:paper --force` schreibt `golden.json` und `index.paper.json` der Gruppe `gebaeude-paper` (je 5 PNGs `stand.png`, `stand.s2`–`s5`); `tools/paper/golden/public.test.ts` grün nach D-MG6-06.
- **AC-03 geprüft:** Stufe 1 aller vier mit `assets_preview`, Stufen 2–5 über die Ausgabe (Kontaktbogen, D-MG6-03); `assets_review` gegen HEAD: 4 × `neu`, kein roter Eintrag.
- **Befunde, behoben:** Querbalken des Zauns ragten ab Stufe 2 seitlich aus der Wand (schmaler); goldenes Runenband am Turm als Falz unsichtbar (Falz hat 25 % Deckkraft) → Fläche; Lagerfugen am Turm fehlten neben den Scharten.
- **Abweichungen:** Angelegt zuerst in `gebaeude/` (Auftrag der Haupt-Session); dort scheitern `tools/buildings` (4 Go-Tests, `gen:gebaeude`) an `golden.json`/`*.paper.json` → Specs und Ausgabe unverändert nach `gebaeude-paper/` verschoben (D-MG6-01). `task gen:paper` schreibt die anderen Gruppen mit neuen Zeilenenden; zurückgenommen. `tools/paper/golden/public.test.ts` außerhalb der Gruppe geändert (D-MG6-06).
- **Keine neuen Tickets;** D-MG6-01 bis D-MG6-07 in B-701.
