# W1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Hub-Ausbau und Mauerstufen
- **Projekt:** –

## Ziel

Hub-Stufen 1 bis 5 und Mauer-/Turm-Stufen 1 bis 5 sind in der Simulation spielbar, Reparatur zwischen den Wellen eingeschlossen. Am Ende sichtbar: `task check:go` grün, Tests für Ausbau, Zerstörung und Reparatur, aktualisierte Golden-Daten.

## Ergebnis

- 2026-10-05 (W1.4, autonom): AC-01 (W1.1: `hub_level_test.go`), AC-02 (W1.2: `site_level_test.go`), AC-03 (W1.3: `repair_test.go`), AC-04 (W1.3: `save_migration_test.go`, Fixture `v4`, Golden in W1.1) mit Nachweis; `task check` und `task check:go` grün (`-race` übersprungen, kein C-Compiler). - Review des Diffs: keine schweren Befunde (Werte nur in `data/`, deterministisch, 2 Spieler zahlen gemeinsam, Hub-Stufe bleibt bei Zerstörung, v1–v3 unverändert, Golden nur `sim-mine-welle` durch die Hub-Sperre, `rng.json` unverändert). Abweichung W1.3 (Felder optional in v4, Fixture v4) begründet. - Ne …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Hub-Ausbau auf Stufe 2 bis 5 bucht Gold und Material laut `hub.json` ab; Gebäude höherer S … |
| AC-02 | – | Mauer und Turm werden am selben Platz auf Stufe 2 bis 5 ausgebaut, Kosten und HP laut Date … |
| AC-03 | – | Zerstörung setzt die Stufe von Mauer und Turm zurück, die Hub-Stufe bleibt; Reparatur stel … |
| AC-04 | – | Spielstand speichert Hub- und Gebäude-Stufen, Golden-Daten sind aktualisiert, `task check: … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-112 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Der Hub wird in fünf Stufen ausgebaut, Mauern und Türme haben fünf Materialstufe … |
