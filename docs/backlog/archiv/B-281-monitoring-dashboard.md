# B-281 · Der Server sammelt Latenzen, Tick-Dauer und Fehler als Verlauf und liefert sie über /api/metrics

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** MON1
- **Erstellt:** 2026-10-05
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, mit Sprint MON1

## Ausgangslage

`GET /api/status` (Token, B-027) liefert nur eine Momentaufnahme: je Raum `tickMs.last`/`p99`, Geräte, Stufen, dazu `failures`, CPU, Heap (`engine/net/status.go`, `engine/room/status.go`). Langsame Ticks landen als 🐢-Warnung im Log (`engine/room/logging.go`), Client-Fehler über `/api/clientlog`, die Latenz je Gerät sieht man nur im Debug-Overlay am Gerät selbst (B-181). Einen Verlauf über Minuten oder einen Spieleabend gibt es nicht; wer Ruckeln (B-194) oder Abstürze nachverfolgen will, liest JSONL-Logs oder fragt per k3c-dev/TUI einzeln ab.

## Ziel

Eine Seite im Browser (PC, Handy) zeigt den laufenden Server als Dashboard: Verläufe von Tick-Dauer, Latenz je Gerät, Speicher/CPU und eine Fehler-Zeitleiste. Damit lässt sich ein Ruckeln oder Bug zeitlich einordnen („um 21:14 p99 auf 18 ms, gleichzeitig Gerät 2 RTT 300 ms, danach Absturz Raum ABCD“), ohne Logs von Hand zu lesen. Der Spielbetrieb merkt nichts davon.

## Beteiligte und Zielgruppen

🧑 als Betreiber am Spieleabend (Handy neben dem TV) und als Entwickler bei der Fehlersuche; 🧑 entscheidet die Offenen Fragen und nimmt die Seite ab.

## Anforderungen

- **Messen im Server, billig:** Ein Ring-Puffer fester Größe je Kennzahl (1 Eintrag/s, 1 h = 3600 Punkte), befüllt aus einem eigenen 1-s-Ticker außerhalb des Raum-Ticks. Im Tick nur das, was heute schon gemessen wird (Tick-Dauer) plus Zähler; keine Allokation, kein Lock über den Raum-Lock hinaus, kein Log-Parsen.
- **Kennzahlen je Sekunde:** Tick-Dauer je Raum (max und p99 des Intervalls), Anzahl Ticks über Budget, RTT je Gerät (aus vorhandener Zeitleiste von N2, sonst einfacher Ping im bestehenden Takt), verworfene Nachrichten/volle Warteschlangen je Verbindung (B-280, B-190), Heap, GC-Pausen, Goroutinen, CPU, Speicher-Dauer (`saver.go`).
- **Ereignis-Zeitleiste:** Abstürze (`failures`), 🐢-Ticks, Trennungen, Client-Fehler aus `/api/clientlog`, letzte 200 in einem Ring-Puffer.
- **Endpunkt:** `GET /api/metrics?since=<ts>` hinter demselben Token wie `/api/status` (B-027), liefert `startedAt` und nur Punkte nach `since` (Delta). Kein WebSocket-Push.
- **Leistung messbar:** Benchmark neben `engine/net/tick_bench_test.go` zeigt, dass die Messung den Tick um höchstens 1 % bzw. 20 µs verlängert; Pi-Ziel aus LT1 (Nacht < 10 ms) bleibt erreichbar.
- **2+ Spieler:** RTT und Warteschlange je Gerät getrennt.
- Die Seite dazu ist B-282 (PLAT, Sprint MON2).

## Nicht-Ziele

- Kein Prometheus/Grafana/OpenTelemetry-Stack, kein externer Dienst, keine Datenbank.
- Keine dauerhafte Speicherung der Verläufe über einen Neustart hinaus (Spielmetrik-Report B-150 deckt die Sitzungsauswertung).
- Keine Steuerung von Räumen (Dungeon-Master-Seite DBG3/B-232).
- Kein Profiling per pprof in der Seite (eigenes Ticket, falls gewünscht).
- Keine Anzeige am TV im Spiel (Debug-Overlay B-093/B-181 bleibt dafür zuständig).

## Regeln und Einschränkungen

