# SP07 · SRV · Räume & WebSocket (Protokoll v2) in Go

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-030, B-031, B-036, B-038, B-060, B-076
- **Start-Commit:** 3a3c9d5
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (SP07 Rev. 1: Protokoll v2 vollständig, coder/websocket, B-047 nach M6, B-030 Rev. 3)

## Ausgangslage

Räume rechnet heute die TS-Simulation im Node-Server (`src/online/room.ts`, `src/online/wsServer.ts`, Protokoll v1,
ein Monarch pro Gerät). Seit SP06 kann die Go-Simulation alles (`engine/sim`: `Campaign`, `Step`, `ToSave`,
`ParseSave`, `FromSave`, `Player.Free` für B-059). Der Go-Server (`cmd/k3c-server`, `engine/net`, `engine/store`)
liefert Build, Spielstände und `/api/status` aus, kennt aber keine Räume und keinen WebSocket. Vertrag für SP07 ist
Protokoll v2: `docs/protocol.md` und die Beispiele in `testdata/protocol/` (Entscheidung 002).

## Ziel

Der Go-Server rechnet bis zu 4 Räume parallel und spricht Protokoll v2 über `/ws`. Am Ende sichtbar: `task check:go`
enthält einen WebSocket-Test mit 3 Räumen (2, 3 und 4 Spieler), und `/api/status` zeigt Räume und Tick-Dauer.

## Beteiligte und Zielgruppen

Spieler an Xbox und Handy (sichtbar ab SP08, Client); 🧑 betreibt den Server im Heimnetz; Entwickler und Agenten
lesen `/api/status` (MCP-Tools dazu in M6, B-047). 🧑 gibt die Spec frei.

## Anforderungen

B-030, B-031, B-036, B-038 › Anforderungen; Protokoll v2 vollständig nach `docs/protocol.md` (B-060). Sprint-eigen:

- `engine/room`: Räume, Geräte, Slots und Monarchen nach *Beitreten*, *Lokale Spieler*, *Verlassen und Abbruch*,
  *Wiederverbinden*, *Grenzen* und *Spielstand* aus `docs/protocol.md`. Eine Goroutine pro Raum, Takt 30 Hz,
  Fristen nach Wanduhr (Uhr für Tests austauschbar). Der Raum speichert selbst über `engine/store`.
- `engine/net`: WebSocket `/ws` mit allen Nachrichten und Fehler-Codes aus `docs/protocol.md` › *Nachrichten*,
  `snap`/`delta` nach *Zustand und Delta*, `seq`/`ack` je Verbindung, Schließen bei vollem Sendepuffer.
- `/api/status` (Token, B-027) zeigt die Räume; `/api/status?room=CODE` liefert einen verdichteten Zustand eines Raums
  (Grundlage für `room_snapshot` in M6).
- WebSocket über `github.com/coder/websocket` (B-076, erste Abhängigkeit in `go.mod`).

## Nicht-Ziele

Browser-Seite und Interpolation (SP08); MCP-Tools für Räume und Simulation (M6, B-047); Node-Server und Protokoll v1
abschalten (SP09); Lobby-Bedienung (B-037); Diagnose-TUI (SP10); Vorhersage auf dem Client (B-039).

## Regeln und Einschränkungen

- Vertrag: `docs/protocol.md` und `testdata/protocol/`. Eine Abweichung ändert erst das Protokoll (eigene Session mit
  beiden Enden, `docs/arbeitsweise.md` › Grenzfall Protokoll), nie still den Server.
- Schichtgrenzen: `engine/sim` importiert nichts aus `engine/room`, `engine/net`; `engine/room` nichts aus
  `engine/net`; `engine/*` nichts aus `cmd/`. Protokoll-Typen liegen in `engine/net/protocol*.go`.
- `free` aus `engine/sim` (B-059) gehört nicht in `snap`/`delta`, der Zustand der Monarchen steht in `seats`.
- Ausnahme außerhalb der Domäne (INF): `go.mod`/`go.sum` für `github.com/coder/websocket` (B-076), `.golangci.yml`
  nur für eine depguard-Regel `engine/room` ohne `engine/net`.
- Komplexitäts-Budget wie immer; Richtwert ≤ ~400 geänderte Code-Zeilen pro Session.

## Beispiele

