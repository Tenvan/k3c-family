# MC1.2 · Asset-Tools guide, list, get, create, set, style, render, delete

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** mcp
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mc1/2-asset-tools
- **Abhängig von:** MC1.1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-05

## Ziel

Auf `develop` bietet die Workbench die schreibenden und lesenden Asset-Tools auf Spec-Basis an: `assets_guide`, `assets_list`, `assets_get`, `assets_create`, `assets_set`, `assets_style_get`, `assets_style_set`, `assets_render` (eins, alle als Job, Job-Status/Warten), `assets_delete`. Die Register-Varianten von `assets_list`, `assets_get`, `assets_set` sind dabei ersetzt (gleiche Namen).

## Kontext

- **Vertrag:** `docs/standards/paper-mcp.md` › 1 und › 2 (Tabelle der Tools mit Eingaben und Antworten). Beispielantworten: `tools/paper/fixtures/mcp/<tool>[-fall].txt` – Aufbau, Reihenfolge und Wortlaut der Zeilen sind verbindlich; Werte (IDs, Pfade, Anzahlen, Job-Nummern, Checkout) kommen aus dem Test-Repo.
- **Basis-Code:** `internal/paperassets` (MC1.1): `CLI`, `Set`, `Assets`, `Geaendert`, `Jobs`, `Vorlage`. Die Tools sind dünne Hüllen: Eingabe prüfen, `paperassets` rufen, Text formatieren.
- **Ist-Code:** `internal/mcpsrv/tools_assets.go` registriert heute die Register-Tools `assets_list/get/set/catalog` (Slot-Register `data/assets.json`, Paket `k3c/tools/assetkit`). `tools.go` ruft `registerAssets(s)`, `registerAssetsResearch(s)`, `registerAssetsImport(s)`. Helfer `add[In]` (`tools.go`) registriert Text-Tools; ein Fehler wird zum Fehler-Ergebnis. `assetRepo(t)` in `tools_assets_test.go` wird von `tools_assets_research_test.go` benutzt → nach `tools_assets_slots_test.go` verschieben, nicht löschen.
- **Checkout:** Jedes Tool nimmt `checkout`. Das Schema-Feld kommt aus `takesCheckout` (`checkout.go`), die Zeile `Checkout: …` am Ende aus `writeTools` (`workspace.go`, angehängt in `observe.go`). Neu: `takesCheckout` gilt für alle `assets_*`; `writeTools` um `assets_create`, `assets_style_set`, `assets_render`, `assets_delete` ergänzen (`assets_set` ist schon drin). Der Handler nimmt den Checkout immer aus `s.ws(ctx).root`.
- **Ablehnungen** (`assets_set`, `assets_create`, `assets_style_set`): Fehler-Ergebnis (`isError`) mit dem Text aus `assets_set-abgelehnt.txt` (`abgelehnt, nichts geändert: n Fehler`, je Fehler `feld · regel · text`, Zeile `Erlaubt: …`); die Checkout-Zeile hängt `observe.go` auch dort an.
- **`assets_style_set`:** nur für Sessions mit `Lane: stil` gedacht; keine technische Sperre, nur Hinweis in der Beschreibung. Betroffene Assets = Specs, die ein geändertes Material oder einen Einzelton nennen (Lagen, Umhang, `tausch`); ändert sich `papier`, sind es alle.
- **Namen:** Regel `^[a-z]+(_[a-z]+)+$` (`TestToolRegelnDerSpec`) – `assets_style_get`/`assets_style_set` passen. Jede Beschreibung endet mit „Nutze es bei: … Statt: …“.
- Neue Tools sind erst nach Merge und Neustart der Workbench aus der Repo-Wurzel live (B-701 D-10); getestet wird nur in-process (`connect(t, New(Config{Root: repo}))`, `callText`).

## Erlaubte Dateien

- `tools/k3c-workbench/internal/mcpsrv/tools_assets.go` (ersetzen: Registrierung, guide, list, get, create)
- `tools/k3c-workbench/internal/mcpsrv/tools_assets_set.go` (neu: set, style_get, style_set, delete)
- `tools/k3c-workbench/internal/mcpsrv/tools_assets_render.go` (neu: render eins/alle/job)
- `tools/k3c-workbench/internal/mcpsrv/tools_assets_test.go`, `tools_assets_set_test.go`, `tools_assets_render_test.go`, `tools_assets_paper_test.go` (Test-Repo-Helfer), `tools_assets_slots_test.go` (verschobenes `assetRepo`)
- `tools/k3c-workbench/internal/mcpsrv/tools.go` (nur Registrierung)
- `tools/k3c-workbench/internal/mcpsrv/checkout.go` (nur `takesCheckout`), `workspace.go` (nur `writeTools`)
- `tools/k3c-workbench/internal/paperassets/` (nur kleine Ergänzungen, die ein Tool braucht, mit Test)

