# B-702 · TestRunProcessExitUndAusgabe wackelt: fehlendes Programm ohne Fehler

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`TestRunProcessExitUndAusgabe` (`tools/k3c-workbench/internal/mcpsrv/check_test.go:216`) schlägt sporadisch fehl: `runProcess` mit dem Programm `gibt-es-nicht-k3c` liefert `err == nil`. Beobachtet in `task check:diff` (MC1.1, 2026-11-06) und einzeln: 1 von 3 Läufen rot (`go test ./internal/mcpsrv/ -run TestRunProcessExitUndAusgabe -count=1`). Code von MC1 ist nicht beteiligt.

## Ziel

Der Test ist stabil; ein fehlendes Programm meldet `runProcess` immer als Fehler.

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

- **AC-01** 20 Läufe `go test ./internal/mcpsrv/ -run TestRunProcessExitUndAusgabe -count=20` grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Vermutet (ungeprüft): Start-Fehler wird bei Zeitlimit 1 s mit einem Exit-Code verwechselt oder der Fehler geht im Job-Object-Pfad (`internal/proc`) verloren.
