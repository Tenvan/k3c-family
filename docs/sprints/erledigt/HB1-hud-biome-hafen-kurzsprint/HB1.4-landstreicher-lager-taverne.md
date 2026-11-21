# HB1.4 · Landstreicher-Lager zwischen Bäumen, Zerstörung bei Rodung, Taverne lockt Landstreicher

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/hb1
- **Abhängig von:** HB1.3
- **Tickets:** B-1187
- **Kriterien:** AC-03, AC-04

## Ziel

Landstreicher-Lager stehen zwischen 2 bis 3 Bäumen und werden bei Rodung zerstört; an der Stelle ist eine Taverne baubar, die bis zu 5 Landstreicher lockt (einen je Tag).

## Kontext

Ticket B-1187. Neue Mechanik nur in Rust: `rust/crates/k3c-sim` (Logik und Test), `k3c-level` (Platzierung); die Go-Engine bleibt (Entscheidung 013). Landstreicher-Lager und Einsiedler/Landstreicher-Logik zuerst per grep suchen (hermit, vagrant, Landstreicher). Taverne ist ein vorhandenes Gebäude (`data/buildings`). Deterministisch über den vorhandenen Rng, keine Rücksicht auf alte Spielstände.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-level/**`
- `data/**`
- `testdata/**`

## Nicht-Ziele

HUD, Hafen (HB1.5), Balancing der Kosten.

## Schritte

1. Lager so platzieren, dass 2 bis 3 Bäume direkt daneben stehen.
2. Sind diese Bäume gefällt, Lager entfernen.
3. Taverne an dieser Stelle baubar; je Tag ein Landstreicher, höchstens 5.
4. Sim-Tests (Lager, Rodung, Taverne, Tageslimit).

## Fertig, wenn

- [ ] AC-03: Lager von 2 bis 3 Bäumen umgeben, bei Rodung zerstört (Test).
- [ ] AC-04: Taverne lockt höchstens 5 Landstreicher, einen je Tag (Test).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-03 geprüft: `k3c-sim/src/unit/camp_guard.rs`: Lager im Wald (Hauptrohstoff Holz) bekommen beim Aufbau mindestens 2 Bäume (2 oder 3, aus dem Ort, ohne Zufallsstrom), im ersten Takt wählt das Lager seine ≤ 3 nächsten Bäume als Wächter; fällt der letzte, ist das Lager zerstört (Event `campDestroyed`) und an der Stelle liegt ein freier Taverne-Platz. AC-04 geprüft: `data/buildings.json` tavern.vagrants.max 2 auf 5 (perDawn bleibt 1, also einer je Tag); Go-Datenbaum `testdata/rust/data-tree.json.gz` per `go test ./internal/detvec/ -write` neu geschrieben (Begründung: gewollte Wertentscheidung 5 Landstreicher). Tests: `unit/camp_guard_tests.rs` (4 grün), k3c-sim 1115 grün außer 2 bekannten roten Tests (`eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`, schon vor HB1.4 rot). Schalter `WorldValues.camp_guards` (nativ, in Go-Golden-Läufen aus), damit die Go-Parität der Kampagnen-Goldens bleibt. Abweichung: Höhlen/Minen unberührt (keine Bäume). Für Ralf im Spiel prüfen: Lage der Wächter-Bäume, Taverne-Platz sichtbar; Grafik des Platzes folgt der Taverne.
