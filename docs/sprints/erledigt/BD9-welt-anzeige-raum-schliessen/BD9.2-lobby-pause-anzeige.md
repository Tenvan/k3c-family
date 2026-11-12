# BD9.2 · Client: Welt im Pausenmenü und in der Lobby, Raum schließen

- **Status:** fertig
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

- AC-01 umgesetzt, geprüft: Weltzeile im Pausenmenü 40 px (`OPTIONS_LAYOUT` in `optionsLogic.ts`: Titel y=110, Welt y=182, erste Option y=250; oben 70 px frei, keine Überlappung mit dem Hinweis), Vitest der Layout-Werte.
- AC-02 umgesetzt, geprüft: Raum-Einträge der Lobby zeigen `worldOf(room.world)` (Protokoll v9 aus BD9.1), Spielstände wie bisher; Vitest von `entryLabel`.
- AC-03 umgesetzt, geprüft: Entf/X auf einem Raum öffnet „Raum schließen?“ (Abbrechen vorgewählt, nennt belegte Plätze), `closeRoom(slot)` ruft `POST api/room/close`, Meldungen für ok/nicht gefunden/Fehler, Spielstände neu geladen (Raumliste schickt der Server); Texte de/en; Vitest von Dialog und `closeRoom`.
- `task check:diff` grün, Review-Agent ohne schwere Befunde; daraus geändert: Text „Belegte Plätze: N“ statt „N Plätze sind belegt“.
- Abweichung: `taken` zählt Plätze, nicht Geräte; die Rückfrage nennt deshalb Plätze.
