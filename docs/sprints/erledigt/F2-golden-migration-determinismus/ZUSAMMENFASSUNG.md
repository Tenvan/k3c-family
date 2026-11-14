# F2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Golden-Ablauf, Spielstand-Migration und Determinismus
- **Projekt:** –

## Ziel

Drift der Golden-Daten, Datenverlust bei Spielständen und Abweichungen auf arm64 machen die CI rot. Am Ende sichtbar: `task golden:update` in `task --list`, Abschnitte in `docs/arbeitsweise.md`, ein grüner arm64-Job in der CI.

## Ergebnis

2026-10-03, Review F2.4 (Agent, Claude Opus 5.5): AC-01, AC-02 (Golden), AC-08 in F2.1; AC-02 (Spielstand), AC-03 bis AC-05 in F2.2; AC-06, AC-07 in F2.3 – alle mit Nachweis, nichts verschoben. arm64: Job `Go · Tests arm64` (`ubuntu-24.04-arm`) grün in PR #35 und auf `develop`; `golden:update` ruft nur `./engine/sim ./engine/level` (`rng.json` liegt in `engine/rng`, unberührt); kein Test gelockert, Fixtures enthalten nur Spielstand; Map-Check ohne offene Treffer (eine Ausnahme mit Begründung). Behobene Befunde: keine. Neues Ticket: B-187 (SRV, `TestGleichzeitigesSpeichern` unter Windows einmal …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `task --list` zeigt `golden:update`; der Lauf erzeugt `testdata/golden/` neu, danach ist ` … |
| AC-02 | – | `docs/arbeitsweise.md` enthält die Abschnitte „Golden aktualisieren“ und „Spielstand-Forma … |
| AC-03 | geprüft | Ein Go-Test lädt je ein Fixture aus `testdata/saves/v1/` und `v2/` über `ParseIslandSave` … |
| AC-04 | geprüft | Ein Spielstand mit neuerer Version ergibt eine Meldung mit beiden Zahlen und lässt die Que … |
| AC-05 | geprüft | Ein Test verlangt zu jeder Version von 1 bis `IslandSaveVersion` ein Fixture-Verzeichnis ( … |
| AC-06 | geprüft | Ein Check findet `range` über Maps in `engine/sim` und `engine/level`, Ausnahmen stehen mi … |
| AC-07 | geprüft | Ein Test belegt gleichen Weltzustand nach 600 Ticks mit und ohne verlangsamten Takt (B-138 … |
| AC-08 | geprüft | Ein CI-Job führt die Go-Golden-Tests für arm64 aus und ist grün (B-071/AC-01). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-137 | erledigt | AC-01 geprüft, AC-02 –, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft | Golden-Daten und Spielstand-Formate haben einen festen Änderungsablauf |
| B-138 | erledigt | AC-06 geprüft, AC-07 geprüft | Determinismus der Simulation wird gegen Map-Reihenfolge und langsame Ticks geprü … |
| B-071 | erledigt | AC-08 geprüft | Die Golden-Tests laufen auch auf arm64 grün |
