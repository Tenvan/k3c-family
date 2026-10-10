# WT1.1 · Schreibende Tools und check_run nehmen das Argument checkout

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wt1/1-checkout-argument
- **Abhängig von:** –
- **Tickets:** B-388
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Alle schreibenden Tools von k3c-dev und `check_run` nehmen das optionale Argument `checkout` (Worktree-Name oder -Pfad); es gilt vor dem Header, `instructions.md` und das Log nennen es (B-388).

## Kontext

- Problem und Messung: `docs/backlog/B-388-checkout-argument-schreibende-tools.md` und `docs/backlog/B-341-header-ursache-live-messen.md` › Notizen. Beschluss 🧑 2026-10-06 (B-275): ohne Header nicht ablehnen, sondern den Checkout nennen; das bleibt.
- `tools/k3c-dev/internal/mcpsrv/workspace.go`: `workspace`, `resolveWorkspace(req)` (Header über `headerRoot`), `workspaceOf(dir)` (Repo-Wurzel oder `isWorktreeOf`), `writeTools` (Tools, deren Antwort `Checkout:` trägt), `addCheckout`.
- `observe.go`: die eine Middleware für alle Tool-Aufrufe; ruft `resolveWorkspace(call)`, legt das Ergebnis unter `wsKey{}` in den Kontext und loggt in `finish` die Felder `header` und `checkout`.
- `tools.go`: `add[In]` registriert jedes Tool; das Schema folgt den Feldern von `In` (`additionalProperties: false`), `s.params[tool]` führt die gültigen Namen für `param_hint.go`. Ein unbekanntes Feld wird also schon beim Schema abgelehnt; `checkout` muss ins Schema der betroffenen Tools.
- Bestehende Tests: `workspace_test.go` (`TestWorkspaceAusHeader` u. a.) mit Hilfen für Temp-Repos und Worktrees; erweitern statt neu bauen.
- `instructions.md` (eingebettet) ist die Anleitung, die Agenten beim Verbinden lesen; Abschnitt „Worktrees“.
- Fallstrick: `task_*` bedient nur die Repo-Wurzel (`instructions.md`); `checkout` soll dort nicht still ignoriert werden, entweder ablehnen mit Hinweis auf `check_run` oder wie `svc_*` auflösen (Entscheidung in der Session, im Ergebnis festhalten).

## Erlaubte Dateien

- `tools/k3c-dev/internal/mcpsrv/` (`workspace.go`, `observe.go`, `tools.go`, `instructions.md`, `workspace_test.go` und neue Dateien daneben)
- `tools/k3c-dev/internal/mcpsrv/codemap.md` (Beschreibung der Auflösung nachziehen)
- Planungs-Dateien für Status und Ergebnis (über die plan-Tools, mit `checkout`)

## Nicht-Ziele

Kein Ablehnen ohne `checkout`. Keine neuen Tools. Keine Änderung an `.mcp.json`, am `headersHelper` oder am Port-Schema. Keine Änderung der Ticket-Nummern-Vergabe (B-387).

## Schritte

1. Failing Go-Test (AC-01): Temp-Repo mit einem Worktree, Aufruf `plan_set` mit Header = Wurzel und `checkout` = Worktree → die Datei im Worktree ändert sich, die der Wurzel nicht.
2. Failing Tests (AC-02): `checkout` = Pfad außerhalb des Repos, unbekannter Name, zwei Worktrees gleichen Namens → Fehler mit Wertebereich bzw. Kandidaten, keine Datei geschrieben.
3. Auflösung: `checkout` aus den rohen Argumenten in `observe` lesen, vor `resolveWorkspace` auswerten; Name über `git worktree list --porcelain` in der Wurzel, Pfad über `workspaceOf`. Fehler wie bisher als `textResult(…, true)`.
4. Schema: `checkout` als optionales String-Feld in die Eingabe der betroffenen Tools (`writeTools` und `check_run`) aufnehmen, ohne es in jedem Eingabe-Typ von Hand zu wiederholen (Hilfsfunktion in `add` oder eigener eingebetteter Typ). `s.params` und `param_hint.go` kennen das Feld.
5. Log: Feld `checkout_arg` je Aufruf in `finish` (AC-04); `instructions.md`: Abschnitt „Worktrees“ ergänzen (Argument immer setzen, Quelle `git rev-parse --show-toplevel`, Header allein genügt nicht).
6. AC-03: bestehende Tests des Pakets unverändert grün; ein Test ohne `checkout` prüft die Zeile `Checkout:` weiter.
7. `check_run dev:test`, danach `task check:dev` (Go-Teil, `golangci-lint`).

## Fertig, wenn

- [ ] AC-01: Go-Test grün: `plan_set` mit `checkout=<worktree>` ändert nur die Datei im Worktree, auch bei Header = Wurzel.
- [ ] AC-02: Go-Tests grün: ungültiges, unbekanntes und mehrdeutiges `checkout` werden abgelehnt, ohne zu schreiben; die Meldung nennt die gültigen Worktrees.
- [ ] AC-03: alle bisherigen Tests des Pakets `mcpsrv` grün; Antwort ohne `checkout` trägt `Checkout:`.
- [ ] AC-04: `instructions.md` nennt `checkout`, das Aufruf-Log führt `checkout_arg=true|false` (Test auf die Log-Attribute oder Sichtprüfung im Ergebnis).
- [ ] `task check:dev` grün.

## Prüfen

```bash
task check:dev
```

Keine manuellen Prüfungen. Die Beobachtung aus einem echten Desktop-App-Worktree (Argument erreicht k3c-dev) braucht ein neu gebautes k3c-dev (`task k3c-dev:build`) und geschieht in der nächsten Worktree-Session von 🧑; sie ist keine Voraussetzung dieser Session.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
