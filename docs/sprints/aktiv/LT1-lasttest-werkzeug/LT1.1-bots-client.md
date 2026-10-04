# LT1.1 · Bots über das Protokoll, Seed und Aufräumen der Test-Räume

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** lt1/1-bots-client
- **Abhängig von:** –
- **Tickets:** B-175
- **Kriterien:** AC-01, AC-02, AC-05

## Ziel

`cmd/k3c-load` startet gegen einen Server n Test-Räume mit je m Bots, die wie ein Client über `/ws` spielen, deterministisch aus einem Seed; nach dem Lauf ist kein Test-Raum mehr offen.

## Kontext

- Anforderungen: B-175 › Anforderungen (Parameter `-url`, `-token`, `-rooms`, `-players`, `-duration`, `-seed`; Bots laufen mit Richtungswechsel, sprinten, geben Münzen; Räume mit Präfix `test-`).
- **Protokoll:** B-175 nennt v3. Maßgeblich ist die Version im Code (`engine/net/protocol.go` › `ProtocolVersion`, heute 3; S2.1 hebt sie auf 4). Die Bots nehmen die Konstante, nicht eine Zahl. Ablauf und Nachrichten: `docs/protocol.md` (hello, create, input), Beispiele `testdata/protocol/*.json`.
- Raum-Präfix: `engine/room/manager.go` › `TestPrefix = "test-"`; der Server räumt solche Räume auf. Das Werkzeug legt nur `test-…`-Räume an, trennt alle Verbindungen am Ende (auch bei Strg+C) und prüft danach über `/api/status`, dass keiner offen ist.
- Schichtgrenzen (B-175 › Regeln): `cmd/k3c-load` importiert nur `engine/net`-Protokolltypen und `engine/rng`, nichts aus `engine/room` oder `engine/sim`. WebSocket: die vorhandene Bibliothek aus `go.mod` (keine neue Abhängigkeit). Ein Client-Vorbild in Go: `cmd/k3c-tui/client.go`.
- Determinismus: Eingabefolge je Bot aus `rng.New(seed)` (nie `math/rand`); Zeitpunkte nach Tick-Takt, nicht nach Wanduhr, damit zwei Läufe dieselbe Folge senden.
- In-Prozess-Server für Tests: `httptest` mit dem Handler aus `engine/net` (Muster in `engine/net/ws_test.go`, `handler_test.go`).
- Token: kommt aus `-token` oder Umgebung, steht nie in Log oder Bericht (AC-05).

## Erlaubte Dateien

- `cmd/k3c-load/` (neu: Befehl, Bots, Tests)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Status-Abfrage, Bericht, Bewertung, `cpu`, Task `load` (LT1.2); Messlauf am Pi (LT1.3); Änderungen am Server.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Befehl mit Flags, Verbindungsaufbau je Bot (hello, create bzw. join, input), Raumcodes `test-…`; Fehler beim Verbinden oder falsches Token → Meldung, Exit-Code 2, keine Räume.
3. Bot-Eingaben aus Seed: Laufen mit Richtungswechsel, Sprint, Münzen geben.
4. Ende (Dauer abgelaufen oder Signal): alle Verbindungen schließen.
5. Tests: (a) `httptest`-Server, 2 Räume × 2 Bots, 5 s → je Raum eine Tick-Reihe gesammelt (Grundlage für den Bericht in LT1.2); (b) zwei Läufe mit gleichem Seed senden dieselbe Eingabefolge (an der Verbindung aufgezeichnet); (c) nach dem Lauf keine Test-Räume offen, Ausgabe enthält das Token nicht.
6. `task check:go`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: Test belegt 2 Räume × 2 Bots gegen In-Prozess-Server mit Tick-Reihe je Raum.
- [x] AC-02: Test belegt gleiche Eingabefolge bei gleichem Seed.
- [x] AC-05: Test belegt: keine Test-Räume offen, kein Token in der Ausgabe.
- [x] `task check:go` grün; keine Datei > 400 Zeilen, keine Funktion > 60 Zeilen.

## Prüfen

```bash
task check:go
```

## Ergebnis

2026-10-04, Agent (worker-lt1). `cmd/k3c-load` (`main.go` Flags/Ablauf, `bots.go` Bots und Eingaben, `api.go` Vorprüfung und Aufräum-Prüfung über `/api/status`, `load_test.go`). Bots: je Bot ein Gerät mit Slot 0, `hello` mit `k3cnet.ProtocolVersion`, erster Bot `create` (Spielstand `test-load-<Lauf>-<n>`, neu), weitere `join` per Code; eine `input` je empfangenem `snap`/`delta` (Takt des Servers); Ende per `-duration` oder Strg+C/SIGTERM mit `leave` und normalem Schließen.

- **AC-01 geprüft:** `TestLaufSammeltTicksUndRaeumtAuf` (2 Räume × 2 Bots, 5 s, In-Prozess-Server per `httptest`, Ausgabe nennt je Raum eine Tick-Reihe) und `TestTickReiheJeRaum` (Reihe je Raum steigt, Code gesetzt, keine Fehler-Nachrichten). Die Reihe kommt aus den Zuständen des ersten Bots (Zeit, Tick); Tick-Dauer und Phase aus `/api/status` folgen in LT1.2.
- **AC-02 geprüft:** `TestGleicherSeedGleicheEingaben`: Zwei Läufe mit Seed `seed-a` senden je Bot dieselbe `input`-Folge (aufgezeichnet vor dem Schreiben auf die Verbindung, gemeinsamer Anfang ≥ 20 Eingaben, weil die Zahl der Zustände mit der Laufzeit schwankt); Seed `seed-b` sendet eine andere.
- **AC-05 geprüft:** `TestLaufSammeltTicksUndRaeumtAuf`: Nach dem Lauf hat kein Test-Raum ein verbundenes Gerät (das Werkzeug prüft das selbst über `/api/status`, sonst Exit 2); nach der Leer-Frist des Servers (Uhr des Managers vorgestellt, `Sweep`) sind Räume und Test-Spielstände weg; Ausgabe und Fehlertexte enthalten das Token nicht. `TestFehlerOhneRaeume`: falsches Token, Server nicht erreichbar, kein Token → Exit 2, keine Räume, Token nicht in der Ausgabe.
- **Abweichung:** „Kein Test-Raum offen“ heißt hier: kein Bot mehr verbunden; die leeren Räume schließt der Server wie geplant erst nach `EmptyFor` (10 min) und sie zählen bis dahin gegen die Grenze von 4 Räumen (ein zweiter Lauf direkt danach bekommt `too_many_rooms`, Exit 2). Neues Ticket **B-200**.
- `task check:go` grün (go test, golangci-lint 0 issues; `-race` lokal übersprungen, kein C-Compiler), `task check` grün (705 Tests). Größte Datei `bots.go` 225 Zeilen, Funktionen < 60 Zeilen (funlen).
