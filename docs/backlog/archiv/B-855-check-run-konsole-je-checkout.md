# B-855 · console_tail und check_run trennen die Konsolenpuffer je Checkout

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ49
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

WZ48.4: `console_tail check:task:check:go` zeigte aus einem Worktree vermutlich den Lauf eines anderen Checkouts (`go test ./...` mit Sim-Paketen, gleiche Zeiten bei zwei Abrufen). So lässt sich ein Prüflauf nicht sicher nachweisen.

## Ziel

Jeder Checkout hat eigene Konsolenpuffer für `check_run`; `console_tail` liefert nur Läufe des eigenen Checkouts.

## Beteiligte und Zielgruppen

Agenten in Worktrees; Claude (Umsetzung).

## Anforderungen

- Puffername oder Schlüssel enthält den Checkout.
- `console_tail` nimmt `checkout` an, Standard wie `check_run`.
- Kopfzeile nennt Checkout und Startzeit des Laufs.

## Nicht-Ziele

Dienste-Konsolen (Vite, Server).

## Regeln und Einschränkungen

Klasse A (Problem, DEV).

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/check_*.go`, `tools/k3c-workbench/internal/mcpsrv/tools_console*.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Zwei Checkouts fahren `task:check:go` → `console_tail` je Checkout zeigt nur den eigenen Lauf.

## Ausnahme- und Fehlerfälle

Kein Lauf im Checkout → klare Meldung statt fremder Ausgabe.

## Akzeptanzkriterien

- **AC-01** Test: zwei Checkouts, getrennte Puffer, `console_tail` mit `checkout` liefert den eigenen Lauf.

## Offene Fragen

keine

## Notizen

Befund WZ48.4 (2026-11-14).
