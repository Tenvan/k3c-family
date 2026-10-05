# B-275 · k3c-dev und Vite arbeiten in Worktrees unter `.claude/worktrees/` richtig

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus dem Worktree `.claude/worktrees/k3c-dev-adjustments-1af643` (GR2.3, GR3.1) liefen `plan_set` und `check_run` von k3c-dev auf der Repo-Wurzel: `workbench_status` meldete `Checkout: Repo-Wurzel`, `plan_set` schrieb die Session-Datei dort. Zusätzlich ignoriert `vite.config.ts` per `watch.ignored` den Pfad `**/.claude/**`; liegt der Worktree selbst darunter, sieht der Dev-Server keine Änderung und liefert alten Code.

## Ziel

Planung, Prüfläufe und Dienste treffen immer den Checkout der Session; der Dev-Server eines Worktrees lädt Änderungen neu.

## Beteiligte und Zielgruppen

Agenten-Sessions in Worktrees, 🧑 beim parallelen Arbeiten.

## Anforderungen

- k3c-dev erkennt den Worktree der Session (Header `X-K3C-Root` oder MCP-roots) auch für Worktrees unter `.claude/worktrees/`.
- Vite ignoriert `.claude/` nur relativ zur eigenen Wurzel, nicht wenn die Wurzel selbst darunter liegt.
- Schreibende Tools (`plan_*`, `svc_start`, `svc_stop`) ändern ohne Header nichts unbemerkt in der Repo-Wurzel: `resolveWorkspace` (`tools/k3c-dev/internal/mcpsrv/workspace.go`) fällt heute still auf die Wurzel zurück. Sie lehnen ab oder nennen den Ziel-Checkout in der Antwort.

## Nicht-Ziele

Neue Dienste oder Port-Schema (bleibt wie in k3c-dev beschrieben).

## Regeln und Einschränkungen

Domäne SRV (k3c-dev) und INF (`vite.config.ts`); Firewall-Regel: Proben nur auf Loopback.

## Beispiele

Session im Worktree ruft `plan_set` → Datei im Worktree geändert, Repo-Wurzel unverändert.

## Ausnahme- und Fehlerfälle

Header fehlt → k3c-dev meldet den gewählten Checkout deutlich, statt still die Wurzel zu nehmen.

## Akzeptanzkriterien

- **AC-01** Aus einem Worktree unter `.claude/worktrees/` nennt `workbench_status` diesen Worktree als Checkout (Test in `dev:test`).
- **AC-02** Der Vite-Dev-Server eines solchen Worktrees lädt eine geänderte Datei unter `src/` neu (Beobachtung).
- **AC-03** Ein schreibendes Tool ohne Header ändert keine Datei der Repo-Wurzel, ohne den Checkout in der Antwort zu nennen (Test in `workspace_test.go`); k3c-dev loggt je Aufruf, ob der Header da war.

## Offene Fragen

Warum kommt der Header aus diesem Worktree nicht an (`headersHelper` in `.mcp.json` mit `process.cwd()`)? Ungeprüft; vermutet: Die Desktop-App startet den Helper mit der Wurzel als cwd oder schickt ihn nicht. Soll ein schreibendes Tool ohne Header ablehnen oder nur den Checkout nennen? Ablehnen bricht Clients ohne `headersHelper`; das entscheidet 🧑.

## Notizen

Beobachtet 2026-10-04 in GR2.3 und GR3.1; Umgehung: Status von Hand, `task check` in der Shell, Vite neu starten.
Erneut 2026-10-05 (Sprint W3, PR #146, Desktop-App, Worktree `sprint-rl1-continue-091347`): `plan_set W3` verschob den Sprint-Ordner im `develop`-Checkout der Wurzel; von Hand in den Worktree übertragen und die Wurzel zurückgesetzt.
