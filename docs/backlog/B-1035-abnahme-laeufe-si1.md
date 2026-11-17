# B-1035 · Abnahme-Läufe SI1: Sim, Balance, Gen samt Golden und Race sind grün

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** SI1
- **Projekt:** KMP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review SI1.5 (2026-11-17): `task:check:go` (go test) ist grün, nachdem der Detvec-Baum für die neuen Einträge in `data/paper.json` neu geschrieben wurde. `task:check` ist grün bis auf `projectRules`, der an lokalen Resten scheitert (`engine/net/.omc`, `tools/k3c-starter`). Sim, Balance, Gen und Race laufen nach B-742 nur hier. SI1.2 (Engine) und SI1.4 (Anzeige) sind verworfen, die Läufe decken daher nur die Assets ab.

## Ziel

Die Abnahme-Läufe des Sprints SI1 sind gelaufen, Abweichungen als Tickets erfasst.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** `gen:paper` rendert `ladeplatz-muenzen`, `ladeplatz-truppen`, `ladeplatz-material`, `fracht-beutel`, `fracht-kiste` und `schiff` (4 Stufen) ohne Fehler, Grafik-Golden stimmt.
- **AC-02** `check_run task:check:race` grün (ohne gcc: als übersprungen nennen).
- **AC-03** Sim und Balance: entfällt bis zur Aufhebung der Sperre (Beschluss 🧑 2026-11-11); danach `sim_test`-Lauf ohne Abweichung zur Baseline.

## Nicht-Ziele

Engine und Anzeige der Fracht (SI1.2/SI1.4 verworfen).

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

- **AC-01** `check_run task:test` samt `gen:paper` grün, Grafik-Golden der Gruppen `objekte` und `gebaeude-paper` stimmt.
- **AC-02** `check_run task:check:race` grün oder als übersprungen genannt.
- **AC-03** Sim und Balance entfallen, solange die Sperre gilt.

## Offene Fragen

keine

## Notizen

–
