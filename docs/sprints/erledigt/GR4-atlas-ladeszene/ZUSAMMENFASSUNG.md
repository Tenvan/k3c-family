# GR4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Atlas und Lade-Szene
- **Projekt:** –

## Ziel

Die Grafiken kommen aus Atlanten, die Lade-Szene zeigt Fortschritt, der Kaltstart hat ein gemessenes Budget. Am Ende sichtbar: `task atlas`, ein Balken beim Start, ein Messwert vom TV.

## Ergebnis

2026-10-03, Review GR4.4 (autonom). AC-01 (GR4.1), AC-02, AC-03, AC-05, AC-06 (GR4.2) mit Nachweis in den Session-Ergebnissen; AC-04: angenommen, Validierung offen (GR4.3, Xbox-Messung, im Fahrplan unter „Offen am Gerät“). Befunde behoben: B-196 (`setup-go` in ci.yml, deploy-pages.yml, release.yml) und das Dockerfile, das ohne Atlas baute (Image hätte „Laden fehlgeschlagen“ gezeigt). Keine neuen Tickets; B-029, B-163, B-196 archiviert. Version: v0.6.0 vorgeschlagen (Minor: neues Werkzeug `task atlas`, Lade-Szene im Spiel; F4 und F5 schlagen ebenfalls v0.6.0 vor, die Nummer vergibt 🧑 beim Relea …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `task atlas` erzeugt Atlas-Bild und -Beschreibung, zweimaliger Lauf liefert byte-gleiche D … |
| AC-02 | – | Das Spiel lädt Figuren und eingebaute Umgebungs-Grafiken aus Atlanten, die Zahl der Reques … |
| AC-03 | geprüft | Beim Start zeigt eine Lade-Szene den Fortschritt, bis alle Assets geladen sind (B-029/AC-0 … |
| AC-04 | – | Das Kaltstart-Budget ist festgelegt und am TV gemessen, die maximale Texturgröße der Xbox … |
| AC-05 | geprüft | `task check` und `task build` sind grün (B-163/AC-04). |
| AC-06 | geprüft | Lädt ein Asset nicht, zeigt die Lade-Szene eine Meldung mit Dateinamen statt eines hängend … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-163 | erledigt | AC-01 –, AC-02 –, AC-04 –, AC-05 geprüft | Die Spiel-Grafiken kommen aus einem Atlas, der Kaltstart hat ein Zeitbudget |
| B-029 | erledigt | AC-03 geprüft, AC-06 geprüft | Lade-Szene zeigt Fortschritt |
