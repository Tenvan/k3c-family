# WZ37 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Welt-Overrides im Welten-Editor und im Studio-Chat
- **Projekt:** WZG

## Ziel

Im Welten-Editor und im Chat lassen sich Overrides einer Welt setzen, zurücksetzen und gegen den Basiswert vergleichen. Am Ende sichtbar: Editor zeigt je Wert „geerbt“ oder „überschrieben“; Chat setzt einen Override als Aktionskarte.

## Ergebnis

- 2026-11-02, Review WZ37.3: AC-01 und AC-02 belegt (Details in `WZ37.1` und `WZ37.2` › Ergebnis, Gegenprobe in `WZ37.3` › Ergebnis). - Studio-Go-Tests und Vitest grün; ein Lastflake in `ChunkCount.test.tsx` (isoliert grün, nicht aus WZ37). - Offen bei Ralf: B-682/AC-03 (Abnahme im Studio am PC). Keine Befunde, keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Der Editor übersteuert und setzt zurück, zeigt geerbt und überschrieben (`B-682/AC-01` bis … |
| AC-02 | geprüft | Der Chat liest, setzt und entfernt Overrides als Aktionskarte (`B-683/AC-01`, `AC-02`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-682 | erledigt | AC-01 geprüft | Der Welten-Editor bearbeitet Werte-Overrides einer Welt und zeigt Basiswert gege … |
| B-683 | erledigt | AC-02 geprüft | Der Studio-Chat kann Welt-Overrides lesen, setzen und zurücksetzen |
