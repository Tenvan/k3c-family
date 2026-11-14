# LB1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Lobby zeigt Räume und startet Spiele
- **Projekt:** BED

## Ziel

Die Lobby zeigt Räume und startet oder betritt ein Spiel per Controller. Am Ende sichtbar: Lobby listet offene Räume, Beitritt ohne Raumcode.

## Ergebnis

- 2026-10-10 (LB1.2, autonom): AC-01 mit Nachweis im Ergebnis von LB1.1 (B-037/AC-01 bis AC-03 aus den SP08-Tests von `LobbyFlow`/`lobbyEntries`, AC-04 aus „Spielstand wählen“ und `saveStore.test.ts`); `task check` grün. - Review des Diffs (eigener Agent): keine schweren Befunde; B frei, Lobby ohne `api/saves` bedienbar, Spielstand-Eintrag sendet nur `fresh: false`. - Neue Tickets: B-382 (Neuversuch nach `save_not_found` an „Spielen“ statt am Namen festmachen, ungültiges `savedAt` sortieren). - Version: v0.15.0 vorgeschlagen (Minor: Lobby zeigt Spielstände und startet sie per Controller); gese …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Lobby zeigt Räume und startet Spiele (B-037/AC-01, B-037/AC-02, B-037/AC-03, B-037/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-037 | erledigt | AC-01 – | Lobby zeigt Räume und startet Spiele |
