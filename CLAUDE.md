# K3C – Family Three Crowns

Couch-Koop-Side-Scroller im Stil von Kingdom Two Crowns. **Natives Spiel in Rust** (`rust/crates/k3c-game`, Port aus Go, Projekt RSU)
am PC; die Go-Engine (`engine/`, `cmd/`) bleibt, weil die Workbench sie im Prozess rechnet (Balancing, Simulation, Level, Welten); ungenutzte Teile entfernt RU59, den Rest RU40 und RU41. Browser-Client, Spielserver und Online-Spiel sind entfernt (B-850, WR1). Die Kommunikation mit dem Nutzer ist deutsch.

- Design & Regeln: `docs/game-design.md` (nur bei Bedarf lesen)
- Aktueller Stand & nächste Schritte: `docs/roadmap.md`
- **Second Worker:** Nur wenn Ralf sagt „Du bist der Second Worker“, gilt der Skill `second-worker` (Lane `zweit`); sonst ist jeder Agent Hauptrechner.
- **Arbeitsweise:** `docs/arbeitsweise.md` (Domänen, autonomer Ablauf, Review, Komplexitäts-Budget). Für eine Session
  genügt der Skill `k3c-session` mit `plan_next`; die ganze Datei nur beim Planen oder bei Regelfragen lesen.
- **Glossar:** `docs/glossar.md` – verbindliche Begriffe; beim Schreiben von Specs und Texten lesen, neue Begriffe dort zuerst eintragen.
- **Branches:** Es gibt nur `main` (geschützt, seit 2026-11-14 ohne `develop`); gearbeitet wird in Feature-Branches, PRs zielen auf `main`; **ein PR je Sprint** (Branch `sprint/<präfix>`, ein Commit je Session).
- **Projekte und Sprints:** Die nächste offene Session nach **Projekt-Rang** liefert `plan_next` (k3c-workbench); ohne k3c-workbench
  `docs/projekte/README.md` und `docs/sprints/aktiv/` lesen. `docs/sprints/geplant/` nur beim
  Planen lesen, `docs/sprints/erledigt/` nur auf Nachfrage. Übersicht: `docs/sprints/README.md`.
  Parallel laufen Sessions, die sich weder über Abhängigkeit noch Bereich noch Sperre berühren (Arbeitswellen, `docs/arbeitsweise.md` › Parallelität und Wellen); der Rang ordnet nur bei Kollision.
- **Tickets:** `docs/backlog/` (Index `README.md`, ein Ticket pro Datei). Neue Ideen/Probleme sofort als Ticket anlegen,
  nicht nebenbei umsetzen. **Es gibt keine ungeplanten Tickets** (Regel 🧑 2026-11-14): Jedes neue Ticket bekommt beim Anlegen
  Projekt und Sprint (`plan_create` lehnt sonst ab); fehlt ein passender Sprint, zuerst den Sprint anlegen. Erledigte/verworfene liegen samt Index `archiv/README.md` in `docs/backlog/archiv/` (nur auf Nachfrage lesen); der Index `docs/backlog/README.md` führt nur offene und eingeplante. Duplikate und Ersatz über die Felder `Duplikat von:`/`Ersetzt:` (B-696).
- **Vorlagen sind Pflicht:** Tickets, Sprints und Sessions nur als Kopie von `docs/vorlagen/*.md`; `tests/planning.test.ts` prüft das.
- **Planung nur über MCP:** Tickets, Sprints und Sessions anlegen und ändern ausschließlich mit `plan_create`, `plan_set`,
  `plan_sections` (mehrere Abschnitte in einem Aufruf), `plan_section`, `plan_delete` (lesen: `plan_list`, `plan_get`,
  `plan_next`) von k3c-workbench; von Hand nur, wenn k3c-workbench nicht läuft. Läuft sie, lehnt der Hook
  `tools/hooks/planGuard.ts` Edit, Write und schreibende Shell-Befehle auf `docs/backlog`, `docs/sprints`, `docs/projekte` ab.
  Überblick und Zählung über `plan_list` (offen = Status offen, eingeplant = Status eingeplant, ungeplant = ohne Sprint). **Fehlt dem MCP eine Funktion (🧑 2026-11-25):** Direkter Zugriff auf Ticket-, Sprint-, Session- oder Projektdateien ist untersagt, auch nicht „nur dieses eine Mal“ (Shell, Skript, Edit); stattdessen schlägt Claude eine sinnvolle Erweiterung der Tools zur zeitnahen Umsetzung vor (Ticket oder direkt als Änderung an `tools/k3c-workbench`).
