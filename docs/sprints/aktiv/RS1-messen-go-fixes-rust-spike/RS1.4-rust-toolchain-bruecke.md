# RS1.4 · Rust-Toolchain, Taskfile, CI und Brücke ohne cgo

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