## Nicht-Ziele

- `assets_preview`, `assets_review`, MCP-Prompts, `instructions.md` (MC1.3).
- `assets_catalog` und die übrigen Register-Helfer entfernen (MC1.4).
- `tools_assets_research*.go`, `app_claude*.go`, `tools/k3c-studio/`, `tools/assetkit/` nicht anfassen.

## Schritte

Test-first: je Tool erst den Test (rot), dann die Hülle. Gezielt: `check_run {target: dev:test, pattern: TestAssets}`.

1. `tools_assets_paper_test.go`: `paperRepo(t)` wie `testRepo` aus MC1.1 (Git-Repo, Stil-Datei, monarch/pferd/mauer, Commit, `K3C_PAPER_CLI` auf die Attrappe). `assetRepo` nach `tools_assets_slots_test.go` verschieben; Research-Tests bleiben grün.
2. `TestAssetsToolsNehmenCheckout`: für jedes Tool aus dem Vertrag (Liste im Test) hat das Eingabe-Schema die Eigenschaft `checkout`; die Antworten von `assets_create`, `assets_set`, `assets_style_set`, `assets_render`, `assets_delete` enden mit `Checkout: `. Dann `checkout.go`/`workspace.go` anpassen.
3. `TestAssetsGuide`: `{rig: "figur"}` enthält `Papierschnitt · Rig figur`, `Papier-Materialien (licht/basis/schatten): ` mit den Materialien der Stil-Datei, `Pflicht-Teile figur: rumpf, kopf, oberarm-v`, `Pivots: Hüfte (0,-50)`, `Ablauf: assets_get → assets_set (Merge-Patch) → assets_preview → assets_review`; ohne `rig` alle fünf Rigs; `{rig: "drache"}` → Fehler mit den erlaubten Rigs.
4. `TestAssetsListPaper`: Kopfzeile `3 Assets · 0 ungültig · 0 geändert gegen HEAD`, Zeile `monarch · figur · public/grafik/figuren/monarch.paper.json · gültig`; nach einer Änderung an monarch liefert `{geaendert: true}` nur `monarch … · geändert`; Filter `rig`, `gruppe`.
5. `TestAssetsGetFelder`: `{id: monarch, felder: [umhang, ausgabe]}` → erste Zeile `public/grafik/figuren/monarch.paper.json · Felder umhang, ausgabe`, danach JSON nur mit diesen Schlüsseln; ohne `felder` ganze Spec ohne `$schema`; `{id: x}` → Fehler `Asset „x“ fehlt`.
6. `TestAssetsCreate`: `{id: fass, rig: requisite, beschreibung: …, gruppe: requisiten}` → Datei `public/grafik/requisiten/fass.paper.json` existiert, Zeilen `fass angelegt: … (Rig requisite, Vorlage mit Teil korpus)`, `validate: ok`, `Render: 1 PNG`; zweiter Aufruf → Fehler `fass gibt es schon` und Datei unverändert; ungültige `id` (`Fass!`) → Ablehnung.
7. `TestAssetsSetMaterial`: `{id: monarch, patch: {umhang: {material: purpur}}}` → `monarch geändert: 1 Feld · umhang.material koenigsrot → purpur`, Zeile `Render: n PNGs (idle, lauf, reiten) · Pixel-Anteil … %`. `TestAssetsSetAbgelehnt` (**AC-02**): Patch `scharlach` → `isError`, Text wie `assets_set-abgelehnt.txt`, SHA-256 von Spec und Ausgabe-Ordner unverändert.
8. `TestAssetsStyle`: `assets_style_get` → `public/grafik/stil.paper.json · stilHash <64 Hex>`, `Materialien (n): …`, `Einzeltöne (n): …`. `assets_style_set {patch: {materialien: {stein: {basis: "#a8a294"}}}}` → `Stil geändert: materialien.stein.basis #9a9386 → #a8a294`, `Betroffen: 1 Asset (mauer) · Render als Job j-<n> gestartet`, `Weiter: assets_render {job: "j-<n>", wait: true}`; Patch mit unbekanntem Feld → Ablehnung, Stil-Datei-Hash unverändert.
9. `TestAssetsRender` (**AC-05**): `{id: monarch}` → `monarch gerendert: n PNGs, manifest.json · renderKey …`; `{alle: true}` antwortet sofort `Job j-<n> gestartet: 3 Assets (alle)` und `Weiter: …`; `{job}` → `Job j-<n> läuft k/3 (…)` oder `fertig`; `{job, wait: true}` → `Job j-<n> fertig: 3/3 Assets, … PNGs, 0 Fehler, … s` und je Asset eine Zeile; weder `id` noch `alle` noch `job` → Fehler mit den drei Formen. Fortschritt bei `wait` über `withProgress`.
10. `TestAssetsDelete`: ohne `confirm` → `nicht gelöscht: assets_delete braucht confirm=true (mauer, n PNGs)`, Datei bleibt; mit `confirm: true` → Spec und Ausgabe-Ordner weg, Eintrag aus `public/grafik/gebaeude/index.paper.json` entfernt (falls vorhanden), Zeile `… Gruppen-Index gebaeude nachgezogen`.
11. `TestAlleToolsMitBeschreibungUndAnnotations` und `TestToolRegelnDerSpec` bleiben grün (Annotations: lesend `readOnly()`, schreibend `DestructiveHint` passend, `assets_delete` destruktiv).
12. Einmal `task:check:diff`.

