package main

import "k3c/tools/k3c-workbench/internal/assetsui"

// Asset-Oberfläche (Bindings, docs/standards/paper-mcp.md › Oberfläche): echte Daten aus dem Checkout (RV1).

func (a *App) assets() assetsui.Dienst {
	a.wait()
	return assetsui.Dienst{Root: a.root}
}

// AssetsList liefert den Katalog: je Asset ID, Rig, Gruppe, Pfad, gültig und geändert (Binding).
func (a *App) AssetsList() ([]assetsui.AssetEintrag, error) { return a.assets().AssetsList() }

// AssetsIndex liefert den Katalog ohne Prüfung, ungeprüfte Einträge mit `offen` (Binding, B-732).
func (a *App) AssetsIndex() ([]assetsui.AssetEintrag, error) { return a.assets().AssetsIndex() }

// AssetsFrameOhne liefert einen Frame des Arbeitsstands ohne die Teile in ohne (Binding, B-734).
func (a *App) AssetsFrameOhne(id, animation string, frame int, ohne []string) (assetsui.Frame, error) {
	return a.assets().AssetsFrameOhne(id, animation, frame, ohne)
}

// AssetsGroesse liefert den Spiel-Maßstab eines Assets für den Größenvergleich (Binding, B-733).
func (a *App) AssetsGroesse(id string) (assetsui.Groesse, error) { return a.assets().AssetsGroesse(id) }

// AssetsHintergruende listet die Biome mit Hintergrund (Binding, B-733).
func (a *App) AssetsHintergruende() ([]string, error) { return a.assets().AssetsHintergruende() }

// AssetsHintergrund liefert die drei Ebenen eines Biom-Hintergrunds als Data-URLs (Binding, B-733).
func (a *App) AssetsHintergrund(teil string) (assetsui.Hintergrund, error) {
	return a.assets().AssetsHintergrund(teil)
}

// AssetsCheck prüft die Specs unter pfade und liefert ihre Einträge (Binding, B-732).
func (a *App) AssetsCheck(pfade []string) ([]assetsui.AssetEintrag, error) {
	return a.assets().AssetsCheck(pfade)
}

// AssetsGet liefert die Spec eines Assets als Objekt (Binding).
func (a *App) AssetsGet(id string) (map[string]any, error) { return a.assets().AssetsGet(id) }

// AssetsStyle liefert die Stil-Datei als Objekt (Binding).
func (a *App) AssetsStyle() (map[string]any, error) { return a.assets().AssetsStyle() }

// AssetsPrompts liefert die Prompt-Vorlagen aus data/asset-prompts.json (Binding).
func (a *App) AssetsPrompts() ([]assetsui.Vorlage, error) { return a.assets().AssetsPrompts() }

// AssetsReview vergleicht den Arbeitsstand mit der Basis HEAD, HEAD~1, Commit-SHA oder origin/main (Binding).
func (a *App) AssetsReview(basis string) (assetsui.Review, error) {
	return a.assets().AssetsReview(basis)
}

// AssetsFrame liefert einen Frame als Data-URL mit Teil-Boxen; leere Basis = Arbeitsstand (Binding).
func (a *App) AssetsFrame(id, animation string, frame int, variante, basis string) (assetsui.Frame, error) {
	return a.assets().AssetsFrame(id, animation, frame, variante, basis)
}

// AssetsPixelDiff vergleicht einen Frame des Arbeitsstands mit der Basis, Overlay als Data-URL (Binding).
func (a *App) AssetsPixelDiff(id, animation string, frame int, basis string) (assetsui.PixelDiff, error) {
	return a.assets().AssetsPixelDiff(id, animation, frame, basis)
}
