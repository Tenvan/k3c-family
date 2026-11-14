# TR2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bot-Eingabe im Client für Testläufe
- **Projekt:** –

## Ziel

Ein Client mit `?botfeed=…&players=n` nimmt die Eingaben seiner lokalen Spieler aus dem Bot-Feed der Workbench. Am Ende sichtbar: Test der `BotInput` grün; im Browser bewegen sich n Monarchen nach Bot-Kommandos (mit TR1.3).

## Ergebnis

2026-10-07: Kriterien siehe Ergebnisse TR2.1 und TR2.2 (AC-01, AC-02 per Test). Einbindung ins Spiel und Browser-Nachweis → B-353 (TR3). Keine schweren Befunde; leichte Befunde zu `botInput.ts` → B-354. Neue Tickets: B-353, B-354.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `BotInput` liefert je Slot die Aktionen aus dem Feed, ohne Feed keine (B-349/AC-01). |
| AC-02 | geprüft | Fremder Host abgelehnt, ohne Parameter unverändert (B-349/AC-02). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-349 | erledigt | AC-01 geprüft, AC-02 geprüft | Bots steuern im Client die Monarchen über die Bot-Eingabe |
