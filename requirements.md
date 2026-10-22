# Voraussetzungen

Was auf einem Entwickler-Rechner installiert sein muss. Alles Weitere (TypeScript, Vite, Vitest, Oxlint, Phaser)
holt `npm ci` aus `package-lock.json`. Versionen gelten, solange diese Datei, `.nvmrc`, `go.mod` und
`.github/workflows/ci.yml` übereinstimmen; wer eine davon ändert, passt die anderen mit an.

| Werkzeug | Version | Wofür | Quelle der Version | Prüfen |
|---|---|---|---|---|
| Git | aktuell | Repo, Branches | – | `git --version` |
| Node.js (mit npm) | 22 (neuere laufen lokal meist auch) | Client bauen, testen, Dev-Server | `.nvmrc` | `node --version` |
| Go | 1.27 | Go-Server, `data/embed.go`, `task check:go` | `go.mod` | `go version` |
| golangci-lint | 2.14 | Go-Lint mit Komplexitäts-Budget | `.github/workflows/ci.yml` | `golangci-lint --version` |
| Wails-CLI | 2.16.0 | Fenster von `tools/k3c-workbench` bauen (`task k3c-workbench`, `task k3c-workbench:build`); auch Asset-Studio und Starter | `tools/k3c-workbench/go.mod` | `wails version` |
| WebView2-Laufzeit | aktuell (in Windows 11 enthalten) | Fenster von `k3c-workbench` und Asset-Studio | – | `wails doctor` |
| Go Task | 3.53 (so getestet; der Aufruf `task <projekt>` für den Task `default` eines Namensraums braucht eine neuere 3.x) | Alle Befehle (`Taskfile.yml`, `task --list`) | `Taskfile.yml` | `task --version` |
| C-Compiler (optional) | `gcc` oder `clang` | Race-Detector `go test -race` in `task go:race`; ohne Compiler nur ein Hinweis, die CI prüft es (B-077) | – | `gcc --version` |
| Docker (optional) | aktuell, mit `buildx` | Image des Heimnetz-Servers (`Dockerfile`, `compose.yaml`); sonst prüft es die CI | – | `docker buildx version` |

Zum Testen am Gerät, keine Installation im Repo: ein aktueller Browser (Edge oder Chrome) für `task dev`,
Edge auf der Xbox für den Gamepad-Test (Anleitung im README).

## Installation unter Windows

PowerShell öffnen, alles über `winget` (in Windows 11 enthalten). Nach der Installation **ein neues Terminal
öffnen**, sonst fehlt der `PATH`-Eintrag.

```powershell
winget install --id Git.Git -e
winget install --id OpenJS.NodeJS.22 -e
winget install --id GoLang.Go -e
winget install --id GolangCI.golangci-lint -e
winget install --id Task.Task -e --source winget
go install github.com/wailsapp/wails/v2/cmd/wails@v2.16.0
# danach den Ordner go/bin im Benutzerprofil in den PATH aufnehmen (dort liegt wails.exe)
```

Prüfen (Ausgabe muss zur Tabelle oben passen):

```powershell
git --version; node --version; go version; golangci-lint --version
```

Hinweise:

- `winget` nimmt die jeweils neueste Version. Bei `Go` und `golangci-lint` muss sie ≥ der Tabelle sein
  (Go 1.27, golangci-lint 2.14); eine feste Version erzwingt `--version 2.14.0`.
- Ohne `winget`: Installer von den Download-Links unten, Standardoptionen, „Add to PATH“ aktiv lassen.
- Ohne Go läuft alles außer `task check:go`; Client und `task check` brauchen nur Node.
- Skripte blockiert von der Execution Policy (`npm.ps1 kann nicht geladen werden`)?
  Zuerst `npm.cmd` statt `npm` aufrufen (oder Git Bash/`cmd` nutzen); das ändert keine Systemeinstellung.
  Nur wenn das nicht reicht: `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` (dauerhaft, gilt für alle Skripte des Benutzerkontos, ohne Adminrechte).

## Einrichten

```bash
npm ci
npm ci --prefix tools/k3c-workbench/frontend
npm ci --prefix tools/k3c-studio/frontend
npm ci --prefix tools/k3c-starter/frontend
```

Die zweite Zeile nur für `tools/k3c-workbench` (Frontend der Oberfläche, `task check:dev`), die dritte für das Asset-Studio
(`task check:studio`), die vierte für den Windows-Starter (`task check:starter`); `task install` macht alle vier.

## Alles prüfen

```bash
task check
```

```bash
task check:go
```

Downloads: [nodejs.org](https://nodejs.org/), [go.dev/dl](https://go.dev/dl/),
[golangci-lint](https://golangci-lint.run/docs/welcome/install/).
