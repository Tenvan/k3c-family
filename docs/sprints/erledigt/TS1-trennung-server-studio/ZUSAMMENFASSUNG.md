# TS1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Spielserver und Asset-Studio trennen, Server beobachtet Dateien
- **Projekt:** WZG

## Ziel

Server und Studio sind getrennt, der Server beobachtet die Dateien des Studios, Pages baut nur `site/`. **Am Ende sichtbar:** `go list`-Test zeigt keine Importe von `tools/` in `engine/` und `cmd/k3c-server`; `/studio/` antwortet 404; eine im Studio gespeicherte Welt erscheint im laufenden Server; Studio-Vorschau läuft ohne Server; von 🧑 am PC abgenommen.

## Ergebnis

2026-10-20 (Review TS1.6): AC-01 bis AC-06 geprüft, siehe Session-Ergebnisse TS1.1 bis TS1.5. Behoben im Review: Neustartschleife bei Dateien mit Änderungszeit in der Zukunft (Watch) und Schleifenbremse im Supervisor. Neue Tickets: B-508 bis B-512 (aus TS1.4); offene Frage zur Abnahme: Soll `public/` im Produktivbetrieb einen Neustart auslösen (B-509)? 2026-10-20 (Abnahme TS1.7): AC-07 von 🧑 am PC abgenommen (Quelle Chat); Befund B-513. Version: v0.21.0 vorgeschlagen (Server/Studio getrennt, Datei-Watch mit Selbst-Neustart), gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Studio-Backend in `tools/k3c-studio/`, Import-Regel als Test (`B-507/AC-01`) – TS1.1 |
| AC-02 | geprüft | Studio-Vorschau und Spielstart ohne Spielserver (`B-507/AC-02`) – TS1.2 |
| AC-03 | geprüft | Keine Studio-Seite im Server (`B-507/AC-03`) – TS1.3 |
| AC-04 | geprüft | Datei-Watch mit Neuladen oder verzögertem Neustart und Hinweis (`B-507/AC-04`) – TS1.4 |
| AC-05 | geprüft | Pages nur `site/` (`B-507/AC-05`) – TS1.5 |
| AC-06 | geprüft | Entscheidung, Doku, WZ19.2-Text (`B-507/AC-06`) – TS1.5 |
| AC-07 | – | Abnahme durch 🧑 am PC (`B-507/AC-07`) – TS1.7 |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-507 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft, AC-07 – | Spielserver und Asset-Studio sind getrennt, der Server beobachtet die Dateien de … |
