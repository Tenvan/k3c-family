# tools/k3c-workbench/internal/review/

## Responsibility

Vergleicht die Papierschnitt-Assets des Arbeitsstands mit einer Git-Basis (Vertrag `docs/standards/paper-mcp.md` › Oberfläche) und ist die gemeinsame Anbindung der Render-CLI für `assetsui` und `paperassets`.

## Design

- Render-CLI (`aufruf.go`, B-706): `Skript(root)` wählt `K3C_PAPER_CLI`, sonst `tools/paper/bin/paper.ts`, ohne die Datei die Attrappe `tools/paper/fixtures/cli-stub.ts`; `Aufruf` führt `bun <skript> <args…>` über `internal/proc` mit `Zeitlimit` (2 min) aus; `Auswerten` deutet Exit 0 (stdout), Exit 1 mit `{fehler}` (`*Abgelehnt`) und Absturz (stderr, auch je Schritt eines `batch`).
- `cli.go`: Schnittstelle `Renderer` mit Implementierung `CLI{Root}`: `Validate`, `Diff`, `Frame`, `FrameOhne`, `PixelDiff` → `DiffAntwort`, `FrameAntwort`, `PixelAntwort`.
- Basis (`basis.go`): `Aufloesen(root, basis)` → `Stand`; erlaubt `HEAD`, `HEAD~1`, `origin/main` oder Commit-SHA (`reBasis`, sonst `FehlerBasis`), `fetchMemo` merkt den letzten Fetch je Checkout.
- Git-Lesen (`git.go`): `BasisDateien`, `SpecsInCommit`, `SpecsImArbeitsstand`, `Show(commit, pfad)`.
- Hash (`hash.go`): `Kanonisch`, `SpecHash`, `RenderKey` (Port von `hash.ts` aus `tools/paper`).
- Cache (`cache.go`): `Quelle` (Spec + Stil-Datei mit Hashes, aus Basis oder Arbeitsstand: `QuelleBasis`, `QuelleArbeitsstand`), `Bild` rendert einen Frame nach `.cache/paper/<renderKey>/` nur, wenn er fehlt (`Gerendert` mit Teil-Boxen); `Kennung` trennt Ergebnisse je Renderer-Version; `SpecUnlesbar`; `Reittiere`.
- Vergleich (`review.go`): `Vergleich(root, basis, r)` → `Ergebnis` mit je geändertem `Asset` (Feld-Änderungen `FeldAenderung`, `Fehler`, Pixel-Anteil); `Geaendert` liefert je Spec, ob sie sich gegenüber der Basis geändert hat.
- `pixel.go`: `Overlay` (Pixel-Diff eines Frames mit Overlay im Cache), `PixelAnteil` (Frame 0 der ersten Animation; nicht renderbare Basis = 1). `stil.go`: `ErsteAnimation`, `Animationen` in Datei-Reihenfolge.

## Flow

1. `assetsui.AssetsReview(basis)` → `Vergleich` → `Aufloesen` → `SpecsInCommit`/`SpecsImArbeitsstand` → je Spec `QuelleBasis` + `QuelleArbeitsstand` → `CLI.Diff` → `PixelAnteil` → `Ergebnis`.
2. `assetsui.AssetsFrame` → `Bild` (Cache-Treffer oder `CLI.Frame`) → Data-URL.
3. `paperassets` nutzt `Aufruf`/`Auswerten` für `validate`, `render`, `batch`.

## Integration

- Konsumenten: `internal/assetsui` (alle Dateien), `internal/paperassets` (`basis.go`, `batch.go`, `cli.go`, `reiter.go`, `stil.go`).
- Abhängigkeiten: `internal/proc`; externe Programme `bun` und `git`; Render-CLI `tools/paper/bin/paper.ts`.
