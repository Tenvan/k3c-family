# MC1.1 · paperassets: CLI-Aufruf, Merge-Patch, atomares Schreiben, Git-Basis, Render-Jobs

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** mcp
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mc1/1-paperassets
- **Abhängig von:** PV1.5
- **Tickets:** B-699
- **Kriterien:** AC-02, AC-05

## Ziel

Auf `develop` liegt das Go-Paket `tools/k3c-workbench/internal/paperassets`: Es ruft die Render-CLI (bis RD1 die Attrappe), wendet JSON-Merge-Patches atomar an (ungültig = nichts geändert), kennt die Git-Basis und führt Render-Jobs für alle Assets. Die MCP-Tools (MC1.2, MC1.3) sind danach nur noch dünne Hüllen.

## Kontext

- **Vertrag:** `docs/standards/paper-mcp.md` › 1 (gemeinsame Regeln, `basis`) und › 2 (`assets_set`, `assets_render`); `docs/standards/paper-spec.md` › 1 (Ablage), › 4 (Rig-Typen, Pflicht-Teile), › 7 (Ausgabe, Manifest), › 8 (CLI-Vertrag: ein JSON-Objekt auf stdout, Exit 0 ok, 1 fachlich abgelehnt `{ok:false, fehler:[{feld, regel, text}]}`, 2 Aufruf-/Systemfehler mit Text auf stderr).
- **CLI:** Aufruf `bun <skript> <befehl> …` im Checkout als Arbeitsordner, gestartet über `internal/proc` (`proc.Command`, ohne Konsolenfenster; `nowindow_test.go` wacht in `mcpsrv` darüber). `<skript>` = Umgebungsvariable `K3C_PAPER_CLI`, sonst `tools/paper/bin/paper.ts` (gibt es erst nach RD1). Tests setzen `K3C_PAPER_CLI` auf den **absoluten** Pfad von `tools/paper/fixtures/cli-stub.ts` des eigenen Checkouts (z. B. über `filepath.Abs("../../../../tools/paper/fixtures/cli-stub.ts")` vom Paketordner aus).
- **Eigenheiten der Attrappe** (`cli-stub.ts`): `validate` prüft echt über `tools/paper/schema/check.ts`, liest die Stil-Datei aber immer aus **ihrem** Repo (`public/grafik/stil.paper.json`), nicht aus dem Test-Repo. `render` schreibt nichts, sondern nennt in `dateien` Fixture-PNGs (`tools/paper/fixtures/render/…`) bzw. `platzhalter.png`. Pfade außerhalb ihres Repos gibt sie **absolut** zurück → `paperassets` normalisiert jede Pfadangabe der CLI: liegt sie im Checkout, relativ mit `/`, sonst unverändert.
- **Atomar (005/AC-06):** Patch anwenden → Ergebnis als `<id>.paper.json` (Regel `id-dateiname`!) in einen Temp-Ordner `.cache/paper/tmp/<zufall>/<gruppe>/` im Checkout (gleiches Laufwerk, `.cache/` ist in `.gitignore`) → `validate` → `render` → Spec per Temp-Datei + `os.Rename` ersetzen → Ausgabe-Ordner `<tmp>/<gruppe>/<id>/`, falls die CLI ihn geschrieben hat, an `public/grafik/<gruppe>/<id>/` umbenennen (alter Ordner erst nach erfolgreichem Umbenennen löschen). Jeder Fehler vorher lässt Spec und Ausgabe unberührt; der Temp-Ordner wird immer entfernt. Dateien, die die CLI außerhalb des Temp-Ordners nennt (Attrappe), werden nur gezählt, nie verschoben.
- **Merge-Patch:** RFC 7396 (`null` löscht, Objekte rekursiv, Arrays und Skalare ersetzen). Keine neue Abhängigkeit, eigene Funktion von ~30 Zeilen.
- **Basis:** `assetsui.CheckBasis` (PV1.8, `internal/assetsui/stubs.go`) wiederverwenden, nicht kopieren. „Geändert gegen HEAD“ = Spec oder Ausgabe-Ordner in `git status --porcelain -- public/grafik` (inkl. unversionierter Dateien).
- **Jobs:** eigener Mechanismus, **nicht** `task_*`. IDs `j-<n>` fortlaufend je Prozess, Speicher in-memory je Checkout (Schlüssel = Checkout-Pfad). Ein Job rendert alle gültigen Specs nacheinander; ungültige werden mit der ersten Regel übersprungen (`banner · übersprungen (ungültig: material-unbekannt)`).
- **Rig-Vorlagen:** Pflicht-Teile laut `paper-spec.md` › 4 als Go-Tabelle (`rigs.go`); Pivots der Figur aus derselben Tabelle. Leere `lagen` sind laut Schema erlaubt.
- Fallstricke: Windows-Pfade (immer `filepath` intern, `/` nur in Antworten); `os.Rename` über ein vorhandenes Ziel-Verzeichnis schlägt unter Windows fehl → Ziel vorher umbenennen (`<id>.alt-<zufall>`), dann löschen.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/paperassets/` (neu: `cli.go`, `patch.go`, `set.go`, `basis.go`, `job.go`, `rigs.go` und je eine `_test.go`; jede Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen)

## Nicht-Ziele

- Keine MCP-Tools, keine Registrierung (MC1.2).
- Kein Review-Vergleich gegen eine Basis, kein `git show` alter Stände (gehört RV1, `internal/review`).
- Keine Änderung an `tools/paper/` (Vertrag und Attrappe sind fest; Fehler darin → Ticket im Projekt PAP).

## Schritte

Test-first: zuerst jeweils den Test schreiben und rot sehen (`check_run dev:test` mit `pattern`), dann den Code.

1. Test-Helfer `testRepo(t)` in `paperassets_test.go`: `t.TempDir()`, `git init`, Stil-Datei aus `public/grafik/stil.paper.json` und die Specs `tools/paper/fixtures/specs/{monarch,pferd,mauer}.paper.json` nach `public/grafik/{figuren,reittiere,gebaeude}/` kopieren, ein Commit (feste Autor-/Commit-Zeit über Umgebung), `t.Setenv("K3C_PAPER_CLI", <abs. cli-stub.ts>)`; fehlt `bun` im PATH → `t.Fatal` mit Hinweis auf `requirements.md` (kein Skip; CI-Job `k3c-workbench` hat bun).
2. `TestCLIValidate`: `CLI.Validate(spec)` → ok für monarch; `specs/ungueltig/fremdes-material.paper.json` → `[]Fehler` mit `regel = material-unbekannt`; unbekannter Befehl → Go-Fehler mit stderr-Text (Exit 2). Dann `cli.go`: `type CLI struct{Root, Skript string}`, `NewCLI(root)` liest `K3C_PAPER_CLI`, Methoden `Validate`, `Render`, `Frame(spec, animation, frame, variante, stufe, out)`, `Diff`, `PixelDiff`, Typ `Fehler{Feld, Regel, Text}` (= `assetsui.Fehler` wiederverwenden).
3. `TestCLIPfadeRelativ`: Antwortpfade im Checkout kommen relativ mit `/`, außerhalb unverändert.
4. `TestMergePatch` (tabellarisch): Feld ersetzen, `null` löscht, verschachteltes Objekt, Array ersetzt komplett, Patch kein Objekt → ersetzt alles. Dann `patch.go`.
5. `TestSetAendertMaterial`: `Set(root, "monarch", {umhang:{material:"purpur"}})` → Ergebnis `Felder = [{umhang.material, koenigsrot, purpur}]` (über `CLI.Diff`), `PNGs > 0`, Pixel-Anteil je Animation (Frame 0 alt gegen neu über `CLI.Frame` + `CLI.PixelDiff`), Spec auf der Platte neu, `.cache/paper/tmp/` leer.
6. `TestSetUngueltigAendertNichts` (**AC-02**): SHA-256 über Spec-Datei und alle Dateien unter `public/grafik/figuren/monarch/` vorher/nachher gleich bei Patch `{umhang:{material:"scharlach"}}`; Rückgabe `Abgelehnt` mit `teile…`/`umhang.material · material-unbekannt`. Zweiter Fall mit `K3C_PAPER_CLI` auf ein Test-Skript, das mit Exit 2 endet (`TestSetRenderFehlerAendertNichts`): Fehler, Hashes gleich, Temp-Ordner weg. Dann `set.go`.
7. `TestBasis`: `HEAD`, `HEAD~1`, `abc1234`, `origin/develop` ok; `main` → Text `Basis „main“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop`; `TestGeaendertGegenHEAD`: nach Änderung an `monarch.paper.json` und neuer Datei `fass.paper.json` liefert `Geaendert(root)` genau `{monarch, fass}`. Dann `basis.go` (inkl. `Assets(root) []Asset{ID, Rig, Gruppe, Pfad}` über `public/grafik/*/*.paper.json` ohne `index.paper.json` und `stil.paper.json`).
8. `TestJobAlle` (**AC-05**): `Jobs.Start(root)` kehrt sofort mit `j-1` und `n = 3` zurück (Test-CLI mit künstlicher Verzögerung über ein Wrapper-Skript oder einen injizierbaren Runner); `Status` liefert `läuft k/n (<asset>)`; `Wait(ctx, id)` liefert je Asset `{ID, PNGs, Manifest}` und `Fehler 0`; mit einer zusätzlichen ungültigen Spec `banner` → `übersprungen (ungültig: material-unbekannt)`. `TestJobUnbekannt`: `j-99` → Fehler `Job „j-99“ unbekannt`. `TestJobWaitAbbruch`: abgebrochener `ctx` beendet `Wait` mit Fehler, der Job läuft weiter. Dann `job.go` (Mutex, kein globaler Zustand außer der Job-Tabelle).
9. `TestRigVorlagen`: `Vorlage(rig, id, beschreibung)` für `requisite`, `gebaeude`, `hintergrund`, `vierbeiner` besteht `CLI.Validate`; für `figur` meldet `Validate` höchstens `proportion` (leere Lagen), nie `pflicht-teil-fehlt`; unbekannter Rig → Fehler mit den erlaubten Rigs. Dann `rigs.go`.
10. `check_run dev:test` mit `pattern: paperassets`, danach einmal `task:check:diff`.

## Fertig, wenn

- [ ] AC-02: `TestSetUngueltigAendertNichts` und `TestSetRenderFehlerAendertNichts` grün (Hashes von Spec und Ausgabe vorher = nachher).
- [ ] AC-05: `TestJobAlle` grün: `Start` kehrt vor dem ersten Render zurück, `Wait` liefert das Ergebnis aller Assets.
- [ ] `check_run {target: dev:test, pattern: paperassets}` grün.
- [ ] `check_run {target: task:check:diff}` grün; kein `t.Skip`, kein TODO im Diff.

## Prüfen

```bash
task check:diff
```

Im Lauf: `check_run {target: dev:test, pattern: paperassets, checkout: <eigener Worktree>}`. Einmal vor dem Commit `task:check:diff`.
Review-Agent (Sonnet) vor dem Commit: Pfad- und Datei-Code (atomares Schreiben).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
