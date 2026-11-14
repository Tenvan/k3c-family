# BS6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Client: Baumschule, Hafen und Schiff, neue Münzskala
- **Projekt:** BAS

## Ziel

Am TV sieht man Baumschule (mit „erst roden“), Hafen und Schiff je Abschnitt und die kleinen Münzpreise; nach der Abnahme misst das Balancing wieder über alle Seeds. **Am Ende sichtbar:** Browser-Pane mit Baumschule und Hafen, Bericht „Seed-Modus: all“.

## Ergebnis

–

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Client zeigt Baumschule und Hinweis „erst roden“; „Farm“ nicht mehr in `src/` (B-636/AC-02 … |
| AC-02 | geprüft | Client zeichnet Hafen und Schiff je Abschnitt, Hinweis bei fehlender Bedingung (B-637/AC-0 … |
| AC-03 | geprüft | Kosten-Anzeigen und HUD passen zur Skala 2–30. |
| AC-04 | – | **Meilenstein 4:** `seedMode: all`, Abschluss-Lauf `coop2` standard über alle Seeds mit `b … |
| AC-05 | geprüft | Abnahme am Gerät durch 🧑 (angenommen bis zur Hardware-Session). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-634 | erledigt | AC-04 – | Balancing läuft bis zur Abnahme nur mit Seed k3c-dev und ist schneller |
| B-636 | eingeplant | AC-01 geprüft | Die Farm heißt Baumschule und steht auf gerodeten Waldbauplätzen |
| B-637 | eingeplant | AC-02 geprüft | Ein Hafen-Chunk auf Inselstufe 1 baut das Boot zum Verlassen der Insel |
