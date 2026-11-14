# B-704 · Lastabhängige Go-Tests scheitern bei parallelen Agenten

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ42
- **Projekt:** WZG
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Bei vier parallelen Agenten (Welle 1 PAP, 2026-11-06) scheiterten in `task check:diff`/`check:go`/`check:dev` wiederholt Go-Tests, die einzeln grün sind: `cmd/k3c-load` `TestTickReiheJeRaum` und `TestLaufSammeltTicksUndRaeumtAuf` (zu wenige Ticks in 2 s), `internal/botdev` `TestBotGeraeteSteuernMonarchen` (Verbindung beim Warten auf `joined` verloren), `internal/mcpsrv` `TestRunProcessExitUndAusgabe` (fehlendes Programm mit 1 s Zeitlimit → Zeitüberschreitung statt Fehler). Reproduzierbar auch einzeln: `internal/simrun` `TestLaufIDNachNeustartHinterHoechstemOrdner` (TempDir-Aufräumen unter Windows: „reports: The directory is not empty“). Außerdem bricht `golangci-lint` ab, wenn eine andere Session gleichzeitig lintet („parallel golangci-lint is running“).

## Ziel

Die genannten Tests sind unter Last stabil (Fristen aus Takten statt Wanduhr, großzügigere Zeitlimits oder `-short`-Ausnahme), und `check:diff` wartet bzw. wiederholt bei parallelem golangci-lint.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Die genannten Tests (`cmd/k3c-load` `TestTickReiheJeRaum`, `TestLaufSammeltTicksUndRaeumtAuf`; `internal/botdev` `TestBotGeraeteSteuernMonarchen`; `internal/mcpsrv` `TestRunProcessExitUndAusgabe`; `internal/simrun` `TestLaufIDNachNeustartHinterHoechstemOrdner`) sind unter Last stabil: Fristen aus Takten statt Wanduhr, großzügigere Zeitlimits oder `testing.Short()`-Skip mit Grund.
- Kein Sleep als Behebung; kein Test wird gelöscht, solange sein Paket bleibt.
- `check:diff` scheitert nicht an parallelem `golangci-lint` (Prüf-Slot aus B-739 belegen).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Domäne DEV, Klasse A (Problem). Entscheidung 009: Online- und Lasttests entfallen mit B-823; bis dahin nur stabil halten (Default: `testing.Short()`-Skip für Online- und Lasttests).

- **Bereich:** `cmd/k3c-load/**`, `tools/k3c-workbench/internal/botdev/**`, `tools/k3c-workbench/internal/mcpsrv/check_run*.go`, `tools/k3c-workbench/internal/mcpsrv/check_test.go`, `tools/k3c-workbench/internal/simrun/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die drei Tests laufen 10× hintereinander mit `-count=10` grün, während eine zweite `go test ./...`-Last läuft.
- **AC-02** `task check:diff` scheitert nicht mehr an „parallel golangci-lint is running“.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