- **SDD:** Ticket und Sprint-README sind die Spec (Kriterien `AC-01` …), Sessions erfüllen genannte Kriterien.
  Freigabe (`Spec: freigegeben`) und manuelle Abnahmen nur durch den Nutzer; Ausnahme: Sagt der Nutzer ausdrücklich „autonom abarbeiten“, darf Claude Specs freigeben (Feld `Freigabe`: „autonom, Claude“), offene Fragen mit Default entscheiden und im Nachgang melden; manuelle Abnahmen bleiben beim Nutzer. **Klasse A** (Typ Problem oder Schuld, Domäne DEV, INF oder PAP, ohne Spielregeln, Protokoll, Spielstand-Format, neue Abhängigkeit) gibt Claude dauerhaft selbst frei („Klasse A, Claude“) und meldet gesammelt. Details: `docs/arbeitsweise.md` › SDD.
- **Spielstruktur (003):** Ein Raum hat einen Spielstand mit n Inseln, jede Insel n Stufen; Stufen sind pro Spieler frei begehbar und laufen alle weiter. Im Go-Code rechnet ein Raum eine Insel mit mehreren Stufen im selben Takt, Einzelwechsel je Spieler und einem Material-Vorrat je Insel (`engine/sim/island*.go`, SP12–SP14); noch offen: mehrere Inseln und Inselwechsel (B-103). Regeln: `docs/rules/stufen.md`.
- Architektur-Entscheidungen: `docs/decisions/` – **001: Die Spiel-Engine ist Go** (`engine/`), neue Mechaniken entstehen
  dort; Spielserver und Browser-Client von 001 sind seit WR1 entfernt (B-850).
  **004: Asset-Studio getrennt vom Spiel**, Schnittstelle sind die Dateien (TS1, B-507).
  **005: Papierschnitt-Grafik aus Specs**, Assets nur über MCP `assets_*` (Server `k3c-studio`), Review im Bereich Assets des
  Studios; das Asset-Studio kehrt seit WZ63 als eigenständiges Studio zurück (Nachtrag in 005), die Workbench hat keine Assets mehr.
  **008: Natives Spiel in Go + Ebitengine** (Projekt NAT, B-775), überholt durch den Rust-Port (Entscheidung 011 dazu steht aus, RU41). Das Go-Spiel `native/` ist seit RU37 entfernt; der letzte Stand steht im Tag `go-referenz-final`, Bau und Messung beschreibt `docs/standards/go-referenz.md`.
  **012: Port-Orakel ist das Verhalten, nicht Bit-Gleichheit.** **013: Neue Mechaniken nur in Rust, Go-Engine eingefroren** (Entwurf, B-1143; Annahme durch Ralf offen).
- Altes Godot-Projekt (nur Referenz): `C:\WORKSPACE\FamilyCrowns`

## Befehle

Voraussetzungen (Node, Go, golangci-lint mit Versionen): `requirements.md`.

```bash
task               # alle Aufgaben anzeigen (Go Task, Taskfile.yml; einziger Einstieg für Befehle)
task install       # Abhängigkeiten holen
task test          # Vitest (Regeln, Planung, Papierschnitt); Filter: task test -- planning
task check:diff    # nur was die geänderten Dateien dieses Commits brauchen (einmal vor jedem Commit; Basis HEAD, Tests nur der geänderten Pakete; Teilaufgaben check:diff:plan, :js, :go)
task check         # Lint + Typecheck + Tests der Wurzel (tests/, tools/paper), parallel
task check:go      # go test + golangci-lint (Rechner mit Go)
task check:race    # Go-Race-Detector (langsam; Abnahme-Ticket, check:all, CI)
task check:dev     # k3c-workbench und k3c-studio (samt tools/workkit): Frontend, go test, golangci-lint
task check:all     # alles inklusive Build
task go:golden     # testdata/golden/ neu schreiben, nur bei gewollter Regel-/Wertänderung (Begründung in den Commit)
task build         # Rust-Spiel bauen (wie rust:game:build, Paket nach dist/k3c-game/; Einzelschritte rust:game:exe, :run, :dev)
task buildall      # alle EXE bauen: Workbench und Studio nach bin/, Rust-Spiel samt Grafik und data/ (alle Welten) nach dist/k3c-game/; laufende EXE vorher schließen; weitergebbar erst mit LIZENZEN.txt im Rust-Paket (Ticket in RU35)
task rust:devwin:exe  # Cheat-Fenster k3c-devwin.exe bauen (F4 im Rust-Spiel, `--devwin`, `K3C_DEVWIN=1`; `task build` legt sie neben k3c-game.exe ins Paket, höchstens 5 MB)
task gen:paper    # Papierschnitt-Grafik rendern (nicht eingecheckt, B-737; build und test rufen es selbst auf)
task pages         # GitHub-Pages-Seite nach _site/ (kopiert nur site/, kein Build)
task k3c-workbench       # Workbench als Client/Server-Dev (Backend -dev + Vite 5191 im Browser); Fenster: k3c-workbench:window, EXE: k3c-workbench:build
task k3c-studio          # Studio (Welten, Assets, Hub) als Client/Server-Dev (Backend -dev MCP 5182, UI-API 8193, Vite 5193); Fenster: k3c-studio:window, EXE: k3c-studio:build
```

