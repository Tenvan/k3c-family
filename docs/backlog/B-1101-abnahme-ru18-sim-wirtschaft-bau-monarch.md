# B-1101 · Abnahme RU18: Sim, Balance, Gen, Race und Golden-Abgleich der neuen Rust-Sim-Teile

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU18
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU18 hat Wirtschaft, Bau, Hub, Händler, Monarch und Skills nach Rust portiert (`rust/crates/k3c-sim/src/{economy,building,hub,merchant,monarch,skills}`). Sim-, Balance-, Gen-Läufe, Race und Golden-Abgleich liefen in den Sessions nicht (Sim-Sperre 2026-11-11, B-742). Lane: zweit (der Lauf gehört auf den zweiten Rechner, falls dieser Abnahmen fährt).

## Ziel

RU18 ist ohne offene Rechenabweichung gegenüber Go abgenommen.

## Beteiligte und Zielgruppen

Agenten führen die Läufe aus, 🧑 entscheidet über Abweichungen.

## Anforderungen

- Port ohne Regeländerung; deterministisch; zwei Spieler gleichzeitig.
- Golden werden nie zugunsten von Rust neu geschrieben.

## Nicht-Ziele

Neue Funktionen, Regeländerungen, Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Sim-Sperre vom 2026-11-11 gilt, bis 🧑 sie für diese Abnahme aufhebt. Entscheidung 012 (Port-Orakel), `docs/standards/rust-port.md`.

- **Bereich:** `rust/crates/k3c-sim/src/economy/**`, `rust/crates/k3c-sim/src/building/**`, `rust/crates/k3c-sim/src/hub/**`, `rust/crates/k3c-sim/src/monarch/**`, `rust/crates/k3c-sim/src/skills/**`, `rust/crates/k3c-sim/src/merchant/**`
- **Abhängig von:** RU18
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

nicht relevant: Abnahme-Ticket, die Kriterien sind die Beispiele.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Abweichung beheben oder als Ticket erfassen, nie ein Golden ändern.

## Akzeptanzkriterien

- **AC-01** Sim: `sim_test` läuft ohne Abweichung (nach Aufhebung der Sperre).
- **AC-02** Balance: Balancing-Lauf liegt in den Zielkorridoren.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Golden grün.
- **AC-04** Race: `check_run task:check:race` grün.
- **AC-05** Golden-Abgleich: Die Rust-Sim-Teile von RU18 (Wirtschaft, Bau, Hub, Händler, Monarch, Skills) liefern gegen `testdata/golden` gleiche Ergebnisse (Ganzzahlen und Ereignisse gleich, Fließkomma 1e-4 relativ); die mit `ignore = "wartet auf RU19"` markierten Tests laufen nach RU19 mit.

## Offene Fragen

keine

## Notizen

Review RU18.6: keine schweren Befunde; Sprint-Nummern der Platzhalter korrigiert (siehe Session-Ergebnis).
