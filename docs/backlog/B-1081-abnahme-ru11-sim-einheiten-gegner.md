# B-1081 · Abnahme RU11: Sim, Balance und Gen für Einheiten und Gegner in Rust

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU11
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU11 hat Einheiten und Gegner nach Rust portiert (`rust/crates/k3c-sim/src/{unit,enemies}`). Die Umsetzungs-Sessions liefen ohne Sim-, Balance- und Gen-Läufe (Beschluss 2026-11-11, B-742).

## Ziel

Die Läufe für Sim, Balance und Gen sind einmal gesammelt gemacht, sobald RU14 (Welt und Takt) und RU22 stehen: Go und Rust rechnen für Einheiten und Gegner gleich.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Die ignorierten Tests laufen und sind grün: Marken RU10 (12), RU14 (40), RU15 (2), RU18 (11), RU19 (3), RU21 (3), `lang` (1). Danach der Bitgleich-Vergleich Go gegen Rust für Wellen, Zielwahl, Wachen und Truppen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Alle `#[ignore]` aus `unit/` und `enemies/` sind entfernt oder haben eine offene Sprint-Nummer; `cargo test -p k3c-sim` grün.
- **AC-02** Der Vergleichslauf Go gegen Rust zeigt für Einheiten und Gegner keine Abweichung (Ganzzahlen gleich, Floats 1e-4).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Lane `zweit`: Der Second Worker fährt dieses Ticket (Sim, Balance und Gen).

**Stand 2026-11-18 (RU47)** (Second Worker; Details im Ergebnis RU47.1 und RU47.2, Sprint RU47)
- Rust: `cargo test --release -p k3c-sim` 384 grün, 0 rot, 296 ignoriert; ganzer Workspace 1381 grün, 326 ignoriert, 4 rot (nur `still.rs`/`still_hud.rs`, nicht Sim).
- Marken in `unit/` und `enemies/`: `unit` 30 ignoriert (RU18 25, RU19 2, RU10 1, RU15 1), `enemies` 31 ignoriert (RU18 18, RU10 12, RU19 1). Die RU10-Zahl 12 aus der Anforderung stimmt für `enemies`; RU14 (40) ist als Marke weg, RU15/RU18/RU19/RU21 sind anders verteilt als hier genannt (siehe B-1095).
- `cargo test -- --ignored`: `unit` 1 grün (`unit::hammer_tests::gegner_traegt_den_ersatzhammer_nicht_weg`, Marke RU19, Kandidat zum Freischalten), 29 rot; `enemies` 0 grün, 31 rot. Ursachen: Platzhalter RU18 (`new_hub_site`, `step_players`, `check_presets`), RU19 (`unit_x`, `emit`) und Insel-Helfer in `enemies/testutil.rs` (`ruin_island`, `island_with`, `end_island`; Marke RU10, vermutlich echte Abhängigkeit RU18 wegen `create_island`).
- Go-Seite: `task go:test` grün (21 Pakete, 99 s), `testdata/` unverändert (RU47.2).
- AC-01 noch offen (an RU18, RU19 hängend), AC-02 (Bitgleich-Vergleich Go gegen Rust) wartet auf RU18/RU19/RU22. Status bleibt `offen`.
