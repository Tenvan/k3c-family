# WE8.8 · Das Rust-Spiel kündigt Events an und zeigt Schleier (ersetzt WE8.4)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we8
- **Abhängig von:** WE8.7, WE6.6
- **Tickets:** B-995
- **Kriterien:** AC-04

## Ziel

Das Rust-Spiel kündigt Überfall, Sandsturm und Nebel an und zeigt den Schleier.

## Kontext

Spec: `B-995 › Anforderungen` (AC-01, AC-02). Die Go-Session WE8.4 ist verworfen; diese Session ersetzt sie in Rust (Entscheidung 013): Anzeige in `rust/crates/k3c-game/src/` (`hud/`, `fx/`, `world/`), Ereignisse aus WE8.7. Die Controller-Belegung bleibt unverändert. Das Spiel zeichnet nur; jede Anzeige mit 2 Spielern prüfen; `k3c-hud` vor dem Commit.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/**`
- `rust/crates/k3c-game/tests/still_*.rs`

## Nicht-Ziele

Neue Spielregeln (WE8.7). Neue Assets; fehlt eines, vorläufig zeichnen und melden.

## Schritte

1. Ankündigung im HUD je Spielerhälfte, wenn ein Event für die Nacht feststeht.
2. Schleier für Sandsturm und Nebel zeichnen (fx/world); Standbild-Test.
3. `k3c-hud` vor dem Commit.

## Fertig, wenn

- [ ] AC-04: Das Spiel kündigt die Events an und zeigt Schleier (B-995/AC-01, AC-02).
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

AC-04 umgesetzt (Commit e5761087d, #881: Ankündigung über `hud/state.rs` `new_events_key`/`event_banner` aus `night_events`, Schleier in `world/event_veil.rs`). Geprüft: `cargo test -p k3c-game --lib event_veil` 3 grün, hud::kampf grün. Keine neue Codeänderung in dieser Session. `k3c-hud`-Review entfällt (kein Diff).
