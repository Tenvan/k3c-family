# WZ36 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Studio-Chat: Aktionskarten für jede Bearbeitung, Resume, Welt-Prüfung
- **Projekt:** WZG

## Ziel

Jede Bearbeitung des Studios lässt sich im Chat anstößen; der Chat setzt die Claude-Sitzung per Resume fort. Am Ende sichtbar: Chat schlägt eine Bearbeitung als Karte vor, Studio prüft und führt sie aus; lange Chats antworten schneller.

## Ergebnis

2026-10-31: AC-01 bis AC-03 durch Go-Tests und Vitest belegt (WZ36.1 bis WZ36.3), Abnahme am PC durch Ralf (WZ36.4). Review-Rest LOW: Glob in `chat_session.go:52` sucht in allen `projects/*` (UUIDs, geringes Risiko), kein Ticket.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Jede Bearbeitung aus dem Inventar lässt sich als Aktionskarte auslösen (`B-679/AC-01` bis … |
| AC-02 | geprüft | Der Chat setzt die Sitzung per Resume fort, mit Rückfall auf den Verlauf (`B-687/AC-01` bi … |
| AC-03 | geprüft | Kein Weltvorschlag ist ausführbar, wenn die Welt die Prüfung nicht besteht; Abnahme der sc … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-679 | erledigt | AC-01 geprüft | Der Chat des Asset-Studios kann jede Bearbeitung des Studios als Aktionskarte au … |
| B-687 | erledigt | AC-02 geprüft | Der Studio-Chat setzt die Claude-Sitzung per Resume fort und fällt auf den Verla … |
| B-686 | erledigt | AC-03 geprüft | Der Studio-Chat liefert nur Welten, die die Prüfung bestehen, und korrigiert Feh … |
