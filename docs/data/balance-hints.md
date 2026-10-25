# balance-hints.json

- **Zweck:** Richtwerte der Kennzahlen (Ampeln grün, gelb, rot) und Hebel je Ziel für die Auswertung der Balancing-Berichte (B-632): Kurzauswertung, Tuning-Hinweise, „Nächster Lauf“, `balance-*.md` und `balance-*.html`.
- **Gelesen von:** Balancing-Tester der Workbench (`tools/k3c-workbench/internal/balance/hints.go`), nicht die Simulation
- **Wichtigste Werte:** `figures.idleShare.green`, `figures.survivalDrop.green`, `figures.goldBalance.green`, `levers.castleHeld.low`
- **Regeln:** `docs/rules/zielkorridore.md` § 5 (Startwerte und Begründung)

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `version` | Ganzzahl | – | Version des Dateiformats | 1 |
| `notes` | Text | – | Hinweis für Menschen | – |
| `figures.*.name` | Text | – | Anzeigename | Leerlauf der Bauern |
| `figures.*.unit` | Text | – | Einheit für die Anzeige | %, Ticks, Gold |
| `figures.*.green` | [von, bis] | Einheit | Band der grünen Ampel; `null` = offen | [null, 25] |
| `figures.*.yellow` | [von, bis] | Einheit | Band der gelben Ampel, umfasst `green`; außerhalb rot | [null, 35] |
| `figures.*.allGrades` | Wahrheitswert | – | auch Grade außer normal bewerten (Standard: nur normal) | true |
| `levers.*.low` | Liste | – | Hebel, wenn der Wert unter der Untergrenze liegt | je Ziel |
| `levers.*.high` | Liste | – | Hebel, wenn der Wert über der Obergrenze liegt | je Ziel |
| `levers.*.low.*.path` | Text | – | Wert (auch in `high`) in `data/` als `datei.json:a.b.c` (wie bei Varianten) | `buildings.json:wall.levels.0.hp` |
| `levers.*.low.*.direction` | Text | – | `+` erhöhen, `-` senken | +, - |
| `levers.*.low.*.step` | Zahl | Anteil | relative Schrittweite (0,1 = 10 %), über 0 bis 1 | 0,1 bis 0,5 |
| `levers.*.low.*.label` | Text | – | Hinweistext | Mauer-HP Stufe 1 erhöhen |

Kennzahlen (`figures`): `firstWallTick`, `firstWorkshopTick`, `idleShare`, `survivalDrop`, `goldBalance`, `woodBalance`,
`stoneBalance`. Ziel-Kennzahlen (`levers`): die `measure`-Namen aus `balance-targets.json`. Unbekannte Felder,
Kennzahlen oder Ziele lehnt der Loader ab; ein Test prüft, dass jeder Hebel auf eine Zahl in `data/` zeigt.

## Wirkung auf das Balancing

Keine auf das Spiel: Die Datei steuert nur Ampeln und Vorschläge der Berichte. Engere Bänder machen die Kennzahl-Ampeln
strenger; andere Hebel ändern die Tuning-Hinweise und den Abstimm-Vorschlag im „Nächsten Lauf“.

## Beispiele für Varianten

Nicht als Variante nutzbar (nur Tester). Nach einer Änderung lässt sich die Auswertung eines vorhandenen Berichts neu
erzeugen: `task balance:run -- --insights reports/simtest-run-7/balance-<Zeit>.json` oder in der Workbench „HTML-Bericht
öffnen“.
