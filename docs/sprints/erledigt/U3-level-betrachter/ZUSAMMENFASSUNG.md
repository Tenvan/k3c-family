# U3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Level-Betrachter
- **Projekt:** –

## Ziel

Die Seite `leveltest.html` zeigt Seed und Biom als Level-Streifen mit Warnungen und startet das Level im Spiel und ist von der Testseite (`testing.html`) aus erreichbar. Am Ende sichtbar: auf dem TV von der Testseite die Kachel „Level-Betrachter“ öffnen, mit Controller Seed ändern, scrollen und „Im Spiel starten“.

## Ergebnis

- 2026-10-02 (U3.4, Review): AC-01 bis AC-04, AC-06, AC-07 belegt (Ergebnisse U3.1–U3.3, AC-03-Beispiel Seed `test` im Review beobachtet, siehe U3.4). - AC-05 offen: wartet auf Abnahme durch 🧑 am TV. - Behobene Befunde: keine. Schwerer Befund außerhalb PLAT: „Im Spiel starten“ mit vorhandenem gleichnamigem Spielstand wird vom Server abgewiesen (`save_exists`), der Hinweis auf der Seite stimmt nicht. - Neue Tickets: B-096.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Seite eingetragen, `installPageChrome()` vorhanden, Projektregeln grün (B-092/AC-01). |
| AC-02 | – | Abbildungs-Funktion getestet (B-092/AC-02). |
| AC-03 | geprüft | Das Golden-Level ergibt im Zeichenmodell genau seine Abschnitte und Objekte (B-092/AC-03). |
| AC-04 | geprüft | Hinweis ohne Server (B-092/AC-04). |
| AC-05 | offen | 🧑 hat die Seite mit Controller am TV bedient (B-092/AC-05). |
| AC-06 | – | Die Testseite `testing.html` führt über eine Kachel „Level-Betrachter“ zu `leveltest.html` … |
| AC-07 | geprüft | Start-URL-Funktion nur für Wald und Seeds im Namensformat, getestet (B-092/AC-07). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-092 | verworfen | AC-01 –, AC-02 –, AC-03 geprüft, AC-04 geprüft, AC-05 offen, AC-06 –, AC-07 geprüft | Eine Testseite zeigt ein generiertes Level (Seed und Biom) ohne zu spielen |
