# 002 · Protokoll v2: Räume, Geräte mit lokalen Spielern, Snapshots

Stand: 2026-09-30 · Status: **beschlossen** (🧑 2026-09-30, Chat) · Backlog: B-030, B-036, B-038, B-039, B-059 · Sprint: SP02 ·
Details: [`docs/protocol.md`](../protocol.md)

## Kontext

- Entscheidung [001](001-server-engine-go.md): Der Go-Server rechnet alle Räume, der Browser schickt nur Eingaben und
  zeichnet. Mehrere Räume und mehrere lokale Spieler pro Gerät gehören von Anfang an in den Kern.
- Protokoll v1 (`src/online/`) kennt einen Monarchen pro Gerät, Räume nur per `?online=RAUM`, bis zu 8 Spieler pro Raum,
  schickt jeden Tick den vollen Zustand, und jeder Client baut das Level selbst aus dem Seed. Ab SP09 gibt es im
  Browser keinen Level-Generator mehr.
- Zielbild: 2 Controller an der Xbox + 1 Handy im selben Raum, mehrere Räume parallel, später auf einem Raspberry Pi.

## Optionen

| Frage | Optionen | Gewählt |
|---|---|---|
| Beitreten | nur Raumliste · nur Code per URL · beides | beides (Liste im Heimnetz ist nicht geheim, Code für Links) |
| Spieler pro Gerät | ein Monarch pro Verbindung (wie v1) · Slots pro Gerät | Slots 0–3 pro Gerät, eine Verbindung |
| Abbruch | Monarch sofort weg · Monarch wartet eine Frist | wartet 60 s, danach frei |
| Versionsfeld | in jeder Nachricht · nur im Handschlag | nur `hello`/`welcome` (spart Bytes bei 30 Hz) |
| Zustand | jeden Tick voll · voll + Delta · Binärformat | voll + Delta als JSON (≈ 61 statt 320 KB/s pro Gerät) |
| Level | Seed schicken · Level schicken | Level (`level`), weil der Browser nichts mehr generiert |

## Entscheidung

Beschlüsse von 🧑 (2026-09-30, Chat, Sprint-README SP02 › Regeln):

1. **Beitreten:** Der Server liefert eine Raumliste; jeder Raum hat zusätzlich einen kurzen Code zum direkten Beitreten.
2. **Grenzen:** höchstens 4 Monarchen pro Raum, 4 lokale Spieler pro Gerät, 4 Räume pro Server.
3. **Abbruch:** Die Monarchen eines abgebrochenen Geräts stehen still; binnen 60 s steuert es sie mit seiner Geräte-ID
   weiter, danach sind sie frei. Ein leerer Raum wird nach 10 min aufgeräumt.
4. **Spielstand:** Ein Raum ist genau ein Spielstand; beim Erstellen neu oder gespeichert, der Raum speichert selbst.

Entscheidung 🧑 (2026-09-30, Chat, Review SP02, Befund K1): Für den **Stufenwechsel** zählen nur besetzte Monarchen,
freie und wartende reisen mit. Ein gespeicherter Stand legt keine Monarchen an, sie entstehen beim Beitreten; Gold gilt
pro Index.

Nachrichten-Grundsätze (SP02.2): JSON über WebSocket `/ws`; Version `v` = 2 nur im Handschlag, sonst `version` und
Verbindung zu; Raum tickt mit **30 Hz** und schickt pro Tick genau einen Zustand, voll (`snap`) nach `level` und sonst
**Delta**; der Server überträgt das **Level** statt des Seeds; feste Fehler-Codes. Nachrichten, Felder, Codes und
Beispiele stehen nur in [`docs/protocol.md`](../protocol.md) und `testdata/protocol/`.

## Folgen

- SP07 setzt den Server um (Räume, WebSocket, Grenzen, Fristen), SP08 den Client (Slots, Raumwahl, Interpolation).
  Beide prüfen gegen `testdata/protocol/`. Bis dahin läuft v1 unverändert.
- Die Simulation braucht pro Monarch „gesteuert ja/nein“ (B-059, im Go-Port).
- Eine Änderung am Protokoll ändert `docs/protocol.md`, die Beispiele und beide Enden in einer Session
  (Grenzfall Protokoll in `docs/arbeitsweise.md`).
- JSON reicht nach der Schätzung fürs Heimnetz; Runden oder ein Binärformat erst, wenn die Messung am Pi (SP11) es verlangt.
