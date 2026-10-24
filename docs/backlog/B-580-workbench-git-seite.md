# B-580 · Die Workbench hat eine Git-Seite für dieses Repo

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-24 🧑 Chat (Mock bestätigt, Umsetzung beauftragt)

## Ausgangslage

Git-Pflege läuft heute in der Shell oder über Claude: Status, veraltete Branches nach Squash-Merges, Stash, Push und PRs.
Die Workbench (`tools/k3c-workbench`) kennt nur den PR- und CI-Stand je Sprint (`internal/github`, Planungsseite). Nach
Squash-Merges sammeln sich lokale Branches mit gelöschtem Remote („gone“), die `git branch --merged` nicht als gemergt erkennt.

## Ziel

Ein Reiter „Git“ in der Workbench zeigt den Zustand des Repos auf einen Blick und erledigt die täglichen Handgriffe mit
einem Klick, passend zur Arbeitsweise (`develop` als Basis, ein PR je Sprint, Squash-Merge, `main` nur per Fast-Forward durch 🧑).

## Beteiligte und Zielgruppen

🧑 Ralf als Entwickler; Claude als Empfänger der Prompt-Aufträge.

## Anforderungen

- Status: aktueller Branch, Upstream mit Ahead/Behind, Ahead/Behind zu `origin/develop`, sauberer oder geänderter
  Working Tree, Stashes, Worktrees, Rückstand von `main` zu `origin/main`, Zeit des letzten Fetch und eine Empfehlung.
- Branches: lokale und nur entfernte Branches mit Remote-Zustand (synchron, voraus, zurück, gone, fehlt), Ahead/Behind zu
  `origin/develop` und Zustand (aktuell, in Worktree, gemergt, per Squash gemergt, ungemergt). Squash-Merges werden erkannt.
- Aufräumen: Markierte Branches löschen (nur gemergte, Bestätigung), dazu Prompts für Claude „Markierte aufräumen“ und
  „Veraltete aufräumen“ (kopieren oder als neue Code-Session öffnen).
- Geänderte Dateien: Working Tree (Status je Datei) und Diff des Branches zu `origin/develop` mit Zeilenzahlen.
- Fetch, Pull (nur Fast-Forward), Push (mit Upstream, nie Force), Stash anlegen und letzten Stash zurückholen.
- PRs: offene PRs gegen `develop` mit CI- und Merge-Stand; anlegen (aktueller Branch), mergen, per Squash mergen,
  schließen; Prompt „Beheben“ für PRs mit roter CI oder Konflikt.
- Alle Git- und gh-Aufrufe laufen ohne Shell mit festen Argumenten; Namen und Nummern aus der Oberfläche werden geprüft.

## Nicht-Ziele

- Kein Diff-Betrachter je Datei, kein Commit aus der Oberfläche, kein Rebase, kein Force-Push.
- Keine Pflege anderer Repos; kein Merge nach `main` (bleibt bei 🧑).

## Regeln und Einschränkungen

- Atomic Design der Wails-Oberflächen (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme), Radix nur über `@webkit`.
- Neue Bindings stehen in `uiMethods` (`uiapi.go`) und `GoApp` (`frontend/src/api/wails.ts`), `TestUIMethodsWieWailsTS`.
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Logging mit Emoji (🔨 bzw. ❌, 🧹 beim Aufräumen).
- Push auf `main` und `develop` lehnt die Seite ab; Branches `main` und `develop` lassen sich nicht löschen.

## Beispiele

- Branch `sprint/bd2`, Remote gone, Inhalt per Squash in `develop` → Zustand „per Squash gemergt“, markierbar, löschbar.
- Branch mit einem ungepushten Commit → Status zeigt „1 nicht gepusht“, Empfehlung „Push“.
- PR mit grüner CI und `mergeable` → Knöpfe „Squash“ und „Merge“ aktiv.

## Ausnahme- und Fehlerfälle

- `gh` fehlt oder ist nicht angemeldet → PR-Karte zeigt den Hinweis aus `internal/github`, der Rest der Seite arbeitet.
- Löschen eines ungemergten, aktuellen oder in einem Worktree ausgecheckten Branches → abgelehnt mit Grund.
- Pull ohne Fast-Forward oder Pop mit Konflikt → Fehlermeldung von git, nichts wird erzwungen.
- Ungültiger Branch-Name (Leerzeichen, `..`, führendes `-`) → abgelehnt, bevor git läuft.

## Akzeptanzkriterien

- **AC-01** `go test ./internal/gitops/` deckt Status-, Branch- und PR-Auswertung mit festen git-/gh-Ausgaben ab, auch die
  Squash-Erkennung und die Ablehnungen (ungemergt, aktueller Branch, Worktree, `main`/`develop`, ungültiger Name).
- **AC-02** Der Reiter „Git“ zeigt mit `?mock=1` Status, Branches, geänderte Dateien und PRs (Vitest der Seite).
- **AC-03** Die Prompts für Aufräumen und Beheben nennen die betroffenen Branches bzw. den PR (Vitest).
- **AC-04** Löschen, Merge, Squash, Schließen und Stash-Pop verlangen eine Bestätigung im Dialog (Vitest).
- **AC-05** Gegen das echte Repo (`task k3c-workbench`) zeigt die Seite denselben Stand wie `git status -sb` und `git branch -vv`
  (Beobachtung 🧑).

## Offene Fragen

keine

## Notizen

Mock-Entwurf im Chat vom 2026-10-24 bestätigt. Umgesetzt auf `feat/git-seite` (AC-01 bis AC-04 per Test); offen bleibt
die Beobachtung AC-05 durch 🧑, danach `erledigt`. Review vor dem Commit: Messfehler gilt nie als gemergt, Remote-Löschung
nur ohne fremde Commits, Push immer nach `origin/<branch>`, PR-Aktionen prüfen Basis und Head-Commit.
