package backend

import (
	"slices"
	"strings"
	"testing"

	assets "k3c/tools/assetkit"
)

// B-685/AC-03: Zielmaß per Karte über AssetsTargetSet; Prüfung vorab (negativ, zu groß, unbekannter Slot), entfernen mit null.
func TestChatAktionZielmass(t *testing.T) {
	a := &App{root: studioRepo(t), bus: newBus()}
	list := cards(t, a, "c", block(`{"art": "zielmass_setzen", "slot": "farm", "ziel": {"w": 6, "h": 4}},
		{"art": "zielmass_setzen", "slot": "farm", "ziel": {"w": -1, "h": 4}},
		{"art": "zielmass_setzen", "slot": "farm", "ziel": {"w": 6, "h": 400}},
		{"art": "zielmass_setzen", "slot": "weg", "ziel": {"w": 1, "h": 1}}`))
	if list[0].Block != "" || list[1].Block == "" || list[2].Block == "" || !strings.Contains(list[3].Block, "unbekannter Asset-Slot") {
		t.Fatalf("Karten %+v", list)
	}
	done, err := a.ChatActionRun("c", list[0].ID)
	if err != nil || done.Status != actionDone {
		t.Fatalf("Ausführen %+v %v", done, err)
	}
	if reg, _ := assets.Load(a.root); reg.Find("farm").Target == nil || *reg.Find("farm").Target != (assets.Units{W: 6, H: 4}) {
		t.Fatalf("Zielmaß nicht gespeichert: %+v", reg.Find("farm"))
	}
	clear := cards(t, a, "d", block(`{"art": "zielmass_setzen", "slot": "farm", "ziel": null}`))
	if done, _ := a.ChatActionRun("d", clear[0].ID); done.Status != actionDone {
		t.Fatalf("Entfernen %+v", done)
	}
	if reg, _ := assets.Load(a.root); reg.Find("farm").Target != nil {
		t.Error("Zielmaß nicht entfernt")
	}
}

// B-685: Die Art steht im Aktionskatalog.
func TestChatAktionZielmassImKatalog(t *testing.T) {
	if !slices.ContainsFunc(actionCatalog(), func(s actionSpec) bool { return s.art == "zielmass_setzen" }) {
		t.Error("zielmass_setzen fehlt im Katalog")
	}
}

// B-685/AC-03: AssetsTargetSet hält die Schreibregel ein.
func TestAssetsTargetSetSchreibregel(t *testing.T) {
	a := &App{root: studioRepo(t)}
	v := a.AssetsList()
	saved, err := a.AssetsTargetSet("farm", &assets.Units{W: 2, H: 3}, v.Stand)
	if err != nil || saved.Slot.Target == nil || saved.Stand == v.Stand {
		t.Fatalf("%+v %v", saved, err)
	}
	if _, err := a.AssetsTargetSet("farm", nil, v.Stand); err == nil {
		t.Error("veralteter Stand geschrieben")
	}
}
