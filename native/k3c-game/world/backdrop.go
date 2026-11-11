package world

// Kulisse je Biom (B-795/AC-02): drei Schichten aus Papier-Kacheln `hintergruende/hg-<teil>-<fern|mitte|nah>-v<n>`
// (Browser: src/scenes/render/worldSprites.ts › BACKGROUND_LAYERS, paperAssets.ts › paperBackgroundFile), Farben aus
// data/biomes/*.json › palette. Seit NV3 steht `fern` fest (Faktor 0, native/k3c-ebiten/level.go); `mitte` und `nah`
// scrollen mit den Faktoren des Browsers. Variante je Kachel aus World.Level.Backdrops (Server, nach Seed).

import (
	"encoding/json"
	"fmt"
	"image/color"
	"log/slog"
	"strconv"
	"strings"

	"k3c/data"
)

const (
	// tileUnits ist die Breite einer Kulissen-Kachel in Units (engine/level › BackdropTileUnits, Regeldatei § 10).
	tileUnits = 24.0
	// framePerUnit sind Rahmen-Einheiten der Papier-Manifeste je Unit (32 px je Unit, BACKGROUND_DENSITY).
	framePerUnit = 32.0
	// layerCount ist die Zahl der Kulissen-Schichten fern, mitte, nah.
	layerCount = 3
)

// layerNames sind die Namensteile der Schichten von hinten nach vorn.
var layerNames = [layerCount]string{"fern", "mitte", "nah"}

// biomeLayers: je Biom der Namensteil der Assets und die Scroll-Faktoren fern → nah (fern fest wie NV3).
var biomeLayers = map[string]struct {
	part   string
	scroll [layerCount]float64
}{
	"forest":   {"wald", [layerCount]float64{0, 0.35, 0.5}},
	"cave":     {"hoehle", [layerCount]float64{0, 0.2, 0.4}},
	"mine":     {"mine", [layerCount]float64{0, 0.25, 0.45}},
	"ironhold": {"eisenstollen", [layerCount]float64{0, 0.25, 0.45}},
	"crystal":  {"kristall", [layerCount]float64{0, 0.25, 0.45}},
}

// Rückfall-Farben ohne Palette (wie NV3).
var (
	defaultSky    = color.RGBA{158, 199, 219, 255}
	defaultGround = color.RGBA{74, 56, 36, 255}
)

// look ist das Aussehen eines Bioms: Farben, Scroll-Faktoren und die Asset-Namen je Schicht und Variante (fest
// angelegt, damit das Zeichnen keine Namen baut).
type look struct {
	sky, ground color.RGBA
	scroll      [layerCount]float64
	assets      [layerCount][]string // [Schicht][Variante − 1] = `gruppe/id`
}

// looks sind die Aussehen je Biom-ID (= Level).
type looks map[string]*look

// loadLooks liest Paletten (data/biomes) und Varianten (data/paper.json › hintergruende); Fehler stehen im Log, die
// Welt zeichnet dann mit Rückfall-Farben bzw. ohne Kulisse.
func loadLooks() looks {
	group, variants, err := backdropGroup()
	if err != nil {
		slog.Error("❌ Kulissen-Daten nicht lesbar", "err", err)
	}
	palettes, err := readPalettes()
	if err != nil {
		slog.Error("❌ Paletten nicht lesbar", "err", err)
	}
	return buildLooks(group, variants, palettes)
}

// palette ist der Ausschnitt aus data/biomes/*.json, den die Welt braucht.
type palette struct {
	Sky    string `json:"sky"`
	Ground string `json:"ground"`
}

// buildLooks baut die Aussehen aller Biome mit Palette oder Kulisse.
func buildLooks(group string, variants int, palettes map[string]palette) looks {
	out := looks{}
	get := func(id string) *look {
		if l, ok := out[id]; ok {
			return l
		}
		l := &look{sky: defaultSky, ground: defaultGround}
		out[id] = l
		return l
	}
	for id, p := range palettes {
		l := get(id)
		l.sky, l.ground = parseHex(p.Sky, defaultSky), parseHex(p.Ground, defaultGround)
	}
	for id, b := range biomeLayers {
		l := get(id)
		l.scroll = b.scroll
		for i, name := range layerNames {
			for v := 1; v <= variants; v++ {
				l.assets[i] = append(l.assets[i], fmt.Sprintf("%s/hg-%s-%s-v%d", group, b.part, name, v))
			}
		}
	}
	return out
}

// assets nennt alle Kulissen-Assets des Levels (alle Varianten aller Schichten); unbekanntes Level: keine.
func (ls looks) assets(level string) []string {
	l, ok := ls[level]
	if !ok {
		return nil
	}
	var out []string
	for _, a := range l.assets {
		out = append(out, a...)
	}
	return out
}

// of liefert das Aussehen des Levels, sonst ein leeres mit Rückfall-Farben.
func (ls looks) of(level string) *look {
	if l, ok := ls[level]; ok {
		return l
	}
	return &fallbackLook
}

var fallbackLook = look{sky: defaultSky, ground: defaultGround}

// variant liefert die Variante (1 …) der Kachel k aus der Folge backdrops (wiederholt, auch für negative k); ohne
// Folge 1. Mehr Varianten, als es Dateien gibt (n), laufen reihum (wie paperBackgroundFile).
func variant(backdrops []int, k, n int) int {
	if len(backdrops) == 0 || n <= 0 {
		return 1
	}
	m := len(backdrops)
	v := max(1, backdrops[((k%m)+m)%m])
	return (v-1)%n + 1
}

// backdropGroup liest Gruppe und Zahl der Varianten der Kulissen aus data/paper.json.
func backdropGroup() (string, int, error) {
	raw, err := data.Files.ReadFile("paper.json")
	if err != nil {
		return "hintergruende", 1, err
	}
	var p struct {
		Backdrops struct {
			Group    string `json:"gruppe"`
			Variants int    `json:"varianten"`
		} `json:"hintergruende"`
	}
	if err := json.Unmarshal(raw, &p); err != nil || p.Backdrops.Group == "" || p.Backdrops.Variants < 1 {
		return "hintergruende", 1, fmt.Errorf("paper.json › hintergruende: %v", err)
	}
	return p.Backdrops.Group, p.Backdrops.Variants, nil
}

// readPalettes liest die Paletten aller Biome aus data/biomes/*.json.
func readPalettes() (map[string]palette, error) {
	entries, err := data.Files.ReadDir("biomes")
	if err != nil {
		return nil, err
	}
	out := map[string]palette{}
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		raw, err := data.Files.ReadFile("biomes/" + e.Name())
		if err != nil {
			return out, err
		}
		var b struct {
			ID      string  `json:"id"`
			Palette palette `json:"palette"`
		}
		if err := json.Unmarshal(raw, &b); err != nil {
			return out, fmt.Errorf("biomes/%s: %w", e.Name(), err)
		}
		out[b.ID] = b.Palette
	}
	return out, nil
}

// parseHex liest eine Farbe `#rrggbb`; sonst def.
func parseHex(s string, def color.RGBA) color.RGBA {
	if len(s) != 7 || s[0] != '#' {
		return def
	}
	v, err := strconv.ParseUint(s[1:], 16, 32)
	if err != nil {
		return def
	}
	return color.RGBA{uint8(v >> 16), uint8(v >> 8), uint8(v), 255}
}
