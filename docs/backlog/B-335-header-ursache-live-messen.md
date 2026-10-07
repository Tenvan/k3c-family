# B-335 · Die Ursache für den fehlenden Header `X-K3C-Root` aus Worktrees ist live gemessen

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus Worktrees unter `.claude/worktrees/` (Desktop-App) landeten `plan_set` und `workbench_status` in der Repo-Wurzel (B-275). M9.1 hat offline gemessen: Der `headersHelper` aus `.mcp.json` liefert im Worktree den Worktree-Pfad, k3c-dev ordnet einen Aufruf mit Header richtig zu (Test `TestWorkspaceAusHeader`), und die betroffenen Worktrees (`k3c-dev-adjustments-1af643`, `sprint-rl1-continue-091347`) haben `hasTrustDialogAccepted: true` in `~/.claude.json`. Die Ursache liegt damit vermutet im Client oder im damals laufenden k3c-dev, nicht in `.mcp.json` oder im heutigen Code; live ist das ungeprüft.

## Ziel

Klar ist, warum der Header aus einem Desktop-App-Worktree nicht ankommt, und ob es eine Abhilfe im Repo oder in der Client-Konfiguration gibt.

## Beteiligte und Zielgruppen

🧑 (Desktop-App, startet k3c-dev), Agenten-Sessions in Worktrees.

## Anforderungen

- Eine Session in einem Desktop-App-Worktree ruft ein Tool von k3c-dev auf; die Log-Zeile `✅ <tool>: ok` (`ns=mcp`, Felder `header`, `checkout`, seit M9.1) entscheidet zwischen den Vermutungen:
  - V1 `header=true`, `checkout=Repo-Wurzel …`: Der Client führt den Helper mit der Wurzel als cwd aus. Laut Claude-Code-Doku (MCP › headersHelper) läuft er im „Project directory the server is declared in“; vermutet nimmt der Client aus dem Worktree die `.mcp.json` der Wurzel (beide deklarieren `k3c-dev`).
  - V2 `header=false`: Der Helper läuft nicht oder der Header wird nicht geschickt.
  - V3 Alter Stand: k3c-dev lief zum Zeitpunkt der Beobachtung (GR2.3 2026-10-04, W3 2026-10-05) mit einer EXE vor #114 (Header-Auswertung, 2026-10-04 14:44); dann tritt der Fehler heute nicht mehr auf.
- Ergebnis und Abhilfe stehen hier unter Notizen; liegt die Abhilfe im Repo, wird sie ein eigenes Ticket.

## Nicht-Ziele

Ablehnen ohne Header (Beschluss 🧑 2026-10-06, B-275); neue MCP-Tools.

## Regeln und Einschränkungen

Domäne SRV; k3c-dev vorher mit `task k3c-dev:build` neu bauen, damit das Log die Felder `header` und `checkout` hat. Firewall-Regel: Proben nur auf Loopback.

## Beispiele

Session im Worktree `x` ruft `workbench_status` → Log `"header":true,"checkout":"Worktree x (…)"`: kein Fehler mehr (V3).

## Ausnahme- und Fehlerfälle

k3c-dev läuft nicht → keine Messung, Ticket bleibt offen.

## Akzeptanzkriterien

- **AC-01** Je eine Log-Zeile aus einer Desktop-App-Session in der Wurzel und in einem Worktree steht unter Notizen, mit der zutreffenden Vermutung (V1, V2, V3 oder eine neue).
- **AC-02** Die Abhilfe ist benannt: Ticket im Repo, Einstellung beim Client oder „kein Fehler mehr“.

## Offene Fragen

keine

## Notizen

Offline-Messung M9.1 (2026-10-07): `node -p "JSON.stringify({'X-K3C-Root': process.cwd()})"` gibt im Worktree `sprint-m9` dessen Pfad, in der Wurzel `C:/WORKSPACE/k3c`; `claude mcp get k3c-dev` im Worktree meldet `Scope: Project config (shared via .mcp.json)`, aber nicht, aus welcher Datei. Die EXE der Wurzel (`tools/k3c-dev/build/bin/k3c-dev.exe`, gebaut 2026-10-06 14:13) enthält die Header-Auswertung. `logs/k3c-dev.jsonl` der Wurzel zeigt bis 2026-10-07 keine Konsolen-Quelle `check:<worktree>/…`, also keinen Prüflauf, der einem Worktree zugeordnet wurde.
