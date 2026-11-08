package balance

import (
	"fmt"
	"slices"
	"strings"
)

// Werte-Gruppen (B-640, BS7.1): Jedes Ziel und jeder Hebel gehört zu genau einer Gruppe. Nach einem Meilenstein sind die
// abgenommenen Gruppen eingefroren (Targets.Frozen); eine Delta-Runde justiert nur die freien.
const (
	GroupBasis  = "basis"  // Mauern, Türme, Truppen, Hub, Kampf, Nächte
	GroupHolz   = "holz"   // Bäume, Holz fürs Markieren, Baumschule
	GroupSchiff = "schiff" // Schiffskosten, Endspiel-Korridor
)

// Groups sind die bekannten Werte-Gruppen.
var Groups = []string{GroupBasis, GroupHolz, GroupSchiff}

// IsGroup meldet, ob name eine bekannte Gruppe ist.
func IsGroup(name string) bool { return slices.Contains(Groups, name) }

// checkFrozen: nur bekannte Gruppen, keine doppelten.
func checkFrozen(frozen []string) error {
	seen := map[string]bool{}
	for _, g := range frozen {
		if !IsGroup(g) {
			return fmt.Errorf("frozen: gruppe %q unbekannt (bekannt: %s)", g, strings.Join(Groups, ", "))
		}
		if seen[g] {
			return fmt.Errorf("frozen: gruppe %q doppelt", g)
		}
		seen[g] = true
	}
	return nil
}

// IsFrozen meldet, ob die Gruppe eingefroren ist.
func (t *Targets) IsFrozen(group string) bool { return slices.Contains(t.Frozen, group) }

// checkGroups verlangt in der Datei der Richtwerte eine bekannte Gruppe an jedem Hebel.
func (h *Hints) checkGroups() error {
	for key, l := range h.Levers {
		for _, lv := range slices.Concat(l.Low, l.High) {
			if !IsGroup(lv.Group) {
				return fmt.Errorf("hebel %s, pfad %s: group %q, erlaubt: %s", key, lv.Path, lv.Group, strings.Join(Groups, ", "))
			}
		}
	}
	return nil
}
