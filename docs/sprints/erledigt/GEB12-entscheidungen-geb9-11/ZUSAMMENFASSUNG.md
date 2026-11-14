# GEB12 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Entscheidungen zu GEB9 bis GEB11 (Server-Stufenfolge, Markierer im Snapshot, Wald-Dichte)
- **Projekt:** GEB

## Ziel

Alle Entscheidungen aus GEB9 bis GEB11 stehen mit Default fest und liegen Ralf als Frage-Tickets im KRT-Sprint KR2 vor; die Folgearbeit ist geplant, der kleine CLI-Default D8 ist umgesetzt. Am Ende sichtbar: je Entscheidung D1–D8 ein Frage-Ticket (B-538 bis B-545), Vermerke in den Specs von GEB9, GEB10, GEB11, das Baugerüst erscheint nachts nicht mehr.

## Ergebnis

2026-10-24: AC-01 (GEB12.1/KR2.1: D1–D8 entschieden, D2 `markedBy` bleibt), AC-02 und AC-03 (GEB12.2), AC-04 (GEB12.3) erfüllt. Neue Tickets: B-531 (Werte D3), B-568 bis B-572.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Jede Entscheidung D1–D8 ist von Ralf bestätigt, geändert oder verworfen und im Frage-Ticke … |
| AC-02 | – | Je Entscheidung D1–D8 existiert ein Frage-Ticket im Sprint KR2 mit Default und Folgen; die … |
| AC-03 | – | Folgearbeit ist geplant: B-531 (D3) und B-532 (D2) im Projekt GEB mit Prio und Verweis auf … |
| AC-04 | – | Das Baugerüst vor der Burg erscheint nur bei `hubUpgrade.state = waitingWorker` ohne `dang … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-458 | eingeplant | – | Hub wächst in Ausbaustufen sichtbar und kleinteilig (Lagerfeuer bis Burg, K2C-St … |
| B-531 | erledigt | AC-03 – | Wald hat mehr Bäume je Abschnitt, Höhle bekommt Deko-Büsche |
| B-532 | erledigt | AC-03 – | Snapshot nennt, wer einen Knoten markiert hat |
| B-538 | erledigt | AC-01 geprüft | Entscheidung D1: Hub-Wachstum bleibt reine Grafik, keine feinere Stufenfolge im … |
| B-539 | erledigt | – | Entscheidung D2: Wimpel-Farbe bleibt Client-Annäherung, markedBy erst nach GEB10 … |
| B-540 | erledigt | – | Entscheidung D3: Wald-Dichte und Höhlenbüsche einplanen, Werte setzt Ralf |
| B-541 | erledigt | – | Entscheidung D4: Bäume bleiben 11,25 Kacheln hoch |
| B-542 | erledigt | – | Entscheidung D5: Plantage-Pappel steht hinter der Farm |
| B-543 | erledigt | – | Entscheidung D6: Geschlossene Truhe ohne Serverzustand |
| B-544 | erledigt | – | Entscheidung D7: Hub-Gebäude folgen der Hub-Stufe (Defaults aus GEB9) |
| B-545 | erledigt | AC-01 geprüft | Entscheidung D8: Baugerüst nur ohne Gefahr |
