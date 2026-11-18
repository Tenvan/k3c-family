# B-1116 · replay/report.rs nutzt Report, FrameStats und FrameLog aus game/measure.rs

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU30
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`replay/report.rs` (RU38.3) hat eigene `Report`, `FrameStats`, `percentile` und `replay/frames.rs` ein eigenes `FrameLog`. Es fehlen das Feld `bilder` (`FrameReport`) und die Kennwerte je Bild. RU30.4 hat die vollständige Fassung in `game/measure.rs`, `game/measure_frames.rs` angelegt, `replay/` blieb unberührt (Nicht-Ziel).

## Ziel

Ein Bericht, ein Satz Rechenfunktionen: `replay/` benutzt per `pub use` die Typen und `percentile`/`round1` aus `game/measure*.rs`; die Doppelungen entfallen.

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

- **AC-01** `replay/report.rs` und `replay/frames.rs` enthalten keine eigene Fassung von `Report`, `FrameStats`, `percentile`, `FrameLog` mehr.
- **AC-02** Der Bericht der Wiedergabe trägt `bilder` mit `--measure-frames`-Kennwerten wie im Spiel.
- **AC-03** `cargo test -p k3c-game` und `replay_headless`/`replay_trace` bleiben grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
