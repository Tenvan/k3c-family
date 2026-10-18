# BT1.2 · Wails-Starter für Windows

- **Status:** offen
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

Spec: B-041 (AC-01, AC-02), Sprint-Kriterium AC-02. Vorbild: `tools/k3c-studio` (Asset-Studio: eigenes Go-Modul mit `replace` auf das Hauptmodul, Frontend unter `frontend/`, `wails.json`, Tasks `studio:frontend`/`check:studio`, CI-Job `k3c-studio` mit `wails build -s` unter Windows, feste EXE `bin/k3c-studio.exe`) und `tools/k3c-dev`. Server-Start: `cmd/k3c-server/main.go` (Konfiguration über Umgebungsvariablen `K3C_*`, Logzeile `dev=…`, Port 8080, `dist/` ausliefern); der Starter nutzt diesen Kern und kopiert keine Logik, falls die Server-Logik dafür nicht importierbar ist (liegt in `package main`), legt die Session sie in ein Paket unter `engine/` oder `cmd/internal` und lässt `cmd/k3c-server` es nutzen (kleinste Änderung, Test). Regeln: feste EXE mit festem Pfad (keine Firewall-Abfrage), Proben nur auf Loopback, kein `go run`; Komplexitäts-Budget; keine neue Abhängigkeit außer Wails. Nicht Pflicht für den Betrieb (Entscheidung 001), Docker-Betrieb bleibt wie er ist.

## Erlaubte Dateien

- `tools/k3c-starter/` (neu: Go-Modul, Wails-Fenster, kleines Frontend, Tests, `codemap.md`)
- `cmd/k3c-server/` und ein neues Paket für den gemeinsamen Server-Start, nur soweit der Starter es braucht (Grenzfall, begründen im Ergebnis)
- `Taskfile.yml` (Tasks `k3c-starter:build`, ggf. `check:starter` in `check:all`), `.github/workflows/ci.yml` (Job für den Starter), `.gitignore`, `go.work`/`go.mod`-Verweise falls nötig
- Planungsdateien dieses Sprints, B-041

## Nicht-Ziele

Kein QR-Code, keine Server-Suche (B-040 verworfen), kein Docker-Ersatz, keine Pi-Variante, kein Autostart mit Windows, keine neuen Spielfunktionen.

## Schritte

1. Server-Start so zugänglich machen, dass der Starter ihn im Prozess aufrufen kann (kleines Paket, `cmd/k3c-server` ruft es auf; Verhalten unverändert, bestehende Tests grün).
2. `tools/k3c-starter/` nach dem Muster von `tools/k3c-studio` anlegen: Wails-Fenster mit Status (läuft/gestoppt/Fehler), LAN-Adresse (`http://<IP>:<Port>`), Schalter Start/Stopp; belegter Port → Meldung im Fenster statt Absturz. Go-Tests für Port-Fehler und Adressermittlung.
3. `Taskfile.yml`: `k3c-starter:build` (nach `bin/k3c-starter.exe`, Frontend zuerst, `wails build -s`), in `check:all` einhängen; `ci.yml`: Job analog `k3c-studio` (Frontend, `go test`, Lint, `wails build -s`).
4. Probe nur auf Loopback: EXE starten, `curl http://127.0.0.1:<Port>/api/health`, EXE beenden. Doppelklick am PC zeigt Fenster mit Status und Adresse: Abnahme durch 🧑 (live), vom Agenten nur vorbereitet.
5. `task check`, `task check:go`, `task check:all` (soweit auf dem Rechner möglich).

## Fertig, wenn

- [ ] AC-02: `tools/k3c-starter` startet Server und Fenster, `go test ./...` und `golangci-lint` im Modul grün, `task k3c-starter:build` erzeugt `bin/k3c-starter.exe`, der CI-Job baut es (B-041/AC-01); Probe auf Loopback liefert `/api/health` (Agent).
- [ ] AC-02 (Abnahme, 🧑): Doppelklick auf `bin/k3c-starter.exe` zeigt Status und Adresse, Port belegt → Meldung im Fenster (B-041/AC-02); bis dahin `angenommen, Validierung offen`.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task k3c-starter:build
task check
task check:go
```

Doppelklick am PC (Abnahme) nur durch 🧑.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
