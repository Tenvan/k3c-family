# SV2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Voll-Ausbau-Stand regelkonform aus der Welt-Datei, Review-Nachträge SV1
- **Projekt:** WRT

## Ziel

Der Voll-Ausbau-Stand ist ein Zustand, den das Spiel auch regulär erreichen kann, mit denselben Stufen wie ein neues Spiel. Am Ende sichtbar: `task check:go` grün, Messläufe auf dem Voll-Stand sind aussagekräftig.

## Ergebnis

2026-10-25, Claude (SV2.3): `task check` und `task check:go` grün; `check:race` lokal übersprungen (kein C-Compiler), die CI prüft es. AC-01 und AC-02 erfüllt durch SV2.1 (Tiefen aus der Welt-Datei, `DevBuildAll` je Inselstufe, Golden byte-gleich). AC-03 durch SV2.1/SV2.2 (echter `store.Saves`, 415 ohne JSON). Review ohne schwere Befunde. Auf Tiefe 4 (Hub 1) bleibt `stairsUp` regulär ungebaut. Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Der Voll-Stand hat die Tiefen der Welt-Datei, `islandDepths` entfällt, wenn nichts anderes … |
| AC-02 | erfüllt | Der Voll-Stand hält die Ausbauregeln ein, die Tests aus SV1.3 bleiben grün (B-425/AC-01, B … |
| AC-03 | geprüft | Ein Go-Test legt `voll-<seed>` über einen echten `store.Saves` an; `/api/dev/fullsave` prü … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-500 | erledigt | AC-01 geprüft | Voll ausgebauter Spielstand nimmt Tiefen aus den Biomen statt aus der Standard-W … |
| B-425 | erledigt | AC-02 erfüllt | Der Voll-Ausbau-Stand hält die Ausbauregeln des Spiels ein |
| B-426 | erledigt | AC-03 geprüft | Kleine Befunde aus dem Review von SV1 sind erledigt |
