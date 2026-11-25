# B-1027 · Abnahme-Läufe RU3

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** BAL11
- **Projekt:** BAL
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Abnahme-Ticket des Sprints RU3 (Regel B-742): `k3c-rng`, `k3c-det` (pow, exp, log, fmax, fmin, golden) und `k3c-data` sind portiert; Go-Code unter `engine/sim`, `engine/level`, `engine/rng` und `testdata/golden` ist unverändert, neu ist nur der Test-Ordner `engine/internal/detvec`.

## Ziel

Die Läufe, die RU3 braucht, sind grün oder mit Grund verschoben.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

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

- **AC-01** Sim: verschoben (Sperre 2026-11-11); RU3 ändert keine Sim.
- **AC-02** Balance: verschoben (Sperre 2026-11-11); RU3 ändert keine Werte.
- **AC-03** Gen (`gen:paper`, Golden): nicht relevant, RU3 ändert keine Grafik.
- **AC-04** `check_run task:check:race` ist grün (ohne C-Compiler übersprungen, dann so vermerken).
- **AC-05** `task rust:test` ist auf einem zweiten Rechner mit AVX/FMA grün (Vektoren aus `engine/internal/detvec` und `cargo test -p k3c-det` bitgleich, B-1023).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Lane `zweit`: Der Second Worker fährt dieses Ticket (Sim, Gen und Race sind auf dem Hauptrechner gesperrt).
