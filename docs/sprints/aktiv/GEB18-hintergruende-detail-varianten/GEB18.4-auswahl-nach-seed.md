# GEB18.4 · Level-Generator wählt die Hintergrund-Variante je Abschnitt nach Seed

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** geb18/4-auswahl-nach-seed
- **Abhängig von:** GEB18.1
- **Tickets:** B-618
- **Kriterien:** AC-03

## Ziel

Der Level-Generator legt je Abschnitt der Welt eine Hintergrund-Variante fest, deterministisch nach Seed und ohne gleiche Variante direkt nebeneinander.

## Kontext

`engine/level/` (Generator, RNG-Ströme aus `engine/rng`), Golden unter `testdata/golden/`; Anzahl der Varianten je Biom aus den Daten (Default 5); Abschnittslänge = Kachelbreite der Schicht „mitte“.

## Erlaubte Dateien

- `engine/level/**` und zugehörige Tests
- `data/` nur ein Wert für die Zahl der Varianten je Biom
- `testdata/golden/` (`task go:golden`, Begründung im Commit)

## Nicht-Ziele

Protokoll und Client (GEB18.5), Grafik.

## Schritte

1. Eigener RNG-Strom für Hintergründe, damit andere Folgen unverändert bleiben.
2. Variante je Abschnitt, Nachbarn verschieden.
3. Go-Tests: gleicher Seed → gleiche Folge, anderer Seed → andere Folge, keine gleichen Nachbarn; Golden neu mit Begründung.

## Fertig, wenn

- [ ] AC-03: Go-Tests grün, Golden neu und begründet.

## Prüfen

`check_run go:test` mit Muster `level`, vor dem Commit `task check:diff`; Reviewer (Domäne SIM).

## Ergebnis

- AC-03 umgesetzt: `engine/level/backdrop.go` legt je Hintergrund-Kachel (`BackdropTileUnits` = 24) eine Variante 1 … n fest, aus dem eigenen Strom `<biom>:<seed>:backdrop` (wie `:sites`), Nachbarn verschieden, Länge Weltbreite/24 + 1; `Layout.Backdrops` (vorerst `json:"-"`, in die level-Nachricht mit GEB18.5); Zahl der Varianten aus `data/biomes/*.json › backdropVariants` (5).
- AC-03 geprüft: `go test` (`check_run go:test`) grün, neu `backdrop_test.go`: gleicher Seed gleiche Folge, anderer Seed/Biom andere, Werte 1 … 5, keine gleichen Nachbarn, Länge, n = 1 und n = 0, Level mit und ohne Folge haben gleiche Chunks und Entities; Golden unverändert (eigener Strom, Feld ohne JSON), daher kein `go:golden` nötig. `go:lint`, Vitest `biome`, `k3c-server -check-data` grün. Review (Sonnet): keine schweren Befunde. `task check:diff` nicht gelaufen (Ralf: in dieser Claude-Session gesperrt).
- Abweichung: Abschnitt = Hintergrund-Kachel (24 Units) statt Kachelbreite der Schicht „mitte“ in Welt-Units; der Client wählt je Kachel im Raum der Schicht (Scroll-Faktor), dieselbe Folge für alle drei Schichten. Nebenbei: Studio-Hintergründe auf Spaltenbreite skaliert (Wunsch Ralf, eigener Commit).
