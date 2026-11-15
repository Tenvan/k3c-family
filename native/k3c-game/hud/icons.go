package hud

// Icons des HUD (B-890): Vorrat und Gold als Papierschnitt-Icon mit Zahl statt mit Namen. Die Icons kommen aus der
// Gruppe icons (data/paper.json › icon-*); hud meldet sie als Asset-Quelle für jedes Level an. Ohne Grafik (Tests)
// zeichnet das HUD statt des Icons den Namen.

import (
	"strconv"

	"k3c/engine/sim"
)

// iconAnim ist die Animation der Icons.
const iconAnim = "idle"

// goldIcon ist das Münz-Icon (die Spielerkarte zeigt seit B-919 den Geldsack, purse.go).
const goldIcon = "icons/icon-muenze"

// resourceIcons ordnet jedem Rohstoff (resourceOrder) sein Icon zu.
var resourceIcons = map[string]string{
	"wood": "icons/icon-holz", "stone": "icons/icon-stein", "copper": "icons/icon-kupfer",
	"iron": "icons/icon-eisen", "crystal": "icons/icon-kristall",
}

// hudAssets sind alle Icons des HUD, sortiert.
var hudAssets = []string{"icons/icon-eisen", "icons/icon-holz", "icons/icon-kristall", "icons/icon-kupfer", goldIcon,
	"icons/icon-stein"}

// Assets meldet die Icons des HUD für jedes Level (view.AssetSource).
func (*hud) Assets(string, *sim.World) []string { return hudAssets }

// stockPart ist ein Teil der Vorrats-Anzeige: Rohstoff (Icon; "" = nur Text) und Zahl bzw. Text.
type stockPart struct {
	res, text string
	full      bool
}

// stockSlots ist die Zahl der Teile (fünf Rohstoffe und die Skill-Punkte).
const stockSlots = textSlot(len(resourceOrder) + 1)

// stockParts schreibt die Teile der Vorrats-Anzeige in dst (geleert): wie sharedText die Rohstoffe mit Vorrat oder
// den Primärrohstoff, je „n“ bzw. „n/max“, dahinter die freien Skill-Punkte als Text.
func stockParts(dst []stockPart, k stockKey) []stockPart {
	dst = dst[:0]
	for i, r := range resourceOrder {
		n := k.stock[i]
		if n <= 0 && r != k.primary {
			continue
		}
		p := stockPart{res: r, text: strconv.Itoa(n)}
		if k.max > 0 {
			p.text += "/" + strconv.Itoa(k.max)
			p.full = n >= k.max
		}
		dst = append(dst, p)
	}
	if k.points > 0 {
		dst = append(dst, stockPart{text: tf(k.lang, "hud.skillPoints", "n", k.points)})
	}
	return dst
}
