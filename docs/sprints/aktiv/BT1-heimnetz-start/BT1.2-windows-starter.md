# BT1.2 · Wails-Starter für Windows

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** bt1/2-windows-starter
- **Abhängig von:** BT1.1
- **Tickets:** B-041
- **Kriterien:** AC-02

## Ziel

Das Wails-Fenster `tools/k3c-starter` startet den Spielserver und zeigt Status und LAN-Adresse; `task k3c-starter:build` erzeugt `bin/k3c-starter.exe`, die CI baut es mit.

## Kontext

Spec: B-041 (AC-01, AC-02), Sprint-Kriterium AC-02. Vorbild: `tools/k3c-studio` (Asset-Studio: eigenes Go-Modul mit `replace` auf das Hauptmodul, Frontend unter `frontend/`, `wails.json`, Tasks `studio:frontend`/`check:studio`, CI-Job `k3c-studio` mit `wails build -s` unter Windows, feste EXE `bin/k3c-studio.exe`) und `tools/k3c-workbench`. Server-Start: `cmd/k3c-server/main.go` (Konfiguration über Umgebungsvariablen `K3C_*`, Logzeile `dev=…`, Port 8080, `dist/` ausliefern); der Starter nutzt diesen Kern und kopiert keine Logik, falls die Server-Logik dafür nicht importierbar ist (liegt in `package main`), legt die Session sie in ein Paket unter `engine/` oder `cmd/internal` und lässt `cmd/k3c-server` es nutzen (kleinste Änderung, Test). Regeln: feste EXE mit festem Pfad (keine Firewall-Abfrage), Proben nur auf Loopback, kein `go run`; Komplexitäts-Budget; keine neue Abhängigkeit außer Wails. Nicht Pflicht für den Betrieb (Entscheidung 001), Docker-Betrieb bleibt wie er ist.

## Erlaubte Dateien

- `tools/k3c-starter/` (neu: Go-Modul, Wails-Fenster, statische Oberfläche `web/`, Tests, `codemap.md`)
- `Taskfile.yml` (Tasks `k3c-starter:build`, `check:starter`, in `check:all`), `.github/workflows/ci.yml` (Job für den Starter), `CLAUDE.md` (Struktur: ein Satz zum Starter)
- Planungsdateien dieses Sprints, B-041
- `cmd/k3c-server/` bleibt unverändert: der Starter ruft die EXE als Kindprozess auf (kein Umbau von `package main`)

## Nicht-Ziele

Kein QR-Code, keine Server-Suche (B-040 verworfen), kein Docker-Ersatz, keine Pi-Variante, kein Autostart mit Windows, keine neuen Spielfunktionen.

## Schritte

1. Server-Start so zugänglich machen, dass der Starter ihn im Prozess aufrufen kann (kleines Paket, `cmd/k3c-server` ruft es auf; Verhalten unverändert, bestehende Tests grün).
2. `tools/k3c-starter/` nach dem Muster von `tools/k3c-studio` anlegen: Wails-Fenster mit Status (läuft/gestoppt/Fehler), LAN-Adresse (`http://<IP>:<Port>`), Schalter Start/Stopp; belegter Port → Meldung im Fenster statt Absturz. Go-Tests für Port-Fehler und Adressermittlung.
3. `Taskfile.yml`: `k3c-starter:build` (nach `bin/k3c-starter.exe`, Frontend zuerst, `wails build -s`), in `check:all` einhängen; `ci.yml`: Job analog `k3c-studio` (Frontend, `go test`, Lint, `wails build -s`).
4. Probe nur auf Loopback: EXE starten, `curl http://127.0.0.1:<Port>/api/health`, EXE beenden. Doppelklick am PC zeigt Fenster mit Status und Adresse: Abnahme durch 🧑 (live), vom Agenten nur vorbereitet.
5. `task check`, `task check:go`, `task check:all` (soweit auf dem Rechner möglich).

## Fertig, wenn

- [x] AC-02: `tools/k3c-starter` startet Server und Fenster, `go test ./...` und `golangci-lint` im Modul grün (`task check:starter`), `task k3c-starter:build` erzeugt `bin/k3c-starter.exe`, der CI-Job `k3c-starter` baut es mit `wails build -s` (B-041/AC-01); Probe auf Loopback (Port 18080) liefert `/api/health` (Agent).
- [ ] AC-02 (Abnahme, 🧑): Doppelklick auf `bin/k3c-starter.exe` zeigt Status und Adresse, Port belegt → Meldung im Fenster (B-041/AC-02); bis dahin `angenommen, Validierung offen`.
- [x] `task test` und `task check:starter` grün; `task check` nur durch den bestehenden Oxlint-Fehler (`src/tools/dm.ts`, auch auf `develop`) rot.

## Prüfen

```bash
task k3c-starter:build
task check
task check:go
```

Doppelklick am PC (Abnahme) nur durch 🧑.

## Ergebnis

- **AC-02 umgesetzt, geprüft (Agent), Abnahme am PC offen:** `tools/k3c-starter` (Wails-Fenster, eigenes Go-Modul, statische Oberfläche `web/index.html` ohne Node-Build) startet `bin/k3c-server.exe` als Kindprozess und zeigt Status, Adressen (`localhost` und IPv4 im Heimnetz, mit „Kopieren“) und die letzten 40 Meldungen des Servers; Doppelklick startet den Server, Fenster schließen beendet ihn.
  - Abweichung vom Entwurf: Der Starter ruft die Server-EXE als Kindprozess auf, statt `cmd/k3c-server` umzubauen (`package main` mit Umgebungs-Konfiguration und Signal-Handler); `cmd/k3c-server` bleibt unverändert, die Logik ist nicht kopiert. `task k3c-starter:build` baut dafür `dist/`, `bin/k3c-server` und `bin/k3c-starter`; `K3C_DEV` ist im Starter aus, außer es ist gesetzt.
  - Tests (`server_test.go`, Testprogramm als falscher Server auf Loopback): Start, „läuft“ mit Adressen, sauberes Beenden (Server bekommt Strg+Pause und beendet sich selbst), Port belegt → Meldung, unerwartetes Ende, fehlende EXE, Layout-Suche.
  - Probe mit der echten EXE auf Loopback (Port 18080, nicht Port 8080): `/api/health` antwortet; Fenster schließen → Server-Log „🛑 Server fährt herunter (Signal)“, Port frei; Starter hart beendet (`Stop-Process -Force`) → Server verschwindet über das Job-Objekt, Port frei. Das Fenster selbst (Anzeige) hat der Agent nur mit Mock-Daten im Browser-Pane gesehen, nicht in der Wails-Hülle.
  - Dazu: Task `check:starter` (in `check:all`), CI-Job `k3c-starter` (Windows), `CLAUDE.md` nennt den Starter.
- **Review (getrennter Reviewer) und behobene Befunde:** (1) verwaister Server nach hartem Ende des Starters → Job-Objekt `KILL_ON_JOB_CLOSE`; (2) hartes Beenden überspringt das Speichern → sauberes Beenden per Strg+Pause (eigene Prozessgruppe und verborgene Konsole), Kill erst nach 10 s; (3) falsche Meldung bei Stop während des Starts → Flag `stopping` statt Textvergleich. Offen/akzeptiert: Die Port-Prüfung testet nur Loopback (der Server bindet alle Adressen); ein fremder Dienst, der den Port zwischen Prüfung und Start belegt, wird vom Server-Ende abgefangen („beendet sich unerwartet“).
- **Vom Agenten nicht gemacht:** Abnahme am PC (Doppelklick, Port belegt) und ein Lauf mit Port 8080.
- Neue Tickets: keine.
