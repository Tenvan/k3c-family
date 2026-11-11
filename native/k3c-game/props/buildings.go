package props

// Welche Grafik ein Gebäude zeigt (Spiegel von src/scenes/render/buildingSprites.ts, hubGrowth.ts, backWallRules.ts,
// damageRules.ts, scaffoldView.ts, buildSignView.ts). Reine Auswahl ohne Ebitengine; ein leerer look = keine Grafik,
// dann zeichnet die Platzhalter-Form.

import (
	"math"

	"k3c/engine/sim"
)

// Gruppen der Papier-Assets (data/paper.json › gebaeude, objekte; Münze in icons).
const (
	buildingGroup = "gebaeude-paper/"
	objectGroup   = "objekte/"
)

// look ist eine Grafik: Asset `gruppe/id` und Animation im Manifest.
type look struct{ asset, anim string }

func (l look) ok() bool { return l.asset != "" }

// stageAnims: Animation der Gebäude-Stufe n (1 = `stand`, n > 1 = `stand.s<n>`), fest, damit nichts je Bild entsteht.
var stageAnims = [...]string{"stand", "stand", "stand.s2", "stand.s3", "stand.s4", "stand.s5"}

// stageAnim liefert die Animation der Stufe n (begrenzt auf 1 … 5).
func stageAnim(n int) string { return stageAnims[min(max(n, 1), len(stageAnims)-1)] }

// hubBuildings: Hub-Gebäude → Papier-Asset; ohne eigene `levels` in den Daten folgen sie der Hub-Stufe (GEB9).
var hubBuildings = map[string]string{
	"workshop": bg + "werkstatt", "tavern": bg + "taverne", "healer": bg + "heilplatz", "smithy": bg + "schmiede",
	"armory": bg + "ruestkammer", "storage": bg + "lager", "nursery": bg + "farm", "barracks": bg + "kaserne",
}

// bg ist die Gruppe der Gebäude (kurz für die Tabellen; Asset-Pfade stehen fertig darin, damit je Bild kein String
// entsteht).
const bg = buildingGroup

// staged: Platzarten mit Grafik je Stufe und ihre Stufenzahl (Mauer, Turm 1–5; Schiff: Kiel, Rumpf, Segel).
var staged = map[string]struct {
	id     string
	stages int
	clamp  bool // fehlende Stufe = höchste (Hub-Gebäude, Schiff); sonst Grundform
}{
	"wall": {bg + "mauer", 5, false}, "tower": {bg + "turm", 5, false}, "ship": {bg + "schiff", 3, true},
}

// plainSites: Platzarten mit einer Grafik ohne Stufen.
var plainSites = map[string]string{"stairsUp": bg + "treppe-hoch", "stairsDown": bg + "treppe-runter"}

// isHubBuilding: Hub-Gebäude ohne eigene Stufen zeigen die Grafik der Hub-Stufe (GEB9, B-569).
func isHubBuilding(kind string) bool {
	_, hub := hubBuildings[kind]
	return hub && len(buildings[kind].Levels) == 0
}

// siteStage ist die Stufe der Grafik: Hub-Gebäude ohne Stufen je Hub-Stufe, sonst die Stufe des Platzes (0 = 1).
func siteStage(kind string, level, hubLevel int) int {
	if isHubBuilding(kind) {
		return max(1, hubLevel)
	}
	return max(1, level)
}

// siteLook ist die Grafik eines gebauten Platzes in Stufe stage; Baustellen und unbekannte Arten haben keine.
// Mauer und Turm über Stufe 5 zeigen die Grundform, Hub-Gebäude und Schiff die höchste Stufe (B-458).
func siteLook(kind, state string, stage int) look {
	if state != "built" {
		return look{}
	}
	if id, ok := plainSites[kind]; ok {
		return look{id, "stand"}
	}
	if id, ok := hubBuildings[kind]; ok {
		return look{id, stageAnim(min(max(stage, 1), 5))}
	}
	s, ok := staged[kind]
	if !ok {
		return look{}
	}
	switch {
	case s.clamp:
		stage = min(max(stage, 1), s.stages)
	case stage < 1 || stage > s.stages:
		stage = 1
	}
	return look{s.id, stageAnim(stage)}
}

// scaffoldClass: Größenklasse des Gerüsts je Platzart = Breite des Gebäudes in Kacheln (B-586).
var scaffoldClass = map[string]string{
	"wall": bg + "geruest-1", "tower": bg + "geruest-3", "nursery": bg + "geruest-4", "stairsUp": bg + "geruest-4",
	"stairsDown": bg + "geruest-4", "workshop": bg + "geruest-6", "storage": bg + "geruest-6", "barracks": bg + "geruest-6",
	"tavern": bg + "geruest-6", "healer": bg + "geruest-6", "smithy": bg + "geruest-6", "armory": bg + "geruest-6",
	"ship": bg + "geruest-6",
}

// scaffoldLook ist das Gerüst der Platzart (unbekannte Art: Klasse 6).
func scaffoldLook(kind string) look {
	id, ok := scaffoldClass[kind]
	if !ok {
		id = bg + "geruest-6"
	}
	return look{id, "stand"}
}

// marker ist, was am ungebauten Platz steht: Gerüst im Bau, sonst Bauschild bei bezahlbarem Platz, nie beides.
type marker uint8

