# B-1108 · Abnahme Smoke-Version: Bildvergleich mit Grafik rot, Speicher-Rundlauf und veraltete Ignore-Marken (Lane zweit)

- **Domäne:** CLI
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

Lane: zweit (Abnahme-Läufe sind auf dem Hauptrechner gesperrt). Mit gerenderter Grafik (`task gen:paper`) schlagen in `k3c-game` `tests/still.rs` (bildvergleich_an_fuenf_staenden, lichtkarte_fehlt_in_rust) und `tests/still_hud.rs` (plaetze_stimmen_mit_go_ueberein, bilder_stimmen_mit_go_ueberein) an, ohne Grafik werden sie übersprungen (vor RU48.3 nicht von dieser Sprint-Änderung berührt, ungeprüft ob schon auf main rot). Ferner: in `k3c-sim` bleiben ca. 90 `#[ignore]`-Tests; die Marken nennen RU10/RU15/RU18/RU19, ihre Ursachen sind Test-Hilfen (B-1103) oder echte Lücken, darunter der Rundlauf Speichern, Laden, Speichern (`island::save_tests::island_save_round_trip`, `save_v3_tests`, `save_boss_tests`: Ergebnis weicht ab).

## Ziel

Auf dem zweiten Rechner: Bildvergleich mit Grafik prüfen und Schwellen oder Rust-Zeichner korrigieren; je ignoriertem Test Ursache klären, Marke aktualisieren oder aufheben; den Rundlauf Speichern/Laden beheben (Weiter-Funktion im Menü hängt daran). Sim-, Balance- und Gen-Läufe nur hier.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
