package props

// Papier-Assets je Level (view.AssetSource): alles, was props im Level zeichnen kann, auch wenn es gerade nicht
// sichtbar ist. Gebäude, Gerüste, Burg und Objekte immer; die Natur nach dem Look des Bioms (Wald: Bäume und Busch,
// sonst Wurzeln und Höhlenbusch). Fehlt eine Datei, beendet game das Spiel mit dem Pfad (art.ErrMissing).

import "slices"

// buildingAssets: alle Gebäude-Assets, die siteLook, hubLookOf, scaffoldLook, backWallLook und entityLook wählen.
var buildingAssets = []string{
	"burg", "burg-ausbau", "lagerfeuer", "hub-geruest", "hintergrundmauer", "mauer", "turm", "schiff",
	"werkstatt", "taverne", "heilplatz", "schmiede", "ruestkammer", "lager", "farm", "kaserne",
	"treppe-hoch", "treppe-runter", "hafen", "rekrutierungslager",
	"geruest-1", "geruest-3", "geruest-4", "geruest-6",
}

// objectAssets: Objekte in jedem Biom (Felsen, Erz, Adern, Abbau-Schilder, Plantage, Portal, Ausgang, Truhen, Stern).
var objectAssets = []string{
	"felsen", "felsen-b", "felsen-c", "erzbrocken", "ader-stein", "ader-kupfer", "ader-eisen", "ader-kristall",
	"schild-axt", "schild-spitzhacke", "setzling", "pappel-niedrig", "portal", "ausgang", "truhe", "truhe-offen", "stern",
}

// natureAssets: Natur je Look (1 Wald, sonst Höhle und Mine).
var natureAssets = map[bool][]string{
	true:  {"tanne", "pappel", "birke", "busch"},
	false: {"wurzel", "hoehlenbusch"},
}

// levelAssets nennt die Assets `gruppe/id` des Levels (Biom-ID), sortiert und jedes einmal.
func levelAssets(level string) []string {
	out := make([]string, 0, len(buildingAssets)+len(objectAssets)+5)
	for _, id := range buildingAssets {
		out = append(out, buildingGroup+id)
	}
	for _, id := range objectAssets {
		out = append(out, objectGroup+id)
	}
	for _, id := range natureAssets[natureLook(level) == 1] {
		out = append(out, objectGroup+id)
	}
	out = append(out, coinLook.asset)
	slices.Sort(out)
	return slices.Compact(out)
}
