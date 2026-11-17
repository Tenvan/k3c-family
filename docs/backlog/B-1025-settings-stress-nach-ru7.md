# B-1025 · Rust-Settings fehlt das Feld Stress, bis RU7.4 stress.go portiert

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

RU4.5 hat `native/k3c-game/view/settings.go` nach `rust/crates/k3c-game/src/view/settings.rs` portiert. Das Feld `Stress StressTest` (Typ aus `view/stress.go`) fehlt dort, weil `stress.go` erst RU7.4 portiert und `stress.rs` nicht im Bereich von RU4 liegt.

## Ziel

`Settings` in Rust hat das Feld `stress` wie Go: `default_settings()` setzt `DefaultStress()`, `normalized()` setzt einen Null-Wert auf `DefaultStress()` und wendet `StressTest::normalized()` an; `settings_tests.rs` erwartet wie Go `want.Stress = DefaultStress()`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Port ohne Regeländerung (B-1014). Reihenfolge der Felder wie in Go (nach `dev_mode`).

## Nicht-Ziele

Änderungen am Go-Code.

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

- **AC-01** `Settings` hat `stress: StressTest`; `default_settings()` und `normalized()` behandeln es wie `settings.go`.
- **AC-02** `settings_normalized` prüft `stress` wie `TestSettingsNormalized`; die Hinweise auf B-1025 in `settings.rs` und `settings_tests.rs` sind entfernt.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aufgefallen in RU4.5. Gehört in RU7.4 (nach `stress.rs`); `settings.rs` liegt im Bereich von RU4, RU7.4 braucht die Datei dafür in seinen Erlaubten Dateien.
