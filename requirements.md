# Voraussetzungen

Was auf einem Entwickler-Rechner installiert sein muss. Alles Weitere (TypeScript, Vite, Vitest, Oxlint, Phaser)
holt `npm ci` aus `package-lock.json`. Versionen gelten, solange diese Datei, `.nvmrc`, `go.mod` und
`.github/workflows/ci.yml` übereinstimmen; wer eine davon ändert, passt die anderen mit an.

| Werkzeug | Version | Wofür | Quelle der Version | Prüfen |
|---|---|---|---|---|
| Git | aktuell | Repo, Branches | – | `git --version` |
| Node.js (mit npm) | 22 (neuere laufen lokal meist auch) | Client bauen, testen, Dev-Server | `.nvmrc` | `node --version` |
| Go | 1.27 | Go-Server, `data/embed.go`, `npm run check:go` | `go.mod` | `go version` |
| golangci-lint | 2.14 | Go-Lint mit Komplexitäts-Budget | `.github/workflows/ci.yml` | `golangci-lint --version` |

Zum Testen am Gerät, keine Installation im Repo: ein aktueller Browser (Edge oder Chrome) für `npm run dev`,
Edge auf der Xbox für den Gamepad-Test (Anleitung im README).

## Installation unter Windows

PowerShell öffnen, alles über `winget` (in Windows 11 enthalten). Nach der Installation **ein neues Terminal
öffnen**, sonst fehlt der `PATH`-Eintrag.

```powershell
winget install --id Git.Git -e
winget install --id OpenJS.NodeJS.22 -e
winget install --id GoLang.Go -e
winget install --id GolangCI.golangci-lint -e
```

Prüfen (Ausgabe muss zur Tabelle oben passen):

```powershell
git --version; node --version; go version; golangci-lint --version
```

Hinweise:

- `winget` nimmt die jeweils neueste Version. Bei `Go` und `golangci-lint` muss sie ≥ der Tabelle sein
  (Go 1.27, golangci-lint 2.14); eine feste Version erzwingt `--version 2.14.0`.
- Ohne `winget`: Installer von den Download-Links unten, Standardoptionen, „Add to PATH“ aktiv lassen.
- Ohne Go läuft alles außer `npm run check:go`; Client und `npm run check` brauchen nur Node.
- Skripte blockiert von der Execution Policy (`npm.ps1 kann nicht geladen werden`)?
  Einmalig `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned`, oder `npm.cmd` statt `npm` aufrufen.

## Einrichten

```bash
npm ci
```

## Alles prüfen

```bash
npm run check
```

```bash
npm run check:go
```

Downloads: [nodejs.org](https://nodejs.org/), [go.dev/dl](https://go.dev/dl/),
[golangci-lint](https://golangci-lint.run/docs/welcome/install/).
