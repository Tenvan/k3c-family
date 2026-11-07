# B-731 · Zustands-Snapshot ohne JSON-Roundtrip (stateOf) – 98 % der Raum-Tick-Zeit

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** RST
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

RS1.1/RS1.6 (Entscheidung 006): Das CPU-Profil von `Room.Tick` (4 Geräte) zeigt 97,7 % in `pushStatesSaved` → `net.stateOf` (`engine/net/protocol.go`: `json.Marshal(w)` + `json.Unmarshal` in `map[string]any`, dazu dasselbe für `sim.EconomyOf`), nur 1,9 % in `sim.StepIsland`. Ein Raum-Tick kostet 1,1 ms, davon ~20 µs Simulation. Heap: `reflect.mapassign` und `bytes.Clone` dominieren. Auf dem Pi 3 (Tick p99 8–10 ms, B-042) liegt die Zeit damit in der Serialisierung, nicht in der Sim.

## Ziel

Der Raum-Tick erzeugt Snapshot und Delta ohne JSON-Roundtrip über `map[string]any`; Raum-Tick (`BenchmarkTickDevices4`) mindestens 3× schneller und Allokationen je Tick auf ein Zehntel; das Protokoll (`docs/protocol.md`) bleibt byte-gleich.

## Beteiligte und Zielgruppen

Spieleabende auf dem Pi (Tick p99), alle Raum-Tests; Entscheidung 006 (Rust oder Go-Optimierung).

## Anforderungen

- Zustand direkt in das Delta-Format schreiben (typisierter Encoder oder Diff auf Structs) statt Marshal→Unmarshal; Feldnamen, Reihenfolge und Zahlenformat wie bisher.
- Golden- und Protokoll-Tests unverändert grün; Messung vorher/nachher nach `docs/messung/rst-baseline.md`.

## Nicht-Ziele

Kein Rust, keine Protokolländerung, keine Regeländerung in `engine/sim`.

## Regeln und Einschränkungen

Entscheidung 001 und 006; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; 2 Spieler gleichzeitig; keine Rücksicht auf alte Spielstände.

## Beispiele

`BenchmarkTickDevices4`: heute 1,1–1,8 ms und 5 571 allocs/op; Ziel unter 0,4 ms und unter 600 allocs/op.

## Ausnahme- und Fehlerfälle

Weicht ein Byte des Protokolls ab, bleibt der alte Weg, bis der Unterschied erklärt ist.

## Akzeptanzkriterien

- **AC-01** `BenchmarkTickDevices4` (feste Iterationszahl) ist mindestens 3× schneller als die Baseline in `docs/decisions/006-rust-engine.md`; allocs/op ≤ 10 % der Baseline.
- **AC-02** `engine/net`-Protokolltests und Golden-Dateien unverändert grün; ein Vergleichstest zeigt altes und neues Snapshot-JSON byte-gleich.

## Offene Fragen

keine

## Notizen

Gefunden in RS1.6 (Spike `collectCoins`): Teil-Umstellung der Sim auf Rust spart höchstens ~2 % Tick-Zeit; erst die Serialisierung ist der Hebel.
