package art

// Assets der Seekarte (B-964): data/paper.json › seekarte ordnet je Welt-ID eine Insel-Miniatur zu, dazu die drei
// Papierschichten des Meers, das Schiff, die Aufsätze und die Zustandszeichen, je als Asset `gruppe/id`. Hat eine Welt
// keinen Eintrag, gilt die Miniatur von `standard`. Die Assets sind weich wie die Weltbilder: Fehlt eins, zeichnet das
// HUD die Karte aus Formen.

import (
	"encoding/json"
	"fmt"
	"log/slog"
	"slices"

	"k3c/data"
)

// SeaLayers sind die Papierschichten des Meers von hinten nach vorn.
type SeaLayers struct {
	Back  string `json:"hinten"`
	Mid   string `json:"mitte"`
	Front string `json:"vorn"`
}

// SeaMarks sind die Zustandszeichen einer Insel.
type SeaMarks struct {
	Done   string `json:"abgeschlossen"`
	Locked string `json:"gesperrt"`
}

// SeaMap ist die Zuordnung der Seekarte.
type SeaMap struct {
	Islands map[string]string `json:"inseln"`
	Sea     SeaLayers         `json:"meer"`
	Ship    string            `json:"schiff"`
	Toppers []string          `json:"aufsaetze"`
	Marks   SeaMarks          `json:"zeichen"`
}

// ParseSeaMap liest den Abschnitt `seekarte` aus dem Inhalt von paper.json; jedes Feld muss gesetzt sein, `inseln`
// braucht `standard`, `aufsaetze` mindestens einen Eintrag.
func ParseSeaMap(raw []byte) (*SeaMap, error) {
	var p struct {
		Sea *SeaMap `json:"seekarte"`
	}
	if err := json.Unmarshal(raw, &p); err != nil {
		return nil, fmt.Errorf("paper.json › seekarte: %w", err)
	}
	m := p.Sea
	if m == nil {
		return nil, fmt.Errorf("paper.json › seekarte fehlt")
	}
	if m.Islands[StandardWorld] == "" {
		return nil, fmt.Errorf("paper.json › seekarte.inseln: Eintrag %q fehlt", StandardWorld)
	}
	if len(m.Toppers) == 0 {
		return nil, fmt.Errorf("paper.json › seekarte.aufsaetze ist leer")
	}
	if slices.Contains(m.Assets(), "") {
		return nil, fmt.Errorf("paper.json › seekarte: leerer Eintrag (inseln, meer, schiff, aufsaetze, zeichen)")
	}
	return m, nil
}

// DataSeaMap liest die Zuordnung aus data/paper.json; ein Fehler steht im Log und liefert nil (Seekarte aus Formen).
func DataSeaMap() *SeaMap {
	raw, err := data.Files.ReadFile("paper.json")
	if err == nil {
		var m *SeaMap
		if m, err = ParseSeaMap(raw); err == nil {
			return m
		}
	}
	slog.Error("❌ Assets der Seekarte nicht lesbar", "err", err)
	return nil
}

// Island liefert die Miniatur der Welt id, ersatzweise die der Standard-Welt ("" ohne Zuordnung). Ohne Allokation.
func (m *SeaMap) Island(id string) string {
	if m == nil {
		return ""
	}
	if a, ok := m.Islands[id]; ok {
		return a
	}
	return m.Islands[StandardWorld]
}

// Topper liefert den Aufsatz zum Wert seed (fester Hash der Insel-ID): dieselbe Insel, derselbe Aufsatz. Ohne Allokation.
func (m *SeaMap) Topper(seed uint32) string {
	if m == nil || len(m.Toppers) == 0 {
		return ""
	}
	return m.Toppers[int(seed%uint32(len(m.Toppers)))]
}

// Assets nennt alle Assets der Seekarte, sortiert und ohne Doppelte.
func (m *SeaMap) Assets() []string {
	if m == nil {
		return nil
	}
	out := []string{m.Sea.Back, m.Sea.Mid, m.Sea.Front, m.Ship, m.Marks.Done, m.Marks.Locked}
	out = append(out, m.Toppers...)
	for _, a := range m.Islands {
		out = append(out, a)
	}
	slices.Sort(out)
	return slices.Compact(out)
}
