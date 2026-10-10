# B-386 · Schreibende k3c-dev-Tools nehmen den Ziel-Checkout als Argument, weil der Client aus Desktop-App-Worktrees die Wurzel meldet

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WT1
- **Projekt:** WZG
- **Erstellt:** 2026-10-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-10, 🧑 im Chat, Revision 1 (mit Sprint WT1; checkout nimmt Ordnername und Pfad)

## Ausgangslage

B-275 ist erledigt (M9): k3c-dev nennt bei schreibenden Tools den Checkout. Die Ursache blieb offen (B-341), der Fehler tritt weiter auf. Gemessen am 2026-10-10 im Log `logs/k3c-dev.jsonl`: alle 356 `ns=mcp`-Zeilen seit Einführung der Felder haben `header=true` und `checkout=Repo-Wurzel`, keine einzige ein `Worktree …`. Um 16:33 legte eine Session aus dem Worktree `sprint-br1-fortsetzung-1b4aa6` per `plan_create` drei Tickets an; die Dateien landeten in der Wurzel (`header=true`, `checkout=Repo-Wurzel`) und mussten von Hand in den Worktree verschoben werden; das folgende `plan_set` meldete „nicht gefunden“. Der `headersHelper` aus `.mcp.json` liefert im Worktree den Worktree-Pfad (`node -p …` dort geprüft). Der Client schickt den Header also, aber mit dem Pfad der Wurzel: Er führt den Helper im Projektordner der Wurzel aus (V1 aus B-341).

## Ziel

Eine Session in einem Worktree kann schreibende Tools von k3c-dev unabhängig davon auf ihren Checkout richten, was der Client als `X-K3C-Root` schickt.

## Beteiligte und Zielgruppen

Agenten-Sessions in Worktrees der Desktop-App, 🧑 (räumt heute Dateien in der Wurzel auf).

## Anforderungen

- Die schreibenden Tools (`plan_create`, `plan_set`, `plan_section`, `plan_delete`, `svc_*`, `task_start`, `task_stop`) und `check_run` nehmen ein optionales Argument `checkout`: Ordnername oder Pfad eines Worktrees dieses Repos.
- Ist `checkout` gesetzt, gilt es vor dem Header; `workspaceOf` prüft es wie den Header (nur Repo-Wurzel oder ein Worktree von ihr, sonst Fehler mit Wertebereich).
- Ohne `checkout` bleibt alles wie heute (Header, sonst Repo-Wurzel, Antwort nennt den Checkout; Beschluss 🧑 2026-10-06).
- `instructions.md` sagt Agenten in Worktrees, `checkout` immer zu setzen, und nennt `git rev-parse --show-toplevel` als Quelle.
- Das Log nennt je Aufruf, ob `checkout` gesetzt war.

## Nicht-Ziele

Ablehnen ohne Header oder ohne `checkout` (Beschluss 🧑 2026-10-06, B-275); neue Tools; Änderungen an der Client-Konfiguration der Desktop-App; die Ursachenmessung in der Desktop-App selbst (B-341).

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-dev/`). Firewall-Regel: Proben nur auf Loopback. Eine Datei bleibt ≤ 400 Zeilen, `workspace.go` ist heute 170.

## Beispiele

Session im Worktree `sprint-br1-fortsetzung-1b4aa6` (Header meldet die Wurzel) ruft `plan_create {kind: ticket, …, checkout: "sprint-br1-fortsetzung-1b4aa6"}` → Datei im Worktree, Wurzel unverändert, Antwort endet mit `Checkout: Worktree sprint-br1-fortsetzung-1b4aa6 (…)`.

## Ausnahme- und Fehlerfälle

`checkout` nennt einen Ordner, der kein Worktree dieses Repos ist → Fehler `<pfad> ist kein Worktree von <wurzel>`, nichts wird geschrieben. `checkout` nennt einen Namen, der mehrere Worktrees trifft → Fehler mit den Kandidaten.

## Akzeptanzkriterien

- **AC-01** Ein schreibendes Tool mit `checkout=<worktree>` ändert nur Dateien dieses Worktrees, auch wenn der Header die Wurzel nennt (Test in `workspace_test.go`).
- **AC-02** Ein ungültiges oder mehrdeutiges `checkout` wird abgelehnt, ohne zu schreiben (Test).
- **AC-03** Ohne `checkout` verhält sich jedes Tool wie vorher, die Antwort nennt den Checkout (bestehende Tests grün).
- **AC-04** `instructions.md` nennt das Argument; das Log führt je Aufruf `checkout_arg=true|false`.

## Offene Fragen

Nimmt der MCP-SDK-Schemagenerator ein optionales Feld an allen betroffenen Tools ohne Mehraufwand? Ungeprüft; die Session klärt es zuerst.

## Notizen

Entstanden aus der Rückfrage zu B-275 am 2026-10-10 (Wahl 🧑 im Chat: explizites Ziel-Argument). Folgt aus der Messung in B-341 (V1 belegt, Log-Auszug oben). B-275 bleibt erledigt. Nummer von Hand B-386 statt B-382, weil `plan_create` aus der Wurzel B-382 vergab und B-382 bis B-385 in zwei Worktrees schon belegt sind (siehe B-387).
