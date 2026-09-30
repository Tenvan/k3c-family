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
