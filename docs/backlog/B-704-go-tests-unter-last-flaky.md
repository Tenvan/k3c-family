# B-704 · Lastabhängige Go-Tests scheitern bei parallelen Agenten

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-11-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei vier parallelen Agenten (Welle 1 PAP, 2026-11-06) scheiterten in `task check:diff`/`check:go`/`check:dev` wiederholt Go-Tests, die einzeln grün sind: `cmd/k3c-load` `TestTickReiheJeRaum` und `TestLaufSammeltTicksUndRaeumtAuf` (zu wenige Ticks in 2 s), `internal/botdev` `TestBotGeraeteSteuernMonarchen` (Verbindung beim Warten auf `joined` verloren), `internal/mcpsrv` `TestRunProcessExitUndAusgabe` (fehlendes Programm mit 1 s Zeitlimit → Zeitüberschreitung statt Fehler). Reproduzierbar auch einzeln: `internal/simrun` `TestLaufIDNachNeustartHinterHoechstemOrdner` (TempDir-Aufräumen unter Windows: „reports: The directory is not empty“). Außerdem bricht `golangci-lint` ab, wenn eine andere Session gleichzeitig lintet („parallel golangci-lint is running“).

## Ziel

Die genannten Tests sind unter Last stabil (Fristen aus Takten statt Wanduhr, großzügigere Zeitlimits oder `-short`-Ausnahme), und `check:diff` wartet bzw. wiederholt bei parallelem golangci-lint.

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

- **AC-01** Die drei Tests laufen 10× hintereinander mit `-count=10` grün, während eine zweite `go test ./...`-Last läuft.
- **AC-02** `task check:diff` scheitert nicht mehr an „parallel golangci-lint is running“.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
