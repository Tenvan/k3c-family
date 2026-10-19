package studio

import (
	"net/http"
	"time"

	assets "k3c/tools/assetkit"
)

// AssetSlotView ist ein Asset-Slot mit seinem Status.
type AssetSlotView struct {
	assets.Slot
	Status string `json:"status"`
}

// AssetsView ist das Slot-Register für die Oberfläche. Stand ist der Hash von data/assets.json beim Lesen; AssetsSet
// braucht ihn (Schreibregel). Preview ist die Basis der Bild- und Sound-Adressen: das Studio liefert public/
// selbst aus (publicHandler), deshalb immer "/".
type AssetsView struct {
	Slots   []AssetSlotView `json:"slots"`
	Error   string          `json:"error"`
	Preview string          `json:"preview"`
	Stand   string          `json:"stand"`
}

// AssetSaved ist das Ergebnis von AssetsSet: der gespeicherte Slot und der neue Stand des Registers.
type AssetSaved struct {
	Slot  AssetSlotView `json:"slot"`
	Stand string        `json:"stand"`
}

// AssetsList liest das Slot-Register frisch (Binding); ein ungültiges Register steht in Error, die Liste bleibt leer.
func (a *App) AssetsList() AssetsView {
	view := AssetsView{Slots: []AssetSlotView{}, Preview: "/"}
	stand, err := assets.Stamp(a.root)
	if err != nil {
		view.Error = err.Error()
		return view
	}
	reg, err := assets.Load(a.root)
	if err != nil {
		view.Error = err.Error()
		return view
	}
	view.Stand = stand
	for _, s := range reg.Slots {
		view.Slots = append(view.Slots, AssetSlotView{Slot: s, Status: assets.Status(a.root, s)})
	}
	return view
}

// AssetsSet ersetzt die Zuordnungen eines Asset-Slots (Reihenfolge = Präferenz), aber nur auf dem Stand stand aus
// AssetsList; sonst Fehler „Register geändert, bitte erneut speichern“ ohne Schreibzugriff (Binding, B-459/AC-07).
func (a *App) AssetsSet(id string, wahl []assets.Choice, stand string) (AssetSaved, error) {
	slot, next, err := assets.SetChoicesIf(a.root, id, wahl, stand)
	if err != nil {
		return AssetSaved{}, err
	}
	return AssetSaved{Slot: AssetSlotView{Slot: *slot, Status: assets.Status(a.root, *slot)}, Stand: next}, nil
}

// AssetsCatalog sucht im Asset-Katalog wie assets_catalog (Binding); limit ≤ 0 = 200.
func (a *App) AssetsCatalog(query, art string, limit int) ([]assets.Asset, error) {
	list, err := assets.Catalog(a.root)
	if err != nil {
		return nil, err
	}
	if limit <= 0 {
		limit = 200
	}
	hits := assets.Search(list, query, art, limit)
	if hits == nil {
		hits = []assets.Asset{}
	}
	return hits, nil
}

// AssetsHits liefert die gespeicherten Recherche-Treffer eines Slots (Binding).
func (a *App) AssetsHits(id string) ([]assets.Hit, error) {
	hits, err := assets.HitsOf(a.root, id)
	if hits == nil {
		hits = []assets.Hit{}
	}
	return hits, err
}

// AssetsResearch sucht im Netz nach Alternativen für einen Slot und speichert neue Treffer (Binding).
func (a *App) AssetsResearch(id, query string) (assets.ResearchResult, error) {
	return assets.RunResearch(a.context(), a.root, id, query, assets.NewSearcher(a.root), time.Now())
}

// AssetsHitDrop verwirft einen Recherche-Treffer (Binding).
func (a *App) AssetsHitDrop(id, hit string) error {
	return assets.DropHit(a.root, id, hit)
}

// AssetsDownload übernimmt einen Treffer mit direktem Download und erkannter Lizenz (Binding).
func (a *App) AssetsDownload(id, hit string) (string, error) {
	return assets.DownloadHit(a.context(), a.root, id, hit, http.DefaultClient, time.Now())
}

// AssetsLicenseOK bestätigt die Lizenz eines Treffers (Binding, nur 🧑; bewusst kein MCP-Tool).
func (a *App) AssetsLicenseOK(id, hit string) error {
	return assets.ConfirmLicense(a.root, id, hit, time.Now())
}
