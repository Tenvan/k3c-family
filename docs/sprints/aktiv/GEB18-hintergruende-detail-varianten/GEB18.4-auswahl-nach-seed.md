# GEB18.4 · Level-Generator wählt die Hintergrund-Variante je Abschnitt nach Seed

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
