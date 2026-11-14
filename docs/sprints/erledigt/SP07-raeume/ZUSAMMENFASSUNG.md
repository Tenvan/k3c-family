# SP07 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Räume & WebSocket (Protokoll v2) in Go
- **Projekt:** –

## Ziel

Der Go-Server rechnet bis zu 4 Räume parallel und spricht Protokoll v2 über `/ws`. Am Ende sichtbar: `task check:go` enthält einen WebSocket-Test mit 3 Räumen (2, 3 und 4 Spieler), und `/api/status` zeigt Räume und Tick-Dauer.

## Ergebnis

- 2026-10-01, Review über `3a3c9d5..main` per unabhängigem Subagent (auf Auftrag von 🧑 im selben Lauf). - Kriterien: AC-02, AC-03, AC-04 (SP07.1); AC-05, AC-06 (SP07.2); AC-01, AC-07, AC-08 (SP07.3). Alle mit Tests und Gegentest belegt. - Behobene Befunde: Sperren bei Panic, Speichern eines abgestürzten Raums, ersetzte Verbindung, `room_closed` beim Beenden, zwei Vertragsabweichungen, Frist im Handschlag (SP07.4). - Offen: B-077 (Race-Detector in der CI). Neue Tickets im Sprint: B-077.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Ein Test lässt 3 Räume (2, 3 und 4 Spieler) mit je eigener Goroutine im 30-Hz-Takt paralle … |
| AC-02 | geprüft | Ein Gerät mit 2 lokalen Spielern und eines mit 1 Spieler teilen einen Raum, jeder steuert … |
| AC-03 | geprüft | Abbruch, Wiederverbinden und Grenzen: binnen 60 s zurück mit denselben oder geänderten Slo … |
| AC-04 | geprüft | Spielstand pro Raum: `create` prüft in der Reihenfolge aus `docs/protocol.md`; der Raum sp … |
| AC-05 | geprüft | Handschlag und Fehler: `hello`/`welcome` mit `v` 2; ein ungültiges erstes `hello` ergibt ` … |
| AC-06 | geprüft | Ein Go-Test verbindet per WebSocket, tritt einem Raum bei und empfängt `joined`, `level` u … |
| AC-07 | geprüft | Delta und `seq`/`ack`: Ein Test wendet in einem Lauf jedes `delta` (`{set, del}` für Liste … |
| AC-08 | geprüft | `/api/status` zeigt je Raum Code, Name, Stufe, Geräte, Monarchen (`taken`/`waiting`/`free` … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-030 | erledigt | – | Geräte verbinden sich nach Abbruch wieder |
| B-031 | erledigt | – | Online-Verbindung ist automatisch getestet |
| B-036 | erledigt | – | Mehrere Spiele laufen gleichzeitig |
| B-038 | erledigt | – | Lokale und Online-Spieler teilen sich einen Raum |
| B-060 | erledigt | – | Die Spec von SP07 deckt alle Server-Regeln aus Protokoll v2 ab |
| B-076 | erledigt | – | Der Go-Server spricht WebSocket über github.com/coder/websocket |
