package menu

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// fakeWorlds sind Weltbilder ohne Grafik: standard und sengoku mit Bildern, die Welt "ohne" hat keinen Eintrag.
var fakeWorlds = art.WorldImages{
	"standard": {Big: "g/std", Small: "g/std-k"},
	"sengoku":  {Big: "g/sen", Small: "g/sen-k"},
}

var threeWorlds = []view.WorldInfo{{ID: "standard", Name: "Standard"}, {ID: "sengoku", Name: "Sengoku"}, {ID: "ohne", Name: "Ohne"}}

func worldMenus(has func(string) bool) *menus {
	m := newMenus()
	m.worlds, m.has = fakeWorlds, has
	return m
}

// B-938 AC-01: „Neues Spiel“ öffnet zuerst die Weltauswahl; B geht zurück, A öffnet die Lobby mit der Welt.
func TestWorldPickIsFirstStepOfNewGame(t *testing.T) {
	s := &fakeSession{worlds: threeWorlds}
	main := newMain(newMenus())
	main.Update(view.Input{}, s)
	main.sel = mainNew
	st := main.Update(press(pad1, view.ActionConfirm), s)
	pick, ok := st.Next.(*worldPickScreen)
	if st.Kind != view.StepPush || !ok {
		t.Fatalf("Schritt %+v", st)
	}
	if st := pick.Update(press(pad1, view.ActionBack), s); st.Kind != view.StepPop || !hasCue(st, view.CueBack) {
		t.Errorf("B: %+v", st)
	}
	stepThrough(pick, s, press(pad1, view.ActionRight))
	st = pick.Update(press(pad2, view.ActionConfirm), s)
	lb, ok := st.Next.(*lobbyScreen)
	if st.Kind != view.StepPush || !ok || lb.world != "sengoku" || lb.devs[0] != pad2 {
		t.Fatalf("A: %+v", st)
	}
	lb.row = lobbyStart
	if st := lb.Update(press(pad2, view.ActionConfirm), s); st.Kind != view.StepPlay || s.news[0].World != "sengoku" {
		t.Errorf("Start: %+v, Spiele %+v", st, s.news)
	}
}

// Links/Rechts blättert im Kreis; ohne weitere Welten gibt es keine Pfeile, die Liste ist nie leer.
func TestWorldPickBrowses(t *testing.T) {
	s := &fakeSession{worlds: threeWorlds}
	sc := newWorldPick(worldMenus(nil), pad1)
	stepThrough(sc, s, press(pad1, view.ActionLeft))
	if sc.sel != 2 {
		t.Errorf("links vom Anfang: %d", sc.sel)
	}
	stepThrough(sc, s, press(pad1, view.ActionRight))
	if sc.sel != 0 {
		t.Errorf("rechts vom Ende: %d", sc.sel)
	}
	one := newWorldPick(worldMenus(nil), pad1)
	one.Update(view.Input{}, &fakeSession{})
	one.fill("de")
	if h := one.pg.hero; !h.on || h.left || h.right || h.name != "Standard" || h.art != "g/std" {
		t.Errorf("eine Welt: %+v", h)
	}
	empty := newWorldPick(worldMenus(nil), pad1)
	empty.Update(view.Input{}, &fakeSession{worlds: []view.WorldInfo{}})
	if len(empty.list) != 1 {
		t.Errorf("leere Liste: %v", empty.list)
	}
}

// Bild der Welt, sonst das der Standard-Welt, sonst reiner Text (B-937).
func TestWorldAssetFallback(t *testing.T) {
	has := map[string]bool{"g/std": true, "g/std-k": true, "g/sen": true}
	m := worldMenus(func(a string) bool { return has[a] })
	cases := []struct {
		world string
		big   bool
		want  string
	}{
		{"sengoku", true, "g/sen"},
		{"sengoku", false, "g/std-k"}, // kleines Bild fehlt: Standard
		{"ohne", true, "g/std"},       // keine Zuordnung: Standard
		{"", false, "g/std-k"},        // Spielstand ohne Welt
	}
	for _, c := range cases {
		if got := m.worldAsset(c.world, c.big); got != c.want {
			t.Errorf("%q groß=%v: %q, erwartet %q", c.world, c.big, got, c.want)
		}
	}
	none := worldMenus(func(string) bool { return false })
	if got := none.worldAsset("sengoku", true); got != "" {
		t.Errorf("ohne Bilder: %q", got)
	}
}

