# W12 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Morgengrauen als eigene Phase in Simulation, Protokoll und Regeln
- **Projekt:** WRT

## Ziel

Simulation, Protokoll und Regeltexte kennen das Morgengrauen als eigene Phase nach Q65. Am Ende sichtbar: `task check:go` grün mit vier Phasen, Golden mit Begründung verschoben, Protokoll-Beispiel mit `dawn`.

## Ergebnis

2026-10-25, Claude (W12.4): `task check`, `task check:go` und `check:studio` grün. `dev:test` war einmal rot, ein Flake ohne Bezug zu W12 (B-621, mit -count=5 grün). `check:race` lokal übersprungen (kein C-Compiler). AC-01, AC-02, AC-05 erfüllt durch W12.1 (vier Phasen, `data/light.json`), AC-03 durch W12.2 (Protokoll `dawn`), AC-04 durch W12.3 (Regeltexte). Golden ist im eigenen Commit verschoben. Review (Opus) ohne schwere Befunde in Sim/Server; die zwei Client-Befunde sind behoben, dabei B-470/AC-04 und AC-05 vorgezogen. Gestapelt auf `sprint/geb16`; die dort roten Studio-Tests (Fixture, Zä …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Die Simulation kennt vier Phasen mit Dauern aus `data/biomes/*.json`, das Ereignis `dawn` … |
| AC-02 | geprüft | Die Golden-Daten sind in eigenem Commit mit Begründung verschoben (B-470/AC-06). |
| AC-03 | geprüft | Das Protokoll beschreibt die Phase `dawn` mit Beispiel unter `testdata/protocol/` (B-470 › … |
| AC-04 | geprüft | Regeltexte, Glossar und Game-Design nennen die vier Phasen (B-470/AC-07). |
| AC-05 | erfüllt | Licht-Felder für Fackeln und Lagerfeuer stehen mit vorläufigen Werten in `data/` (B-472 › … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-470 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft | Der Tageszyklus hat eine eigene Phase Morgengrauen (Q65, 3:1:2:1) |
