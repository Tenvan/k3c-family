# BAL9.2 · Bot-Performance: wanted und nodeFor ohne Map-Zugriffe je Tick

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bal9/2-bot-performance
- **Abhängig von:** –
- **Tickets:** B-654
- **Kriterien:** AC-03

## Ziel

`wanted` und `nodeFor` der Balancing-Bots kommen ohne Map-Anlegen und ohne quadratische Platz-Schleife je Aufruf aus, der Meilenstein-Lauf bleibt byte-gleich (B-654).

## Kontext

Bot-Code liegt in `tools/k3c-workbench/internal/balance` (`bot_wirtschaft.go` › `nodeFor`, `bot_clear.go` › `wanted`, Aufrufer in `bots.go`). `miniBossID` liest `bosses.json` schon einmal je Prozess (`sync.OnceValue`, B-654 › Notizen). Bots laufen parallel je Szenario und auch online (`botdev`); ein globaler Tick-Cache wäre dort unsicher, daher Speedup ohne Zustand über Aufrufe hinweg.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/balance/bot_clear.go`, `bot_wirtschaft.go`, `bots.go`
- `tools/k3c-workbench/internal/balance/bot_perf_test.go` (neu)

## Nicht-Ziele

Kein Eingriff in `engine/sim`, keine Verhaltensänderung der Bots.

## Schritte

1. `wantSet`: belegte Zonen einmal je Durchlauf über die Plätze sammeln statt je Platz.
2. `nodeFor`: Vorrat markierter Knoten als `sim.Cost` statt Map, Knoten ohne Chance auf „nächster“ vor dem Preis-Zugriff überspringen, zahlbare Plätze und Endboss-Bau nur für Kandidaten.
3. `wallPayable`/`nearestWall`: billige Prüfungen vor `SiteGoldLeft`.
4. Gleichheitstest gegen die alte Fassung, Byte-Vergleich des Meilenstein-Laufs vorher/nachher.

## Fertig, wenn

- [x] AC-03: `wanted`/`nodeFor` ohne Map-Anlegen und ohne Platz-über-Platz-Schleife je Aufruf; Lauf byte-gleich, Laufzeit kürzer (B-654/AC-01).

## Prüfen

```bash
task check:dev
```
Gezielt: `go test ./internal/balance/ -run TestBotSchnellGleich` in `tools/k3c-workbench`.

## Ergebnis

- **AC-03 geprüft (B-654/AC-01):** Meilenstein-Lauf coop2 standard 40 Tage, Seeds k3c-dev,1,2,3 (`-tags fullgame`, vorher = Bot-Dateien aus HEAD, nachher = neu, gleicher `engine/sim`-Stand): `EndHash` und SHA-256 der `Metrics`-JSON aller 4 Seeds byte-gleich, je 1004401 Ticks. Wanduhr im Wechsel vorher/nachher/vorher/nachher (Rechner parallel ausgelastet, verrauscht): vorher 6m40s und 4m40s, nachher 2m46s und 1m09s, also jeweils mindestens 40 % kürzer; Ziel −8 % erfüllt. Die Höhe des Gewinns ist wegen der Last unsicher.
- Test `TestBotSchnellGleich` (`bot_perf_test.go`): vergleicht `wantSet.wanted` und `nodeFor` jeden 5. Tick über 3 Tage coop2 standard mit der alten Fassung (`wantedRef`, `nodeForRef`), grün.
- `bosses.json` wurde schon vorher nur einmal je Prozess gelesen. Ein Preis-Zugriff `prices.nodes[kind]` je Kandidat bleibt (Knoten-Art ist ein String); Map-Anlegen je Aufruf entfällt.
- Prüfungen: `go test ./internal/balance/...` grün, `golangci-lint` 0 Befunde, `go build ./...` grün. `task check:dev` gesamt nicht gelaufen (`check_run` kennt das Ziel nicht).
- Kein Profil nach der Änderung; den Bot-Anteil in % hat niemand neu gemessen.
