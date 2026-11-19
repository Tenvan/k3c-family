# B-1122 · measure.ps1, ops:backup und Release-Workflow kennen das Rust-Paket dist/k3c-game noch nicht

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU57
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU35.2 verschiebt das Go-Paket nach `dist/k3c-game-go/` und macht `dist/k3c-game/` zum Rust-Paket; `rust:game:dist` heißt `rust:game:build` (`dist/k3c-rust` entfällt). Noch auf den alten Stand zeigen: `native/measure.ps1` (`rust:game:dist`, `..\k3c-rust`, `dist\k3c-rust`), `docs/messung/rust-ruckler.md`, `task ops:backup` (Standard `dist/k3c-game/saves`), `.github/workflows/release.yml` (RU35.3 stellte nur den Go-Pfad auf `dist/k3c-game-go` um, damit der Tag-Build nicht bricht).

## Ziel

Messskript, Backup und Release kennen die neuen Pfade; der Release liefert das Rust-Paket, sobald es weitergebbar ist (B-1121).

## Beteiligte und Zielgruppen

Entwickler.

## Anforderungen

- `measure.ps1 -Engine replay` baut mit `rust:game:build` und findet `dist\k3c-game\k3c-game.exe` (Go: `dist\k3c-game-go`).

## Nicht-Ziele

Lizenzdatei (B-1121).

## Regeln und Einschränkungen

Bereiche außerhalb RU35, darum eigenes Ticket.

- **Bereich:** `native/measure.ps1`, `docs/messung/rust-ruckler.md`, `.github/workflows/release.yml`, `scripts/**`
- **Abhängig von:** RU35
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

`task native:game:measure` und Replay-Messung finden je ihre EXE.

## Ausnahme- und Fehlerfälle

nicht relevant: Pfadkorrektur.

## Akzeptanzkriterien

- **AC-01** `grep -rn "k3c-rust\|rust:game:dist\b"` (altes Ziel, ohne `:check`) findet außerhalb erledigter Sprints und Backlog-Archiv nichts mehr (der interne Task `game:dist` und `game:dist:check` bleiben).
- **AC-02** `release.yml` zippt das Rust-Paket `dist/k3c-game/` (nach B-1121), mit Kommentar im Kopf.

## Offene Fragen

Entschieden (autonom, Claude, 2026-11-19): Spielstand-Standardordner des Rust-Pakets ist `saves/` neben der EXE (`default_saves_dir()`), also `dist/k3c-game/saves` wie im Skript `ops:backup`; Go/Rust-Spielstandformat bei „Im Spiel ansehen“ bleibt ungeprüft.

## Notizen

Aus RU35.1/RU35.2. Umgesetzt in RU57.3: `measure.ps1` (replay und rustgame nutzen dieselbe Rust-EXE), `rust-ruckler.md`, `release.yml` (Rust-Bau und Zip), Kommentar in `scripts/backup-saves.sh`.
