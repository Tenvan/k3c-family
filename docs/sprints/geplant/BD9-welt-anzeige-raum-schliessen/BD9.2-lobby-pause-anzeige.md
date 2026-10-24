# BD9.2 · Client: Welt im Pausenmenü und in der Lobby, Raum schließen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** bd9/2-client
- **Abhängig von:** BD9.1
- **Tickets:** B-604, B-581
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Das Pausenmenü zeigt die Welt in 40 px, jeder Lobby-Eintrag nennt seine Welt, und Entfernen auf einem offenen Raum schließt ihn nach Rückfrage.

## Kontext

- Pausenmenü: `src/scenes/OptionsScene.ts:98`, `worldLine` in `optionsLogic.ts`, `FONT_PX = 40`.
- Lobby: `src/scenes/lobbyLogic.ts` (`lobbyEntries`, `entryLabel`, `worldOf`), `LobbyScene.ts` (Entfernen in `updateDialog`, `remove`), `saveMenu.ts` (`DeleteDialog`), `src/core/saveStore.ts` (`deleteSave` als Muster für `closeRoom`).
- Server-Seite aus BD9.1: `rooms[].world`, `POST api/room/close?slot=NAME`.

## Erlaubte Dateien

- `src/scenes/OptionsScene.ts`, `optionsLogic.ts` und Tests
- `src/scenes/lobbyLogic.ts`, `LobbyScene.ts`, `saveMenu.ts` und Tests
- `src/core/saveStore.ts` (oder neue `src/core/roomClose.ts`) und Test
- Texte in den Sprachdateien von `src/core/texts` bzw. `src/scenes/texts`

## Nicht-Ziele

Server (BD9.1); automatisches Löschen.

## Schritte

1. Weltzeile 40 px, Abstand zu den Optionen prüfen (oben 70 px frei).
2. `entryLabel` nutzt `room.world` für Raum-Einträge.
3. Entfernen auf einem Raum-Eintrag: Dialog „Raum schließen?“ (Abbrechen vorgewählt, nennt verbundene Geräte), Aufruf `closeRoom`, Meldung, Liste neu laden.
4. Vitest: Layout-Wert, `entryLabel` mit Welt, Dialog-Logik, `closeRoom` (200/404).

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03 (Client): Vitest grün.
- [ ] `task check:diff` grün; Review-Agent (Eingabe-Code).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
