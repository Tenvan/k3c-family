package props

// Natur und Objekte (Spiegel von src/scenes/render/natureSprites.ts, plantation.ts, worldSprites.ts, mineSignView.ts):
// Form aus der Position (gleiche Mischung wie im Browser, kein Zufall), Look aus dem Biom. Reine Auswahl.

import "math"

// mix ist die ganzzahlige Mischung der gerundeten Position (natureSprites.ts › mix, bitgleich mit Math.imul):
// gleiche Position, gleicher Wert.
func mix(x float64) uint32 {
	h := uint32(int32(int64(math.Floor(x+0.5))))*0x9e3779b1 ^ 0x85ebca6b
	h = (h ^ h>>16) * 0x85ebca6b
	h = (h ^ h>>13) * 0xc2b2ae35
	return h ^ h>>16
}

// animPhase ist der Startframe einer Schleife aus der Weltposition, damit Nachbarn nicht im Gleichtakt wiegen.
func animPhase(x float64, frames int) int {
	if frames <= 1 {
		return 0
	}
	return int(mix(x*7) % uint32(frames))
}

// natureLook ist der Look des Bioms: 1 Wald, 2 Höhle, 3 Mine und alle übrigen; leere ID = Wald.
func natureLook(biome string) int {
	switch biome {
	case "", "forest":
		return 1
	case "cave":
		return 2
	}
	return 3
}

var (
	treeKinds = [...]string{"tanne", "pappel", "birke"}
	rockKinds = [...]string{"felsen", "felsen-b", "felsen-c"}
	veinKinds = map[string]string{
		"stoneVein": "ader-stein", "copperVein": "ader-kupfer", "ironVein": "ader-eisen", "crystalVein": "ader-kristall",
	}
)

// idle ist die Animation aller Natur- und Objekt-Assets.
const idle = "idle"

// objectLooks: die Bilder aller Objekte, einmal angelegt (object entsteht je Bild ohne neuen String).
var objectLooks = map[string]look{}

func init() {
	for _, id := range objectAssets {
		objectLooks[id] = look{objectGroup + id, idle}
	}
	for _, ids := range natureAssets {
		for _, id := range ids {
			objectLooks[id] = look{objectGroup + id, idle}
		}
	}
}

// object ist das Bild des Objekts id (Animation idle).
func object(id string) look {
	if l, ok := objectLooks[id]; ok {
		return l
	}
	return look{objectGroup + id, idle}
}

// treeLook: im Wald Art aus der Position (Tanne, Pappel, Birke), außerhalb des Walds Wurzeln statt Bäumen (GEB10;
// die fünf Wurzel-Formen zeigen als Papier dasselbe Asset, B-701 D-US2-01).
func treeLook(x float64, biome string) look {
	if natureLook(biome) != 1 {
		return object("wurzel")
	}
	return object(treeKinds[mix(x)%uint32(len(treeKinds)*3)%uint32(len(treeKinds))])
}

// rockLook: Fels-Form aus der Position (GEB15.2).
func rockLook(x float64) look { return object(rockKinds[mix(x)%uint32(len(rockKinds))]) }

// nodeLook ist das Bild eines Ressourcen-Knotens; unbekannte Art = keins (Platzhalter).
func nodeLook(kind string, x float64, biome string) look {
	switch kind {
	case "tree":
		return treeLook(x, biome)
	case "rock":
		return rockLook(x)
	case "copperOre":
		return object("erzbrocken")
	case "bush":
		return bushLook(biome)
	}
	if id, ok := veinKinds[kind]; ok {
		return object(id)
	}
	return look{}
}

// bushLook: Busch im Wald, Höhlenbusch sonst.
func bushLook(biome string) look {
	if natureLook(biome) == 1 {
		return object("busch")
	}
	return object("hoehlenbusch")
}

// decoRoots hängt die Versätze (Units) der Deko-Wurzeln eines Baum-Knotens an out an: außerhalb des Walds
// rootsPerNode−1 Stück, ±0,4–1,2 Units, Seite wechselnd (natureSprites.ts › decoRoots); im Wald keine.
func decoRoots(out []float64, x float64, biome string, rootsPerNode int) []float64 {
	if natureLook(biome) == 1 {
		return out
	}
	for i := range max(0, rootsPerNode-1) {
		h := mix(x + float64(i+1)*7919)
		side := -1.0
		if ((h>>4)+uint32(i))&1 != 0 {
			side = 1
		}
		out = append(out, side*float64(4+(h>>8)%9)/10)
	}
	return out
}

// mineSignLook: Abbau-Schild am markierten Knoten, Axt am Baum, Spitzhacke an Fels, Erz und Ader; sonst keins.
func mineSignLook(kind string) look {
	switch {
	case kind == "tree":
		return object("schild-axt")
	case kind == "rock" || kind == "copperOre" || veinKinds[kind] != "":
		return object("schild-spitzhacke")
	}
	return look{}
}

// plotTolerance: Abstand, bis zu dem ein Baum auf einem Plantage-Platz steht (plantation.ts).
const plotTolerance = 0.01

// plotX ist der Ort von Platz i der Plantage um die Baumschule bei farmX (plotX in engine/sim/building_plantation.go).
func plotX(farmX float64, i, slots int, strip float64) float64 {
	if slots < 2 {
		return farmX
	}
	return farmX - strip + float64(i)*2*strip/float64(slots-1)
}

// onPlot meldet, ob x ein Platz der Plantage um farmX ist.
func onPlot(x, farmX float64, slots int, strip float64) bool {
	for i := range slots {
		if math.Abs(x-plotX(farmX, i, slots, strip)) < plotTolerance {
			return true
		}
	}
	return false
}

// entityLook ist das Bild einer festen Entität des Levels (portal, exit, harbor, recruitCamp, bush); dx ist ihr
// Versatz in Units (der Hafen-Steg steht links vom Schiffs-Platz), ok = sie wird gezeichnet.
func entityLook(kind, biome string) (l look, dx float64, ok bool) {
	switch kind {
	case "portal":
		return object("portal"), 0, true
	case "exit":
		return object("ausgang"), 0, true
	case "bush":
		return bushLook(biome), 0, true
	case "harbor":
		return look{buildingGroup + "hafen", "stand"}, -harborShift, true
	case "recruitCamp":
		return look{buildingGroup + "rekrutierungslager", stageAnim(natureLook(biome))}, 0, true
	}
	return look{}, 0, false
}

// harborShift: Der Hafen-Steg steht so viele Kacheln links vom Schiffs-Platz (BS10).
const harborShift = 5.5

// pickupLook: Truhe geschlossen am Boden, Stern 44 px darüber; lift in Units.
func pickupLook(kind string) (l look, lift float64) {
	switch kind {
	case "chest":
		return object("truhe"), 0
	case "skillPoint":
		return object("stern"), 44.0 / unitPx
	}
	return look{}, 0
}

var (
	coinLook      = look{"icons/muenze", idle}
	chestOpenLook = object("truhe-offen")
	seedlingLook  = object("setzling")
	plantTreeLook = object("pappel-niedrig")
)
