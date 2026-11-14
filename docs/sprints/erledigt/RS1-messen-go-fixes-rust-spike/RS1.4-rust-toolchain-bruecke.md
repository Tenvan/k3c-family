# RS1.4 · Rust-Toolchain, Taskfile, CI und Brücke ohne cgo

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** rs1/4-rust-toolchain-bruecke
- **Abhängig von:** –
- **Tickets:** B-725
- **Kriterien:** AC-05

## Ziel

`task rust:build` baut eine Rust-Bibliothek `k3c-core` für windows/amd64 und linux/arm64, Go lädt sie ohne cgo und ruft eine Probe-Funktion auf; CI baut und testet beide Plattformen.

## Kontext

006: Brücke ohne cgo, weil lokal kein C-Compiler liegt (`check:race` läuft deshalb still durch). Windows: `syscall.LoadDLL`/`NewProc`; Linux arm64: ohne cgo z. B. `purego` (prüfen, ob eine Abhängigkeit vertretbar ist) oder Kindprozess mit Pipe als Rückfall. Der Pi läuft im Docker-Image (`Dockerfile`, CI-Job docker baut amd64/arm64). Befehle nur über `task` (Projekt-Taskfile im eigenen Ordner, Namensraum `rust`). `requirements.md` nennt Werkzeug-Versionen.

## Erlaubte Dateien

- `rust/**` (neu: Cargo-Workspace, Crate `k3c-core`)
- `rust/Taskfile.yml`, Eintrag im Root-`Taskfile.yml` (includes)
- `engine/core/**` (neu: Go-Brücke)
- `.github/workflows/ci.yml`, `Dockerfile`, `requirements.md`, `.gitignore`

## Nicht-Ziele

Keine Sim-Logik in Rust (RS1.5/RS1.6), keine Änderung an `engine/sim`.

## Schritte

1. Rust-Version festlegen (stable, in `requirements.md` und `rust-toolchain.toml`).
2. Crate `k3c-core` als `cdylib` mit Probe-Funktion (Version, Addition) und C-ABI.
3. Go-Paket `engine/core`: Lädt die Bibliothek, Fehlermeldung mit Pfad, wenn sie fehlt; Test ruft die Probe.
4. Messung der Aufrufkosten je Aufruf (Benchmark), Zahl in 006.
5. CI: Rust-Build amd64 und arm64, `go test ./engine/core` auf beiden; Dockerfile kopiert die `.so`.

## Fertig, wenn

- [ ] AC-05: `task rust:build` baut beide Ziele; `go test ./engine/core` grün unter Windows ohne cgo; CI-Jobs amd64/arm64 grün.
- [ ] Aufrufkosten Go→Rust je Aufruf stehen in 006.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-05 umgesetzt: `task rust:build|test|lint|build:arm64` (Namensraum `rust`), Crate `k3c-core` (cdylib + rlib, C-ABI, `panic = abort`), Toolchain 1.99 in `rust/rust-toolchain.toml` und `requirements.md`. Geprüft lokal: DLL (windows/amd64) gebaut, `cargo test`/`clippy -D warnings`/`fmt` grün, `go test ./engine/core` grün unter Windows ohne cgo; linux/arm64 lokal nur `cargo check` (kein Linker), Go-Brücke per `GOOS=linux GOARCH=arm64 go vet -tags k3ccore`. CI-Job `rust` (windows amd64 + linux arm64, nur Push/Dispatch wie `go-arm64`): ungeprüft bis zum Merge nach develop.
- Aufrufkosten Go→Rust: 41 ns je Aufruf, 0 allocs (006 › Brücke).
- Abweichungen: (1) Linux-Brücke nur mit Build-Tag `k3ccore`, weil purego das Go-Binary dynamisch an libc bindet und das Image `distroless/static` das nicht laden kann; Standard-Build bleibt statisch. (2) Dockerfile bewusst unverändert (kein Docker lokal; Umstellung auf `distroless/cc` + `.so` erst bei bestandenem Spike). (3) `purego` v0.10.2 als neue Abhängigkeit (nur unter dem Tag genutzt), `golang.org/x/sys` direkt.
- Geprüft: `task check:diff`.
