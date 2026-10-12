# B-416 · Der Checkout-Aufruf von k3c-dev öffnet bei jedem Tool ein Konsolenfenster

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-12
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (Chat, 2026-10-12): Bei den `plan_*`-Tools erscheint jedes Mal ein störendes Shell-Fenster. Ursache (im Code gefunden, am Fenster selbst nicht beobachtet): `tools/k3c-dev/internal/mcpsrv/checkout.go:85` (`worktrees()`) ruft `exec.Command("git", "-C", root, "worktree", "list", "--porcelain")` direkt auf. k3c-dev läuft als Wails-Fenster ohne Konsole; ein Kindprozess ohne `CREATE_NO_WINDOW` öffnet dann unter Windows ein Konsolenfenster. Der Aufruf läuft bei jedem Tool mit `checkout`-Argument (im Worktree Pflicht: `plan_*`, `check_run`, `svc_*`, `task_*`). Die übrigen Git-Aufrufe (`internal/planning/ticketnr.go`, `worktree.go`) nutzen schon `proc.Command` (setzt `CREATE_NO_WINDOW`).

## Ziel

Kein Tool von k3c-dev öffnet ein Konsolenfenster. Der Checkout-Aufruf nutzt `proc.Command` (oder die vorhandene `planning`-Funktion für `worktree list`) statt `exec.Command`.

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

- **AC-01** `internal/mcpsrv/checkout.go` startet `git` nur über `proc.Command`; ein Test (z. B. `grep` im Test auf `exec.Command("git"` in `internal/` ohne `_test`) verhindert den Rückfall.
- **AC-02** Beobachtung am PC (🧑): Nach dem Neubau von k3c-dev öffnet kein `plan_*`-Aufruf ein Fenster. Bis dahin: neu gebaute EXE starten (`task k3c-dev:build`).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Entdeckt im Sprint GR7. Nicht Teil von GR7 (Domäne DEV, eigene Session). Ungeprüft: ob weitere Fenster aus anderen Quellen stammen (z. B. `rundll32` in `app_claude.go:50`, nur beim Öffnen von Links).
