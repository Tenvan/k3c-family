# B-140 · Feedback-Ereignisse laufen im Protokoll mit gemessener Bandbreite zum Client

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** F4
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F4

## Ausgangslage

Der Server schickt Snapshots und Deltas (`engine/net/delta.go`, Protokoll v3 laut `docs/protocol.md`). Die neuen Feedback-Ereignisse aus B-139 kommen nicht beim Client an. Die Snapshot-Größe bei 4 Spielern × 3 Stufen ist ungemessen; `engine/sim/island_bench_test.go` misst nur die Tick-Dauer (`BenchmarkIslandStep3Stages4Players`), keine Bytes je Tick. Das Leistungsziel aus B-042: Tick-Dauer p99 < 10 ms bei 2 Räumen × 3 Spielern.

## Ziel

Die Feedback-Ereignisse werden im Protokoll übertragen, und ein Benchmark nennt Bytes je Tick und Tick-p99 bei 4 Spielern × 3 Stufen mit Ereignissen. Nutzen: Ton und Juice können gebaut werden, und das Bandbreiten-Budget für Pi und WLAN steht als Zahl.

## Beteiligte und Zielgruppen

Entwickler (Server und Client), Betreiber des Pi; Bandbreitenbudget entscheidet 🧑 (Beschluss Q08).

## Anforderungen

- Ereignisse aus B-139 stehen im Snapshot bzw. Delta der Stufe, in der sie passiert sind, höchstens einmal je Tick und Gerät.
- Der Client-Typ in `src/online/protocol.ts` und `docs/protocol.md` kennt die Ereignisse; `testdata/protocol/` bekommt ein Beispiel.
- Benchmark `island_bench_test` erweitert: meldet Bytes je Tick (Mittel, p99) für 4 Spieler × 3 Stufen mit Ereignissen in der Nacht.
- Zielwert Bytes je Tick und KB/s je Client steht in `docs/protocol.md` (Zahl aus Q08).

## Nicht-Ziele

Erzeugung der Ereignisse (B-139), Verwendung im Client (SO1, GR5), Delta-Kompression über das hinaus, was für das Budget nötig ist.

## Regeln und Einschränkungen

Domäne SRV (`engine/net/`); Protokolländerung als eigene Session, nur Protokoll und beide Enden (`docs/arbeitsweise.md` › Grenzfälle); Version des Protokolls erhöhen, falls das Format inkompatibel wird. Entscheidung 002.

## Beispiele

Gegner trifft die Mauer → das Delta dieses Ticks enthält ein `hit`; ein Client in einer anderen Stufe bekommt es nicht.

## Ausnahme- und Fehlerfälle

Ältere Clients (Protokollversion) → klare Ablehnung wie bisher, keine stille Verfälschung.

## Akzeptanzkriterien

- **AC-01** Go-Test: Ein Ereignis aus B-139 erscheint im Snapshot der betroffenen Stufe und nicht in dem einer anderen Stufe (`task check:go`).
- **AC-02** `docs/protocol.md`, `src/online/protocol.ts` und `testdata/protocol/` beschreiben die Ereignisse; ein Test mit dem Beispiel aus `testdata/protocol/` ist grün (`task check`).
- **AC-03** `go test -bench Island ./engine/sim` gibt Bytes je Tick (Mittel, p99) aus; der Wert ist in `docs/protocol.md` als Ist-Wert eingetragen.
- **AC-04** Der Zielwert KB/s je Client steht in `docs/protocol.md`; ein Test oder der Benchmark schlägt fehl, wenn der Wert überschritten ist (bei 4 Spielern × 3 Stufen).

## Offene Fragen

Bandbreitenbudget (Bytes je Tick, KB/s je Client) und Ereignisliste: `docs/fragenkatalog.md` Q08, entscheidet 🧑.

## Notizen

Aus Plan Phase 0 (F3), Lücke 4. Voraussetzung: B-139.
