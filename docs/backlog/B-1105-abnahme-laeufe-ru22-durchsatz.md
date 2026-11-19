# B-1105 · Abnahme-Läufe RU22: Durchsatz Rust gegen Go, Sim und Race (Lane: zweit)

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU51
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Lane: zweit (der Second Worker fährt dieses Ticket, nicht der Hauptrechner). RU22 hat die Golden-Abgleiche als `cargo test` umgesetzt (alle 15 gleich). Der Durchsatz Rust gegen Go und die Sim-/Race-Läufe sind wegen der Sim-Sperre und des belegten Hauptrechners nicht gefahren.

## Ziel

Durchsatz von Rust und Go am selben Lauf (`sim-forest-aufbau`) messen und in `docs/messung/rust-sim-abgleich.md` (Tabelle Durchsatz) eintragen; Sim-Prüfläufe und Race einmal fahren.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Beide Messungen am selben ruhigen Rechner nacheinander, ohne parallele Builds; Median und Spanne, Rechner, Datum.

- **Bereich:** `docs/messung/rust-sim-abgleich.md`
- **Abhängig von:** RU22
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

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

- **AC-01** Sim: `go test ./engine/sim/ -run TestGoldenDurchsatz -durchsatz -count 5 -v` und `cargo test --release -p k3c-sim --test golden_durchsatz -- --ignored --nocapture`; Schritte/s Median und Spanne im G3-Bericht.
- **AC-02** Balance: nicht relevant (RU22 ändert keine Werte).
- **AC-03** Gen: nicht relevant (kein Grafik-Golden berührt); `check_run task:test` einmal grün.
- **AC-04** Race: `check_run task:check:race` (lokal ohne gcc als übersprungen nennen).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
