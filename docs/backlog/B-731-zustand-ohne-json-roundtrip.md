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

RS1.1/RS1.6 (Entscheidung 006): Das CPU-Profil von `Room.Tick` (4 Geräte) zeigt 97,7 % in `pushStatesSaved` → `pushStates` → `net.stateOf` (`engine/net/protocol.go:158`), nur 1,9 % in `sim.StepIsland`. `stateOf` macht `json.Marshal(w)` + `json.Unmarshal` in `map[string]any`, danach dasselbe für `sim.EconomyOf(w)`; anschließend löschen, ergänzen und filtern auf der Map (`addActions`, `addPayable`, `filterSites`). `deltaOf` (`engine/net/delta.go`) vergleicht dann Maps per `reflect.DeepEqual`. Ein Raum-Tick kostet 1,1–1,8 ms und 5 571 allocs/op (`BenchmarkTickDevices4`), davon ~20 µs Simulation. Auf dem Pi 3 (Tick p99 8–10 ms, B-042) liegt die Zeit in der Serialisierung, nicht in der Sim. Der Zustand wird je Stufe einmal gebaut und von allen Geräten der Stufe geteilt (B-276).

## Ziel

Der Raum-Tick erzeugt Zustand und Delta ohne JSON-Roundtrip über `map[string]any`. Das Protokoll (`docs/protocol.md`) bleibt byte-gleich. Messbar: `BenchmarkTickDevices4` mindestens 3× schneller, allocs/op höchstens 25 % der Baseline (Stufe 1); Stufe 2 nur, wenn Stufe 1 das Ziel verfehlt.

## Beteiligte und Zielgruppen

Spieleabende auf dem Pi (Tick p99), alle Raum-Tests; Entscheidung 006 (Rust oder Go-Optimierung).

## Anforderungen

**Umfang (in dieser Reihenfolge, je Stufe ein Commit, Messung nach jeder Stufe nach `docs/messung/rst-baseline.md`):**
1. Stufe 1 – Zustand direkt bauen: `stateOf` baut die Map aus `sim.World` und `sim.EconomyOf` mit handgeschriebenem Code, ohne Marshal/Unmarshal. Zahlen als `float64`, Listen als `[]any`, Feldnamen wie die JSON-Tags der Sim-Typen. Der alte Weg bleibt als Orakel nur im Test (`stateOfJSON` in `_test.go`).
2. Stufe 2 (nur wenn AC-01 nach Stufe 1 verfehlt) – Delta ohne Maps: typisierte Snapshot-Structs je Stufe, `deltaOf` vergleicht Felder und Listen über `id` direkt und schreibt das Delta-JSON mit einem eigenen Encoder (Schlüssel sortiert wie `encoding/json`).
3. Vorher/Nachher-Zahlen in 006 《Go-Fixes》 ergänzen.

**Regeln für die Gleichheit:** Feldnamen, Schlüsselreihenfolge (sortiert), Zahlenformat (`float64`-Darstellung von `encoding/json`), `null` als Wert, Auslassen von `events`/`eventsDropped`/`unset` wie in `delta.go`.

## Nicht-Ziele

Kein Rust, keine Protokolländerung, keine Regeländerung in `engine/sim`, kein neues Wire-Format (z. B. Binary), keine Änderung am Client, kein Umbau von Spielstand-Kodierung (`r.encode`).

## Regeln und Einschränkungen

Entscheidung 001 und 006; Datei ≤ 400 Zeilen (`protocol.go` hat 197, ggf. neue Datei `state.go`), Funktion ≤ 60 Zeilen; 2 Spieler gleichzeitig; Review vor dem Commit (Domäne SRV, Protokollnähe); keine Rücksicht auf alte Spielstände. Neue Felder in `sim.World` dürfen nicht stumm fehlen: ein Test vergleicht per Reflection die Felder von `sim.World` mit dem Ergebnis, damit eine neue Sim-Eigenschaft den Test bricht.

## Beispiele

`BenchmarkTickDevices4` (feste Iterationszahl, `-benchtime 100000x`): heute 1,1–1,8 ms und 5 571 allocs/op; Ziel Stufe 1: ≤ 0,5 ms und ≤ 1 400 allocs/op. Pi 3: Tick p99 sinkt entsprechend (Messung RS1.2, Mensch).

## Ausnahme- und Fehlerfälle

Weicht ein Byte ab, bleibt der alte Weg aktiv, bis der Unterschied erklärt ist (kein halber Umbau im Commit). Ein int/float-Unterschied in der Map (Orakel liefert `float64`) ist ein Befund, kein Grund, den Vergleich aufzuweichen. Leere Stufe, Game Over, Zeitraffer (`devTimescale`/`devPaused`), `saved`-Ereignis aus `pushStatesSaved` und Vorschau (`preview.go`) werden mitgetestet.

## Akzeptanzkriterien

- **AC-01** `BenchmarkTickDevices4` (feste Iterationszahl) ist mindestens 3× schneller als die Baseline in `docs/decisions/006-rust-engine.md`; allocs/op ≤ 25 % der Baseline. Wird das nach Stufe 1 verfehlt, folgt Stufe 2 mit dem Ziel allocs/op ≤ 10 %.
- **AC-02** Ein Vergleichstest zeigt Snapshot-JSON und Delta-JSON von altem Orakel und neuem Weg byte-gleich: feste Seeds, alle sechs Biome, über ≥ 600 Ticks mit zwei Spielern, plus Zeitraffer, `saved`-Ereignis und Game Over.
- **AC-03** `engine/net`-Protokolltests und Golden-Dateien (`testdata/golden/`) unverändert grün; `docs/protocol.md` unverändert.
- **AC-04** Reflection-Test: Jedes exportierte Feld von `sim.World` mit JSON-Tag erscheint im Zustand oder steht in einer ausdrücklichen Ausnahmeliste (`seed`, `widthUnits`, `free`).
- **AC-05** `sim_test` online (`perf`, 4 Geräte) zeigt keinen schlechteren Tick-p99 als vorher; Zahlen in 006.

## Offene Fragen

1. **Ziel Allokationen:** Spec nimmt Stufe 1 mit ≤ 25 % und optional Stufe 2 mit ≤ 10 % an. Soll gleich auf 10 % (Stufe 2) gezielt werden? Default: Stufe 1 zuerst, Entscheidung nach Messung.
2. **Orakel:** Der alte JSON-Weg bleibt dauerhaft als Test-Orakel (Default: ja, nur `_test.go`).
3. **Pi-Messung:** AC-05 nur auf dem PC, Pi-Werte folgen mit RS1.2 (Default: ja).

## Notizen

Gefunden in RS1.6 (Spike `collectCoins`): Teil-Umstellung der Sim auf Rust spart höchstens ~2 % Tick-Zeit; erst die Serialisierung ist der Hebel. Nach B-731 neu bewerten, ob ein Rust-Kern noch nötig ist (Entscheidung 006). Zuordnung zu einem Sprint offen, Projekt RST.