**Taskfile-Gruppen:** `Taskfile.yml` hält nur die Kernbefehle (Workspace) und die Prüf-Aggregate `check:*`; jedes Projekt hat sein
Taskfile im eigenen Ordner und erscheint als Namensraum (`task --list`, Tasks-Seite der Workbench): `go` (engine/),
`k3c-workbench` und `k3c-studio` (tools/), `workkit` (tools/workkit, gemeinsamer Go-Code), `rust` (rust/), `balance`, `gen` (Generatoren:
`gen:rules`), `ops` (`ops:aufraeumen`, `ops:backup`). Neue Aufgaben kommen in das Taskfile ihres Projekts, nicht
in die Wurzel.

**Dev ohne Neubau-Fenster:** Jedes EXE-Projekt kennt `build` (EXE bauen), `run` (bauen und starten) und `dev`.
Der Projektname allein (`task k3c-workbench`) startet `dev`. `dev` ist Client/Server: Das Backend läuft mit `-dev` als kleiner Supervisor
(`engine/devrun`), baut sich als Kind (`-headless`, ohne Fenster) und startet es bei jeder Go-Änderung neu; bei Build-Fehler läuft
die alte Version weiter, ein Job-Objekt beendet verwaiste Kinder. Die Oberfläche der Wails-App (`k3c-workbench` 5191, `k3c-studio` 5193)
läuft dabei unter Vite mit Hot Reload im Browser und leitet `/api` an das Kind weiter. `<app>:window` startet
das echte Fenster mit `wails dev`. Supervisor und Kinder werden mit `go build` (Konsole) gebaut, nie per
`wails build`, damit sie sauber per CTRL_BREAK beenden. Wails-Builds (`k3c-workbench:build`) und das
Frontend laufen nur bei geänderten Quellen (`sources`/`generates`); `--force` erzwingt.

Spielserver, Lasttest, TUI und Windows-Starter sind entfernt (B-887, Entscheidung B-850): kein Online-Spiel, das Spiel ist nativ.
Berichte unter `reports/` liest `reports_list`/`report_read`.

**Zwei MCP-Server:** `k3c-workbench` (5180) für Entwicklung, Planung, Prüfläufe, Dienste und Logs; `k3c-studio` (5181) für Welten, Inseln, Chunks, Hub, Assets, `level_generate` und seit WZ63.5 Balancing/Sim mit Berichten und Spielständen (`mcp__k3c-studio__world_*|hub_*|assets_*|sim_*|replay_run|reports_list|report_read|saves_list`, EXE `bin/k3c-studio.exe`); kein Tool steht in beiden.

**k3c-workbench zuerst:** Sind die MCP-Tools `mcp__k3c-workbench__*` verbunden, gehen Prüfungen über `check_run` (statt `task check`,
`task test`, `go test` …), Dienste über `svc_start`/`svc_stop`, Logs über
`logs_errors`/`logs_query`; Berichte und Spielstände über `reports_list`/`report_read`/`saves_list`, Balancing über
`sim_run` und Level über `level_generate` gehen an das Studio (`mcp__k3c-studio__*`). **Simulationen und Balancing sind bis auf Widerruf gesperrt** (Beschluss 🧑 2026-11-11: kein `sim_test`, `sim_run`, Balancing, Lasttest; **Ausnahme: Second Worker, 🧑 2026-11-18**). Danach gilt wieder: **Jeder autonome Testlauf** (Balancing, Stabilität;
offline) startet und läuft **nur über `sim_test`** (`start`, `status`, `stop`, `list`),
nie über `task balance` oder Binaries in der Shell (B-348). Die Shell-Befehle unten nur, wenn k3c-workbench nicht läuft. Fehlen die MCP-Tools, erst prüfen, ob die Workbench läuft, und sie sonst starten (`bin/k3c-workbench.exe`), dann den MCP neu verbinden (`docs/arbeitsweise.md` › Ablage).

