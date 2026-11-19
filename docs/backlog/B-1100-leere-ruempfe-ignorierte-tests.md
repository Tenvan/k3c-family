# B-1100 · 21 ignorierte k3c-sim-Tests haben leere Rümpfe und zählen als portiert, ohne es zu sein

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU52
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Beim Aufheben der Ignore-Tests in RU18.4 (`cargo test -p k3c-sim -- --ignored`) liefen 21 Tests grün, deren Rumpf leer ist (`fn x() {}`): 7 in `crown/tests.rs`, 6 in `portal/tests.rs`, 7 in `state/views_tests.rs` bzw. `state/save_tests.rs`, 1 weiterer. Sie tragen `ignore` mit dem Grund eines späteren Sprints (RU10, RU18, RU19, RU21), prüfen aber nichts. Die Go-Tests dahinter (z. B. `TestCrownKronenraubIstGameOver`) sind damit nicht portiert, obwohl die Zeile im Ergebnis der Port-Session sie als „wartet“ führt.

## Ziel

Jeder leere Rumpf wird mit dem Go-Test gefüllt (Leitfaden § 6: jeder Go-Test wird portiert) oder, wo der Test noch nicht laufen kann, als Platzhalter mit `unimplemented!` und Sprint-Nummer geführt, damit er beim Aufheben des Ignore nicht grün wird.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Liste der Tests: `cargo test -p k3c-sim -- --ignored` und je grünem Test den Rumpf prüfen (leer = Fund).
- Jeder Fund: Go-Test portieren oder `unimplemented!("RUnn: ...")` einsetzen; `ignore`-Grund auf die genaue Voraussetzung setzen.
- RU22 (Abnahme) prüft, dass kein ignorierter Test einen leeren Rumpf hat.

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

- **AC-01** Kein Test in `k3c-sim` hat einen leeren Rumpf; `grep -rn "() {}" rust/crates/k3c-sim/src --include=*tests.rs` findet nichts.
- **AC-02** Jeder bisher leere Test ist portiert oder bricht beim Ausführen mit dem liefernden Sprint ab.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
