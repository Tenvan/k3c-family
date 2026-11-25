# B-1086 · Abnahme-Läufe Sim Insel

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** BAL11
- **Projekt:** BAL
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

**Stand 2026-11-18 (RU47)** (Second Worker; Details im Ergebnis RU47.1 und RU47.2, Sprint RU47)

- Rust: `cargo test --release -p k3c-sim` 384 grün, 0 rot, 296 ignoriert.
- Marken: `island` 103 ignoriert (RU19 91, RU18 11, RU15 2), `boss` 23 (RU10 21, RU19 2). Die Marke RU10 steht nur noch an `boss`, `enemies` und `combat` (insgesamt 48 Tests nur mit RU10, 53 mit RU10 überhaupt).
- `cargo test -- --ignored`: `island` 0 grün / 103 rot (Platzhalter `decode_world_story` RU19 und `new_hub_site` RU18), `boss` 0 / 23 rot (Helfer `boss_island`, `deep_island`; vermutlich echte Abhängigkeit RU18, weil `create_island` `create_world` und `new_hub_site` braucht).
- Echter Befund: `island::worlds_tests::eingebettete_welten_gueltig` schlägt fehl, weil die eingebetteten Welten `rom.json` und `standard.json` sind, der Test aber nur `standard.json` erwartet (Test veraltet).
- AC-01 Sim und AC-02 Balance: bis RU18/RU19/RU22 nicht möglich. AC-03: `task go:test` grün, `testdata/` unverändert (RU47.2), `gen:paper` grün (RU45.1). AC-04: `check:race` grün, 749 s (RU45.1).
- Status bleibt `offen`.
