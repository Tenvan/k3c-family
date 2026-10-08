# TR3 · CLI · Bot-Eingabe im Spiel einbinden

- **Status:** aktiv
- **Projekt:** –
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-353
- **Start-Commit:** 3a4d923
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1 (mit B-353)

## Ausgangslage

TR2.1 hat `BotInput` und `botInputs()` in `src/input/botInput.ts` gebaut. Das Spiel fragt sie aber noch nicht ab, bindet Slots erst nach `confirm`, und die Lobby tritt mit `?room=` nur mit einem Spieler bei (B-353). Deshalb bewegen sich im Browser eines `sim_test`-Laufs keine Monarchen.

## Ziel

Ein Client mit `?room=…&players=n&botfeed=…` tritt mit n lokalen Spielern bei, und jeder folgt sofort seinem Bot.

Am Ende sichtbar: `sim_test` mit `clients: 1`, `players: 2` zeigt bewegte Monarchen des Clients im Lauf-Bericht.

## Beteiligte und Zielgruppen

Workbench (`sim_test`), Agenten und 🧑 bei Testläufen.

## Anforderungen

B-353 › Anforderungen.

## Nicht-Ziele

Bot-Entscheidung im Client, neue Anzeigen, Änderungen an `src/input/botInput.ts` (PLAT, TR2) und an der Workbench (SRV).

## Regeln und Einschränkungen

Domäne CLI (`src/scenes/`). Regeln aus `CLAUDE.md` (Eingabe nur über `PlayerInput`, 2+ Spieler, B unbelegt). `GameScene.ts` hat schon 403 Zeilen: Logik nach `localSlots.ts`/`lobbyLogic.ts`, in der Szene nur verdrahten. Einschiebbar, weil TR1/TR2 erst damit ihren Browser-Nachweis bekommen. Setzt TR2 auf `develop` voraus.

## Beispiele

B-353 › Beispiele.

## Ausnahme- und Fehlerfälle

B-353 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Bot-Slots ohne `confirm` gebunden, senden die Bot-Kommandos (B-353/AC-01).
- **AC-02** `?room=…&players=n` tritt mit n Slots bei; ohne Parameter unverändert (B-353/AC-02).
- **AC-03** `sim_test` mit Client: Monarchen bewegen sich (B-353/AC-03).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| TR3.1 | `TR3.1-bot-eingabe-einbinden.md` | Umsetzung | autonom | fertig |
| TR3.2 | `TR3.2-review.md` | Review | autonom | offen |

## Abnahme

–
