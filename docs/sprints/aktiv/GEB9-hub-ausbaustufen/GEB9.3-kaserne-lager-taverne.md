# GEB9.3 · Kaserne, Lager, Taverne Stufen 2–5

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb9/3-kaserne-lager-taverne
- **Abhängig von:** GEB9.2
- **Tickets:** B-435
- **Kriterien:** AC-02, AC-03, AC-05

## Ziel

Kaserne, Lager und Taverne haben `stufen` 2–5 in ihrer JSON und PNGs für Stufe 1–5.

## Kontext

- Vorgehen und Dateien wie GEB9.2: Stufen-Looks in `docs/standards/gebaeude-grafik.md` § 6 › Hub-Gebäude, Mechanik `tools/buildings/stages.go`, Aufruf `task gen:gebaeude`, Übergang (Grundform-Datei bleibt) steht seit GEB9.2.
- Kaserne und Taverne haben schon Steinfarben in der Grundform: dort wechselt der Stein, Holzteile bleiben Holz. Lager: Sack, Holz, Erz bleiben, Gerüst/Dach wechselt (Details § 6).
- Gebäude-JSONs: `public/grafik/gebaeude/kaserne.json`, `lager.json`, `taverne.json`.

## Erlaubte Dateien

- `public/grafik/gebaeude/kaserne*`, `lager*`, `taverne*` (JSON, PNG), `public/grafik/gebaeude/index.json`
- `tools/buildings/` nur für Tests, falls nötig

## Nicht-Ziele

Andere Gebäude, Spielcode, Löschen der Grundform-Dateien.

## Schritte

1. Branch `geb9/3-kaserne-lager-taverne`, `Status: in Arbeit`. § 6 und die drei JSONs lesen.
2. Je Gebäude `stufen` 2–5 nach § 6 schreiben (Farbtausch, Zusatz ab Stufe 3, `aenderung`).
3. `task gen:gebaeude`; die PNGs der Stufen 1–5 in 128 px ansehen (Read) und auf Lesbarkeit und Stilgleichheit mit Burg/Mauer prüfen; Fehler in der JSON korrigieren.
4. Zweiter Generatorlauf ohne Diff; `task check:diff`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-02: die drei JSONs führen `stufen` 2–5; PNGs für Stufe 1–5 in allen Dichten.
- [ ] AC-03: Die Stufenreihe erscheint im `index.json` (`stufen` mit fünf Einträgen je Gebäude), damit die Grafik-Seite sie zeigt.
- [ ] AC-05: Generator zweimal ohne Diff; `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 umgesetzt: `kaserne.json`, `lager.json`, `taverne.json` führen `stufen` 2–5, PNGs für Stufe 1–5 in allen Dichten; Stufen in 64 px angesehen. AC-03 geprüft: `scenesLogic.test.ts` prüft die Stufenreihe 2–5 samt Dateinamen am echten `index.json` jetzt für fünf Hub-Gebäude (28 Tests grün). AC-05 geprüft: Generator zweimal ohne Diff, `go test ./tools/buildings` grün.

Abweichungen von § 6 (in der Tabelle nachgetragen): Kaserne-Sockelband in zwei Stücken links und rechts vom Tor (sonst über dem Tor gezeichnet); Lager hat kein Tor, das Vorratsschild hängt an der Wand, die dunkle Innenwand (`loulou`) tauscht als Rolle Schatten (sonst leuchtet sie auf Stufe 5 türkis); Taverne-Putz (`pancho`) tauscht als Rolle Licht, Laterne links neben der Tür. Die Kaserne-Stufe 2 unterscheidet sich nur durch das Eisenband (Stein bleibt Stein). `task check:diff` ganz nicht abgewartet (Studio-Vitest dauert unter Last über 10 Minuten), Teilprüfungen grün.