**Tasks und Ausführungen laufen ausschließlich über `task`** (Neues in `Taskfile.yml`, nicht in `package.json`).
Vor jedem Commit: `task check:diff` (bzw. `check_run task:check:diff`) einmal grün; es prüft nur die geänderten Dateien dieses Commits, minutenlange Läufe gehören nicht dazu; währenddessen nur gezielte Tests
(`docs/arbeitsweise.md` › Tempo im Lauf).
Die CI (`.github/workflows/ci.yml`) baut und testet zusätzlich das Rust-Spiel unter Windows. `tests/projectRules.test.ts`
prüft Repo-Regeln automatisch (Go-Dateigröße, Golden-Größe, kein Browser-Client und kein Spielserver mehr).

## Struktur

- `data/` – Balancing als JSON (Biome, Gegner, Truppen, Gebäude, Monarch), einzige Quelle für Engine und
  Rust-Spiel (`data/embed.go`: von der Platte aus `K3C_DATA_DIR`, ohne Ordner per `go:embed`). Werte gehören hierher, nicht in den Code.
- `engine/` – Go, bleibt, weil die Workbench die Engine im Prozess rechnet; ungenutzte Teile sind seit RU59 entfernt (Belegtabelle: `docs/arbeitsweise.md` › Go-Engine-Bestand): `sim/` (Simulation, deterministisch), `level/` (Level-Generator), `room/` (Raum mit Stufen und Spielstand,
  Zustand und Ereignisse wie in `docs/protocol.md`), `store/` (Spielstände → `saves/`, Berichte → `reports/`). `engine/`
  importiert nichts aus `tools/` (`engine/internal/rules/imports_test.go`).
- `tools/k3c-workbench` – Entwickler-Werkzeug (Wails-Fenster + MCP-Server `k3c-workbench`) mit den Seiten Dienste, Tasks, Planung,
  Git, MCP, Monitoring und Balancing.
  `tools/k3c-studio` – Studio für Game- und Level-Designer (WZ63): eigene Wails-App mit Welten-Editor, Gebäude-Planer und Assets
  (Review, Katalog, Styleguide, Sound, Quellen; „Im Spiel ansehen“) und eigenem MCP-Server `k3c-studio`; Assets ändern nur die
  MCP-Tools `assets_*`. Das Asset-Studio aus Entscheidung 005 kehrt damit als Studio zurück, die Workbench hat keine Assets mehr.
  `tools/workkit` – gemeinsamer Go-Code beider Apps (eigenes Go-Modul: `mcpkit`, `applog`, `console`, `proc`, `usage`, `uikit`,
  `claude`, `winicon`); importiert weder Workbench noch Studio (depguard), die Apps importieren einander nie.
  `tools/paper/` – Papierschnitt-Renderer und Verträge (Entscheidung 005), Aufruf `bun tools/paper/bin/paper.ts`.
  `tools/assetkit` – Asset-Logik (eigenes Go-Modul, per `replace` eingebunden; geprüft mit `k3c-workbench:gotest`/`lint`).
  `tools/webkit` – gemeinsame Oberflächen-Bausteine der Wails-Frontends (k3c-workbench) als **Atomic Design** auf
  Radix Themes: `lib/`, `atoms/`, `molecules/`, `organisms/`, `templates/` (`@webkit/<ebene>`); Seiten sind die Feature-Ordner der App.
  Verbindlich für jedes Wails-Programm: Radix nur über `@webkit`, rohe Controls nur in `atoms/`, Importe nur nach unten
  (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme, geprüft von `tests/atomicDesign.test.ts`).
  CLIs: `tools/ruletable` (`task gen:rules`), `tools/cleanup` (`task ops:aufraeumen`), `tools/checkdiff` (`task check:diff`). `engine/devrun` – Supervisor der Tasks `<app>:dev` (Neubau und Neustart bei Go-Änderungen).
