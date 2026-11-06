# RV1.2 · internal/assetsui: echte Daten, Frame und Pixel-Diff

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** review
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** rv1/2-assetsui-echte-daten
- **Abhängig von:** RV1.1
- **Tickets:** B-699
- **Kriterien:** AC-05, AC-07

## Ziel

Auf `develop` liefern alle sieben Asset-RPC-Methoden (`AssetsList`, `AssetsGet`, `AssetsStyle`, `AssetsPrompts`, `AssetsReview`, `AssetsFrame`, `AssetsPixelDiff`) echte Daten aus dem Checkout statt der Fixture-Stubs aus PV1.8.

## Kontext

- **Vertrag:** `docs/standards/paper-mcp.md` › 4 Oberfläche (RPC): Signaturen, Antworttypen, Fehlertexte (`Basis „x“ ungültig: …`, `Asset „x“ fehlt`, unbekannte Animation → Text mit den vorhandenen Animationen). Bilder als Data-URL `data:image/png;base64,…`.
- **Stand PV1.8:** `tools/k3c-workbench/internal/assetsui/stubs.go` (Typen, `CheckBasis`, `checkID`, `Stubs{Root}` liest `tools/paper/fixtures/ui/<Methode>.json`), Tests `stubs_test.go`. Gebunden in `tools/k3c-workbench/app_assets.go` (`func (a *App) assets() assetsui.Stubs`), Allowlist `uiMethods` in `uiapi.go` bleibt unverändert.
- **RV1.1** liefert `internal/review`: `Aufloesen`, `Show`, `SpecsImArbeitsstand`, `Vergleich`, `Renderer`/`CLI`, Cache `.cache/paper/<renderKey>/`, Hash-Funktionen. Review-Typen aus `review` als Alias übernehmen (`type Review = review.Ergebnis` o. ä.), keine doppelten Strukturen.
- **Test-Repo** wie RV1.1 (`bun tools/paper/fixtures/review-repo/erzeuge.ts`). Die Fixtures unter `tools/paper/fixtures/ui/` beschreiben den Stand des Test-Repos mit Basis `HEAD~1`. **Abweichung:** `AssetsList.json` meldet `pferd` und `mauer` mit `geaendert: true`, laut Vertrag zählt aber `HEAD` (dort unverändert). Der Test erwartet die Fixture mit `geaendert: false` für beide; Ticket im Projekt PAP zum Nachziehen der Fixture.
- `AssetsFrame`/`AssetsPixelDiff` mit der CLI-Attrappe: `frame` liefert feste PNGs; für den Pixel-Vergleich ersetzt der Test `Frame` wie in RV1.1 durch die PNGs aus `tools/paper/fixtures/render/`.
- Leere `animation` = erste Animation der Spec (wie MCP-Standardwert); die Oberfläche braucht das für gelöschte Assets.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/assetsui/`
- `tools/k3c-workbench/app_assets.go` (nur `assets()` auf den neuen Dienst umstellen)

## Nicht-Ziele

- Keine neuen RPC-Methoden und keine geänderten Signaturen (sonst Ticket im Projekt PAP).
- Keine Änderung an `internal/review` außer einem Fix mit Test, falls RV1.2 einen Fehler findet (dann im Ergebnis nennen).
- Keine Oberfläche, kein `wails.ts`.

## Schritte

1. **Rot** (`assetsui_test.go`, ersetzt `stubs_test.go`; Helfer `testRepo(t)` wie RV1.1): `TestAssetsListTestRepo` – nach `id` sortiert gleich `AssetsList.json` mit `geaendert: false` für pferd und mauer, banner `gueltig: false` mit `fehler` (AC-07); `TestAssetsGet` – monarch im Test-Repo = `AssetsGet.json` mit `umhang.material: purpur`, ohne `$schema`; `TestAssetsGetFehlt` → `Asset „niemand“ fehlt`; `TestAssetsStyle` – Repo-Wurzel = `AssetsStyle.json`; `TestAssetsPrompts` – Repo-Wurzel = `AssetsPrompts.json` (AC-07).
2. **Rot:** `TestAssetsReviewTestRepo` – Basis `HEAD~1` mit Frame-Attrappe = `AssetsReview.json` (nach `id`), banner `ungueltig` mit `fehler`, die vier übrigen Einträge vollständig (AC-05); `TestAssetsReviewLeer` – `""` = `HEAD`; `TestAssetsBasisUngueltig` – `AssetsReview("main")`, `AssetsFrame(…, "main")`, `AssetsPixelDiff(…, "")` → Vertragsfehlertext.
3. **Rot:** `TestAssetsFrame` – monarch, `reiten`, 0, `""`, `""` → `bild` beginnt mit `data:image/png;base64,`, dekodiert = `render/monarch/monarch-galopp-0.png`, `teile` = `teile` aus `AssetsFrame.json`; `TestAssetsFrameBasis` – mit Basis `HEAD~1` kommt das Bild aus dem Cache (zweiter Aufruf ohne CLI); `TestAssetsFrameAnimationUnbekannt` – `flug` → Text nennt `idle` und `reiten`; `TestAssetsFrameLeereAnimation` → erste Animation; `TestAssetsPixelDiff` – monarch, Basis `HEAD~1`, Frame-Attrappe → `anteil`, `boxen`, `overlay` gleich `AssetsPixelDiff.json`.
4. **Grün:** `stubs.go` → `assets.go` mit `Dienst{Root string; R review.Renderer}` (Standard `review.CLI`); `AssetsList` validiert je Spec (höchstens 4 CLI-Aufrufe parallel), `geaendert` gegen `HEAD` über Hash-Vergleich und unversionierte Dateien; `AssetsGet` sucht die `id` über alle Gruppen; Frames und Overlays als Data-URL. `app_assets.go`: `assets()` liefert den `Dienst`. Stub-Code und Fixture-Laden entfernen.
5. `TestUIMethodsWieWailsTS` bleibt grün; `go test ./internal/assetsui/ ./...` grün, Commit `feat(dev): Asset-RPC mit echten Daten (RV1.2)`.

## Fertig, wenn

- [ ] AC-07: `TestAssetsListTestRepo`, `TestAssetsGet`, `TestAssetsStyle`, `TestAssetsPrompts` grün (`cd tools/k3c-workbench && go test ./internal/assetsui/`).
- [ ] AC-05: `TestAssetsReviewTestRepo` grün (ungültiger Eintrag mit `fehler`, übrige vollständig).
- [ ] `TestAssetsFrame*`, `TestAssetsPixelDiff`, `TestAssetsBasisUngueltig`, `TestUIMethodsWieWailsTS` grün; kein Verweis mehr auf `fixtures/ui` im Produktionscode (`grep -r fixtures/ui tools/k3c-workbench/internal/assetsui/*.go` nur in `_test.go`).
- [ ] `task check:diff` grün.

## Prüfen

```bash
cd tools/k3c-workbench && go test ./internal/assetsui/ ./internal/review/
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf). Code-Review vor dem Commit (Pfad-Code, ab etwa 150 Zeilen).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
