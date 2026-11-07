# RS1.6 · Spike: heißer Sim-Pfad in Rust, Messung und Entscheidung

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** live
- **Branch:** rs1/6-spike-sim-pfad
- **Abhängig von:** RS1.2, RS1.3, RS1.5
- **Tickets:** B-725
- **Kriterien:** AC-07

## Ziel

Der heißeste Sim-Pfad aus dem RS1.1-Profil läuft als Rust-Spike; gemessen auf PC und Pi gegen das optimierte Go aus RS1.3; 006 nennt mit Zahlen, ob der Kern voll umgestellt wird.

## Kontext

Abbruchkriterium aus 006: weniger als 2× Durchsatz gegen optimiertes Go auf PC **und** Pi, oder Brückenkosten > 20 % der Tick-Dauer → Rust verwerfen. Der Raum liest den Sim-Zustand für Snapshot/Delta (`engine/net/delta.go`); ein Spike muss deshalb auch die Kosten zeigen, den Zustand zurück nach Go zu bringen. Die Workbench nutzt `engine/sim` an 36 Stellen (`sim_run`, Balancing). Brücke: `engine/core` (RS1.4), Muster: RS1.5.

## Erlaubte Dateien

- `rust/k3c-core/**`
- `engine/core/**`
- `engine/sim/*_test.go`, `engine/sim/*_bench_test.go` (Vergleich, kein Umbau der Logik)
- `docs/decisions/006-rust-engine.md`
- `docs/projekte/RST-rust-engine.md` (Notizen)

## Nicht-Ziele

Kein voller Umbau, keine Änderung der Sim-Logik in Go, kein Schalter im Spielbetrieb.

## Schritte

1. Heißesten Pfad aus RS1.1/RS1.3 wählen (z. B. Gegner-/Truppen-Schritt einer Stufe), Grenze so legen, dass Eingabe und Ergebnis klein bleiben.
2. Pfad in Rust, Vergleichstest gegen Go auf festen Seeds (bitgleich).
3. Benchmark Go vs. Rust inkl. Brücke auf PC, dann auf dem Pi (Benchmark-Binary arm64).
4. Hochrechnung: Anteil des Pfads an der Tick-Dauer × Gewinn = erwartete Tick-Dauer p99 auf dem Pi.
5. 006 ergänzen: Ergebnis, Empfehlung voller Umbau ja/nein, bei Ja Folgesprint-Skizze.

## Fertig, wenn

- [ ] AC-07: 006 › Messbefunde enthält den Spike-Vergleich PC und Pi, Brückenanteil und die Entscheidung mit Begründung.
- [ ] Vergleichstest Go↔Rust grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-07 teilweise geprüft (nur PC): Spike `collectCoins` in Rust (`rust/k3c-core/src/coins.rs`), Vergleichstest Go↔Rust bitgleich grün (`TestRustCollectCoinsBitgleich`), Benchmark `BenchmarkCollectCoins` (Go 257 ns / Rust 227 ns bei 27 Münzen; 3810/1207 bei 327; 17939/3899 bei 3027; mit Packen der Eingabe 2089/16842/78476 ns). Brückenaufruf 41 ns.
Befund: Der Sim-Pfad ist nur ~2 % der Raum-Tick-Zeit, 97,7 % liegen in `stateOf` (JSON-Roundtrip). Teilweise Migration mit Go-Zustand ist netto negativ (Packkosten); Empfehlung in 006: Kern nicht voll umstellen, zuerst B-731.
Pi-Werte fehlen (RS1.2 Mensch offen), die Entscheidung bleibt bei Ralf. Neues Ticket: B-731 (Projekt RST).
