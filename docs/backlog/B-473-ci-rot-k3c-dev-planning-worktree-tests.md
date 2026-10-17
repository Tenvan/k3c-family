# B-473 · CI auf develop rot: k3c-dev internal/planning Worktree-Tests scheitern auf dem Windows-Runner

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der CI-Job „k3c-dev · Frontend · Tests · Lint · Wails (Windows)“ ist auf `develop` rot (Läufe zu `80311389`, `9583f02d`, `ba0c3136`). `go test ./...` in `tools/k3c-dev/internal/planning` scheitert: `TestTicketNummerAusAnderemWorktree`, `TestTicketNummerWorktreeNichtLesbar`, `TestTicketNummerAusRemoteRef` (Antwort „Warnung: Ordner ist keine Repo-Wurzel, Worktrees und Refs nicht gelesen“, `numbers_test.go`), `TestCheckWorktreesMergeBase`, `TestCheckWorktreesArchiviertesTicket` (`wtdiff_test.go`: Worktree-Änderungen vorhanden, Erwartung nicht erfüllt). Alle übrigen Jobs sind grün. Ursache offen; vermutlich Repo-Wurzel-Erkennung oder Git-Verhalten im Temp-Ordner auf dem Runner. Blockiert den Release-Tag v0.19.0 (Checkliste „Release“).

## Ziel

CI auf `develop` grün; Ursache der fünf Testfehler behoben, ohne die Tests abzuschwächen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
