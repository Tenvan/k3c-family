# B-1205 · Kämpfer holen die Rüstung am Gebäude ab, statt sie aus der Ferne zu bekommen

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WK1
- **Projekt:** NAC
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf-Entscheidung 2026-11-21 (Dialog), Claude trägt ein
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`armor_convert` (`rust/crates/k3c-sim/src/building/upgrades.rs`) macht mit nativen Regeln sofort den nächsten Soldaten oder Bogenschützen der Stufe zur Elite, sobald Rüstung im Regal liegt: ohne Reichweite, auch im Kampf, mit voller Heilung. Reviewer-Befund FG1.4, Entscheidung Ralf 2026-11-21.

## Ziel

Der Kämpfer holt die Rüstung selbst am Gebäude ab, wie Bogen und Schwert, und wird dabei zur Elite.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet und nimmt am PC mit Pad ab.

## Anforderungen

- **AC-01** Ein Soldat oder Bogenschütze läuft zum Rüstungsschmied und holt die Rüstung; erst beim Abholen wird er Elite.
- **AC-02** Nur außerhalb der Gefahr; im Kampf oder auf Expedition holt niemand die Rüstung.
- **AC-03** Je Rüstung höchstens ein Kämpfer unterwegs; ohne Kandidat bleibt sie im Regal.

## Nicht-Ziele

Werte der Rüstung (B-1206), Balancing.

## Regeln und Einschränkungen

Neue Mechanik nur in Rust (Entscheidung 013), deterministisch, 2 Spieler; ohne native Regeln bleibt alles wie in Go.

- **Bereich:** `rust/crates/k3c-sim/src/building/**`, `rust/crates/k3c-sim/src/unit/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Rüstung fertig, Soldat steht frei: er geht zum Gebäude, nimmt sie, wird Elite-Soldat.

## Ausnahme- und Fehlerfälle

Gegner in Sicht: der Kämpfer bleibt bei der Verteidigung, die Rüstung bleibt im Regal.

## Akzeptanzkriterien

- **AC-01** Test: Elite entsteht erst nach dem Abholen am Gebäude.
- **AC-02** Test: bei Gefahr kein Abholen.
- **AC-03** Test: ohne Kandidat bleibt die Rüstung im Regal, nie zwei Kämpfer für eine Rüstung.

## Offene Fragen

keine

## Notizen

Quelle: B-1203, FG1.4.
