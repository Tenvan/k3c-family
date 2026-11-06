package main

import "k3c/tools/k3c-workbench/internal/assetsui"

// Asset-Oberfläche (Bindings, docs/standards/paper-mcp.md › Oberfläche). Bis RV1 antworten die Stubs mit den
// Beispielantworten aus tools/paper/fixtures/ui/ (PV1.8); WB1 und RV1 bauen parallel dagegen.

func (a *App) assets() assetsui.Stubs {
	a.wait()
	return assetsui.Stubs{Root: a.root}
}

// AssetsList liefert den Katalog: je Asset ID, Rig, Gruppe, Pfad, gültig und geändert (Binding).
func (a *App) AssetsList() ([]assetsui.AssetEintrag, error) { return a.assets().AssetsList() }

// AssetsGet liefert die Spec eines Assets als Objekt (Binding).
func (a *App) AssetsGet(id string) (map[string]any, error) { return a.assets().AssetsGet(id) }

// AssetsStyle liefert die Stil-Datei als Objekt (Binding).
func (a *App) AssetsStyle() (map[string]any, error) { return a.assets().AssetsStyle() }

// AssetsPrompts liefert die Prompt-Vorlagen aus data/asset-prompts.json (Binding).
func (a *App) AssetsPrompts() ([]assetsui.Vorlage, error) { return a.assets().AssetsPrompts() }

// AssetsReview vergleicht den Arbeitsstand mit der Basis HEAD, HEAD~1, Commit-SHA oder origin/develop (Binding).
func (a *App) AssetsReview(basis string) (assetsui.Review, error) { return a.assets().AssetsReview(basis) }

// AssetsFrame liefert einen Frame als Data-URL mit Teil-Boxen; leere Basis = Arbeitsstand (Binding).
func (a *App) AssetsFrame(id, animation string, frame int, variante, basis string) (assetsui.Frame, error) {
	return a.assets().AssetsFrame(id, animation, frame, variante, basis)
}

// AssetsPixelDiff vergleicht einen Frame des Arbeitsstands mit der Basis, Overlay als Data-URL (Binding).
func (a *App) AssetsPixelDiff(id, animation string, frame int, basis string) (assetsui.PixelDiff, error) {
	return a.assets().AssetsPixelDiff(id, animation, frame, basis)
}
