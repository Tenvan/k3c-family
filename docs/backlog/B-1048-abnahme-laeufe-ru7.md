# B-1048 · Abnahme-Läufe RU7: Grafik laden samt gerenderter Grafik und Race sind grün

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU7
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review RU7.7 (2026-11-17): `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `task check` und `task check:go` grün. Gerenderte Grafik (`gen:paper`) und Race laufen nach B-742 nur hier.

## Ziel

Die Abnahme-Läufe des Sprints RU7 sind gelaufen, Abweichungen als Tickets erfasst.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Sim und Balance: entfällt bis zur Aufhebung der Sperre (Beschluss 🧑 2026-11-11).
- **AC-02** Balance: entfällt wie AC-01.
- **AC-03** `gen:paper`, danach `cargo test -p k3c-game` mit `K3C_GRAFIK_DIR` auf die gerenderte Grafik: alle Gruppen aus `public/grafik/<gruppe>/index.paper.json` laden über `art::Library`, Grafik-Golden stimmt.
- **AC-04** `check_run task:check:race` grün (ohne gcc: als übersprungen nennen).

## Nicht-Ziele

Änderungen am Code von RU7.

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

- **AC-01** Sim und Balance: entfällt, Sperre vom 2026-11-11.
- **AC-02** Balance: entfällt, Sperre vom 2026-11-11.
- **AC-03** `gen:paper` plus `cargo test -p k3c-game` mit gerenderter Grafik grün.
- **AC-04** `check_run task:check:race` grün oder als übersprungen genannt.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Lane `zweit`: Der Second Worker fährt dieses Ticket (Sim, Gen und Race sind auf dem Hauptrechner gesperrt).
