# WT1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Worktree-Sessions richten k3c-dev-Tools per Argument auf ihren Checkout
- **Projekt:** WZG

## Ziel

Eine Session in einem Worktree richtet jedes schreibende Tool und `check_run` mit dem Argument `checkout` auf ihren Worktree, unabhängig vom Header des Clients. Am Ende sichtbar: `plan_create {kind: ticket, …, checkout: "<worktree>"}` legt die Datei im Worktree an und lässt die Wurzel unverändert, `check_run` mit `checkout` prüft den Worktree, ein ungültiger Wert wird mit der Liste der Worktrees a …

## Ergebnis

2026-10-10, Review WT1.2: AC-01 bis AC-04 geprüft (WT1.1 › Ergebnis), behoben: Pfadvergleich mit Windows-Kurznamen (`canon` in `workspace.go`, CI), keine neuen Tickets. `task check` und `task check:dev` grün. Review auf Anweisung 🧑 im selben Lauf wie WT1.1; Folge-PR #246 nach dem früh gemergten #244. Nach dem Merge k3c-dev neu bauen und starten (`task k3c-dev:build`), sonst kennen laufende Sessions `checkout` nicht. Version: v0.17.0 vorgeschlagen (Minor: neues Argument `checkout` im Werkzeug; v0.16.0, falls K4 nicht vorher getaggt wird); gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Ein schreibendes Tool mit `checkout=<worktree>` ändert nur Dateien dieses Worktrees, auch … |
| AC-02 | geprüft | Ein ungültiges oder mehrdeutiges `checkout` wird abgelehnt, ohne zu schreiben (`B-388/AC-0 … |
| AC-03 | geprüft | Ohne `checkout` verhält sich jedes Tool wie vorher, die Antwort nennt den Checkout (`B-388 … |
| AC-04 | geprüft | `instructions.md` nennt das Argument, das Log führt je Aufruf `checkout_arg=true/false` (` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-388 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft | Schreibende k3c-dev-Tools nehmen den Ziel-Checkout als Argument, weil der Client … |
