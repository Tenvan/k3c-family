# DBG1.1 · Protokoll: Nachricht `dev`, Fehlercode `forbidden`, Beispiele, beide Enden

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** dbg1/1-protokoll
- **Abhängig von:** –
- **Tickets:** B-178
- **Kriterien:** AC-01, AC-05

## Ziel

Die Client-Nachricht `dev` ist im Protokoll beschrieben, hat Beispiele, wird vom Server gelesen und vom Client-Typ gekannt. Ohne Dev-Mode lehnt der Raum sie mit `forbidden` ab und warnt im Log. Die Aktionen selbst (Gold, Material, Zeitraffer) folgen in DBG1.2 und DBG1.3.

## Kontext

- Protokoll v3 (`docs/protocol.md`, Abschnitte *Nachrichten* und *Fehler-Codes*). Die Version ändert sich nicht (Ticket B-178 › Anforderungen): ein älterer Server kennt `dev` nicht und antwortet `bad_request`. Eine Protokolländerung ist eine eigene Session, die nur das Protokoll und beide Enden anfasst (`docs/arbeitsweise.md` › Protokoll); der Client-Parser ist die Domänen-Ausnahme dieser Spec.
- Server heute: `engine/net/protocol.go` hat `inMsg` (alle Felder aller Client-Nachrichten, Zeiger = Pflichtfeld), die Konstanten `codeBadRequest` usw., `messages` (deutsche Texte je Code) und `codeOf` (übersetzt `room.Err…` in Codes). `engine/net/dispatch.go`: `handle` → `roomMessage(r, m)` mit `switch m.T` (`addSlot`, `removeSlot`, `input`, `leave`; alles andere `bad_request`). Ohne Raum ergibt jede Nachricht außer `create`/`join` schon `bad_request` (`case r == nil`).
- Raum: `engine/room/room.go` hat die Fehler `ErrBadRequest` usw. (Text = Code), `engine/room/actions.go` die Aktionen eines Geräts mit `own(id, peer)` (nil bei fremder, ersetzter Verbindung oder geschlossenem Raum) unter `r.mu`. Der Dev-Mode ist `Manager.Dev` (`engine/room/manager.go`, gesetzt aus `K3C_DEV` in `cmd/k3c-server/main.go`: leer oder 1 = an, 0 = aus); aus dem Raum erreichbar als `r.m.Dev`. (Das Ticket schreibt `Room.Dev`; im Code liegt das Flag am Manager.)
- Nachricht (Ticket-Vorschlag, hier festgelegt): `{ "t": "dev", "action": "gold" | "material" | "timescale", … }` mit den Feldern `slot` (gold, material), `amount` (gold, material), `resource` (material), `factor` (timescale). Pflichtfelder je Aktion: gold `slot`, `amount`; material `slot`, `amount`, `resource`; timescale `factor`. `slot` bei material nennt den Spieler, dessen Insel gemeint ist (Insel des Spielers, B-178 › Anforderungen). **Vorschlag, 🧑 bestätigt mit der Freigabe:** `amount` ist eine ganze Zahl 1…1000.
- Prüfreihenfolge im Raum (neue Methode `(*Room).Dev(id string, peer Peer, a DevAction) error`, Datei `engine/room/dev.go`): (1) kein Dev-Mode → `ErrForbidden` (`forbidden`) und `r.log().Warn("Dev-Aktion abgelehnt", "device", short(id), "aktion", a.Action)`; (2) Verbindung nicht gültig (`own` nil) → `ErrBadRequest`; (3) Aktion und Felder prüfen. **Vorschlag:** `forbidden` kommt vor jeder Feldprüfung, damit ein Server ohne Dev-Mode nichts über Felder verrät. In dieser Session kennt `Dev` noch keine Aktion: jede Aktion endet im `default` mit `ErrBadRequest`; DBG1.2 und DBG1.3 ergänzen die `case`.
- Fehler-Code `forbidden` ist neu: in `docs/protocol.md` › Fehler-Codes (Verbindung bleibt), in `messages` mit deutschem Text („Nur im Dev-Mode erlaubt“), in `codeOf`.
- Beispiele: `testdata/protocol/` hat `c2s-*.json` und `s2c-*.json`. `engine/net/ws_test.go › TestFormWieBeispiele` prüft die Schlüssel der Server-Nachrichten gegen ihre Beispiele; `src/online/clientConnection.test.ts` importiert Beispiele per `import … from '../../testdata/protocol/…json'`. Bestehende Beispiele bleiben **unverändert**. Neu: `c2s-dev-gold.json`, `c2s-dev-material.json`, `c2s-dev-timescale.json`, `s2c-error-forbidden.json`.
- Client: `src/online/clientProtocol.ts` hat `ErrorCode` und `ClientMessage`; `src/online/clientConnection.ts` verarbeitet `error` in `fail(code, message)` (Fall `default`: bleibt in Liste bzw. Raum und zeigt den Text). `forbidden` muss dort bewusst behandelt sein (Text anzeigen, Verbindung und Raum bleiben). `src/online/protocol.ts` ist die alte v1-Datei und bleibt unberührt.
- Dateigrenzen: `engine/net/ws_test.go` (399 Zeilen) und `engine/room/room_test.go` (398) sind voll, neue Tests kommen in neue Dateien (`engine/net/dev_test.go`, `engine/room/dev_test.go`, `src/online/clientProtocol.test.ts`). Test-Hilfen in `ws_test.go`: `wsServer(t)` (Manager ohne Dev-Mode; für Dev-Tests `m.Dev = true` setzen), `hello`, `create`, `expectError`, `sameKeys`; in `room_test.go`: `newFixture()`, `need`, `peer`.

