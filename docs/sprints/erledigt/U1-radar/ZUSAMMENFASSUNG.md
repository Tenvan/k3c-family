# U1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Radar im HUD
- **Projekt:** –

## Ziel

Jeder Spieler sieht in seiner Zelle eine Radar-Leiste über das ganze Level. Am Ende sichtbar: im Spiel mit zwei Spielern und einer Nacht-Welle wandern die Gegner-Marker vom Portal zur Burg, die Monarchen sind unterscheidbar.

## Ergebnis

- 2026-10-02, Review U1.3 (Agent): AC-01 belegt (U1.1 › Ergebnis), AC-02 und AC-03 belegt (U1.2 › Ergebnis, nachgeprüft in U1.3 › Ergebnis). - AC-04 offen: wartet auf Abnahme durch 🧑 am TV. - Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Marker-Funktion getestet (B-090/AC-01). |
| AC-02 | – | Radar in allen Layout-Zellen, Home-Button-Streifen frei (B-090/AC-02). |
| AC-03 | geprüft | `task check` grün, `src/scenes` ohne Spiel-Logik (B-090/AC-03). |
| AC-04 | offen | 🧑 hat das Radar am TV abgenommen (B-090/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-090 | verworfen | AC-01 –, AC-02 –, AC-03 geprüft, AC-04 offen | Ein Radar im HUD zeigt Burg, Portale, Ausgang, Mitspieler und Gegner |
