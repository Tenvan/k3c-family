# U2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Level-Abfrage per HTTP
- **Projekt:** –

## Ziel

`GET /api/level?seed=…&biome=…` liefert das Level ohne Raum und ohne Simulation. Am Ende sichtbar: `curl` auf den Server liefert zweimal dieselbe JSON-Antwort, unbekanntes Biom ergibt 400.

## Ergebnis

- 2026-10-02, Review U2.2 (Agent): AC-01 bis AC-04 belegt (U2.1 › Ergebnis, nachgeprüft in U2.2 › Ergebnis). - AC-03 über `room.Manager.Rooms()` statt `/api/status` belegt: gleichwertig, beide lesen dieselbe Raumliste (`Manager.rooms`). - Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Antwort stimmt mit `engine/level` überein und ist wiederholbar (B-091/AC-01). |
| AC-02 | geprüft | 400 und 405 wie spezifiziert (B-091/AC-02). |
| AC-03 | geprüft | Kein Raum und keine Datei entstehen (B-091/AC-03). |
| AC-04 | geprüft | README nennt den Aufruf, `task check:go` grün (B-091/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-091 | erledigt | AC-01 –, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft | Der Server liefert ein generiertes Level per HTTP, ohne einen Raum anzulegen |
