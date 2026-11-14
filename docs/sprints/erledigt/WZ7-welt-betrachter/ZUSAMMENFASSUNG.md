# WZ7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Welt-Betrachter im Asset-Studio mit Chunk- und Live-Vorschau
- **Projekt:** WZG

## Ziel

Der Welt-Betrachter läuft im Asset-Studio statt in `leveltest.html`: Chunk-Vorschau beim Öffnen maximiert, darunter die Live-Vorschau mit Spielgrafik, umschaltbarer Hub-Stufe, Gebäuden, Tageszeit, Hintergrund-Schichten (ohne Bürger und Gegner), dazu „Im Spiel öffnen“. Am Ende sichtbar: im Studio-Fenster die Seite Welt-Betrachter mit beiden Vorschauen; auf der Entwicklerseite keine Kachel „Level“ m …

## Ergebnis

2026-10-18 (Abnahme WZ7.7 durch 🧑 am PC): AC-01 bis AC-08 mit Nachweis aus WZ7.6; AC-09 verschoben nach B-477 (Mängel der Ansicht, Sprint WZ16). B-092 verworfen, B-426 Punkt 1 und 4 verworfen. Neue Tickets: B-477. Version: v0.19.0 gesetzt (2026-10-18, Minor wegen Wirkung im Werkzeug, zusammen mit WZ16; Tag auf `ea750844`, Release-Lauf grün, `/api/health` und Landingpage im Nachbau mit den Flags des Release-Workflows v0.19.0).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-460/AC-01` Chunk-Vorschau im Asset-Studio mit dem Umfang von `leveltest.html`, ohne Spi … |
| AC-02 | geprüft | `B-460/AC-02` Chunk-Vorschau beim Öffnen maximiert, verkleinerbar. |
| AC-03 | geprüft | `B-460/AC-03` exportierte Go-Funktion (SRV) mit Options-Struktur, ohne globalen Zustand, l … |
| AC-04 | geprüft | `B-460/AC-04` Live-Vorschau mit Spielgrafik auf wiederverwendbarer Phaser-Bühne, Hub-Stufe … |
| AC-05 | geprüft | `B-460/AC-05` Tageszeit (vier Einträge Tag, Dämmerung, Nacht, Morgengrauen; Setzfunktion s … |
| AC-06 | geprüft | `B-460/AC-06` „Im Spiel öffnen“ neu oder voll ausgebaut über die Landingpage, nur Loopback … |
| AC-07 | geprüft | `B-460/AC-07` `leveltest.html` mit Skript, Logik, Tests, Kacheln und Texten entfernt; `git … |
| AC-08 | geprüft | `B-460/AC-08` Tests in `task check:all` und CI. |
| AC-09 | verschoben | 🧑 nimmt den Welt-Betrachter am PC ab und entscheidet über die Umplanung von B-092 und die … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-460 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft, AC-07 geprüft, AC-08 geprüft | Der Welt-Betrachter im Asset-Studio zeigt Chunks maximiert und eine Live-Vorscha … |
