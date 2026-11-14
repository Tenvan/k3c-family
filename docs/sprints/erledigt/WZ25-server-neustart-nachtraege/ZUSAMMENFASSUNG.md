# WZ25 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Nachträge zum Server-Neustart: Räume bleiben offen, Assets ohne Build, Werkzeuge passen dazu
- **Projekt:** WZG

## Ziel

Ein Neustart des Servers nach einer Studio-Änderung unterbricht das Spiel nur kurz: Räume sind danach wieder offen, neue Grafiken und Sounds sind sofort da, und die Werkzeuge spielen sauber mit. Am Ende sichtbar: Grafik im Studio ändern → Geräte kehren nach dem Neustart in ihren Raum zurück und zeigen die neue Grafik ohne `task build`.

## Ergebnis

2026-10-22: AC-01 bis AC-05 nachgewiesen (WZ25.1–WZ25.5), Go-Tests und Vitest grün, Reviewer ohne blockierende Befunde (Merkdatei-Alter, Punkt-Ordner, Temp-Dateien im Watcher behoben). Für Ralf im Nachgang: B-509 `public/` vor `dist/` mit Last-Modified/no-cache; B-508 Merkdatei `restart-rooms.state` (2 min gültig); Rückkehr der Geräte in den Raum am echten Gerät nicht geprüft (manuelle Abnahme).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Nach einem Neustart wegen geänderter Dateien sind die vorher offenen Räume wieder offen, G … |
| AC-02 | geprüft | Eine im Studio geänderte Grafik oder ein Sound erreicht die Clients nach dem Neustart ohne … |
| AC-03 | geprüft | k3c-dev startet den Spielserver bei Änderungen in `data/` nicht mehr selbst neu (B-510). |
| AC-04 | geprüft | k3c-dev beendet einen übernommenen Spielserver auch nach dessen Selbst-Neustart vollständi … |
| AC-05 | geprüft | Jede Datei, die das Studio in `data/` oder `public/` schreibt, entsteht per Temp-Datei und … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-508 | erledigt | AC-01 geprüft | Offene Räume sind nach einem Neustart des Servers wieder offen, Geräte kehren oh … |
| B-509 | erledigt | AC-02 geprüft | Im Studio geänderte Grafiken und Sounds erreichen die Clients ohne neuen Build |
| B-510 | erledigt | AC-03 geprüft | k3c-dev startet den Spielserver bei Änderungen in data/ nicht mehr selbst neu, d … |
| B-512 | erledigt | AC-04 geprüft | k3c-dev beendet einen übernommenen Spielserver auch nach dessen Selbst-Neustart … |
| B-511 | erledigt | AC-05 geprüft | Das Studio schreibt jede Datei in data/ und public/ atomar, der Server sieht nie … |
