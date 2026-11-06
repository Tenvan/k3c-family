# MG6.3 · Hub-Stände B: Schmiede, Rüstkammer, Lager, Kaserne

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gebaeude
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg6/3-hub-staende-b
- **Abhängig von:** MG6.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Schmiede, Rüstkammer, Lager und Kaserne (6 × 4 Kacheln) liegen als Paper-Specs unter `public/grafik/gebaeude-paper/` mit den Stufen 1–5 als Delta nach der Materialkette.

## Kontext

- Vorbild: `public/grafik/gebaeude/{schmiede,ruestkammer,lager,kaserne}.json`; Lager und Kaserne sind Steinbauten (GEB5).
- Materialkette, Maß und Farbzuordnung aus MG6.1/MG6.2 (README › D-MG6-02, D-MG6-04).
- `assets_preview` kennt keine Stufe (D-MG6-03).

## Erlaubte Dateien

- `public/grafik/gebaeude-paper/{schmiede,ruestkammer,lager,kaserne}.paper.json`, ihre Ausgabe-Ordner, `index.paper.json`, `golden.json` der Gruppe
- `docs/sprints/aktiv/MG6-papier-gebaeude/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG6-Zeilen)

## Nicht-Ziele

Alte Gebäude-JSONs bleiben bis CU1; kein Code; kein `assets_style_set`.

## Schritte

1. Je Gebäude `assets_create`, `assets_set` mit Basis und `stufen`.
2. Stufe 1 per `assets_preview`, Stufen 2–5 über die Ausgabe ansehen, Befunde beheben.
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

- **AC-01 geprüft:** `assets_list {gruppe: gebaeude-paper}` meldet schmiede, ruestkammer, lager, kaserne gültig (11 Assets, 0 ungültig).
- **AC-02 geprüft:** `task gen:paper` (gen.ts) trägt alle vier mit je 5 Stufen in `golden.json` ein; Vitest `tools/paper`, `src/tools`, Planung 1669/1669, `go test ./tools/buildings` grün.
- **AC-03 geprüft:** Stufe 1 aller vier mit `assets_preview`, Stufen 2–5 über die Ausgabe (D-MG6-03); `assets_review` gegen HEAD: 4 × `neu`, kein roter Eintrag.
- **Befunde, behoben:** Amboss war `stoff` (Materialkette, wäre in Stufe 5 blau) → `maehne`; Sockelband der Kaserne (Stufe 2) lief über das Tor → links und rechts davon geteilt; doppelter Wimpel der Kaserne entfernt.
- **Materialien (D-MG6-02):** Rüstkammer-Schiefer ist `stoff` und wird in Stufe 2 mit `holz` zu `stein`; Kaserne beginnt in Stein (Stufe 2 nur Eisenband, Kette ab Stufe 3, Holztor bleibt); Kamin, Esse und Sockel bleiben in Stufe 2 Stein und folgen ab Stufe 3 der Kette.
- `check:diff` rot nur im Frontend der Workbench (2 zeitabhängige Tests unter Last, wie MG6.2). Keine neuen Tickets.
