# B-1050 · Abnahme RU8: Race- und Golden-Läufe des Rust-Sim-Zustands stehen aus

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU8
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU8 hat den Sim-Zustand und die Typen in Rust gebaut (`rust/crates/k3c-sim/src/state/`). Sim-, Balance-, Gen- und Race-Läufe sind gesperrt und bei der Abnahme von RU8 nicht gelaufen (Regel B-742).

## Ziel

Race- und Golden-Läufe des Sprints RU8 sind einmal grün gelaufen, sobald die Sperre aufgehoben ist.

## Beteiligte und Zielgruppen

Entwickler (Claude); 🧑 hebt die Sim-Sperre auf.

## Anforderungen

- `task check:race` und `cargo test -p k3c-sim` mit Golden-Rundlauf (`tests/golden_state.rs`) grün.
- Kein Golden wird neu geschrieben.

## Nicht-Ziele

Neue Funktionen; Balancing.

## Regeln und Einschränkungen

Sim-Sperre vom 2026-11-11; `docs/arbeitsweise.md` › Sim, Balance und Gen.

## Beispiele

nicht relevant: reiner Prüflauf.

## Ausnahme- und Fehlerfälle

Weicht Rust vom Golden ab, gilt Go; Abweichung wird Ticket.

## Akzeptanzkriterien

- **AC-01** `task check:race` grün (oder als ohne C-Compiler übersprungen genannt).
- **AC-02** `cargo test -p k3c-sim --test golden_state` grün, kein Golden neu geschrieben.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
