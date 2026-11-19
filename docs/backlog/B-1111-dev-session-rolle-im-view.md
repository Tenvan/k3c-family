# B-1111 · view::Session liefert keine Entwickler-Rolle (DevSession), Cheats im Rust-Spiel wirkungslos

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
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

In Go liefert die Session ihre Entwickler-Rolle (`DevSession`), `cheats.go` ruft damit die Cheat-Aktionen. In Rust hat `view::session::Session` keinen Zugang; `devtools::pending::dev_of` liefert `None`, das Cheat-Menü öffnet (F9, Pad View+Y), meldet beim Auslösen aber „Cheats gehen nur im lokalen Spiel“. Außerdem wirken F1 (Hilfe) und F10 (Debug) erst, wenn `game/devkeys` (RU30.3) `Frame::dev` setzt. Gefunden in RU29.1/RU29.3.

## Ziel

`Session` bietet den Zugang zur Entwickler-Rolle, `dev_of` nutzt ihn und `pending.rs` in `devtools/` entfällt; Cheats wirken im lokalen Rust-Spiel. Sinnvoll zusammen mit RU30.3.

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

- **AC-01** Ein ausgelöster Cheat im lokalen Rust-Spiel erreicht `DevSession::dev(player, action)` (Test mit lokaler Session).
- **AC-02** `devtools/pending.rs` ist gelöscht.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
