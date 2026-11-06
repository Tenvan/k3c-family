# MG1.2 · Spielerfarben als Varianten spieler1 bis spieler4

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
