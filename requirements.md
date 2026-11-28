# Voraussetzungen

Was auf einem Entwickler-Rechner installiert sein muss. Alles Weitere (TypeScript, Vitest, Oxlint)
holt `bun install --frozen-lockfile` aus `bun.lock`. Versionen gelten, solange diese Datei, `.nvmrc`,
`rust/rust-toolchain.toml` und `.github/workflows/ci.yml` übereinstimmen; wer eine davon ändert, passt die anderen mit an.

| Werkzeug | Version | Wofür | Quelle der Version | Prüfen |
|---|---|---|---|---|
| Git | aktuell | Repo, Branches | – | `git --version` |
| Node.js | 22 (neuere laufen lokal meist auch) | Laufzeit der Werkzeuge (`bunx` startet Vitest, tsc darüber) | `.nvmrc` | `node --version` |
| Bun | 1.4.2 | Pakete installieren (`bun install`), Werkzeuge starten (`bunx`) | `.github/workflows/ci.yml` | `bun --version` |
| Rust | laut `rust/rust-toolchain.toml` (MSVC, Windows) | Rust-Spiel (`task build`, `task rust:game:run`) und Werkzeuge `rust/tools/` (`task check:dev`, `task buildall`) | `rust/rust-toolchain.toml` | `rustc --version` |
| Tauri-CLI | 2.12.6 | Fenster von Workbench und Studio (`:window`, Bau des Studios) über `bunx tauri`; die Tasks holen sie selbst mit `bun install` in `rust/tools` | `rust/tools/package.json` | `bunx tauri --version` (in `rust/tools`) |
| WebView2-Laufzeit | aktuell (in Windows 11 enthalten) | Fenster von `k3c-workbench` und `k3c-studio` | – | – |
| Go Task | ab 3.54.0 (geprüft von `check_run`: Testmuster an Task-Ziele brauchen das Quoten von `CLI_ARGS`, B-868; Zahl in `rust/tools/crates/k3c-workbench/src/mcp/check/taskversion.rs`) | Alle Befehle (`Taskfile.yml`, `task --list`) | `Taskfile.yml` | `task --version` |
| ffmpeg (Spieltest, `gen:soundatlas`) | 8 oder neuer mit Filter `whisper` (gyan.dev „full build“, geprüft mit 9.0.2) | Spieltest-Aufnahme und Auswertung (Transkript, Bilder je Marker) auf der Seite Spieltest, Sound-Atlas | – | `ffmpeg -version`, `ffmpeg -filters` (Zeile `whisper`) |
| Whisper-Modell (nur Spieltest) | `ggml-large-v3-turbo-german-q5_0.bin` (574041195 Byte, SHA-256 `15e92e3db0993c52fffa781513eec9253475331c1be808f8fb409285c9d9d030`, nicht eingecheckt) | Transkript (Sprache de); Ablage `models/` (ignoriert), sonst erste Datei `models/whisper/*.bin`, vorrangig der Pfad in `K3C_WHISPER_MODEL`; Quelle `https://huggingface.co/MolyProduction/whisper-large-v3-turbo-german-ggml-q5_0` (Hand-Download, die Workbench lädt nichts selbst) | – | Datei vorhanden (die Seite Spieltest meldet fehlendes Modell), Hash mit `certutil -hashfile <Datei> SHA256` |

Zum Testen am Gerät, keine Installation im Repo: ein Windows-PC mit Controllern für das Rust-Spiel (`task rust:game:run`).

## Installation unter Windows

PowerShell öffnen, alles über `winget` (in Windows 11 enthalten). Nach der Installation **ein neues Terminal
öffnen**, sonst fehlt der `PATH`-Eintrag.

```powershell
winget install --id Git.Git -e
winget install --id OpenJS.NodeJS.22 -e
winget install --id Oven-sh.Bun -e
winget install --id Task.Task -e --source winget
```

Rust über rustup ([rustup.rs](https://rustup.rs/)) samt den MSVC-Build-Tools; die Version zieht `rustup` aus
`rust/rust-toolchain.toml`.

Prüfen (Ausgabe muss zur Tabelle oben passen):

```powershell
git --version; node --version; bun --version; rustc --version; task --version
```

Hinweise:

- `winget` nimmt die jeweils neueste Version; sie muss ≥ der Tabelle sein.
- Ohne `winget`: Installer von den Download-Links unten, Standardoptionen, „Add to PATH“ aktiv lassen.
- Ohne Rust läuft `task check` (Regeln, Planung, Papierschnitt); das Spiel und `task check:dev` brauchen Rust.
- Skripte blockiert von der Execution Policy? Git Bash oder `cmd` nutzen; das ändert keine Systemeinstellung.
  Nur wenn das nicht reicht: `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` (dauerhaft, gilt für alle Skripte des Benutzerkontos, ohne Adminrechte).

## Einrichten

```bash
bun install --frozen-lockfile
bun install --frozen-lockfile --cwd tools/k3c-workbench/frontend
bun install --frozen-lockfile --cwd tools/k3c-studio/frontend
```

Die zweite und dritte Zeile für die Frontends von Workbench und Studio (`task check:dev`); `task install` macht alle drei.
Einmal je Klon: `task golden-setup`.

Temp im Repo: `check_run` setzt `TMP` und `TEMP` seiner Läufe auf `.work/tmp`, die Rust-Tests legen ihre Ordner unter
`K3C_TMP` bzw. `.work/tmp` an. Das `env` der Wurzel-`Taskfile.yml` setzt dieselben Variablen, überschreibt vorhandene
Windows-Variablen aber vermutlich nicht; bei Läufen in der Shell setzt sie darum der Aufrufer selbst. Für `<Repo>\.work`
einmal eine Ausnahme im Virenscanner (Norton) anlegen, sonst prüft er jede Test-EXE.

## Alles prüfen

```bash
task check
```

```bash
task check:dev
```

```bash
task rust:test
```

Downloads: [nodejs.org](https://nodejs.org/), [bun.sh](https://bun.sh/), [rustup.rs](https://rustup.rs/),
[taskfile.dev](https://taskfile.dev/installation/).
