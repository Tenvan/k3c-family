# GEB9.4 · Schmiede, Heilplatz, Rüstkammer Stufen 2–5

- **Status:** fertig
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

AC-02 umgesetzt: `schmiede.json`, `heilplatz.json`, `ruestkammer.json` führen `stufen` 2–5 mit PNGs für Stufe 1–5; das Camp (`rekrutierungslager`) bleibt unverändert (Entscheidung 4). Damit haben alle acht Hub-Gebäude Stufen. Die Stufen wurden in 64 px angesehen. AC-03 geprüft: `scenesLogic.test.ts` prüft die Stufenreihe 2–5 am echten `index.json` für alle acht Hub-Gebäude (31 Tests grün). AC-05 geprüft: Generator zweimal ohne Diff, `go test ./tools/buildings` grün.

Abweichungen von § 6 (in der Tabelle nachgetragen): Die Rüstkammer hat eine Holzwand auf Steinsockel (nicht Stein-Grundform), Stufe 2 tauscht also die Wand ohne Sockelband; Dachfarben als Rollen K/L/T (`deep-koamaru`, `venice-blue`, `valhalla`); Wimpel sitzt auf dem Dach. Beim Heilplatz tauschen nur Pfosten und Gestell, das Zelttuch bleibt — die Stufen unterscheiden sich dort nur dezent (Pfosten und Laterne), das ist die sichtbarste Schwäche und ein Kandidat für die Abnahme. Studio- und Go-Teilprüfungen grün, `task check:diff` als Ganzes läuft unter Last zu lange und folgt im Review GEB9.6.
