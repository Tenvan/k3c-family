# B-353 · Das Spiel fragt die Bot-Eingabe ab und setzt ihre Spieler ohne Tastendruck in den Raum

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** TR3
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑

## Ausgangslage

TR2.1 hat `src/input/botInput.ts` gebaut: `botInputs(location.search, location.hostname)` liefert bei `?botfeed=…&players=n` je lokalem Spieler eine `BotInput` (Slot 0…n-1), sonst nichts. `GameScene.allInputs()` kennt aber nur Tastatur, Pads und Touch. `LocalSlots.join` bindet einen Slot erst, wenn eine Eingabe `confirm` drückt, und die Lobby tritt mit `?room=` nur mit Slot 0 (plus `?mock`) bei. Ein Client aus `sim_test` (TR1.3) sitzt deshalb im Raum, aber seine Monarchen bewegen sich nicht.

## Ziel

`game.html?room=<Code>&players=n&botfeed=<Adresse>` tritt mit n lokalen Spielern bei, und jeder Slot folgt sofort seiner `BotInput`. Damit zeigt `sim_test` mit `clients` den Browser-Nachweis aus TR1 und TR2.

## Beteiligte und Zielgruppen

Workbench (`sim_test`), Agenten und 🧑 bei Testläufen.

## Anforderungen

- `GameScene` nimmt die Bot-Eingaben aus `botInputs()` zu ihren Eingaben, Slot i wird fest an Bot i gebunden, ohne auf `confirm` zu warten.
- Mit `?room=` und `?players=n` tritt die Lobby mit den Slots 0…n-1 bei (wie heute `slotsFor(mock)`).
- Ohne `?botfeed` ändert sich nichts.

## Nicht-Ziele

Bot-Entscheidung im Client, neue Anzeigen, Änderungen an `src/input/botInput.ts` (PLAT, TR2).

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/`). Regeln aus `CLAUDE.md` (Eingabe nur über `PlayerInput`, 2+ Spieler, B unbelegt).

## Beispiele

`game.html?room=ABCD&players=2&botfeed=ws://127.0.0.1:5180/bot/4/0` → zwei Monarchen im Raum ABCD, beide laufen nach den Bot-Kommandos.

## Ausnahme- und Fehlerfälle

Feed weg → die Bot-Slots bleiben gebunden und stehen (die `BotInput` meldet keine Eingabe), bis der Feed wieder da ist.

## Akzeptanzkriterien

- **AC-01** Test: Mit Bot-Eingaben sind die Slots 0…n-1 ohne `confirm` gebunden und senden die Kommandos der Bots.
- **AC-02** Test: `?room=…&players=n` tritt mit n Slots bei; ohne `?botfeed` und `?players` unverändert.
- **AC-03** `sim_test` mit `clients: 1`, `players: 2`: die Monarchen des Clients bewegen sich (Feed-Zähler und Positionen im Lauf-Bericht).

## Offene Fragen

keine

## Notizen

Entstanden in TR2.1 (Session: „fehlt ein Weg ohne CLI-Änderung, Ticket (CLI)“).
