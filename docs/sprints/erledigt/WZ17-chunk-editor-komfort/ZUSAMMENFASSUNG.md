# WZ17 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Chunk-Editor komfortabel: Inselgrößen, getrennter Zoom, Seed-Chunk umwandeln
- **Projekt:** WZG

## Ziel

Der Editor bietet durchsuchbare Auswahllisten mit Emoji-Icons, eine Chunk-Anzahl je Stufe über die Inselgrößen (sehr klein 16 · klein 20 · mittel 24 · groß 32 · sehr groß 40 · mega groß 48), die die Level-Länge im Spiel ändert, einen getrennten Zoom der Vorschauen (Alt + Mausrad, gleiche Weltposition) und die Umwandlung eines Seed-Chunks in feste Objekte (`B-485 › Ziel`). **Am Ende sichtbar:** im …

## Ergebnis

2026-10-18 (Review WZ17.6): AC-01 bis AC-06 und AC-08 mit Nachweis aus Code, Tests und getrenntem Reviewer (Ergebnisse WZ17.1–WZ17.6); AC-07 von 🧑 im laufenden Studio je Punkt mit „passt“ abgenommen (WZ17.4, WZ17.5, WZ17.7). Befunde: keine schweren. Neue Tickets: B-486 (behoben), B-487, B-489 (Spielstand und Chunk-Anzahl), B-490 (Sim-Nachträge), B-491 (Kleinigkeiten), B-496 (Fensterposition), B-497 (große Inseln, Last). Version: v… vorgeschlagen: keine Änderung (Werkzeug-Sprint; Golden, Spielstand-Format und Protokoll unverändert).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-485/Offene Fragen 1–3` Format und Regeln der Chunk-Anzahl und der Inselgrößen als REG-B … |
| AC-02 | geprüft | `B-485/AC-02` Ohne Chunk-Anzahl sind `level-*.json` und `sim-*.json` byte-gleich; mit den … |
| AC-03 | geprüft | `B-485/AC-03` Die Prüfung der Planung lehnt einen Platz außerhalb der Anzahl ab und nennt … |
| AC-04 | – | `B-485/AC-04` Größen-Vorauswahl setzt die Anzahl; Chunk- und Live-Vorschau zeigen die Läng … |
| AC-05 | – | `B-485/AC-01`, `B-485/AC-05` Durchsuchbare Listen mit Emoji-Icons; Alt + Mausrad zoomt nur … |
| AC-06 | – | `B-485/AC-06` Umwandlung eines Seed-Chunks in feste Objekte ohne Änderung des Bildes. |
| AC-07 | geprüft | `B-485/AC-07` 🧑 nimmt Bedienung, Größen-Auswahl, getrennten Zoom und Umwandlung am PC im l … |
| AC-08 | geprüft | Das Review weist jedes Kriterium nach; `task check:all` und `task check:go` sind grün, bes … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-485 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 –, AC-05 –, AC-06 –, AC-07 geprüft | Chunk-Editor im Welt-Betrachter komfortabler bedienen |
