# B-405 · Der vollständige Go-Test von tools/k3c-dev läuft ohne Hänger durch

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beobachtet in WZ3.1/WZ3.2 (2026-10-11): `go test ./...` in `tools/k3c-dev` läuft nicht durch; `internal/balance` (`balance.test.exe`) hing über mehrere Minuten, die Läufe wurden gestoppt. `planning` und `mcpsrv` waren einzeln grün. Ob der Hänger mit WZ3 zusammenhängt, ist nicht belegt. Der Katalog von `check_run` erfasst die Pakete unter `tools/k3c-dev` nicht.

## Ziel

Der hängende Test ist benannt und behoben, und `go test ./...` in `tools/k3c-dev` läuft über `check_run` vollständig durch.

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
