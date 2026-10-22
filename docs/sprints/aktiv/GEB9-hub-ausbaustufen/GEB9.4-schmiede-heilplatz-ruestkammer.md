# GEB9.4 · Schmiede, Heilplatz, Rüstkammer Stufen 2–5

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb9/4-schmiede-heilplatz-ruestkammer
- **Abhängig von:** GEB9.2
- **Tickets:** B-435
- **Kriterien:** AC-02, AC-03, AC-05

## Ziel

Schmiede, Heilplatz und Rüstkammer haben `stufen` 2–5 in ihrer JSON und PNGs für Stufe 1–5; das Camp bleibt Grundform.

## Kontext

- Vorgehen wie GEB9.3 (Stufen-Looks `docs/standards/gebaeude-grafik.md` § 6 › Hub-Gebäude, `tools/buildings/stages.go`, `task gen:gebaeude`).
- Schmiede und Rüstkammer haben Stein- bzw. Eisenfarben in der Grundform, Glut, Funken, Wappen und Waffen bleiben unverändert (Details § 6). Heilplatz: Holz/Stoff-Grundform.
- Das Camp (`rekrutierungslager.json`) bleibt Grundform (Entscheidung 4, Sprint-README), keine Änderung.
- Gebäude-JSONs: `public/grafik/gebaeude/schmiede.json`, `heilplatz.json`, `ruestkammer.json`.

## Erlaubte Dateien

- `public/grafik/gebaeude/schmiede*`, `heilplatz*`, `ruestkammer*` (JSON, PNG), `public/grafik/gebaeude/index.json`

## Nicht-Ziele

Camp, andere Gebäude, Spielcode, Löschen der Grundform-Dateien.

## Schritte

1. Branch `geb9/4-schmiede-heilplatz-ruestkammer`, `Status: in Arbeit`. § 6 und die drei JSONs lesen.
2. Je Gebäude `stufen` 2–5 schreiben.
3. `task gen:gebaeude`, Stufen 1–5 in 128 px ansehen, Lesbarkeit und Stil prüfen, korrigieren.
4. Zweiter Lauf ohne Diff; `task check:diff`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-02: die drei JSONs führen `stufen` 2–5; PNGs für Stufe 1–5; Camp unverändert (Entscheidung 4).
- [ ] AC-03: `index.json` führt je Gebäude fünf Stufen, die Grafik-Seite zeigt die Reihe.
- [ ] AC-05: Generator zweimal ohne Diff; `task check:diff` grün.

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
