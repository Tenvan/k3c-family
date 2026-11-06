# MG6.4 · Burg und Lager: Lagerfeuer, Burg, Rekrutierungslager

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gebaeude
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg6/4-burg-und-lager
- **Abhängig von:** MG6.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Lagerfeuer (Stufen 1–3: Feuer, Zelt, Holzhütte), Burg (Hub-Stufen 1–5), Burg-Ausbau (Überlage der Ausbauschritte 1–3: leer, Banner und Fackeln, dazu Wehrtürmchen; D-MG6-05) und Rekrutierungslager (Looks 1–3 je Biom als Stufen) liegen als Paper-Specs unter `public/grafik/gebaeude-paper/`.

## Kontext

- Vorbild: `public/grafik/gebaeude/{lagerfeuer,burg,rekrutierungslager}.json`; Burg 8 × 9, Lagerfeuer 8 × 5, Rekrutierungslager 3 × 2 Kacheln.
- Stufen sind kumulativ: Die Burg-Stufen 6–13 (Banner, Fackeln, Wehrtürmchen je Material) passen nicht in eine reine Delta-Kette nach Stufe 5 (README › D-MG6-05).
- Materialkette aus MG6.1; `assets_preview` kennt keine Stufe (D-MG6-03).

## Erlaubte Dateien

- `public/grafik/gebaeude-paper/{lagerfeuer,burg,burg-ausbau,rekrutierungslager}.paper.json`, ihre Ausgabe-Ordner, `index.paper.json`, `golden.json` der Gruppe
- `docs/sprints/aktiv/MG6-papier-gebaeude/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG6-Zeilen)

## Nicht-Ziele

Alte Gebäude-JSONs bleiben bis CU1; kein Code; kein `assets_style_set`; Baugerüste (`hub-geruest`, `geruest-*`) gehören MG8 (D-MG6-01).

## Schritte

1. Je Gebäude `assets_create`, `assets_set` mit Basis und `stufen`.
2. Stufe 1 per `assets_preview`, weitere Stufen über die Ausgabe ansehen, Befunde beheben.
3. `task gen:paper`, `assets_list`, `assets_review`.
4. `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: alle vier gültig (`assets_list`).
- [ ] AC-02: `golden.json` passt (`task test -- paper` grün).
- [ ] AC-03: alle vier in allen Stufen angesehen, Befunde im Ergebnis; `assets_review` ohne roten Eintrag.

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
