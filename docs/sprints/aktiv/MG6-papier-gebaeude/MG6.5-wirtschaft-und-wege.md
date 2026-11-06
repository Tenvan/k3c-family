# MG6.5 · Wirtschaft und Wege: Farm, Treppen, Hafen, Schiff

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gebaeude
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg6/5-wirtschaft-und-wege
- **Abhängig von:** MG6.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Farm (Stufen 1–5), Treppe hoch, Treppe runter, Hafen (statisch) und Schiff (Stufen 1–3: Kiel, Rumpf, Segel) liegen als Paper-Specs unter `public/grafik/gebaeude/`.

## Kontext

- Vorbild: `public/grafik/gebaeude/{farm,treppe-hoch,treppe-runter,hafen,schiff}.json`; Farm 4 × 3, Treppe hoch 4 × 4, Treppe runter 4 × 3, Hafen 6 × 4, Schiff 6 × 5 Kacheln.
- Statische Gebäude haben keine `stufen`.
- Materialkette aus MG6.1; `assets_preview` kennt keine Stufe (D-MG6-03).

## Erlaubte Dateien

- `public/grafik/gebaeude/{farm,treppe-hoch,treppe-runter,hafen,schiff}.paper.json`, ihre Ausgabe-Ordner, `index.paper.json`, `golden.json` der Gruppe
- `docs/sprints/aktiv/MG6-papier-gebaeude/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG6-Zeilen)

## Nicht-Ziele

Alte Gebäude-JSONs bleiben bis CU1; kein Code; kein `assets_style_set`.

## Schritte

1. Je Gebäude `assets_create`, `assets_set` mit Basis und ggf. `stufen`.
2. Stufe 1 per `assets_preview`, weitere Stufen über die Ausgabe ansehen, Befunde beheben.
3. `task gen:paper`, `assets_list`, `assets_review`.
4. `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: alle fünf gültig (`assets_list`).
- [ ] AC-02: `golden.json` passt (`task test -- paper` grün).
- [ ] AC-03: alle fünf in allen Stufen angesehen, Befunde im Ergebnis; `assets_review` ohne roten Eintrag.

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
