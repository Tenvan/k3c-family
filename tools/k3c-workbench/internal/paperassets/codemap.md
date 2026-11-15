# tools/k3c-workbench/internal/paperassets/

## Responsibility

Asset-Schicht der MCP-Tools `assets_*` (Vertrag `docs/standards/paper-mcp.md`, `paper-spec.md`): Aufruf der Render-CLI, JSON-Merge-Patch, atomares Schreiben einer Spec, Anlegen und Löschen, Stil-Datei, Git-Basis, Render-Jobs und `golden.json` je Gruppe.

## Design

- Render-CLI (`cli.go`): `CLI{Root, Skript}` (`NewCLI` mit `review.Skript`) mit `Render`, `Frame`, `Diff`, `PixelDiff`; Ablehnung als `Abgelehnt` (`AlsAbgelehnt`), `Fehler` aus `assetsui`. `batch.go`: `CLI.Batch(schritte)` als Sammelaufruf (`Schritt`, `SchrittErgebnis`, B-856) über `review.Aufruf`.
- Ablage (`basis.go`): Specs unter `public/grafik/<gruppe>/<id>.paper.json` (`Grafik`); `Asset` mit `Abs`/`Ausgabe`; `Assets`, `Finde` (`<gruppe>/<id>` oder bloße ID, sonst `MehrdeutigFehler`), `Geaendert` gegen `HEAD`, `CheckBasis`.
- Ändern (`set.go`, `set_plan.go`, `patch.go`, `teile.go`): `Set(id, patch)` = Merge-Patch (`MergePatch`, Teil-Patch je Teil-ID in `teilePatch`) → `validate` und Render in `TempOrdner` (`.cache/paper/tmp/…`) in einem Sammelaufruf (`pruefeUndRendere`, `pixelPlan`) → Spec per Temp-Datei und Umbenennen ersetzen → Ausgabe umbenennen; `Schreibe` für ganzen Text; `SetErgebnis` mit Feld-Änderungen und `Pixel` je Animation (Overlay `SetOverlay` unter `.cache/paper/mcp/`). `Format`, `Schluessel`, `Felder` für Antworten.
- Anlegen/Löschen (`anlegen.go`, `rigs.go`): `Anlegen` aus der Rig-Vorlage (`Rig`, `RigNamen`, `FindeRig`, `Vorlage`) mit Sperre je ID; `Loesche` entfernt Spec, Ausgabe-Ordner und Index-Eintrag; `AusgabePNGs`.
- Stil (`stil.go`): `StilSet` patcht `public/grafik/stil.paper.json` mit Schema- und Material-Prüfung, `StilHash`, `FeldDiff`, `Materialien`, `Wert`; betroffene Assets werden ermittelt.
- `gueltig.go`: `Gueltigkeit` merkt `validate` je Spec, solange Spec, Stil-Datei und Reittier-Specs unverändert sind (höchstens 8 parallel). `reiter.go`: `ReiterVon` (Reiter hängen an ihrem Reittier, B-706).
- `job.go`: `Jobs` (`Start`, `Status`, `Wait`) für Render-Läufe mehrerer Assets mit `RenderEins` → `AssetErgebnis`, `Stand`.
- `golden.go` (B-707): zieht den Eintrag des einen Assets in `public/grafik/<gruppe>/golden.json` im Format von `gen:paper` nach (`gruppeNachziehen` aus `Schreibe` und `RenderEins`, `goldenEntfernen` aus `Loesche`).

## Flow

1. `assets_set` → `Finde` → `Set` → `pruefeUndRendere` (`CLI.Batch`) → `Schreibe` → `gruppeNachziehen` (`golden.json`).
2. `assets_create` → `Anlegen` → `Vorlage` → validate → render; `assets_delete` → `Loesche` → `goldenEntfernen`.
3. `assets_render` → `Jobs.Start` → je Asset `RenderEins`; `Status`/`Wait` für den Fortschritt.
4. `assets_style_set` → `StilSet` → `FeldDiff` + betroffene Assets.

## Integration

- Konsumenten: `internal/mcpsrv/tools_assets.go`, `tools_assets_preview.go`, `tools_assets_render.go`, `tools_assets_review.go`, `tools_assets_set.go`.
- Abhängigkeiten: `internal/review` (`Skript`, `Aufruf`, `Auswerten`), `internal/assetsui` (`Fehler`, `FeldAenderung`), `internal/proc`; externe Programme `bun`, `git`; Render-CLI `tools/paper/bin/paper.ts`.
