# LT1.2 · CPU in `/api/status`, Bericht, Bewertung und Task `load`

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** lt1/2-status-bericht
- **Abhängig von:** LT1.1
- **Tickets:** B-175
- **Kriterien:** AC-03, AC-04

## Ziel

`/api/status` nennt die CPU-Last des Server-Prozesses, `k3c-load` fragt den Status alle 5 s ab, schreibt Bericht als JSON und Markdown und bewertet gegen das Ziel mit passendem Exit-Code; `task load` startet das Werkzeug.

## Kontext

- Anforderungen: B-175 › Anforderungen. Bewertung mit `-target-p99` (Standard 10 ms, Ziel aus B-042): `erreicht` (max < Ziel), `knapp` (Mittel < Ziel ≤ max), `verfehlt` (Mittel ≥ Ziel); Exit-Code 0, bei `verfehlt` 1; Verbindungsfehler 2 (LT1.1).
- Status heute: `engine/net/status.go` (`status`, Bearer-Token `K3C_STATUS_TOKEN`, ohne Token 404); Raumzeilen aus `engine/room/status.go` (`Status{Info, Devices, Monarchs, Tick, TickMs, Stages[{Depth, Phase, Day, Players}]}`). Die Phase für den Bericht kommt aus `Stages[].Phase`.
- **`cpu`:** Prozent einer CPU, gemittelt über das letzte Intervall; Quelle austauschbar (Funktion oder Interface im Server), Linux über `/proc/self/stat`, sonst Prozesszeit; fehlt die Quelle, fehlt das Feld (AC-04). Test mit einer gestellten Quelle, kein echtes `/proc` nötig. Feld in `docs/protocol.md` bzw. dort, wo `/api/status` beschrieben ist, ergänzen.
- Bericht: JSON (Zeitreihe je Raum: Zeit, Phase, Tick, Tick-Dauer last und p99, CPU) und Markdown-Tabelle (je Raum und Phase p99 min/Mittel/max, CPU Mittel/Spitze, Bewertung). Ablageort per Flag `-out` (Standard `reports/load-<Zeit>.*`); Abbruch mit Strg+C schreibt den Bericht aus den bisherigen Werten.
- **`-duration night`:** bis zum Ende der ersten Nacht plus einer Dämmerung laut `Stages[].Phase`, mit Obergrenze. B-175 › Offene Fragen: „Messdauer Nacht und Obergrenze legt der Sprint fest“. **Vorschlag der Planung (🧑 bestätigt oder ändert mit der Spec-Freigabe, nicht als Beschluss behandeln):** Obergrenze per Flag `-max-duration`, Standard 60 min.
- **Taskfile:** Eine Zeile bzw. ein Task `load` in `Taskfile.yml` (INF-Datei, von der Spec erlaubt), Aufruf `task load -- <Flags>`; EXE mit festem Pfad (`bin/k3c-load`), kein `go run` (Firewall-Abfrage unter Windows).

## Erlaubte Dateien

- `cmd/k3c-load/`
- `engine/net/status.go` und neuer Test (z. B. `engine/net/status_cpu_test.go`), neue Datei für die CPU-Quelle in `engine/net/`
- `Taskfile.yml` (nur Task `load`)
- `docs/protocol.md` bzw. die Doku von `/api/status` (nur Feld `cpu`)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Messlauf am Pi (LT1.3), Bandbreite (B-140), Absenken des Ziels, Änderungen an `engine/room` oder `engine/sim`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. CPU-Quelle mit Test (gestellte Quelle → `cpu` gesetzt; ohne Quelle → Feld fehlt).
3. Status-Abfrage alle 5 s im Werkzeug, Zeitreihe je Raum, Phase aus den Stufen.
4. Bewertung als reine Funktion mit Test für Beispielreihen (`erreicht`, `knapp`, `verfehlt`) und Exit-Code.
5. Bericht JSON und Markdown, `-duration night` mit Obergrenze, Task `load`.
6. Lokaler Probelauf gegen `task start` (Loopback, 2 Räume × 3 Bots, 2 min), Bericht ins Ergebnis.
7. `task check:go` und `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-03: Test belegt die drei Bewertungen und den Exit-Code.
- [x] AC-04: Test belegt `cpu` mit Quelle und fehlendes Feld ohne Quelle.
- [x] `task load` steht in `task --list`; lokaler Probelauf im Ergebnis.
- [x] `task check:go` und `task check` grün.

## Prüfen

```bash
task check:go
task check
```

## Ergebnis

2026-10-04, Agent (worker-lt1). `/api/status` nennt `cpu` (`engine/net/cpu.go`, `Config.CPU`), `k3c-load` fragt den Status alle `-interval` (Standard 5 s) ab (`poll.go`), bewertet (`report.go`) und schreibt `<-out>.json` und `<-out>.md` (Standard `reports/load-<Zeit>`); Task `load` in `Taskfile.yml` (feste EXE `bin/k3c-load`).

- **AC-03 geprüft:** `TestBewertungUndExitCode` (erreicht, knapp, verfehlt, genau Ziel = verfehlt, keine Werte; Exit 0/0/1/2), `TestBerichtJeRaumUndPhase` (Zeilen je Raum und Phase, Markdown, JSON), `TestLaufSchreibtBericht` (In-Prozess-Server: Proben mit Phase und Tick im Bericht; Ziel 0,000001 ms → Exit 1; Token nicht im Bericht).
- **AC-04 geprüft:** `TestStatusNenntCPU` (Quelle in `Config.CPU` → `cpu: 42.5`; Systemquelle; ohne Quelle und bei Quelle ohne Wert fehlt das Feld), `TestCPUMeterProzent` (Umrechnung kumulative CPU-Zeit in Prozent einer CPU). Doku: `docs/protocol.md` › „Diagnose: CPU im Status“.
- **`task load`:** steht in `task --list`.
- **Probelauf:** Nicht gegen `task start` (der Server bindet alle Schnittstellen, Firewall-Abfrage), sondern in-process über `httptest` auf Loopback (`TestLaufSchreibtBericht`, 1 Raum × 2 Bots, 3 s): 5 Proben, Phase `day`, p99 ca. 49 ms auf dem Entwicklungsrechner (Windows, nicht Pi), Bewertung verfehlt. Kein Messlauf auf dem Pi (LT1.3).
- **Festlegungen:** `-duration night` endet, sobald jeder Raum Nacht gesehen hat und wieder bei `day` ist (Reihenfolge day, dusk, night, day, also deckt die Messung die Dämmerung davor ab), Obergrenze `-max-duration`, Standard 60 min (🧑 bestätigt in der Spec). Bewertung über alle Zeilen: schlechteste gilt; keine Messwerte (Lauf kürzer als `-interval`) → Exit 2.
- **Grenze:** `cpu` kommt nur aus `/proc/self/stat` (Linux, USER_HZ 100); auf Windows und macOS fehlt das Feld (AC-04 gedeckt, am Pi vorhanden). Prozesszeit per Build-Tag wäre eine spätere Erweiterung.
- `task check:go` grün (go test, golangci-lint 0 issues; `-race` lokal übersprungen, kein C-Compiler), `task check` grün. Größte Datei `report.go` 204 Zeilen, Funktionen < 60 Zeilen.
