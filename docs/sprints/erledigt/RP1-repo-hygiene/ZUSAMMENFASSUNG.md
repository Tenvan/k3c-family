# RP1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Repo-Hygiene: Branches aufräumen, Altlasten, Regeln
- **Projekt:** REL

## Ziel

Das Repo enthält nur, was gebraucht wird, und die Regeln entsprechen den Beschlüssen. Am Ende sichtbar: Nur aktive Branches, keine Alt-Binaries, Regeln zu Spielstand und Execution-Policy angeglichen.

## Ergebnis

- 2026-10-18, Review RP1.3 (getrennter Reviewer): AC-01 bis AC-04 am Diff belegt (RP1.1: B-302/AC-01 bis AC-05 mit `tools/aufraeumen` und Test; RP1.2: B-288, B-094, B-058 vorläufig). - Behoben: Fixture-Satz im Spielstand-Abschnitt klargestellt. Restrisiken von `task aufraeumen` (PR-Status nur über Branchnamen, frischer Branch ohne Commit gilt als gemergt) stehen so in B-302; offen für 🧑: Auto-Modus-Regel, Release bei Resten rot, B-058 `RemoteSigned`. - Neue Tickets: keine. - Version: v0.19.1 vorgeschlagen (Patch: Werkzeug und Doku ohne Wirkung im Spiel); gesetzt nur nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Lokale Branches und Worktrees werden an festen Meilensteinen aufgeräumt (B-302/AC-01, B-30 … |
| AC-02 | – | Formatänderungen am Spielstand nehmen keine Rücksicht auf alte Stände (B-288/AC-01). |
| AC-03 | geprüft | Im Repo liegen keine Alt-Binaries und keine npm-Skripte mehr (B-094/AC-01, B-094/AC-02). |
| AC-04 | – | requirements.md empfiehlt keine Sicherheitseinstellung ohne Entscheidung von 🧑 (B-058/AC-0 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-302 | erledigt | AC-01 – | Lokale Branches und Worktrees werden an festen Meilensteinen aufgeräumt |
| B-288 | erledigt | AC-02 – | Formatänderungen am Spielstand nehmen keine Rücksicht auf alte Stände |
| B-094 | erledigt | AC-03 geprüft | Im Repo liegen keine Alt-Binaries und keine npm-Skripte mehr |
| B-058 | erledigt | AC-04 – | requirements.md empfiehlt keine Sicherheitseinstellung ohne Entscheidung von 🧑 |
