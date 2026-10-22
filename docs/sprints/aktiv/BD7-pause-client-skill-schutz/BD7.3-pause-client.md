# BD7.3 · Client: Optionen-Szene pausiert das Gerät am Server

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/bd7
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-04, AC-05

## Ziel

Solange die Optionen-Szene offen ist, ist das Gerät am Server pausiert: Öffnen sendet `{"t":"pause"}`, Schließen `{"t":"resume"}` (Protokoll v8), auch über Abbruch und Wiederbeitritt hinweg.

## Kontext

Server: `engine/room/pause.go` (BD3), Protokoll `docs/protocol.md` › v8, Beispiele `testdata/protocol/c2s-pause.json`, `c2s-resume.json`. Bricht die Verbindung ab, endet die Pause am Server. Client: `src/online/clientConnection.ts` (Zustand `room`/`reconnecting`, `clearRoom` bei `joined` und `leave`), Typen `src/online/clientProtocol.ts` (`ClientMessage`, `PROTOCOL_VERSION = 8`). `GameScene.update` (`src/scenes/GameScene.ts`) öffnet die Szene `options` und sendet während sie offen ist Leerlauf-Eingaben (`paused`). Keine Spiel-Logik im Client (`tests/noSim.test.ts`). Default B-535: nach Wiederbeitritt mit offenem Menü erneut `pause`.

## Erlaubte Dateien

- `src/online/clientPause.ts` (neu), `src/online/clientPause.test.ts` (neu)
- `src/online/clientConnection.ts`, `src/online/clientConnection.test.ts`, `src/online/clientProtocol.ts`
- `src/scenes/GameScene.ts`

## Nicht-Ziele

Server und Protokolländerung; Anzeige des Pause-Zustands anderer Geräte (B-529); Optik der Optionen-Szene.

## Schritte

1. Test zuerst: reine Funktion `pauseMessage(wanted, sent, inRoom)` → `'pause' | 'resume' | null` (`clientPause.test.ts`).
2. `ClientMessage` um `{ t: 'pause' }` und `{ t: 'resume' }` erweitern.
3. `ClientConnection.setPaused(wanted)`: sendet nach `pauseMessage`, merkt den gesendeten Zustand; `clearRoom` setzt ihn zurück (Wiederbeitritt, Verlassen). Test mit Mock-Socket: öffnen, halten, schließen, Abbruch und Wiederbeitritt mit offenem Menü.
4. `GameScene.update`: im Raum je Frame `client.setPaused(this.scene.isActive('options'))`.

## Fertig, wenn

- [ ] AC-04: `task test -- clientPause clientConnection` grün.
- [ ] AC-05: `GameScene` ruft `setPaused` je Frame im Raum; `task check:diff` grün (inkl. `noSim`).

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
