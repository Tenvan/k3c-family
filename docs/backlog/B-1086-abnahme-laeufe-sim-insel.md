# B-1086 · Abnahme-Läufe Sim Insel

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU10
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU10 (Sim Insel nach Rust) lief ohne Sim-, Balance-, Gen- und Race-Läufe (Sperre vom 2026-11-11, B-742). Lane: zweit (Läufe auf dem Second Worker).

## Ziel

Die gesperrten Läufe zum Sprint RU10 nachholen und das Ergebnis eintragen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Läufe nur über `sim_test`, `check_run` und `gen:paper`, nie in der Shell.

## Nicht-Ziele

Neue Funktionen, Regeländerungen.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Sim, Balance und Gen; Golden werden nie zugunsten von Rust neu geschrieben.

- **Bereich:** `rust/crates/k3c-sim/src/island/**`
- **Abhängig von:** RU10
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: reiner Abnahme-Lauf.

## Ausnahme- und Fehlerfälle

Schlägt ein Lauf fehl, entsteht ein Ticket je Ursache, kein geändertes Golden.

## Akzeptanzkriterien

- **AC-01** Sim: `sim_test` für die Insel-Sim ist ohne Abweichung zu Go durchgelaufen.
- **AC-02** Balance: Balancing-Lauf der Insel ohne Abweichung.
- **AC-03** Gen: `check_run task:test` samt `gen:paper` und Golden grün.
- **AC-04** Race: `check_run task:check:race` grün.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
