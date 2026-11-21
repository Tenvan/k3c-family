# B-1200 · Waffenschmied, Rüstungsschmied und Bogner fertigen Ausrüstung; Soldat und Elite aus Bürger

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** FG1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Entscheidungen Ralf (2026-11-21): Bürger + Bogen = Bogenschütze; Bürger + Schwert = Soldat; Soldat + Rüstung = Elite-Soldat; Bogenschütze + Rüstung = Elite-Bogenschütze; Bürger + Werkzeuggürtel = Handwerker (ECO1 B-1193/B-1194). Bogner macht Bögen, Waffenschmied macht Schwerter (ersetzt die Schmiede), Rüstungsschmied macht Rüstungen (ersetzt die Rüstkammer, Hub-HP-Bonus entfällt), Handwerksmeister macht Werkzeuggürtel (in ECO1 „Werkzeugmacher/Werkstatt“: Name abgleichen). Genau ein Beruf je Figur, der Bürger holt Ausrüstung selbst aus dem Regal. Schnittstelle zu ECO1 B-1196 (Produktion nur ganz oben) und B-1197 (Ausrüstung von oben holen).

## Ziel

Die fünf Kombinationen laufen im Standard in der Sim (nur Rust, nativ).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Bogner, Waffenschmied, Rüstungsschmied fertigen je ihre Ausrüstung ins Regal.
- **AC-02** Bürger nimmt Ausrüstung und wird zum richtigen Beruf; Soldat und Bogenschütze nehmen Rüstung und werden Elite; Handwerker nimmt nichts mehr; Test je Kombination.
- **AC-03** Hub-HP-Bonus der Rüstkammer entfällt, Test.

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
