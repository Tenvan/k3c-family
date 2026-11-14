# TR3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bot-Eingabe im Spiel einbinden
- **Projekt:** TST

## Ziel

Ein Client mit `?room=…&players=n&botfeed=…` tritt mit n lokalen Spielern bei, und jeder folgt sofort seinem Bot. Am Ende sichtbar: `sim_test` mit `clients: 1`, `players: 2` zeigt bewegte Monarchen des Clients im Lauf-Bericht.

## Ergebnis

2026-10-08: AC-01/AC-02 geprüft (TR3.1, Tests), AC-03 geprüft in TR3.2 (`sim_test` run-1, Pass). Keine schweren Befunde. Neues Ticket: B-362 (Token des Spielservers in k3c-dev). Version: v0.15.1 vorgeschlagen (Patch: Abnahme ohne neue Funktion, Code kam mit #217).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Bot-Slots ohne `confirm` gebunden, senden die Bot-Kommandos (B-353/AC-01). |
| AC-02 | geprüft | `?room=…&players=n` tritt mit n Slots bei; ohne Parameter unverändert (B-353/AC-02). |
| AC-03 | geprüft | `sim_test` mit Client: Monarchen bewegen sich (B-353/AC-03). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-353 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft | Das Spiel fragt die Bot-Eingabe ab und setzt ihre Spieler ohne Tastendruck in de … |
