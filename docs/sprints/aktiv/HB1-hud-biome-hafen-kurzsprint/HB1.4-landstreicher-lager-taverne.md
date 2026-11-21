# HB1.4 · Landstreicher-Lager zwischen Bäumen, Zerstörung bei Rodung, Taverne lockt Landstreicher

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
