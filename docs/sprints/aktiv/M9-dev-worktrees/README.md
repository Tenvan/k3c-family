# M9 · SRV · k3c-dev in Worktrees und Markdown-Ansicht

- **Status:** aktiv
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-275, B-213
- **Start-Commit:** 7fe4f05
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1; Domänen-Ausnahme INF vite.config.ts (M9.2) bestätigt

## Ausgangslage

k3c-dev bedient aus Worktrees unter `.claude/worktrees/` noch die Repo-Wurzel (B-275); MarkdownView zeigt nummerierte Listen und Häkchen falsch (B-213).

## Ziel

Jede Session plant, prüft und startet Dienste in ihrem eigenen Checkout und liest Sessions in k3c-dev wie im Editor. Am Ende sichtbar: Planung, Prüfläufe und Dienste treffen den Worktree der Session; Session-Dateien lesbar im Detail-Panel.

## Beteiligte und Zielgruppen

🧑 und Agenten in parallelen Worktrees; der Agent baut in `tools/k3c-dev/`.

## Anforderungen

B-275 › Anforderungen; B-213 › Anforderungen.

## Nicht-Ziele

Neue MCP-Tools, Umbau der Planungsseite.

## Regeln und Einschränkungen

Nur `tools/k3c-dev/` und `.mcp.json` (SRV). Eine k3c-dev-Instanz bleibt für alle Worktrees zuständig.

Domänen-Ausnahme: B-275 verlangt auch eine Änderung an `vite.config.ts` (INF, B-275 › Regeln und Einschränkungen). Sie läuft als eigene Session M9.2, die nur `vite.config.ts` ändert; 🧑 bestätigt die Ausnahme mit der Spec-Freigabe, sonst wird M9.2 ein Ticket INF.

Beschluss 🧑 2026-10-06 (Chat): Fehlt der Header `X-K3C-Root`, schreibt ein schreibendes Tool (`plan_create`, `plan_set`, `plan_section`, `plan_delete`, `svc_start`, `svc_stop`) trotzdem in die Repo-Wurzel, nennt in der Antwort aber immer den Checkout. Es lehnt nicht ab. Warum der Header aus Worktrees nicht ankommt, ist ungeprüft (vermutet: Die Desktop-App startet `headersHelper` mit der Wurzel als cwd); die erste Session (M9.1) klärt es.

## Beispiele

Session in `.claude/worktrees/x` ruft `plan_set` → Datei im Worktree geändert, `workbench_status` nennt den Worktree.

## Ausnahme- und Fehlerfälle

Header `X-K3C-Root` fehlt → Repo-Wurzel wie heute, die Antwort schreibender Tools nennt den Checkout (Beschluss oben), das Log vermerkt den fehlenden Header. Header zeigt außerhalb des Repos → Fehler wie heute (`workspaceOf`).

## Akzeptanzkriterien

- **AC-01** k3c-dev und Vite arbeiten in Worktrees unter `.claude/worktrees/` richtig (B-275/AC-01, B-275/AC-02, B-275/AC-03).
- **AC-02** MarkdownView in k3c-dev zeigt nummerierte Listen und Häkchen wie die alte Planungsseite (B-213/AC-01, B-213/AC-02).
- **AC-03** Die Ursache, warum der Header `X-K3C-Root` aus Worktrees nicht ankommt, ist geklärt und im Ergebnis von M9.1 belegt (Log-Zeile mit und ohne Header); liegt sie außerhalb von `.mcp.json` und k3c-dev, steht sie als Ticket im Backlog.
- **AC-04** `task check:dev` und `task check` grün.

## Offene Fragen

- Domänen-Ausnahme für `vite.config.ts` (INF) in M9.2: bestätigt 🧑 mit der Spec-Freigabe 2026-10-07.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M9.1 | `M9.1-checkout-header.md` | Umsetzung | autonom | fertig |
| M9.2 | `M9.2-vite-worktree.md` | Umsetzung | autonom | fertig |
| M9.3 | `M9.3-markdown-listen.md` | Umsetzung | autonom | fertig |
| M9.4 | `M9.4-review.md` | Review | autonom | in Arbeit |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
