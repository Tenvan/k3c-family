# tools/k3c-workbench/internal/assetsui/

## Responsibility

RPC-Schicht der Asset-Oberfläche (Vertrag `docs/standards/paper-mcp.md` › Oberfläche): Katalog, Spec, Stil-Datei, Prompt-Vorlagen, Review, Frames, Pixel-Diff und Größenvergleich aus dem Checkout; Vergleich, Cache und Render-CLI liegen in `internal/review`.

## Design

- `Dienst` (`assets.go`): Wert-Typ mit `Root` (Checkout) und optionaler Render-CLI `R` (leer: `review.CLI` im Checkout); jede Methode ist ein Binding. Typen des Vertrags: `AssetEintrag`, `Frame` (Data-URL mit Teil-Boxen), `PixelDiff` (Overlay als Data-URL), `Vorlage` (aus `data/asset-prompts.json`); Review-Typen, `Fehler` und `FeldAenderung` als Aliase aus `internal/review`.
- `renderPlatz` begrenzt gleichzeitige Renders (je Frame ein RPC, je Render ein bun-Prozess).
- `assets.go`: `AssetsGet`, `AssetsStyle`, `AssetsPrompts`, `AssetsReview(basis)`, `AssetsFrame(id, anim, frame, variante, basis)`, `AssetsPixelDiff`; `pfadZu`/`quelle` lösen ein Asset im Arbeitsstand oder in einem Basis-Commit auf.
- `katalog.go` (B-732): `AssetsList` (vollständig geprüft), `AssetsIndex` (sofort, Prüfergebnis nur aus dem Memo, sonst `offen`), `AssetsCheck(pfade)` prüft Häppchen nach; `geaendert` markiert Änderungen gegen `HEAD`, `memoKey` schlüsselt das Memo.
- `frameohne.go` (B-734): `AssetsFrameOhne` rendert einen Frame ohne die Lagen einzelner Teile, ohne Platten-Cache.
- `groesse.go` (B-733, B-853): `AssetsGroesse`, `AssetsGroesseBasis` (Spec und Maßstab im Stand der Basis), `AssetsHintergruende`/`AssetsHintergrund` (Hintergrund-Ebenen der Biome); Figuren mit Spielrolle skaliert über die Rollenhöhen aus `data/paper.json` (`rollenHoehen`), Gebäude, Objekte und Hintergründe 1 px je Einheit.

## Flow

1. Frontend (Bereich Assets) → Binding in `app_assets.go` → `assetsui.Dienst{Root: a.root}.<Methode>`.
2. Katalog: `AssetsIndex` → Liste sofort; danach `AssetsCheck` in Häppchen → `pruefeAlle` → `eintrag`.
3. Review/Frame: `quelle` (Arbeitsstand oder Basis) → `review.Renderer` → Data-URL bzw. Pixel-Diff.

## Integration

- Konsumenten: `tools/k3c-workbench/app_assets.go` (Bindings), `internal/mcpsrv/tools_assets_prompts.go`, `tools_assets_review.go`, `tools_assets_set.go`, `internal/paperassets` (`cli.go`, `set.go`, `stil.go`: `Fehler`, `FeldAenderung`).
- Abhängigkeiten: `internal/review`; Specs unter `public/grafik/` samt `public/grafik/stil.paper.json`, `data/paper.json`, `data/asset-prompts.json`.
