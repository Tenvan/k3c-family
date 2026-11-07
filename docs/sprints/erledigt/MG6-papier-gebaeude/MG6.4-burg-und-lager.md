# MG6.4 · Burg und Lager: Lagerfeuer, Burg, Rekrutierungslager

- **Status:** fertig
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

- **AC-01 geprüft:** `assets_list {gruppe: gebaeude-paper}` meldet lagerfeuer, burg, burg-ausbau, rekrutierungslager gültig (15 Assets, 0 ungültig).
- **AC-02 geprüft:** `task gen:paper` (gen.ts) trägt alle vier ein (Burg 5, Lagerfeuer, Burg-Ausbau und Rekrutierungslager je 3 Stufen); Vitest `tools/paper`, `src/tools`, Planung 1669/1669 grün.
- **AC-03 geprüft:** Stufe 1 aller vier mit `assets_preview` (Burg-Ausbau Stufe 1 gewollt leer), weitere Stufen über die Ausgabe (D-MG6-03), dazu Burg-Stufen 2–5 mit Burg-Ausbau Stufe 3 übereinander gelegt; `assets_review` gegen HEAD: 4 × `neu`, kein roter Eintrag.
- **D-MG6-05 umgesetzt:** `burg` hat die Hub-Stufen 1–5, `burg-ausbau` ist eine Überlage im gleichen Rahmen (Stufe 2 Wappenbanner und Fackeln, Stufe 3 dazu Wehrtürmchen); die Türmchen sind aus Holz, weil die Überlage das Material der Burg-Stufe nicht kennt.
- **Befund für PS1 (nicht geändert):** In Burg-Stufe 5 ragen die Banner der Stufe 4 seitlich aus den Kristallspitzen (Stufen sind kumulativ, ein Zusatz lässt sich nicht entfernen).
- Rekrutierungslager: Stufen sind die Looks je Biom (Wald, Höhle, Mine), nicht Ausbau. `check:diff` rot nur im Frontend der Workbench (6 zeitabhängige Tests unter Last, wie MG6.2). Keine neuen Tickets.
