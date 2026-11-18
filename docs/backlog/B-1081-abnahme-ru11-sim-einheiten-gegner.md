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

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
