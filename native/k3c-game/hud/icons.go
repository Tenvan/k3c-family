package hud

// Icons des HUD (B-890): Vorrat und Gold als Papierschnitt-Icon mit Zahl statt mit Namen. Die Icons kommen aus der
// Gruppe icons (data/paper.json › icon-*); hud meldet sie als Asset-Quelle für jedes Level an. Ohne Grafik (Tests)
// zeichnet das HUD statt des Icons den Namen.

import (
	"log/slog"
	"slices"
	"strconv"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
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
var hudAssets = []string{purseAsset, "icons/icon-eisen", "icons/icon-holz", "icons/icon-kristall", "icons/icon-kupfer", goldIcon,
	"icons/icon-stein"}

// keyAssets sind die Tasten-Icons (B-911), sortiert: die zwei Kappen und je Xbox-Taste mit Bild (padGlyphs) eins.
var keyAssets = []string{capNarrow, capWide, "icons/xbox-a", "icons/xbox-down", "icons/xbox-lb", "icons/xbox-lt", "icons/xbox-menu",
	"icons/xbox-rb", "icons/xbox-rs", "icons/xbox-rt", "icons/xbox-up", "icons/xbox-x", "icons/xbox-y"}

// Assets meldet die Icons des HUD für jedes Level (view.AssetSource), mit ladbaren Tasten-Icons und ladbarer Seekarte
// auch deren Assets.
func (h *hud) Assets(string, *sim.World) []string {
	if h.assets != nil {
		return h.assets
	}
	return hudAssets
}

// firstUnusable ist das erste Asset, das usable nicht laden kann ("" = alle ladbar).
func firstUnusable(assets []string, usable func(string) bool) string {
	for _, a := range assets {
		if !usable(a) {
			return a
		}
	}
	return ""
}

// useKeyIcons nimmt die Tasten-Icons, wenn usable alle laden kann (weiche Assets wie die Seekarte); sonst malt das HUD
// die Glyphen selbst (drawGlyph).
func (h *hud) useKeyIcons(usable func(string) bool) {
	if a := firstUnusable(keyAssets, usable); a != "" {
		slog.Info("🚫 Tasten ohne Papierschnitt-Icons, Asset nicht ladbar", "asset", a)
		return
	}
	h.keyIcons = true
	h.listAssets()
}

// useSeaMap nimmt die Assets der Seekarte m, wenn usable alle laden kann (Library.Usable: ein fehlendes Bild darf nie ein
// Level verhindern); sonst bleibt die Karte aus Formen (B-964/R5).
func (h *hud) useSeaMap(m *art.SeaMap, usable func(string) bool) {
	if m == nil {
		return
	}
	if a := firstUnusable(m.Assets(), usable); a != "" {
		slog.Info("🚫 Seekarte ohne Diorama, Asset nicht ladbar", "asset", a)
		return
	}
	h.seaMap = m
	h.listAssets()
}

// listAssets stellt die gemeldeten Assets zusammen: Icons, Tasten-Icons und Seekarte, sortiert.
func (h *hud) listAssets() {
	h.assets = slices.Clone(hudAssets)
	if h.keyIcons {
		h.assets = append(h.assets, keyAssets...)
	}
	if h.seaMap != nil {
		h.assets = append(h.assets, h.seaMap.Assets()...)
	}
	slices.Sort(h.assets)
}

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
