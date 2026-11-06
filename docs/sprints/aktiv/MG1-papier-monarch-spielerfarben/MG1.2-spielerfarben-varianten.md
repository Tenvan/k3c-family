# MG1.2 · Spielerfarben als Varianten spieler1 bis spieler4

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** monarch
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg1/2-spielerfarben-varianten
- **Abhängig von:** MG1.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Monarch hat die Varianten `spieler1` bis `spieler4` (Material-Tausch), gerendert als `<animation>@spieler<n>.png` mit Golden.

## Kontext

- Spiel heute (`data/sprites.json` › `players`): P1 König, P2 König-2, P3 König hellblau getönt, P4 König-2 rosa getönt.
- Varianten laut `paper-spec.md` › 3: `varianten: {name: {tausch: {material: material}}}`, `tausch` hat mindestens einen Eintrag; gerendert als `<animation>@<variante>.png`.
- Materialien nur aus der Stil-Datei (14 Stück); Rock = `koenigsblau`, Umhang = `koenigsrot`.

## Erlaubte Dateien

- `public/grafik/figuren/**`
- `docs/sprints/aktiv/MG1-papier-monarch-spielerfarben/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Materialien (`assets_style_set` verboten); das Spiel lädt die Varianten noch nicht (US1).

## Schritte

1. Varianten per `assets_set` setzen (spieler1 = Basis, 2–4 mit klar unterscheidbaren Rock/Umhang-Paaren).
2. Jede Variante in idle und galopp mit `assets_preview` prüfen.
3. `assets_review`, `task gen:paper`, Golden, `check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: `assets_list` meldet monarch gültig mit vier Varianten.
- [ ] AC-02: `golden.json` enthält alle `@spieler<n>`-Dateien, `task test -- paper` grün.
- [ ] AC-03: jede Variante im Preview geprüft, `assets_review` ohne roten Eintrag.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `assets_list figuren` → monarch gültig, Varianten spieler1–spieler4 (Rock blau, smaragd, purpur, rot; D-MG1-03).
- AC-02 geprüft: `task gen:paper` → `golden.json` mit 25 Dateien (5 Animationen × Basis + 4 Varianten); `spieler2` unverändert gegenüber HEAD.
- AC-03 geprüft: Preview spieler1 idle 0, spieler2 galopp 2 und schritt 4, spieler3 lauf 3 und galopp 6, spieler4 idle 3, lauf 6 und trab 5; `assets_review` → `geaendert`, kein roter Eintrag.
- Befunde: erster Versuch mit Umhang-Tausch färbte Nase und Krone mit (gleiches Material `koenigsrot`) → verworfen, B-710 (Teil-Patch in `assets_set`); Satteldecke bleibt blau → B-711; `assets_review` zeigt Pixel-Anteil nur der Basis, nicht der Varianten (Ergänzung zu B-709).
