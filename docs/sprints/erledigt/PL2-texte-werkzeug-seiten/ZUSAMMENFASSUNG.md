# PL2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Werkzeug-Seiten in der gewählten Sprache
- **Projekt:** WZG

## Ziel

Jede Werkzeug-Seite folgt der gewählten Sprache mit Rückfall auf Deutsch. Am Ende sichtbar: Die Seiten unter `src/tools/` zeigen Deutsch oder Englisch nach Wahl.

## Ergebnis

Review 2026-10-08 (PL2.4, eigener Review-Agent): keine schweren Befunde, `task check` und `task check:go` grün. - AC-01 angenommen (B-322/AC-01: Regeltest ohne `OFFEN`, 307 Schlüssel de/en gleich, Platzhalter gleich); Kleinbefund behoben: `isKey` prüft mit `Object.hasOwn`. - B-322/AC-02 geprüft 2026-10-08 (PL2.5, 🧑 am PC): Englisch und Deutsch auf allen Werkzeug-Seiten. B-322 erledigt. - Neues Ticket: B-369 (Landingpage bleibt deutsch, außerhalb von PL2). - Version: Minor vorgeschlagen (neue Sprachwahl in den Werkzeug-Seiten); nicht gesetzt (wartet auf Bestätigung 🧑).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Die Werkzeug-Seiten holen ihre Texte aus den zentralen Textdateien (B-322/AC-01, B-322/AC- … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-322 | erledigt | AC-01 geprüft | Die Werkzeug-Seiten holen ihre Texte aus den zentralen Textdateien |
