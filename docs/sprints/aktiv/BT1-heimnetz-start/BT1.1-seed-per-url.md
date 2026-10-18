# BT1.1 · Start mit Seed und Tiefe per URL, Nachweis B-048

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** bt1/1-seed-per-url
- **Abhängig von:** –
- **Tickets:** B-095, B-048
- **Kriterien:** AC-03, AC-04

## Ziel

`create` kennt ein optionales Feld `seed`, der Client reicht `?seed=` und `?depth=` durch, und Entscheidung 001 nennt die Standardbibliothek; alles belegt durch Tests und Protokoll-Beispiele.

## Kontext

Spec: B-095 (Anforderungen, AC-01 bis AC-03) und B-048, Sprint-Kriterien AC-03 und AC-04 in `docs/sprints/aktiv/BT1-heimnetz-start/README.md`. Stand: Protokollversion 6 (`engine/net/protocol*.go`, `ProtocolVersion`); `create` trägt `save`, `fresh`, `depth`, `slots`, optional `grade`, `goal`, `defeat` (`docs/protocol.md` Zeile „create“, Beispiel `testdata/protocol/c2s-create.json`). Der Seed eines neuen Spiels ist heute der Name des Spielstands (`engine/room/manager.go`, Kommentar „ponytail: Seed ist der Name“); der Client setzt beim Autostart immer `depth: 0` (`src/scenes/lobbyLogic.ts`, Funktion `parseStartParams`, `src/online/protocol.ts` für die Typen). Level zu Seed und Tiefe liefert `GET /api/level` (`engine/net/level.go`). Entscheidung 001 (`docs/decisions/001-server-engine-go.md`) nennt Go, Bubble Tea und Wails, aber nicht die Standardbibliothek; B-001/AC-01 verweist darauf. Protokoll-Änderungen laufen als eigene Session mit beiden Enden (Arbeitsweise › Domänen, Grenzfälle); die Rückwärtskompatibilität (kein `seed` → Name) und die Versionsangabe stehen in `docs/protocol.md`.

## Erlaubte Dateien

- `engine/net/protocol*.go`, `engine/room/manager.go` (und Tests dazu), `docs/protocol.md`, `testdata/protocol/`
- `src/online/protocol.ts`, `src/online/clientConnection.ts`, `src/scenes/lobbyLogic.ts` (+ Test), `src/scenes/LobbyScene.ts`
- `docs/decisions/001-server-engine-go.md`
- Planungsdateien dieses Sprints, B-095, B-048

## Nicht-Ziele

Kein Eingabefeld für Seed in der Lobby, keine neuen Tasten oder Menüs, keine Änderung am Spielstand-Format oder an der Simulation, keine Protokoll-Version-Erhöhung über das Nötige hinaus (neues optionales Feld ist rückwärtskompatibel).

## Schritte

1. Go: `create` um optionales `seed` erweitern (Text, höchstens 64 Zeichen, nur bei `fresh`); ohne `seed` bleibt der Name der Seed. Ungültiger Seed (zu lang) oder ungültige Tiefe → `bad_request` wie bisher. Go-Test: `create` mit `seed` und `depth` erzeugt dasselbe Level wie `GET /api/level` für dieselben Werte (B-095/AC-01).
2. Client: `src/online/protocol.ts` und `clientConnection.ts` (`create(save, fresh, depth, slots, seed?)`), `parseStartParams` wertet `?seed=` und `?depth=` aus; Vitest für `parseStartParams` (B-095/AC-02).
3. `docs/protocol.md` (Zeile „create“, Rückwärtskompatibilität, Fehlerfälle) und `testdata/protocol/c2s-create*.json` aktualisieren; ein Beispiel mit `seed` neu (B-095/AC-03).
4. `docs/decisions/001-server-engine-go.md`: ein Satz nennt die Standardbibliothek (`net/http`, `log/slog`) als Wahl (B-048/AC-01); `B-001/AC-01` bleibt.
5. `task check` und `task check:go` ausführen.

## Fertig, wenn

- [ ] AC-03: Go-Test (`create` mit `seed` und `depth` = `GET /api/level`), Vitest für `parseStartParams` (`?seed=`, `?depth=`), Protokoll-Beispiele und `docs/protocol.md` aktualisiert, ein Client ohne `seed` funktioniert weiter (B-095/AC-01 bis AC-03).
- [ ] AC-04: Entscheidung 001 nennt die Standardbibliothek, `B-001/AC-01` ist damit belegt (B-048/AC-01).
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task test -- lobbyLogic
task check
task check:go
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
