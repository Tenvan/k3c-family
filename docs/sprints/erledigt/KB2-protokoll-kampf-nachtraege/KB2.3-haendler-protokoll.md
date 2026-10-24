# KB2.3 · Protokoll beschreibt Händler-Überfall und HP des Händlers

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb2/3-haendler-protokoll
- **Abhängig von:** KB2.2
- **Tickets:** B-381, B-386
- **Kriterien:** AC-03

## Ziel

Der Händler steht mit `raid`, `hp` und `maxHp` im Zustand. Protokoll, Client-Typ und Beispiel beschreiben das. Die Entscheidung zu Q08 (B-386, Variante B) steht im Protokoll.

## Kontext

`engine/sim/merchant.go` (`Merchant`, HP bisher `json:"-"`), `docs/protocol.md` › Wirtschaft und Snapshot-Größe, `src/model/types.ts` (`Merchant`), `testdata/protocol/s2c-snapshot-wirtschaft.json`, `engine/net/wirtschaft_test.go`.

## Erlaubte Dateien

- `engine/sim/merchant.go`, `engine/net/wirtschaft_test.go`
- `docs/protocol.md`, `testdata/protocol/s2c-snapshot-wirtschaft.json`, `testdata/golden/sim-forest-*.json`
- `src/model/types.ts`, `src/scenes/kampfAnzeige.ts`, Tests mit Händler-Objekten unter `src/`

## Nicht-Ziele

Darstellung der Händler-HP (K5); neue Ereignisse; Spielstand-Format.

## Schritte

1. JSON-Tags `hp`/`maxHp` am `Merchant`.
2. Protokoll, Typ und Beispiel ergänzen, Q08-Entscheidung eintragen.
3. Protokoll-Test für den Überfall, Golden neu schreiben (nur `hp`/`maxHp` beim Händler).

## Fertig, wenn

- [x] AC-03: `TestHaendlerUeberfallImZustand` und `TestWirtschaftBeispiel` sind grün; Doku und Typ nennen die Felder.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-03: umgesetzt. Geprüft mit `go test ./engine/...`, `task typecheck`, `task lint` und gezielten Vitest-Läufen (clientEconomy, citizenLabels, kampfAnzeige).
- Die Golden-Dateien `sim-forest-*` sind per `task go:golden` neu geschrieben. Geändert hat sich nur, dass `merchant` jetzt `hp`/`maxHp` enthält. `KampfWorld` braucht keinen eigenen `raid`-Zusatz mehr. `check:diff` hat Claude auf Ralfs Wunsch in dieser Claude-Session nicht laufen lassen.
