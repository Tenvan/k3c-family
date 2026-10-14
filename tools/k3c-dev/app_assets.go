package main

import (
	"fmt"

	assetreg "k3c/tools/k3c-dev/internal/assets"
	"k3c/tools/k3c-dev/internal/services"
)

// AssetSlotView ist ein Asset-Slot mit seinem Status für die Seite Assets (B-298).
type AssetSlotView struct {
	assetreg.Slot
	Status string `json:"status"`
}

// AssetsView ist das Slot-Register für die Oberfläche; Preview ist die Adresse des Vite-Dienstes (Bilder, Mini-Szene),
// leer, solange Vite nicht läuft.
type AssetsView struct {
	Slots   []AssetSlotView `json:"slots"`
	Error   string          `json:"error"`
	Preview string          `json:"preview"`
}

// AssetsList liest das Slot-Register frisch (Binding); ein ungültiges Register steht in Error, die Liste bleibt leer.
func (a *App) AssetsList() AssetsView {
	a.wait()
	view := AssetsView{Slots: []AssetSlotView{}, Preview: a.previewBase()}
	reg, err := assetreg.Load(a.root)
	if err != nil {
		view.Error = err.Error()
		return view
	}
	for _, s := range reg.Slots {
		view.Slots = append(view.Slots, AssetSlotView{Slot: s, Status: assetreg.Status(a.root, s)})
	}
	return view
}

// AssetsSet ersetzt die Zuordnungen eines Asset-Slots (Reihenfolge = Präferenz) wie assets_set (Binding).
func (a *App) AssetsSet(id string, wahl []assetreg.Choice) (AssetSlotView, error) {
	a.wait()
	slot, err := assetreg.SetChoices(a.root, id, wahl)
	if err != nil {
		return AssetSlotView{}, err
	}
	return AssetSlotView{Slot: *slot, Status: assetreg.Status(a.root, *slot)}, nil
}

// AssetsCatalog sucht im Asset-Katalog wie assets_catalog (Binding); limit ≤ 0 = 200.
func (a *App) AssetsCatalog(query, art string, limit int) ([]assetreg.Asset, error) {
	a.wait()
	list, err := assetreg.Catalog(a.root)
	if err != nil {
		return nil, err
	}
	if limit <= 0 {
		limit = 200
	}
	hits := assetreg.Search(list, query, art, limit)
	if hits == nil {
		hits = []assetreg.Asset{}
	}
	return hits, nil
}

// previewBase ist http://localhost:<Port>/ des laufenden Vite-Dienstes, sonst leer.
func (a *App) previewBase() string {
	if a.ctl == nil {
		return ""
	}
	for _, st := range a.ctl.Statuses() {
		if st.Name == "Vite" && (st.State == services.Running || st.State == services.Adopted) {
			return fmt.Sprintf("http://localhost:%d/", st.Port)
		}
	}
	return ""
}
