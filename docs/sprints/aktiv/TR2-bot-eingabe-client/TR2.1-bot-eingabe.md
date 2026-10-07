# TR2.1 · `BotInput` und `?botfeed` im Client

- **Status:** in Arbeit
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

- [ ] AC-01: Test grün.
- [ ] AC-02: Test grün.

## Prüfen

```bash
task check
```

## Ergebnis

–