## Fertig, wenn

- [ ] AC-01 (Teil MC1.2): je Tool aus Schritt 3–10 ein grüner In-Process-Test, Zeilen wie `tools/paper/fixtures/mcp/`.
- [ ] AC-02: `TestAssetsSetAbgelehnt` grün (Hashes vorher = nachher).
- [ ] AC-05: `TestAssetsRender` grün (Job sofort, `wait` liefert das Ergebnis).
- [ ] `check_run {target: dev:test, pattern: TestAssets}` und `check_run {target: task:check:diff}` grün.

## Prüfen

```bash
task check:diff
```

Im Lauf: `check_run {target: dev:test, pattern: TestAssets, checkout: <eigener Worktree>}`.
Review-Agent (Sonnet) vor dem Commit: Eingabe- und Pfad-Code, über 150 Zeilen.

## Ergebnis

- AC-01 (Teil MC1.2) geprüft: `TestAssetsToolsNehmenCheckout`, `TestAssetsGuide`, `TestAssetsListPaper`, `TestAssetsGetFelder`, `TestAssetsCreate`, `TestAssetsSetMaterial`, `TestAssetsStyle`, `TestAssetsDelete`, `TestAssetsRender`, `TestAssetsRenderFehler` grün (In-Process gegen die Attrappe).
- AC-02 geprüft: `TestAssetsSetAbgelehnt` (Text wie `assets_set-abgelehnt.txt`, Hash von Spec und Ausgabe unverändert), dazu `TestStilSetAbgelehnt`.
- AC-05 geprüft: `TestAssetsRender` (`{alle}` sofort `Job j-n gestartet`, `{job, wait}` liefert `fertig: 3/3 Assets`).
- `task check:diff`: Client 3023/3023, Lint `0 issues`, Workbench-Frontend 367/367, `paperassets` ok; einziger Fehler `TestRunProcessExitUndAusgabe` (B-702, nicht MC1). Planning-Fehler durch B-702 ohne Projekt und Verschachtelungstiefe in `sammle` (TestGoNestingDepth) unterwegs behoben.
- Review (Sonnet): Critical (gleichzeitiges Map-Schreiben in `Gueltigkeit.Pruefe`) und 3 Important (CLI-Fehler nicht cachen, `id: index` reserviert, `StilSet` ohne Abbruchfehler nach dem Schreiben) behoben, Tests `TestGueltigkeitGemischterCache`, `TestGueltigkeitCachtKeineCLIFehler`, `TestAnlegenIndexReserviert`. Minor offen (Ledger): `assets_delete` nennt `manifest.json` nicht, `assets_get` mehrzeilig, Jobs global und ohne Abbruch.
- Abweichungen: `paperassets` um `Anlegen`, `Loesche`, `StilSet` (Schema über jsonschema-go), `Gueltigkeit` (Cache für `assets_list`), `Schluessel`/`Felder` erweitert; `assets_catalog` bleibt bis MC1.4; renderKey/stilHash in voller Länge.
