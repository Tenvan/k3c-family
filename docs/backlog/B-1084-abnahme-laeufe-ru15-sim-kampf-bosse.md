# B-1084 · Abnahme-Läufe RU15: Sim, Balance, Gen und Race für Kampf, Bosse, Krone, Portale in Rust

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU15
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

**Lane: zweit** (Second Worker fährt die Läufe; der Hauptrechner führt keine Sim-, Balancing-, Gen- oder Race-Läufe aus). RU15 hat Kampf, Bosse, Krone und Portale von Go nach Rust (`rust/crates/k3c-sim/src/{combat,boss,crown,portal}`) portiert. Viele Tests tragen `#[ignore]` (wartet auf RU10, RU14, RU18, RU19), weil `Step`, Inseln, `emit` und `applyDamage` noch fehlen.

## Ziel

Die Abnahme-Läufe für RU15 sind gelaufen und belegen gleiches Verhalten wie Go.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Sim: nach RU14/RU10 die ignorierten Tests aus `combat`, `boss`, `crown`, `portal` laufen lassen (`cargo test -p k3c-sim -- --ignored`), jede `#[ignore]`-Marke entfernen, wenn grün.
- **AC-02** Balance: Sim-Läufe Kampf und Bosse gegen Go (Toleranz Sim 1e-4, Entscheidung 012).
- **AC-03** Gen samt Golden: `testdata/golden/` bleibt unverändert und stimmt mit Rust überein.
- **AC-04** Race: `check:race`.
- Hinweis an den Koordinator: `cause_test.go` (3 Tests) und `events_moon_stages_test.go` (5 Tests) haben keine Session in der RU15-Tabelle.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
