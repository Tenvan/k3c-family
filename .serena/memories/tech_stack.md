# Tech Stack

- Spiel: Go 1.27 + Ebitengine 2.10 (`native/k3c-game`, eigenes Modul), Audio über oto.
- Engine: Go 1.27 (`go.mod`), golangci-lint 2.14 (funlen, gocyclo, depguard; `.golangci.yml`).
- Werkzeuge und Tests der Wurzel: TypeScript, Vitest, Oxlint, bun (`tests/`, `tools/paper`).
- Desktop-Tools: Wails 2.16.0 (k3c-workbench), WebView2; Frontends mit Radix Themes über `tools/webkit`.
- Tasks: Go Task 3.53 (`Taskfile.yml`, je Projekt eigenes Taskfile). Optional: gcc/clang (`go test -race`).
- Versionen müssen zwischen `requirements.md`, `go.mod`, `.github/workflows/ci.yml` übereinstimmen.
- Plattform: Windows 11 (PC/Mini-PC), Gamepad und Tastatur.
