# GEB13 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Gebäude-Stufen in den Daten und Stufen-Look für Plantage, Camp, Heilplatz
- **Projekt:** GEB

## Ziel

Gebäude tragen eigene `levels` in `data/buildings.json`, die Grafik folgt der Stufe. Am Ende sichtbar: Plantage, Camp und Heilplatz am TV je Stufe unterscheidbar.

## Ergebnis

2026-10-24: AC-01 (GEB13.1), AC-02 und AC-03 (GEB13.2) umgesetzt; AC-04 (GEB13.3): Ralf hat Pappel, Camp und Heilplatz abgenommen, alle ok. Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `levels` je Gebäude in `data/buildings.json`, Test (B-569/AC). |
| AC-02 | – | Plantage hat eine niedrige Pappel vor der Farm (B-568/AC). |
| AC-03 | – | Camp und Heilplatz zeigen je Stufe einen eigenen Look (B-570/AC, B-571/AC). |
| AC-04 | geprüft | 🧑 nimmt die Looks am TV ab. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-569 | erledigt | AC-01 – | Gebäude bekommen eigene Stufen (levels je Gebäude) |
| B-568 | erledigt | AC-02 – | Plantage bekommt eine eigene niedrige Pappel vor der Farm |
| B-570 | verworfen | AC-03 – | Das Camp bekommt einen Stufen-Look statt der Grundform |
| B-571 | verworfen | AC-03 – | Heilplatz-Stufen sind deutlicher sichtbar (Zelt, Brunnen) |
