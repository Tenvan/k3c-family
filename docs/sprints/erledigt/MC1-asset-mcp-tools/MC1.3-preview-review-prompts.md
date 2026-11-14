# MC1.3 · assets_preview als Bild, assets_review, MCP-Prompts, instructions.md

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** mcp
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mc1/3-preview-review-prompts
- **Abhängig von:** MC1.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-03, AC-04

## Ziel

Auf `develop` liefert `assets_preview` ein Bild (MCP-ImageContent `image/png`) plus Teil-Boxen, `assets_review` den Review-Text gegen eine Basis, und jede Vorlage aus `data/asset-prompts.json` ist als MCP-Prompt `asset-<id>` abrufbar. `instructions.md` beschreibt die neuen Asset-Tools.

## Kontext

- **Vertrag:** `docs/standards/paper-mcp.md` › 2 (`assets_preview`, `assets_review`, Abschnitt MCP-Prompts) und › 3 (Prompt-Vorlagen, Platzhalter `{asset}`, `{rig}`, `{teil}`, `{pfad}`, `{zustand}`, `{basis}`, `{eingabe}`). Beispiele: `tools/paper/fixtures/mcp/assets_preview.txt`, `assets_review.txt`, `assets_review-lokal.txt`.
- **Preview ohne `basis`:** `paperassets.CLI.Frame` (MC1.1) schreibt den Frame in eine Temp-Datei unter `.cache/paper/tmp/`; Bytes lesen, Datei löschen. Die Attrappe liefert für monarch `idle` → `tools/paper/fixtures/render/monarch/monarch-idle-0.png`, für `reiten` → `monarch-galopp-0.png`. Standard: `animation` = erste Animation der Spec (monarch: `idle`), `frame` = 0, `variante` = keine.
- **Preview mit `basis` und `assets_review`:** Den Vergleich mit einer Basis besitzt RV1 (`internal/review`, `internal/assetsui`). MC1 geht über eine kleine Schnittstelle in `tools_assets_review.go`: `type assetReview interface { AssetsReview(basis string) (assetsui.Review, error); AssetsFrame(id, animation string, frame int, variante, basis string) (assetsui.Frame, error); AssetsPixelDiff(id, animation string, frame int, basis string) (assetsui.PixelDiff, error) }` und `var newAssetReview = func(root string) assetReview { return assetsui.Stubs{Root: root} }`. Bis RV1.2 antworten die PV1.8-Stubs (Daten aus `tools/paper/fixtures/ui/`, Basis `HEAD~1` = `32eb7a7`, nur `monarch` kennt Frame/Pixel-Diff). Nach RV1.2 zeigt `newAssetReview` auf die echte Implementierung (eine Zeile; Prüfpunkt in MC1.5).
- **Bilder im Review-Text:** Data-URLs aus `AssetsFrame`/`AssetsPixelDiff` als PNG nach `.cache/paper/mcp/<id>-<animation>-<frame>[-diff].png` schreiben und den Pfad nennen (`Overlay …` bei `geaendert`, Bild bei `neu`). Fehlt ein Bild (Stub kennt das Asset nicht), entfällt der Bild-Teil der Zeile, kein Fehler.
- **ImageContent:** Helfer `add[In]` (`tools.go`) kann nur Text. Neuer Helfer `addImage` in `tools_assets_preview.go`, der wie `add` Parameter, Checkout-Schema und Statistik registriert, aber `*mcp.CallToolResult{Content: [&mcp.ImageContent{Data: png, MIMEType: "image/png"}, &mcp.TextContent{…}]}` liefert (go-sdk v1.8.0: `ImageContent.Data` sind die Roh-Bytes, das SDK kodiert base64).
- **Prompts:** go-sdk v1.8.0 `s.mcp.AddPrompt(&mcp.Prompt{Name, Title, Description, Arguments: []*mcp.PromptArgument{{Name, Required}}}, handler)`; Handler liefert `&mcp.GetPromptResult{Messages: []*mcp.PromptMessage{{Role: "user", Content: &mcp.TextContent{Text}}}}`. Registrierung beim Start aus `data/asset-prompts.json` der Repo-Wurzel (`Config.Root`); der Handler liest die Datei bei jedem Abruf neu aus dem Checkout des Aufrufs, damit Textänderungen ohne Neustart greifen (neue oder gelöschte Vorlagen erst nach Neustart). Argumente = Platzhalter, die im Text vorkommen, in Reihenfolge des ersten Auftretens; alle `Required` außer `zustand` und `basis`. Fehlt ein Pflicht-Argument → Fehler mit dessen Namen.
- **Test-Client:** `cs.ListPrompts(ctx, nil)`, `cs.GetPrompt(ctx, &mcp.GetPromptParams{Name, Arguments: map[string]string{…}})`.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/mcpsrv/tools_assets_preview.go` (neu, inkl. `addImage`), `tools_assets_review.go` (neu), `tools_assets_prompts.go` (neu)
- `tools/k3c-workbench/internal/mcpsrv/tools_assets_preview_test.go`, `tools_assets_review_test.go`, `tools_assets_prompts_test.go` (neu)
- `tools/k3c-workbench/internal/mcpsrv/tools_assets.go` (nur Registrierung der neuen Tools und Prompts)
- `tools/k3c-workbench/internal/mcpsrv/workspace.go` (nur `writeTools`, falls nötig), `checkout.go` (nur `takesCheckout`)
- `tools/k3c-workbench/internal/mcpsrv/instructions.md` (Abschnitt Assets und Liste der Tools mit `checkout` unter Worktrees)
- `tools/k3c-workbench/internal/paperassets/` (nur kleine Ergänzungen mit Test)

## Nicht-Ziele

- Keine Review-Logik (Basis, `git show`, Cache, Diff) – die liefert RV1; MC1 formatiert nur.
- Keine Änderung an `internal/assetsui/` oder `data/asset-prompts.json`.
- Register-Tools entfernen (MC1.4).

## Schritte

Test-first, gezielt mit `check_run {target: dev:test, pattern: TestAssets}`.

1. `TestAssetsPreviewBild` (**AC-03**): `{id: monarch}` → `Content[0]` ist `*mcp.ImageContent` mit `MIMEType == "image/png"`, Bytes gleich `tools/paper/fixtures/render/monarch/monarch-idle-0.png`; `Content[1]` Text `monarch · idle · Frame 0 · Basis – · <b>×<h> px` und `Teile (x,y,b,h px): rumpf …`. `{id: monarch, animation: reiten}` → Bytes von `monarch-galopp-0.png`. Dann `addImage` und das Tool.
2. `TestAssetsPreviewFehler`: `{animation: flug}` → Fehler mit `idle, lauf, reiten`; unbekannte `id` → `Asset „x“ fehlt`; `{basis: main}` → `Basis „main“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop`.
3. `TestAssetsPreviewBasis`: `{id: monarch, basis: "HEAD~1"}` → Bild aus `AssetsFrame` des Stubs, Text `… Basis HEAD~1 …`.
4. `TestAssetsReview` (**AC-01**): ohne `basis` gilt `HEAD`; mit `{basis: "HEAD~1"}` gegen die Stubs: Kopfzeile `Review gegen HEAD~1 (32eb7a7, Basis-Stand aktuell): 5 Assets`, Zeilen `geaendert · monarch · umhang.material koenigsrot → purpur · Pixel 1,53 % · Overlay .cache/paper/mcp/…`, `neu · pferd · public/grafik/reittiere/pferd.paper.json`, `geloescht · fass · …`, `ungueltig · banner · teile[0].lagen[0].material · material-unbekannt · Material „scharlach“ fehlt in der Stil-Datei`, `geaendert · mauer · stil: true …`. Bei `basisStand: lokal` Kopfzeile wie `assets_review-lokal.txt`. Prozent mit Komma, zwei Nachkommastellen. Der Test ersetzt `newAssetReview` nicht, er nutzt die Stubs.
5. `TestAssetsPromptsAlle` (**AC-04**): `ListPrompts` enthält für jede Vorlage aus `data/asset-prompts.json` (im Test aus der Datei gelesen, nicht hart kodiert) `asset-<id>` mit `Title = titel` und genau den Platzhaltern des Textes als Argumente; `zustand`, `basis` nicht Pflicht, alle anderen Pflicht. Platzhalter nur über `\{(asset|rig|teil|pfad|zustand|basis|eingabe)\}` erkennen – der Text enthält auch literale Klammern wie `{rig: {rig}}`.
6. `TestAssetsPromptEinsetzen`: `GetPrompt asset-material-aendern {asset: monarch, rig: figur, pfad: public/grafik/figuren/monarch.paper.json, zustand: „gültig, Version 1“}` → eine Nachricht `user`, Text beginnt mit „Rufe zuerst `assets_guide {rig: figur}` auf“, keiner der sieben Platzhalter steht mehr im Text; ohne `asset` → Fehler nennt `asset`; `asset-teil-material` ohne `teil` → Fehler; `asset-nachbessern` ohne `eingabe` → Fehler; leeres `zustand` ist erlaubt.
7. `instructions.md`: Abschnitt „Grafik- und Sound-Slots“ um einen Unterabschnitt „Assets (Papierschnitt)“ ergänzen: Einstieg `assets_guide`, Ablauf get → set → preview → review, Jobs über `assets_render {alle}` / `{job, wait: true}`, Prompts `asset-<id>`, `checkout` bei jedem `assets_*`, B-701 D-10 (live erst nach Merge). `TestInstructionsNennenAssetTools`: jedes registrierte `assets_*`-Tool steht in `instructions.md`.
8. Einmal `task:check:diff`.

## Fertig, wenn

- [ ] AC-03: `TestAssetsPreviewBild` grün (`image/png`, Bytes = Fixture).
- [ ] AC-04: `TestAssetsPromptsAlle` und `TestAssetsPromptEinsetzen` grün.
- [ ] AC-01 (Teil MC1.3): `TestAssetsPreviewFehler`, `TestAssetsPreviewBasis`, `TestAssetsReview` grün, Zeilen wie `fixtures/mcp/assets_preview.txt` und `assets_review*.txt`.
- [ ] `check_run {target: dev:test, pattern: TestAssets}`, `check_run {target: dev:test, pattern: TestInstructions}` und `check_run {target: task:check:diff}` grün.

## Prüfen

```bash
task check:diff
```

Im Lauf: `check_run {target: dev:test, pattern: TestAssets, checkout: <eigener Worktree>}`.
Review-Agent (Sonnet) vor dem Commit: Pfad-Code (Cache-Dateien), über 150 Zeilen.

## Ergebnis

- AC-03 geprüft: `TestAssetsPreviewBild` grün (`ImageContent` `image/png`, Bytes = `monarch-idle-0.png` bzw. `monarch-galopp-0.png`, Text mit Teil-Boxen `rumpf 53,58,47,78`).
- AC-04 geprüft: `TestAssetsPromptsAlle` (alle 10 Vorlagen als `asset-<id>`, Argumente = Platzhalter, `zustand`/`basis` optional) und `TestAssetsPromptEinsetzen` grün.
- AC-01 (Teil MC1.3) geprüft: `TestAssetsPreviewFehler`, `TestAssetsPreviewBasis`, `TestAssetsReview`, `TestAssetsReviewKopfLokal`, `TestInstructionsNennenAssetTools` grün.
- `task check:diff` exit 0 (Client 3023/3023, Workbench mcpsrv/paperassets ok, Lint 0 issues, Studio 876/876, Starter ok).
- Review (Sonnet): 2 Important behoben (Cache-Dateinamen über `cacheName` bereinigt, `TestCacheNameBleibtImOrdner`; Präfix „Bild “ entfernt wie Fixture). Minor offen: Bild-Fehler ohne Hinweis, `preview {basis}` prüft gegen Arbeitsstand, Prompt-Parsefehler ohne Log.
- Abweichungen: Stil-Zeile ohne Stil-Pfad und ohne vorher/nachher-Bilder (Review-Struktur liefert sie nicht, mit RV1 nachziehen); Prompts aus der Repo-Wurzel; `task install` im Worktree nötig.
