# U4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Debug-Overlay
- **Projekt:** –

## Ziel

Ein umschaltbares Overlay zeigt Verbindung, Snapshot-Takt, Raum, Gerät und Entitäten. Am Ende sichtbar: in der Entwicklungsphase Overlay per Taste ein- und ausblenden (`?dev=0` schaltet es ganz ab).

## Ergebnis

- 2026-10-02, Review U4.3 (Agent): AC-01 belegt (U4.1 › Ergebnis), AC-02 und AC-03 belegt (U4.2 › Ergebnis, Revision 2: Ö, dev standardmäßig an). - AC-04 abgenommen: 🧑 hat das Overlay am Gerät geprüft (Chat, 2026-10-02). - Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: B-098 (dev-Standard vor dem Release zurücknehmen).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Zeilen-Funktion getestet (B-093/AC-01). |
| AC-02 | – | Overlay standardmäßig verfügbar, mit `?dev=0` nicht erzeugt, per Tastatur und Controller u … |
| AC-03 | – | FPS-Anzeige aufgegangen, `task check` grün, keine Spiel-Logik in `src/scenes` (B-093/AC-03 … |
| AC-04 | – | 🧑 hat das Overlay am Gerät abgenommen (B-093/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-093 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Ein Debug-Overlay zeigt Verbindung, Snapshot-Takt und Entitäten im Spiel |