const (
	markNone marker = iota
	markSign
	markScaffold
)

// siteMarker: bezahlt, aber nicht fertig = Gerüst; ungebaut und bezahlbar = Bauschild (B-584, B-586).
func siteMarker(state string, payable bool) marker {
	switch {
	case state == "waitingMaterial" || state == "waitingWorker":
		return markScaffold
	case state != "built" && payable:
		return markSign
	}
	return markNone
}

// signVariant: Bauschild nach Art des Platzes (0 Siedlung, 1 Verteidigung, 2 Treppen).
func signVariant(kind string) int {
	switch kind {
	case "wall", "tower":
		return 1
	case "stairsUp", "stairsDown":
		return 2
	}
	return 0
}

// hubLook ist das Bild der Burg: Asset und Animation, Überlage der Ausbauschritte und das Gerüst davor.
type hubLook struct {
	main, overlay look
	scaffold      bool
}

// hubLookOf wählt den Look aus Hub-Stufe (1–5) und Ausbauschritt (1–3, GEB17.6): Hub-Stufe 1 Lagerfeuer, Zelt,
// Holzhütte (lagerfeuer 1–3), ab Stufe 2 die Burg der Stufe und in Schritt 2 und 3 ihr Schmuck als Überlage
// (burg-ausbau 2–3, D-MG6-05). Das Gerüst steht bei jedem waitingWorker (B-572); im Bau noch kein Schmuck.
func hubLookOf(hubLevel, hubStep int, upgradeState string) hubLook {
	level := min(max(hubLevel, 1), 5)
	step := min(max(hubStep, 1), 3)
	out := hubLook{scaffold: upgradeState == "waitingWorker"}
	if level == 1 {
		out.main = look{buildingGroup + "lagerfeuer", stageAnim(step)}
		return out
	}
	out.main = look{buildingGroup + "burg", stageAnim(level)}
	if step > 1 && !out.scaffold {
		out.overlay = look{buildingGroup + "burg-ausbau", stageAnim(step)}
	}
	return out
}

// hubScaffold ist das Baugerüst vor der Burg.
var hubScaffold = look{buildingGroup + "hub-geruest", "stand"}

// backWall ist ein Abschnitt Hintergrundmauer von from bis to (Units) in der Stufe der äußeren Mauer (B-442).
type backWall struct {
	id       int
	from, to float64
	stage    int
}

// backWalls hängt die Hintergrundmauern zwischen je zwei benachbarten gebauten Mauern derselben Seite an out an:
// links, dann rechts, je von innen nach außen. walls ist ein Puffer für die Mauern einer Seite.
func backWalls(out []backWall, walls []*sim.Site, sites []*sim.Site, hubX float64) ([]backWall, []*sim.Site) {
	for _, side := range [2]float64{-1, 1} {
		walls = walls[:0]
		for _, s := range sites {
			if s.Kind == "wall" && sign(s.X-hubX) == side {
				walls = append(walls, s)
			}
		}
		byDistance(walls, hubX)
		for i := 1; i < len(walls); i++ {
			in, ex := walls[i-1], walls[i]
			if in.State == "built" && ex.State == "built" {
				out = append(out, backWall{id: ex.ID, from: min(in.X, ex.X), to: max(in.X, ex.X), stage: max(1, ex.Level)})
			}
		}
	}
	return out, walls
}

// byDistance sortiert die Mauern einer Seite stabil nach Abstand zur Hub-Mitte (Einfügen, wenige Einträge, ohne
// Abschluss je Bild).
func byDistance(walls []*sim.Site, hubX float64) {
	for i := 1; i < len(walls); i++ {
		for j := i; j > 0 && math.Abs(walls[j].X-hubX) < math.Abs(walls[j-1].X-hubX); j-- {
			walls[j], walls[j-1] = walls[j-1], walls[j]
		}
	}
}

func sign(v float64) float64 {
	switch {
	case v < 0:
		return -1
	case v > 0:
		return 1
	}
	return 0
}

// backWallLook ist die Hintergrundmauer der Stufe 1–5; andere Stufen haben keine Grafik.
func backWallLook(stage int) look {
	if stage < 1 || stage > 5 {
		return look{}
	}
	return look{buildingGroup + "hintergrundmauer", stageAnim(stage)}
}

// damageStage: 0 unbeschädigt, 1 leicht (≥ 2/3 der HP), 2 mittel (≥ 1/3), 3 schwer; nicht gebaut oder ohne MaxHP 0.
func damageStage(state string, hp, maxHP float64) int {
	if state != "built" || maxHP <= 0 || hp >= maxHP {
		return 0
	}
	share := hp / maxHP
	switch {
	case share >= 2.0/3:
		return 1
	case share >= 1.0/3:
		return 2
	}
	return 3
}

// damageShade ist die Abdunklung je Schadensstufe (Anteil Schwarz, damageView.ts › SHADE).
var damageShade = [4]float32{0, 0.12, 0.25, 0.4}

// showRubble: Trümmer nur an einer gebauten Ruine unter der Hälfte ihrer HP.
func showRubble(ruin bool, state string, hp, maxHP float64) bool {
	return ruin && state == "built" && maxHP > 0 && hp/maxHP < 0.5
}
