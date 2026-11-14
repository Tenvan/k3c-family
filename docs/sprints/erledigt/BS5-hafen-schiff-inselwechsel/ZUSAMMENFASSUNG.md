# BS5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Hafen-Chunk, Schiff bauen und Inselwechsel am Hafen
- **Projekt:** BAS

## Ziel

Auf Inselstufe 1 liegt der Hafen-Chunk, dort wird das Schiff in Abschnitten mit Münzen und Material gebaut, und der Inselwechsel läuft am Hafen. Bots, Bericht und Werte folgen in BS9 (Meilenstein 3). **Am Ende sichtbar:** Generator- und Sim-Tests grün, Protokoll und Spielstand tragen den Schiff-Fortschritt.

## Ergebnis

2026-10-29, Claude (autonom): AC-01 bis AC-03 erfüllt (BS5.1 bis BS5.3: harbor-Chunk Slot −7 in standard.json, Platz `ship` mit drei Abschnitten aus data/ship.json, Wechsel nur mit fertigem Schiff + Endboss + allen am Steg, Spielstand-Rundlauf, Protokoll-Absatz). Review (Sonnet): Gate-Reset bei Schiffverlust, Schiff aus allBuilt ausgenommen, Welten ohne Hafen auf nicht-letzter Insel werden abgelehnt (gewollt). Offen: Client (BS6, B-637/AC-05), Bots (BS9), Standard-Welt hat nur eine Insel, der Wechsel greift erst mit Insel 2.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Generator-Test: genau ein Hafen-Chunk auf Insel 1 Tiefe 0 am geplanten Platz (B-637/AC-02) … |
| AC-02 | erfüllt | Sim-Test: Schiff in Abschnitten baubar; Wechsel nur mit fertigem Schiff, besiegtem Endboss … |
| AC-03 | erfüllt | Protokoll und Spielstand tragen den Schiff-Fortschritt (B-637/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-637 | eingeplant | AC-01 erfüllt, AC-02 erfüllt, AC-03 erfüllt | Ein Hafen-Chunk auf Inselstufe 1 baut das Boot zum Verlassen der Insel |
| B-638 | verworfen | – | Das Balancing misst die Ressourcen für das Boot |