- Xbox mit 2 Controllern erstellt `familie`, ein Handy tritt per Code bei → drei Monarchen, jeder eigen gesteuert.
- Handy verliert 10 s WLAN → Monarch 2 ist `waiting`, das Handy kommt mit derselben Geräte-ID zurück und steuert ihn weiter.
- 3 Räume mit 2, 3 und 4 Spielern parallel → jeder Raum tickt unabhängig, `/api/status` zeigt die Tick-Dauer je Raum.

## Ausnahme- und Fehlerfälle

Alle Zeilen aus `docs/protocol.md` › *Fehlerfälle* und › *Fehler-Codes*. Zusätzlich: Ein Raum stürzt ab (Panic im
Tick) → nur dieser Raum endet, seine Geräte bekommen `room_closed`, der Fehler steht im Log und in `/api/status`.

## Akzeptanzkriterien

- **AC-01** Ein Test lässt 3 Räume (2, 3 und 4 Spieler) mit je eigener Goroutine im 30-Hz-Takt parallel laufen, ohne
  dass sie sich beeinflussen (B-036/AC-01, B-036/AC-02). Ein Panic in einem Raum beendet nur diesen (`room_closed`).
- **AC-02** Ein Gerät mit 2 lokalen Spielern und eines mit 1 Spieler teilen einen Raum, jeder steuert seinen eigenen
  Monarchen (B-038/AC-01, B-038/AC-02). Slot hinzufügen und entfernen wirkt wie in `docs/protocol.md`.
- **AC-03** Abbruch, Wiederverbinden und Grenzen: binnen 60 s zurück mit denselben oder geänderten Slots, danach frei;
  ein leerer Raum wird nach 10 min aufgeräumt; beide Fristen laufen nach Wanduhr, auch im pausierten Raum; die
  Grenzen 4/4/4 greifen (B-030/AC-01, B-030/AC-02, B-030/AC-03).
- **AC-04** Spielstand pro Raum: `create` prüft in der Reihenfolge aus `docs/protocol.md`; der Raum speichert bei
  Stufenwechsel, sobald kein Gerät verbunden ist, beim Aufräumen und beim Beenden; ein geladener Stand legt keine
  Monarchen an, Gold gilt pro Index (Tests).
- **AC-05** Handschlag und Fehler: `hello`/`welcome` mit `v` 2; ein ungültiges erstes `hello` ergibt `version` und das
  Schließen. Jeder Fehler-Code aus `docs/protocol.md` ist durch einen Test belegt. Jede Nachricht des Servers hat die
  Schlüssel des passenden Beispiels in `testdata/protocol/`.
- **AC-06** Ein Go-Test verbindet per WebSocket, tritt einem Raum bei und empfängt `joined`, `level` und `snap`; er
  läuft in `task check:go` und damit in der CI (B-031/AC-01).
- **AC-07** Delta und `seq`/`ack`: Ein Test wendet in einem Lauf jedes `delta` (`{set, del}` für Listen mit `id`) auf
  den ersten `snap` an und erhält in jedem Tick den vollen Zustand. `ack` ist das höchste verrechnete `seq` dieser
  Verbindung. Ein voller Sendepuffer schließt die Verbindung, das zählt als Abbruch.
- **AC-08** `/api/status` zeigt je Raum Code, Name, Stufe, Geräte, Monarchen (`taken`/`waiting`/`free`), läuft oder
  pausiert, Tick-Dauer (letzte, p99) und einen Absturz-Fehler; `/api/status?room=CODE` liefert Tag/Nacht, Gold,
  Einheiten und Spieler des Raums (Tests).

## Offene Fragen

keine. Geklärt von 🧑 (2026-10-01, Chat): WebSocket über `github.com/coder/websocket`; B-047 (MCP-Tools) kommt in den
eigenen Sprint M6. Speichern pro Raum gehört in SP07 (B-060 › Offene Fragen), `engine/store` gibt es seit SP03.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP07.1 | `SP07.1-raummodell.md` | Umsetzung | autonom | fertig |
| SP07.2 | `SP07.2-websocket.md` | Umsetzung | autonom | fertig |
| SP07.3 | `SP07.3-takt-delta-status.md` | Umsetzung | autonom | fertig |
| SP07.4 | `SP07.4-review.md` | Review | autonom | offen |

## Abnahme

–
