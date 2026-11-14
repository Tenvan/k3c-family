# BD9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Welt-Anzeige größer und je Spielstand, offene Räume schließen
- **Projekt:** BED

## Ziel

Die Welt ist im Pausenmenü groß lesbar und steht in der Lobby bei jedem Spielstand; ein offener Raum lässt sich über „Entfernen“ schließen. Am Ende sichtbar: Lobby und Pausenmenü am PC.

## Ergebnis

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). BD9.1–BD9.3 fertig (#462), BD9.4 (Abnahme PC) verworfen, B-581 und B-604 verworfen. Status `erledigt`, weil das Werkzeug für Sprints keinen Status `verworfen` kennt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Das Pausenmenü zeigt die Welt in 40 px (B-604/AC-01). |
| AC-02 | geprüft | Jeder Eintrag der Lobby nennt seine Welt, auch offene Räume (B-604/AC-02). |
| AC-03 | geprüft | Entfernen auf einem offenen Raum bietet „Schließen“; danach ist der Stand gespeichert und … |
| AC-04 | – | Ralf hat AC-01 bis AC-03 am PC abgenommen. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-604 | verworfen | AC-01 geprüft, AC-02 geprüft | Die Welt steht groß im Pausenmenü und bei jedem Spielstand in der Lobby |
| B-581 | verworfen | AC-03 geprüft | Ein offener Raum lässt sich in der Lobby über Entfernen schließen |