## Erlaubte Dateien

- `engine/net/protocol.go`, `engine/net/dispatch.go`
- `engine/room/room.go` (nur `ErrForbidden`), `engine/room/dev.go` (neu)
- `engine/net/dev_test.go`, `engine/room/dev_test.go` (neu)
- `docs/protocol.md`
- `testdata/protocol/c2s-dev-gold.json`, `c2s-dev-material.json`, `c2s-dev-timescale.json`, `s2c-error-forbidden.json` (neu)
- `src/online/clientProtocol.ts`, `src/online/clientConnection.ts` (nur Fehlerbehandlung `forbidden`), `src/online/clientProtocol.test.ts` (neu)
- `docs/sprints/geplant/DBG1-dev-aktionen-server/`, `docs/sprints/aktiv/DBG1-dev-aktionen-server/` (nur Status), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Wirkung der Aktionen (DBG1.2, DBG1.3), `engine/sim/`, Bedienung im Client (DBG2), Protokollversion ändern, Änderung bestehender Beispiele.

## Schritte

1. Branch anlegen und pushen, `Status: in Arbeit`. `Start-Commit` der Sprint-README setzen, falls noch `–` (`git rev-parse --short origin/develop` vor dem Branch).
2. `room.ErrForbidden = errors.New("forbidden")` in `room.go` ergänzen. In `protocol.go`: `room.ErrForbidden` in die Liste von `codeOf`, Text in `messages`; `inMsg` um `Action string`, `Amount int`, `Resource string`, `Factor int` erweitern (`slot` gibt es schon als `*int`).
3. `engine/room/dev.go`: `DevAction{Action string; Slot *int; Amount int; Resource string; Factor int}` und `(*Room).Dev` mit der Prüfreihenfolge aus dem Kontext (Schritte 1 und 2; im `switch a.Action` gibt es nur `default: return ErrBadRequest`). Die Methode sperrt `r.mu` wie die anderen Aktionen.
4. `dispatch.go`: in `roomMessage` den Fall `"dev"`: `r.Dev(c.device, c, room.DevAction{…})` aus `m`. Ohne Raum bleibt es bei `bad_request` (greift schon).
5. Beispiele schreiben: `c2s-dev-gold.json` `{"t":"dev","action":"gold","slot":0,"amount":50}`, `c2s-dev-material.json` `{"t":"dev","action":"material","slot":0,"resource":"wood","amount":100}`, `c2s-dev-timescale.json` `{"t":"dev","action":"timescale","factor":4}`, `s2c-error-forbidden.json` wie `s2c-error.json` mit Code `forbidden` und dem Text aus `messages`.
6. `docs/protocol.md`: Zeile `dev` in die Nachrichten-Tabelle (Gerät → Server, nur Dev-Mode, Felder je Aktion, Beispiele), Zeile `forbidden` in die Fehler-Codes (Situation: `dev` ohne Dev-Mode; Verbindung bleibt), `bad_request` um „ungültige Felder von `dev`“ ergänzen, ein kurzer Absatz *Dev-Aktionen* (nur Dev-Mode `K3C_DEV`, vor dem Release aus; ältere Server antworten `bad_request`; die Wirkung der Aktionen ergänzen DBG1.2 und DBG1.3).
7. Client: `ErrorCode` um `'forbidden'`, `ClientMessage` um die drei `dev`-Formen (je Aktion eine Union-Form mit ihren Pflichtfeldern); in `clientConnection.ts › fail` den Fall `forbidden` ausdrücklich behandeln (Hinweistext anzeigen, Raum und Verbindung bleiben, kein Absturz).
8. Tests Server (`engine/net/dev_test.go`): (a) Server ohne Dev-Mode: Gerät im Raum sendet jedes der drei Beispiele (aus `testdata/protocol/` gelesen) → `expectError("forbidden")`; die Verbindung bleibt (danach geht `input`). (b) Mit `m.Dev = true`: jedes Beispiel wird gelesen und kommt als `bad_request` zurück (Aktionen noch nicht gebaut); eine Nachricht `dev` ohne Raum ergibt `bad_request`. (c) Die Fehlernachricht `forbidden` hat die Schlüssel von `s2c-error-forbidden.json` (Muster `sameKeys` in `ws_test.go`).
9. Test Raum (`engine/room/dev_test.go`): ohne Dev-Mode `ErrForbidden` und der Log-Eintrag „Dev-Aktion abgelehnt“ steht im Log (Logger des Managers mit Puffer, Muster in `engine/room/logging_test.go`); mit Dev-Mode und fremder Verbindung `ErrBadRequest`.
10. Test Client (`src/online/clientProtocol.test.ts`): die vier Beispiele importieren; die drei `c2s-dev-*` sind dem Typ `ClientMessage` zuweisbar (`satisfies`), `s2c-error-forbidden` hat Code `forbidden`; ein Test, dass `forbidden` den Raum nicht verlässt (Vorbild: die Fehler-Tests in `clientConnection.test.ts`, Hilfen kopieren statt diese Datei zu ändern).
11. `task check:go` und `task check`; `npx vitest run tests/planning.test.ts`. Ergebnis schreiben, `Status: fertig`, Tabelle der Sprint-README anpassen.

