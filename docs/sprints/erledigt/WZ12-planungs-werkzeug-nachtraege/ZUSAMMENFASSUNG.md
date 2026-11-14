# WZ12 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Planungs-Werkzeug: Platzhalter-Zeilen und Worktree-Pills
- **Projekt:** WZG

## Ziel

`plan_create` ersetzt Platzhalter-Zeilen ohne Datei durch die neue Session-Zeile, und die Planungsseite zeigt Worktree-Änderungen als Pill am Projekt, Sprint, an der Session oder am Ticket statt als Leiste. Am Ende sichtbar: `task check:dev` grün, die Planungsseite ohne Worktree-Leiste, Pills mit Tooltip (Branch und Änderung) im Mock; von 🧑 am PC abgenommen.

## Ergebnis

2026-10-18, Review WZ12.3 (autonom, getrennte Reviewer): AC-01 bis AC-05 geprüft (`task:check`, `dev:test`, `go:lint` grün); AC-06 offen (WZ12.4). Behoben: Pills veraltet/überholt (Fokus-Neuladen, Antwort-Zähler), Auswahl vor erstem Check, Verdrahtungstests. Tickets: B-483, B-484. Version: v– vorgeschlagen (reines Entwickler-Werkzeug).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `plan_create` ersetzt Platzhalter-Zeilen ohne Datei (`–`, `– (Entwurf)`) durch die neue Se … |
| AC-02 | geprüft | Die Planungsseite zeigt keine Worktree-Leiste mit Änderungs-Chips mehr (`B-467/AC-01`). |
| AC-03 | geprüft | Projekt, Sprint, Session und Ticket mit Änderung in einem Worktree tragen eine Worktree-Pi … |
| AC-04 | geprüft | Neu im Worktree angelegte Einträge bleiben sichtbar (`B-467/AC-03`). |
| AC-05 | geprüft | Komponententest in `task test`, `task check:dev` grün (`B-467/AC-04`). |
| AC-06 | offen | 🧑 nimmt die Planungsseite mit Pills am PC ab. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-436 | erledigt | AC-01 geprüft | plan_create scheitert an Platzhalter-Zeilen „– (Entwurf)“ in der Session-Tabelle |
| B-467 | erledigt | AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft | Worktree-Änderungen stehen als Pill am Projekt, Sprint oder an der Session statt … |
