# MG6.2 · Hub-Stände A: Werkstatt, Taverne, Heilplatz

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gebaeude
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg6/2-hub-staende-a
- **Abhängig von:** MG6.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Werkstatt, Taverne und Heilplatz (Handelsstände 6 × 4 Kacheln) liegen als Paper-Specs unter `public/grafik/gebaeude-paper/` mit den Stufen 1–5 als Delta: Wand, Pfosten und Balken folgen der Materialkette, Tresen, Tuchdach und Ware bleiben, ab Stufe 3 ein Wimpel am rechten Pfosten.

## Kontext

- Vorbild: `public/grafik/gebaeude/{werkstatt,taverne,heilplatz}.json` (`vorlage`, `stufen[].aenderung`), Maß 6 × 4 Kacheln.
- Materialkette und Maß aus MG6.1 (README › D-MG6-02, D-MG6-04); bleibende Holzteile in `leder`, Stoff rot `koenigsrot`, Stoff hell `hermelin`.
- `assets_preview` kennt keine Stufe (D-MG6-03): Stufen ≥ 2 aus `<asset>/stand.s<n>.png`.

## Erlaubte Dateien

- `public/grafik/gebaeude-paper/{werkstatt,taverne,heilplatz}.paper.json`, ihre Ausgabe-Ordner, `index.paper.json`, `golden.json` der Gruppe
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

- [ ] AC-01: alle drei gültig (`assets_list`).
- [ ] AC-02: `golden.json` passt (`task test -- paper` grün).
- [ ] AC-03: alle drei in allen Stufen angesehen, Befunde im Ergebnis; `assets_review` ohne roten Eintrag.

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