## Fertig, wenn

- [x] AC-01: Die Tests aus Schritt 8a und 9 belegen `forbidden` für alle drei Beispiele ohne Dev-Mode und die Warnung im Log (`go test ./engine/net ./engine/room -run Dev`).
- [x] AC-05: `docs/protocol.md` beschreibt `dev` und `forbidden`; die vier Beispiele liegen in `testdata/protocol/`; der Server liest sie (Test 8), der Client-Typ und die Fehlerbehandlung kennen sie (Test 10).
- [x] `task check:go` und `task check` grün; bestehende Beispiele unverändert (`git diff --stat testdata/protocol` zeigt nur neue Dateien).

## Prüfen

```bash
go test ./engine/net ./engine/room -run Dev
task check:go
task check
npx vitest run tests/planning.test.ts
```

## Ergebnis

Umgesetzt am 2026-10-03.
- AC-01 geprüft: `go test ./engine/net ./engine/room -run Dev` grün. `TestDevOhneDevModeForbidden` (WebSocket, alle drei Beispiele → `forbidden`, danach `leave` geht), `TestDevOhneDevModeVerboten` (Raum: `ErrForbidden` vor jeder Feldprüfung, Warnung „Dev-Aktion abgelehnt“ mit gekürztem Gerät und Aktion im Log).
- AC-05 geprüft: `docs/protocol.md` (Nachricht `dev`, Code `forbidden`, Abschnitt *Dev-Aktionen*), vier neue Beispiele in `testdata/protocol/`; der Server liest sie (`TestDevMitDevModeLiestBeispiele`: mit Dev-Mode `bad_request`, weil die Aktionen erst in DBG1.2/DBG1.3 kommen; ohne Raum `bad_request`), die Fehlernachricht hat die Schlüssel von `s2c-error-forbidden.json`; Client: `DevMessage` in `ClientMessage`, `forbidden` in `ErrorCode` und `fail` (`src/online/clientProtocol.test.ts`).
- `task check:go` und `task check` grün (`check:race` lokal übersprungen, kein C-Compiler; die CI prüft `-race`); `git status testdata/protocol` zeigt nur die vier neuen Dateien.
Abweichungen: Der Dev-Mode-Server im Test ist ein eigener Helfer `devServer` (setzt `Dev` vor dem Start, damit kein Datenrennen entsteht) statt `m.Dev = true` nach `wsServer`. `resource` ist im Client-Typ `ResourceKind` statt `string`. Keine neuen Tickets.
