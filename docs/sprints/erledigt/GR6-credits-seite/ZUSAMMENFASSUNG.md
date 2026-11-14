# GR6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Credits-Seite
- **Projekt:** –

## Ziel

Die Credits-Seite entsteht aus den CREDITS-Dateien, ein Test sichert Vollständigkeit. Am Ende sichtbar: Seite mit allen Einträgen auf der Landingpage, roter Test bei fehlendem Eintrag.

## Ergebnis

2026-10-03, Agent (Claude Sonnet) in GR6.3. AC-01, AC-02: Ergebnis GR6.1; AC-03, AC-04: Ergebnis GR6.2 (`src/tools/credits.test.ts`, `task check` grün, 672 Tests). Sichtprüfung am TV: angenommen, Validierung offen (🧑). Befunde: keine schweren; CC-BY-Namensnennungen (Warped Caves, LPC-Reittiere) unverändert in den CREDITS-Dateien und auf der Seite, keine zweite Kopie, Test liest die echten Verzeichnisse. Neue Tickets: keine. Ein PR je Sprint. Version: v0.7.0 vorgeschlagen (Minor: neue Credits-Seite aus den CREDITS-Dateien mit Vollständigkeits-Test; aktuell v0.5.0, davor stehen F4 v0.6.0 und X1 …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Jedes Verzeichnis unter `public/grafik/` und `public/sprites/` hat einen Credit-Eintrag, e … |
| AC-02 | – | Die Seite zeigt alle Einträge der CREDITS-Dateien, erzeugt aus den CREDITS-Dateien als ein … |
| AC-03 | – | CC-BY-Einträge zeigen Urheber, Lizenz und Quelle (B-165/AC-03). |
| AC-04 | geprüft | Die Seite ist über die Landingpage erreichbar, `task check` ist grün (B-165/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-165 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 geprüft | Eine Credits-Seite entsteht aus den CREDITS-Dateien, ein Test prüft die Vollstän … |
