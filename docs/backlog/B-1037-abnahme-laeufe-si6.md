# B-1037 · Abnahme-Läufe SI6: Sim, Balance, Gen samt Golden und Race sind grün

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** SI6
- **Projekt:** KMP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review SI6.6 (2026-11-17): `task:check` grün (ein EPERM-Rename im Temp-Ordner bei `render.test.ts`, Einzellauf grün), `task:check:go` go-test-Teil grün, golangci-lint lokal nicht installiert. Sim, Balance, Gen und Race laufen nach B-742 nur hier. SI6.3 (Engine), SI6.4 (Anzeige) und SI6.5 (Festmusik) sind verworfen (Go-Stopp, Rust-Umstellung); die Läufe decken daher nur die Fest-Assets ab.

## Ziel

Die Abnahme-Läufe des Sprints SI6 sind gelaufen, Abweichungen als Tickets erfasst.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** `gen:paper` rendert `fest-laterne`, `fest-bluete` und `fest-fackel` ohne Fehler, Grafik-Golden stimmt.
- **AC-02** `check_run task:check:race` grün (ohne gcc: als übersprungen nennen).
- **AC-03** Sim und Balance: entfällt bis zur Aufhebung der Sperre (Beschluss 🧑 2026-11-11); danach `sim_test`-Lauf ohne Abweichung zur Baseline. Hinweis: Neubewertung des Korridors „Schiff fertig“ nach der friedlichen Insel (`zielkorridore.md`).

## Nicht-Ziele

Engine, Anzeige und Festmusik des Fests (SI6.3 bis SI6.5 verworfen).

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

–
