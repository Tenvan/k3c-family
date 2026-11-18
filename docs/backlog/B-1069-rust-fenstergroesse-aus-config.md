# B-1069 · k3c-platform öffnet das Fenster fest mit 1280x720, --size der Wiedergabe wirkt nicht

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU30
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`k3c_platform::run_window` öffnet das Fenster fest mit `LogicalSize(1280, 720)` und `PresentMode::Fifo` (`rust/crates/k3c-platform/src/backend/mod.rs`, `gpu.rs`); `Config` hat weder Größe noch VSync. Die Wiedergabe (RU39.1) liest `--size BxH` und schreibt den Wert nur in den Bericht (`fenster`); ohne VSync lässt sich Rust nicht messen.

## Ziel

Fenstergröße und VSync kommen aus `Config`, `k3c-game` (Start, Wiedergabe) gibt `--size` und `--novsync` durch wie das Go-Spiel.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Port ohne Regeländerung (B-1014).

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

- **AC-01** `k3c-game replay <datei> --size 1920x1080` öffnet ein Fenster mit 1920x1080 (logisch); der Bericht nennt dieselbe Größe.
- **AC-02** Ein Schalter ohne VSync (`--novsync`) setzt einen Present-Modus ohne Warten; `measure.ps1 -Engine replay` ohne `-Vsync` gibt ihn weiter.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Gefunden in RU39.1 (Messwerkzeug). Für G2 unkritisch: Messanleitung nimmt `-Size 1280x720` für beide Programme und `-Vsync`.