- `rust/` – Cargo-Workspace: Spiel `rust/crates/k3c-game` (Leitfaden `docs/standards/rust-port.md`), Rig `k3c-rig`, Simulation `k3c-sim`, dazu `k3c-data`, `k3c-room`, `k3c-platform`; Paket `task build` nach `dist/k3c-game/`. Rig-Teile liegen in `public/rig/`, der Export in `tools/rigexport/` (`task gen:rig`), das Kopierwerkzeug in `tools/sync/`, das Messskript in `tools/measure/`.
- Papierschnitt-Grafik (Entscheidung 005): Das Spiel lädt Figuren, Monarch mit Reittier, Gebäude und Hintergründe über die
  Gruppen-Indizes `public/grafik/<gruppe>/index.paper.json` (je Asset `manifest.json` + PNG-Sheets); die Zuordnung steht in
  `data/paper.json`. Atlas und alte Packs lädt das Spiel nicht mehr (Entfernung in CU1). **PNG und `manifest.json` sind nicht eingecheckt** (B-737): `task gen:paper` rendert nach `.work/grafik/` und kopiert nur Geändertes nach `public/grafik/<gruppe>/<asset>/` (ignoriert); `task build` und `test` rufen es vorher auf, `check:diff` nicht (B-742). Eingecheckt bleiben Specs, `index.paper.json` und `golden.json`. Natur, Objekte, Icons und Gerüste sind seit MG8/CU1 ebenfalls Papierschnitt.
- Namen und Ablage: Datei- und Ordnernamen im Code englisch, Gruppen-Präfixe in `engine/sim` (`docs/arbeitsweise.md` › Namen und Ablage).
- `site/` – Präsentationsseite für GitHub Pages (B-183), rein statisch, ohne Server. Pages lädt nur `site/` hoch (kein Node, kein Build); Bilder liegen als Kopien in `site/img/` (B-507).

## Regeln

- Neue Mechanik: Logik + Test in `rust/crates/k3c-sim`, das Spiel (`rust/crates/k3c-game`) zeichnet nur; die Go-Engine bekommt keine neuen Mechaniken (Entscheidung 013, Entwurf).
- Keine Sprünge, nur horizontale Bewegung. Welt-Koordinaten in **Units** (1 Unit = `UNIT_PX` = 32 px).
- Level-Generierung und Simulation sind deterministisch und laufen in Go: nur `engine/rng` (`rng.New(seed)`) verwenden, niemals `math/rand`.
- Jede Mechanik muss mit **2 Spielern gleichzeitig** funktionieren (Split-Screen, eigene Eingabe pro Spieler).
- Controller-Taste **B** geht in jedem Menü **zurück** (wie Esc); im laufenden Spiel bleibt sie unbelegt.
- **Logging mit Emojis:** Jede Log-Meldung (Go `slog`) beginnt mit einem Emoji zur Aktion:
  🚀 Start · 🛑 Stopp · 🔁 Neustart · ✅ geschafft · ❌ fehlgeschlagen · 💥 Fehler/Absturz · 🚫 abgelehnt · ⏳ Warten/Frist ·
  🐢 zu langsam · 🔌 verbunden · 👑 Spieler kommt · 👋 geht/getrennt · 💾 gespeichert · 📂 geladen · 🩹 wiederhergestellt ·
  🧹 aufgeräumt · 📨 Nachricht · 📄 Datei/Log · 🔒 Sicherheit · 🚦 Zustandswechsel · 🤒 ungesund · 💓 Takt · 🏰 Raum ·
  🪜 Stufenwechsel · ⏩ Zeitraffer · 🌙 Nacht/Welle · 🏹 Kampf · 🪙 Münzen · 🔨 Bau · 🐛 Dev. Nur Emojis aus einem
  Codepunkt (kein U+FE0F). Das Thema setzt die Konsole selbst aus `ns` (`engine/conlog` › `Topics`); ein neuer `ns` bekommt dort sein Emoji.
- Klein bleiben: kein Framework-Overhead. Prozess steht nur in `docs/arbeitsweise.md`, keine weiteren Prozess-Dokumente.
  Ein Sprint bleibt in seiner Domäne; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Lieber spielbarer Code.
- **Code-Review vor dem Commit nur bei Risiko:** Ein Reviewer-Agent (getrennt vom Schreiben, Sonnet) läuft vor dem Commit
  bei Domäne SIM oder SRV, bei Protokoll- oder Spielstand-Änderungen, bei Shell-, Pfad- oder Eingabe-Code oder ab etwa
  150 geänderten Code-Zeilen, im
  Hintergrund parallel zu `task check:diff`; schwere Befunde werden vor dem Commit behoben, ein Fix unter etwa 30 Zeilen
  ohne zweites Review. Die Sprint-Review-Session bleibt Pflicht (`docs/arbeitsweise.md` › Tempo im Lauf).