- Entscheidung 001: Daten entstehen im Go-Server, der Browser zeichnet nur.
- Diagnose-Schutz wie B-027/B-143: ohne `K3C_STATUS_TOKEN` 404, falsches Token 401.
- Seitenregeln aus `CLAUDE.md` (Shell, `installPageChrome()`, `pages.ts`, nie `requestFullscreen()` direkt); `tests/projectRules.test.ts` muss grün bleiben.
- Logging mit Emojis; neue Meldungen höchstens beim Start/Stopp des Sammlers (🚀/🛑), nicht je Sekunde.
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Domäne SRV für Sammler und Endpunkt; die Seite ist ein eigener CLI-Teil (eigene Session oder Folge-Sprint).
- Speicherbudget: Ring-Puffer gesamt < 2 MB bei 4 Räumen × 4 Geräten.

## Beispiele

- Spieleabend, 2 Räume, 3 Geräte: Handy öffnet die Monitoring-Seite → Ampel grün, Tick p99 4 ms, RTT 12/15/40 ms.
- Xbox ruckelt kurz (B-194): Verlauf zeigt RTT-Spitze für Gerät 2 bei gleichbleibender Tick-Dauer → Ursache liegt im Netz/Client, nicht im Server.
- Raum stürzt ab: Ereignis „💥 Raum ABCD“ in der Zeitleiste, Klick springt in den Verlauf, davor sichtbar steigende Tick-Dauer.
- Seite 1 h offen: Poll alle 5 s überträgt nur Deltas (wenige kB je Abfrage).

## Ausnahme- und Fehlerfälle

- Kein Token gesetzt → Endpunkt 404, Seite zeigt „Diagnose aus“.
- Server neu gestartet → `since` liegt vor `startedAt`, Antwort liefert vollen Puffer und neuen `startedAt`, Seite verwirft alte Punkte.
- Raum schließt → seine Reihe endet, bleibt bis zum Pufferende sichtbar.
- Mehrere Seiten offen → jede pollt einzeln, Kosten bleiben O(Punkte seit `since`); keine Wirkung auf den Tick.
- Puffer voll → älteste Punkte fallen raus, kein Wachstum.

## Akzeptanzkriterien

- **AC-01** `go test ./engine/...`: Test füllt den Ring-Puffer über seine Größe, Länge bleibt fest, älteste Punkte fallen raus.
- **AC-02** `GET /api/metrics?since=…` liefert `startedAt` und nur neuere Punkte; ohne Token 404, falsches Token 401 (Test wie `status_test.go`).
- **AC-03** Benchmark: Tick mit aktivem Sammler höchstens 1 % bzw. 20 µs langsamer als ohne; Ergebnis in den Notizen.
- **AC-04** Test mit 2 Geräten liefert zwei getrennte RTT- und Warteschlangen-Reihen.
- **AC-05** Ereignis-Ring enthält nach einem simulierten Absturz, einem 🐢-Tick und einer Trennung je einen Eintrag mit Zeit, Raum und Art.
- **AC-06** `docs/protocol.md` oder Diagnose-Doku beschreibt `/api/metrics` mit Beispielantwort.

## Offene Fragen

- ~~Zeitfenster/Auflösung~~ entschieden 2026-10-05 (🧑): 1 h bei 1 s (3600 Punkte je Reihe), keine Verdichtung.
- ~~RTT-Quelle~~ geklärt in MON1.1: eigener WebSocket-Ping des Servers je Sekunde (Kontrollframe, Browser antworten selbst, kein Protokollwechsel). Die Zeitleiste aus N2 (`ack`) misst nur der Client (Glossar „Latenz“), dem Server fehlt sie.

## Notizen

Bausteine vorhanden: `room.Manager.Status()` (Tick last/p99, `failures`), `engine/net/cpu.go`, `tick_bench_test.go`, `task load` (LT1). Verwandt: B-181 (Latenz im Overlay), B-194 (Ruckeln Xbox), B-280/B-190 (Warteschlange), B-150 (Spielmetrik-Report), B-232/DBG3. Verworfen: Prometheus-Exporter (Abhängigkeit + externer Stack, Overkill fürs Heimnetz).

**Benchmark AC-03 (MON1.1, 2026-10-05, Windows-PC, 22 Kerne):** `go test ./engine/net/ -run '^$' -bench TickMonitor -benchtime=3s -count=4`. „ohne“ 408, 416, 400, 401 µs/Tick (Mittel 406 µs); „mit“ (Sammler 1000-mal so oft wie im Betrieb: jede Millisekunde `Sample` und 4 Geräte-Punkte) 434, 395, 397, 433 µs (Mittel 415 µs). Unterschied +8 µs im Mittel, unter 20 µs, und im Rauschen (zwei „mit“-Läufe schneller als jeder „ohne“-Lauf). Im Betrieb (1 Punkt/s) entfällt davon rechnerisch ein Tausendstel; im Tick selbst kommt nur ein Inkrement (`secTicks`) dazu.
