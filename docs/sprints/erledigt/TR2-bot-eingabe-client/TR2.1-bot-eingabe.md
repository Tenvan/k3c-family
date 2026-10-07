# TR2.1 · `BotInput` und `?botfeed` im Client

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** tr2/1-bot-eingabe
- **Abhängig von:** –
- **Tickets:** B-349
- **Kriterien:** AC-01, AC-02

## Ziel

`src/input/` hat eine `BotInput`, die je Slot die Aktionen aus dem Bot-Feed liefert; `game.html?botfeed=…&players=n` startet mit n lokalen Spielern, deren Eingabe der Feed ist.

## Kontext

- Eingabe-Abstraktion: `src/input/` (`PlayerInput`, Tastatur, Gamepad, `touchInput.ts`); `src/scenes/GameScene` fragt Aktionen ab, nie Tasten. Wie Touch per `?touch=1` erzwungen wird, ist das Muster für `?botfeed`.
- Feed-Nachricht: B-349 › Notizen (`{"slot":0,"moveX":…,"sprint":…,"pay":…,"attack":…,"skill":0}`). Die letzte Nachricht je Slot gilt bis zur nächsten.
- Erlaubte Feed-Adressen: `ws://127.0.0.1:*`, `ws://localhost:*` oder der Host der Seite; sonst ablehnen und mit `🚫` loggen (`clientLog`).
- Wie viele lokale Spieler ein Gerät öffnet und wie Raum-Beitritt per URL läuft, im Code nachsehen (`src/online/`, `src/scenes/`); fehlt ein Weg ohne CLI-Änderung, Ticket (CLI) und `blockiert` für diesen Teil.

## Erlaubte Dateien

- `src/input/` (neu `botInput.ts`, `botInput.test.ts`; Auswahl der Eingabe), `game.html` (nur falls nötig)
- `docs/sprints/geplant/TR2-bot-eingabe-client/`, `docs/sprints/aktiv/TR2-bot-eingabe-client/` (Status), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Szenen, Anzeige, Bot-Entscheidung, Workbench.

## Schritte

1. Sprint aktivieren (Ordner nach `aktiv/`, Fahrplan, `Start-Commit`), Branch, `Status: in Arbeit`.
2. `BotInput` mit Feed-Verbindung, Wiederverbindung und Aktionen je Slot.
3. Auswahl über `?botfeed` und `?players`.
4. Tests: Aktionen aus Nachrichten, ohne Feed keine, fremder Host abgelehnt, ohne Parameter unverändert.
5. `task check`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [x] AC-01: Test grün.
- [x] AC-02: Test grün.

## Prüfen

```bash
task check
```

## Ergebnis

- **AC-01** (B-349/AC-01): umgesetzt, geprüft mit `src/input/botInput.test.ts` (Aktionen je Slot aus Feed-Nachrichten, letzte Nachricht gilt weiter, `justPressed` nur im ersten Frame; ohne Feed oder nach Abbruch keine Aktion, neuer Versuch nach 1 s; ungültige Nachrichten verworfen).
- **AC-02** (B-349/AC-02): umgesetzt, geprüft mit `src/input/botInput.test.ts` (fremder Host, `http:` und kaputte Adresse abgelehnt, kein Socket; ohne `?botfeed` keine Bot-Eingabe; Loopback, `[::1]` und Host der Seite erlaubt, `players` 1–4).
- `src/input/botInput.ts`: `parseBotFeed`, `BotFeed` (WebSocket, letzte Nachricht je Slot, Wiederverbindung, Log 🔌/👋/🚫), `BotInput` (`PlayerInput`: Bezahlen = `confirm`, Schlag = `attack`, Skill k = `skill<k>`), `botInputs(search, host)` als Auswahl.
- **Schritt 3 blockiert für die Einbindung:** `GameScene.allInputs()` kennt nur Tastatur, Pads und Touch, `LocalSlots.join` bindet erst nach `confirm`, die Lobby tritt mit `?room=` nur mit Slot 0 bei. Das ändert nur `src/scenes/` (CLI, hier nicht erlaubt) → **B-353**. Bis dahin bewegen sich im Browser noch keine Monarchen (Sprint › „Am Ende sichtbar“).
- k3c-dev meldete `Checkout: Repo-Wurzel` (Header fehlt, B-341): Planung von Hand gepflegt, Prüfung mit `task check` in der Shell, grün (80 Dateien, 1526 Tests).