- **Sim und Balancing gesperrt (Beschluss 🧑 2026-11-11, bis auf Widerruf):** keine Simulationen, kein Balancing, kein
  Lasttest, auch nicht am Feature oder im Abnahme-Ticket. **Gen nur am Feature (B-742):** Sessions, Sprints und Projekte
  schließen ohne Gen-Läufe ab (`gen:paper` samt Grafik-Golden, `check:race`); zum Abschluss eines Sprints oder Projekts gibt
  es dafür **genau ein Abnahme-Ticket** (`docs/arbeitsweise.md` › Sim, Balance und Gen).
  **Balancing schreibt nur Welten-Dateien (🧑 2026-11-22):** Das Balancing des Studios ändert Quelldateien nur unter
  `data/worlds/` (Ausgaben unter `reports/`, technisch erzwungen, WZ63.7); braucht es Änderungen an Code oder anderen
  Daten (`data/*.json`, `engine/`, `rust/`, `tools/`), zeigt Claude zuerst den **Plan im Chat** und wartet auf Ralfs
  Freigabe, auch bei automatischem Balancing.
- **Schreiben nur im Repo (🧑 2026-11-18):** Jeder Ordner außerhalb des Repos ist für Schreibarbeiten verboten; kein Temp, `%TEMP%`, `/tmp` oder Scratchpad. Temporäre Dateien liegen in `.work/` (ignoriert) und werden nach der Nutzung wieder gelöscht. Worktrees legt Claude selbst an (in dem von Claude vorgesehenen Ordner), dafür gibt es keine Regel. **Technisch erzwungen:** `task` und `check_run` setzen `GOTMPDIR`/`TMP`/`TEMP`; der Hook `tools/hooks/tempGuard.ts` lehnt direkte `go run|test|build|vet` in Bash und PowerShell ab, die diese Variablen nicht selbst setzen (sonst landet `checkdiff.exe` in `%TEMP%\go-build…`).
- **Kein automatischer Worktree-Wechsel:** Nicht von sich aus in einen Worktree wechseln (`change_directory`, `EnterWorktree`,
  Lesen/Schreiben unter `.claude/worktrees/`). Nur auf ausdrückliche Anweisung von 🧑; dazu zählt der **Wellen-Auftrag** („Welle starten“, „parallel abarbeiten“), er gilt für alle Sessions dieser Welle.
  In einem Checkout, in dem eine andere Session arbeitet: kein Branchwechsel, kein Stash, kein Reset. Sonst im aktuellen Checkout bleiben;
  Inhalte anderer Branches nur lesend per `git show <branch>:<pfad>`.
  **Sim-Code (B-925):** Änderungen an `engine/sim`, `engine/level`, `engine/room`, `cmd/k3c-sim` und Daten in `data/` mit Wirkung auf Sim, Level oder Balancing laufen in einem eigenen Worktree mit eigener Dev-Instanz der Workbench auf eigenen Ports (`task k3c-workbench:dev:worktree`, Agent `k3c-sim-dev`), weil die Workbench die Engine im Prozess rechnet (B-924); das gilt als ausdrückliche Anweisung für diesen Worktree. Die Wurzel-Workbench bleibt unberührt.

## Haltung

Claude arbeitet wie ein erfahrener Spieleentwickler; Fachhaltung je Gebiet steht in den Agenten unter `.claude/agents/`.

- **Spielgefühl vor Technik:** Eine Änderung zählt, wenn Spieler sie am PC merken (Tempo, Feedback, Lesbarkeit); ohne Spielnutzen keine neue Komplexität.
- **Frame-Budget ist Pflicht:** 60 Hz, zwei Ansichten, PC; Ruckler kommen aus Ausreißern, darum zählt der schlechteste Frame.
- **Messen statt raten:** Leistung, Ruckler und Balance erst messen, dann ändern; Unbelegtes heißt „vermutet“.
- **Prototyp vor Ausbau:** kleinster spielbare Schritt zuerst, dann anspielen lassen und nachschärfen.
- **Tests beweisen Regeln, nicht Spaß:** Bei Änderungen an Takt, Eingabe, Kamera, Bewegung oder Werten nennt der Bericht, was Ralf im Spiel prüfen sollte.

## Repository Map

A full codemap is available at `codemap.md` in the project root.

Read `codemap.md` only when working in an area you do not know yet (architecture, entry points, data flow);
for deep work on a specific folder, read that folder's `codemap.md`. A session whose files are named in its
session file does not need it.
