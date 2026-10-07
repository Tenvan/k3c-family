# TR3.1 · Bot-Eingaben in GameScene und Lobby

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** tr3/1-bot-eingabe-einbinden
- **Abhängig von:** TR2
- **Tickets:** B-353
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

`GameScene` nimmt die Bot-Eingaben aus `botInputs()` und bindet Slot i fest an Bot i; die Lobby tritt mit `?players=n` mit n Slots bei; ein `sim_test`-Lauf mit Client zeigt bewegte Monarchen.

## Kontext

- `src/input/botInput.ts` (TR2): `botInputs(location.search, location.hostname)` liefert bei `?botfeed=…&players=n` je Slot 0…n-1 eine `BotInput` (`slot`, `PlayerInput`), sonst `[]`. Nicht ändern (PLAT).
- `src/scenes/GameScene.ts` (403 Zeilen, Grenze 400): `create()` baut Tastatur, Pads, Touch; `allInputs()` sammelt sie; `update()` ruft `this.slots.join(...)` und `this.slots.commands(seated)`. Nur verdrahten, Zeilen anderswo einsparen.
- `src/scenes/localSlots.ts`: `LocalSlots.join` bindet einen Slot erst nach `justPressed('confirm')`; hier einen Weg ergänzen, feste Eingaben je Slot vorzubinden (z. B. Konstruktor-Option), Bot-Slots fallen nie heraus. Tests in `localSlots.test.ts`.
- `src/scenes/lobbyLogic.ts`: `parseStartParams` kennt `room` und `mock`; `slotsFor(mock)` liefert die Slots beim Beitritt. `?players=n` (1–4) ergänzen, sodass `?room=…&players=n` mit Slots 0…n-1 beitritt; `LobbyScene.enterGame()` gibt Mock-Slots an `GameScene` weiter, Bot-Slots sind keine Mocks. Tests in `lobbyLogic.test.ts`.
- `sim_test` (TR1.3) öffnet `game.html?room=<Code>&players=<n>&botfeed=<Feed>`; Feed unter `tools/k3c-dev/internal/botfeed`.

## Erlaubte Dateien

- `src/scenes/GameScene.ts`, `src/scenes/LobbyScene.ts`, `src/scenes/localSlots.ts`, `src/scenes/localSlots.test.ts`, `src/scenes/lobbyLogic.ts`, `src/scenes/lobbyLogic.test.ts`
- `docs/sprints/geplant/TR3-bot-eingabe-spiel/`, `docs/sprints/aktiv/TR3-bot-eingabe-spiel/` (Status), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

`src/input/`, Workbench, Anzeigen, Bot-Entscheidung.

## Schritte

1. Sprint aktivieren (Ordner nach `aktiv/`, Fahrplan, `Start-Commit`), Branch, `Status: in Arbeit`.
2. `LocalSlots`: feste Eingaben je Slot ohne `confirm`, mit Test.
3. `lobbyLogic`: `?players=n` beim Beitritt, mit Test; ohne Parameter unverändert.
4. `GameScene`/`LobbyScene` verdrahten (Bot-Eingaben zu den Eingaben, fest gebunden).
5. `sim_test` mit `clients: 1`, `players: 2` starten (nur über das MCP-Tool), Feed-Zähler und Positionen im Bericht nachsehen.
6. `task check`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [x] AC-01: Test in `localSlots.test.ts` grün.
- [x] AC-02: Test in `lobbyLogic.test.ts` grün.
- [ ] AC-03: `sim_test`-Bericht mit bewegten Monarchen des Clients (verschoben auf TR3.2).

## Prüfen

```bash
task check
```

`sim_test` nur über k3c-dev; ohne laufende Workbench wartet AC-03 (`blockiert` mit Grund).

## Ergebnis

- **AC-01** (B-353/AC-01): umgesetzt, geprüft mit `localSlots.test.ts` (feste Eingaben binden Slot i ohne `confirm`, senden ihre Kommandos, ein echter Controller bekommt den nächsten freien Slot; ohne feste Eingaben unverändert).
- **AC-02** (B-353/AC-02): umgesetzt, geprüft mit `lobbyLogic.test.ts` (`?room=…&players=3` → Slots 0–2, ohne `?players` Slot 0, ungültige Werte = 1, `?autostart` mit `players`).
- **AC-03** verschoben auf TR3.2 (Entscheidung 🧑, 2026-10-07): k3c-dev meldet aus dem Worktree `Checkout: Repo-Wurzel` (B-341), `sim_test` würde den Code der Wurzel öffnen, nicht diesen. Die Review prüft den Lauf, sobald TR3 auf `develop` und im Wurzel-Checkout ist.
- Umsetzung: `LocalSlots` nimmt feste Eingaben je Slot (`fixed`), `pageBots()` holt die Bot-Eingaben einmal je Seite; `StartParams.players` und `slotsFor(mock, players)`; `GameScene` verdrahtet nur (keine neue Zeile, durch zusammengelegte Importe 399 statt 403 Zeilen). `LobbyScene` unverändert: Bot-Slots sind keine Mock-Slots.
- `task check` grün (80 Dateien, 1531 Tests), in der Shell wegen B-341.

