# WZ21 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Studio-Chat: eigene Umgebung, Grundgerüst als Instruktion, Entwurf geschützt
- **Projekt:** WZG

## Ziel

Der Studio-Chat schickt das Grundgerüst selbst mit, und Abbruch, Mock und Weltwechsel verhalten sich sauber. Am Ende sichtbar: Ralf schreibt nur „Erstelle eine Welt Nordsee“ und bekommt einen Entwurf; ein Abbruch lässt keinen CLI-Prozess zurück.

## Ergebnis

2026-10-22: AC-02 und AC-03 nachgewiesen (WZ21.2–WZ21.4), `check:diff` grün, Reviewer ohne schwere Befunde. B-516/AC-04 von Ralf im Studio abgenommen („ok“, 2026-10-22).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | verworfen | entfällt: B-515 ist mit PR #406 erledigt (geerbte Umgebung und doppelter Anmeldefehler beh … |
| AC-02 | – | Das Grundgerüst ist eine feste Instruktion im Code und geht bei Welt-Aufträgen automatisch … |
| AC-03 | geprüft | Die offenen Punkte 2 bis 4 aus B-514 sowie die Nachträge (Mock `mockWorldDraft.ts` setzt ` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-516 | erledigt | AC-02 – | Prompt-Grundgerüst des Studio-Chats ist eine feste Instruktion im Code statt Tex … |
| B-514 | erledigt | AC-03 geprüft | Chat-Entwurf: Planung bei gleicher Insel-id schützen, CLI-Prozessbaum, Test-Rieg … |