// B-938 AC-02: Spielstand-Liste und Pause-Menü tragen das kleine Bild der Welt.
func TestSmallWorldImageInSavesAndPause(t *testing.T) {
	m := worldMenus(nil)
	a, b := save("a", 1, true), save("b", 2, true)
	b.World = "sengoku"
	bad := save("c", 3, false)
	sv := newSaves(m)
	sv.Update(view.Input{}, &fakeSession{saves: []view.SaveInfo{a, b, bad}})
	sv.fill("de")
	want := []string{"g/std-k", "g/sen-k", ""}
	for i, w := range want {
		if got := sv.pg.rows[i].art; got != w {
			t.Errorf("Spielstand %d: %q, erwartet %q", i, got, w)
		}
	}
	p := newPause(m)
	p.Update(view.Input{}, &fakeSession{world: "sengoku"})
	p.fill("de")
	if got := p.pg.rows[pauseResume].art; got != "g/sen-k" {
		t.Errorf("Pause: %q", got)
	}
	p2 := newPause(m)
	p2.Update(view.Input{}, &fakeSession{})
	p2.fill("de")
	if got := p2.pg.rows[pauseResume].art; got != pauseArt[pauseResume] {
		t.Errorf("Pause ohne Welt: %q", got)
	}
}

// Zwei Spieler: Die Weltauswahl bedient jedes Gerät; das bestätigende wird Spieler 1 der Lobby. Blättern allokiert nicht.
func TestWorldPickTwoPlayersAndNoAllocs(t *testing.T) {
	s := &fakeSession{worlds: threeWorlds}
	sc := newWorldPick(worldMenus(nil), pad1)
	sc.Update(view.Input{}, s)
	l, r := press(pad1, view.ActionLeft), press(pad2, view.ActionRight)
	if n := testing.AllocsPerRun(50, func() { sc.Update(l, s); sc.Update(r, s) }); n > 0 {
		t.Errorf("%v Allokationen je Update", n)
	}
	if st := sc.Update(press(pad2, view.ActionConfirm), s); st.Next.(*lobbyScreen).devs[0] != pad2 {
		t.Error("Spieler 1 ist nicht das bestätigende Gerät")
	}
}

// B-937: Weltbilder sind weich. Nur ladbare Bilder stehen in der Liste; ohne Bibliothek oder ohne ladbares Bild bleiben
// nur die Karten-Illustrationen, das Laden eines Levels scheitert nie an einem Weltbild.
func TestWorldImagesAreSoftAssets(t *testing.T) {
	only := func(a string) bool { return a == "g/std-k" }
	got := artList(fakeWorlds, true, only)
	if !slices.Contains(got, "g/std-k") || slices.Contains(got, "g/sen") || slices.Contains(got, "g/std") {
		t.Errorf("nur ladbare Bilder: %v", got)
	}
	base := artList(nil, true, nil)
	if got := artList(fakeWorlds, true, func(string) bool { return false }); !slices.Equal(got, base) {
		t.Errorf("ohne ladbare Bilder: %v, erwartet %v", got, base)
	}
	if got := Art(nil); slices.Contains(got, "hintergruende/weltbild-standard") {
		t.Errorf("ohne Bibliothek: %v", got)
	}
	var lib *art.Library
	if lib.Usable("hintergruende/weltbild-standard") {
		t.Error("nil-Bibliothek lädt")
	}
	m := worldMenus(nil)
	m.usable = only
	if a := m.Assets("wald", nil); !slices.Contains(a, "g/std-k") || slices.Contains(a, "g/sen-k") {
		t.Errorf("Assets je Level: %v", a)
	}
}

// Die gerenderten Weltbilder (Rig hintergrund) tragen die Animation "stand" und sind ladbar; ohne gen:paper (CI) übersprungen.
func TestRenderedWorldImagesUsable(t *testing.T) {
	lib, err := art.New(art.Config{GrafikDir: "../../../public/grafik", FontPath: "fehlt.ttf"})
	if err != nil {
		t.Fatal(err)
	}
	for id, img := range defaultWorlds {
		for _, a := range []string{img.Big, img.Small} {
			m, err := art.ReadManifest("../../../public/grafik", a)
			if err != nil {
				t.Skipf("%s nicht gerendert: %v", id, err)
			}
			if !slices.Contains(m.AnimNames(), worldAnim) || !lib.Usable(a) {
				t.Errorf("%s: Animationen %v, ladbar %v", a, m.AnimNames(), lib.Usable(a))
			}
		}
	}
}
