# tools/k3c-workbench/internal/datadoc/

## Responsibility

Kennt die Dateien in `data/`, die Simulation, Level-Generator oder Balancing-Tester lesen, samt ihrer Beschreibungen unter `docs/data/<name>.md`, und liefert daraus Liste, Beschreibung und Zahlenwerte für die Balancing-Seite.

## Design

- Feste Liste `files` (`entry`: Name in `data/`, Doc-Name, Leser `BySim`/`ByLevel`/`ByTester`, `Vary`); `Vary` markiert Dateien, die `sim.UseData` neu lädt und die daher als Variante taugen. Ein Test hält die Liste gleich mit den Dateien, die `engine/` lädt.
- `File` (Zeile der Liste): Zweck und wichtigste Werte per Regex (`purposeLine`, `keyLine`) aus der Beschreibung; `Missing`, wenn `docs/data/<doc>.md` fehlt.
- `Value`/`FileValue`: Zahlenwert mit Pfad `a.b.0.c`, Form wie `internal/balance/vary.go` (`datei.json:a.b.c`).
- Pfadschutz: `Doc` nimmt nur Namen der Liste (`docName`), nie einen freien Pfad.
- Zahlen-Walker `walkNumbers`/`walk` über `json.Decoder` (Token-Strom, Reihenfolge der Datei bleibt erhalten).

## Flow

1. `List(root)`: je `entry` die Beschreibung lesen → `firstMatch` für Zweck und Schlüsselwerte → `[]File`.
2. `Doc(root, name)`: Namen prüfen → `docs/data/<name>.md` lesen.
3. `Values(root, name)`: `data/<name>` lesen → `walkNumbers` → `[]Value`; `AllValues(root)` sammelt das für alle variierbaren Dateien.

## Integration

- Konsument: `tools/k3c-workbench/app_balance.go` (Bindings `BalanceDataFiles`, `BalanceDataDoc`, `BalanceDataValues`, `BalanceDataValuesAll`) für die Balancing-Seite des Frontends.
- Abhängigkeiten: nur Standardbibliothek; Daten aus `data/` und `docs/data/`.
