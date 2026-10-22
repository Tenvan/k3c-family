# Tech Stack

- Client: TypeScript 7, Phaser 4, Vite 8, Vitest 5 (happy-dom), Oxlint. Node 22 (`.nvmrc`), npm.
- Server: Go 1.27 (`go.mod`), golangci-lint 2.14 (funlen, gocyclo, depguard; `.golangci.yml`).
- Desktop-Tools: Wails 2.16.0 (k3c-workbench, k3c-studio, k3c-starter), WebView2; Frontends mit Radix Themes über `tools/webkit`.
- Tasks: Go Task 3.53 (`Taskfile.yml`, je Projekt eigenes Taskfile). Optional: gcc/clang (`go test -race`), Docker.
- Versionen müssen zwischen `requirements.md`, `.nvmrc`, `go.mod`, `.github/workflows/ci.yml` übereinstimmen.
- Plattform: Windows 11, Zielgerät Edge auf Xbox (Gamepad API); Server im Heimnetz, Dev-Port 5183, Server 8080.
