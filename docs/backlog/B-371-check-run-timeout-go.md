# B-371 · check_run go:test und task:check:go brechen im MCP-Aufruf mit Timeout ab

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`check_run` mit `go:test` und `task:check:go` endet im MCP-Aufruf mit „The operation timed out.“ (2026-10-08 PJ3.4, 2026-10-09 K2.4), obwohl `task check:go` in der Shell grün durchläuft (`k3c-load` ~21 s, `engine/net` ~21 s, dazu Lint). Agenten müssen dann auf die Shell ausweichen. Zusätzlich lehnen laufende k3c-dev-Instanzen die Domäne `DEV` aus DV1 noch ab (alter Build), und `console_tail` hat andere Parameter als in den Server-Anweisungen beschrieben.

## Ziel

Lange Prüfläufe liefern über `check_run` ein Ergebnis statt eines Timeouts (z. B. Lauf im Hintergrund mit Abfrage, oder Frist über der Dauer von `task check:go`).

## Beteiligte und Zielgruppen

Agenten in Review- und Umsetzungs-Sessions.

## Anforderungen

`check_run` hält lange Läufe aus oder gibt eine Lauf-ID zum Abfragen zurück; Doku von `console_tail` passt zum Schema.

## Nicht-Ziele

Go-Tests selbst beschleunigen.

## Regeln und Einschränkungen

Nur `tools/k3c-dev/`.

## Beispiele

`check_run task:check:go` nach ~90 s → `task:check:go · exit 0 · 90 s` statt Timeout.

## Ausnahme- und Fehlerfälle

Hängt ein Lauf wirklich, bricht er nach einer festen Frist mit klarer Meldung ab.

## Akzeptanzkriterien

- **AC-01** `check_run task:check:go` liefert Exit-Code und Fehlerzeilen, auch wenn der Lauf länger als 60 s dauert.
- **AC-02** Die Server-Anweisungen nennen die aktuellen Parameter von `console_tail`.

## Offene Fragen

Ob der Timeout aus dem MCP-Client oder aus k3c-dev kommt, ist ungeprüft.

## Notizen

Aus K2.4 (2026-10-09).
