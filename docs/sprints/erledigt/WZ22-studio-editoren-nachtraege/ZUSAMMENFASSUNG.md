# WZ22 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Studio-Editoren: Fehler sichtbar, Im Spiel öffnen wiederholbar
- **Projekt:** WZG

## Ziel

Die Editoren des Studios melden jeden Fehler sichtbar, und „Im Spiel öffnen“ ist in jeder Welt wiederholbar. Am Ende sichtbar: zweimal „Im Spiel öffnen“ in `nordmeer` ohne Fehlermeldung.

## Ergebnis

2026-10-22: AC-01 bis AC-03 nachgewiesen (WZ22.1–WZ22.3), `check:diff` grün, Reviewer ohne Befund. Im Nachgang für Ralf: B-499 Default `<welt>-<seed>` (Wiederholen lädt den Stand); B-491 Punkte 4, 5, 8, 10 verworfen (Gründe in WZ22.1).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Abgelehnte `planLoad` und `planCheck` zeigen ihren Grund am Editor, der Entwurf bleibt erh … |
| AC-02 | – | Die sieben Punkte aus B-491 sind behoben oder bewusst verworfen, je Punkt mit Test (B-491) … |
| AC-03 | geprüft | „Im Spiel öffnen“ startet in jeder Welt wiederholt ohne Fehler und lädt nie den Stand eine … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-487 | erledigt | AC-01 geprüft | Chunk-Editor: abgelehnte Aufrufe von planLoad und planCheck bleiben stumm |
| B-491 | erledigt | AC-02 – | Welten Editor: kleine Nachträge zu Schieber, Klick-Sprung und Planungsliste |
| B-499 | erledigt | AC-03 geprüft | Studio „Im Spiel öffnen“ in eigener Welt scheitert beim zweiten Mal mit demselbe … |
