package hud

// Tasten-Icons im HUD (B-911, NA1.5): Jeder Glyph-Schlüssel hat ein Papierschnitt-Icon aus data/paper.json, ohne Icon
// bleibt die gemalte Glyphe; Kappen wachsen mit der Beschriftung, Bedienhinweise stehen als Zeile mit Icons.

import (
	"encoding/json"
	"os"
	"slices"
	"strings"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
)

// paperIcons liest die Assets der Gruppe icons aus data/paper.json (`icons/<id>`).
func paperIcons(t *testing.T) map[string]bool {
	t.Helper()
	raw, err := os.ReadFile("../../../data/paper.json")
	if err != nil {
		t.Fatal(err)
	}
	var root map[string]json.RawMessage
	if err := json.Unmarshal(raw, &root); err != nil {
		t.Fatal(err)
	}
	out := map[string]bool{}
	for _, part := range root {
		var entries map[string]json.RawMessage
		if json.Unmarshal(part, &entries) != nil {
			continue
		}
		for id, e := range entries {
			var a struct{ Gruppe string }
			if json.Unmarshal(e, &a) == nil && a.Gruppe == "icons" {
				out["icons/"+id] = true
			}
		}
	}
	return out
}

// allGlyphs ruft fn für jede Glyph aus glyphOf über alle Aktionen, Eingabe-Profile und Sprachen.
func allGlyphs(fn func(lang, profile, action string, g glyph)) {
	actions := append(slices.Clone(KeyActions[:]), glyphActions[:]...)
	for _, lang := range []string{"de", "en"} {
		for _, p := range []string{wasd, arrows, gp} {
			for _, a := range actions {
				fn(lang, p, a, glyphOf(lang, a, p))
			}
		}
	}
}

// AC-05 (B-911/AC-01): Für jeden Glyph-Schlüssel aus glyphOf über alle Aktionen, Profile und Sprachen nennt IconAsset
// ein Icon, das in data/paper.json steht und eine Spec in public/grafik/icons hat; das HUD meldet genau diese Icons.
func TestJederGlyphSchluesselHatEinIcon(t *testing.T) {
	icons := paperIcons(t)
	used := map[string]bool{}
	allGlyphs(func(lang, p, a string, g glyph) {
		if g.key == "" {
			if g.icon != "" {
				t.Errorf("%s/%s/%s: Icon %q ohne Schlüssel", lang, p, a, g.icon)
			}
			return
		}
		if g.icon == "" || g.icon != IconAsset(g.key, g.label) || !icons[g.icon] {
			t.Errorf("%s/%s/%s: Schlüssel %q, Icon %q nicht in data/paper.json", lang, p, a, g.key, g.icon)
			return
		}
		spec := "../../../public/grafik/" + g.icon + ".paper.json"
		if _, err := os.Stat(spec); err != nil {
			t.Errorf("%s: Spec fehlt: %v", g.icon, err)
		}
		used[g.icon] = true
	})
	for _, name := range padGlyphs { // auch Tasten mit Bild ohne Aktion (Y, RT)
		a := IconAsset("pad:"+name, name)
		if !icons[a] {
			t.Errorf("pad:%s: Icon %q nicht in data/paper.json", name, a)
		}
		used[a] = true
	}
	for a := range used {
		if !slices.Contains(keyAssets, a) {
			t.Errorf("Icon %q ist nicht als Asset gemeldet (keyAssets)", a)
		}
	}
	if len(used) != len(keyAssets) || !slices.IsSorted(keyAssets) {
		t.Errorf("keyAssets %v, gebraucht %d", keyAssets, len(used))
	}
}

// Die Kappe folgt der Länge der Beschriftung, nicht dem Tastennamen: ein Zeichen schmal, sonst breit, je Sprache.
func TestKappeNachBeschriftung(t *testing.T) {
	cases := []struct{ key, label, want string }{
		{"key:E", "E", capNarrow}, {"key:Z", "Ü", capNarrow}, {"key:F11", "F11", capWide}, {"key:SPACE", "Leertaste", capWide},
		{"key:SPACE", "␣", capNarrow}, {"key:CONTROLRIGHT", "Strg rechts", capWide}, {"pad:LB", "LB", "icons/xbox-lb"},
		{"pad:UP", "D-Pad ↑", "icons/xbox-up"}, {"", "dance", ""}, {"key:", "x", ""},
	}
	for _, c := range cases {
		if got := IconAsset(c.key, c.label); got != c.want {
			t.Errorf("IconAsset(%q, %q) = %q, erwartet %q", c.key, c.label, got, c.want)
		}
	}
	if g := glyphOf("de", "skill2", arrows); g.icon != capWide || g.label != "Entf" {
		t.Errorf("Profil pfeile, Deutsch: %+v", g)
	}
	if g := glyphOf("de", "skill1", gp); g.icon != "icons/xbox-lb" {
		t.Errorf("Pad: %+v", g)
	}
}

// Kappen-Maße: schmal bleibt quadratisch, breit wächst mit der Beschriftung bis capWideMax, danach schrumpft die
// Schrift; die Beschriftung bleibt immer in der Textfläche (nichts läuft über).
func TestKappeWaechstUndSchrumpft(t *testing.T) {
	const gh = 40.0
	for _, tw := range []float64{0, 8, 20, 29, 60, 100, 121, 122, 200, 900} {
		checkCapBox(t, gh, tw, false)
		checkCapBox(t, gh, tw, true)
	}
	if w, px := capBox(gh, 30, true, false); w != gh*capWideMin || px != gh*glyphLabelK {
		t.Errorf("kurze Beschriftung: Breite %v, Schrift %v", w, px)
	}
	if w, px := capBox(gh, 100, true, false); w != 100+gh*capWideMargin || px != gh*glyphLabelK {
		t.Errorf("lange Beschriftung streckt die Kappe: Breite %v, Schrift %v", w, px)
	}
	if w, px := capBox(gh, 900, true, true); w != 900+gh*capWideMargin || px != gh*glyphLabelK {
		t.Errorf("Debug-Schrift: Breite %v, Schrift %v", w, px)
	}
}

// checkCapBox prüft, dass die Beschriftung der Breite tw in der Textfläche der Kappe bleibt und die Kappe ihre Grenzen hält.
func checkCapBox(t *testing.T, gh, tw float64, wide bool) {
	t.Helper()
	w, px := capBox(gh, tw, wide, false)
	text, lo, hi := gh*capNarrowText, gh, gh
	if wide {
		text, lo, hi = w-gh*capWideMargin, gh*capWideMin, gh*capWideMax
	}
	if drawn := tw * px / (gh * glyphLabelK); drawn > text+1e-9 || px > gh*glyphLabelK || px <= 0 {
		t.Errorf("tw %v breit %v: Beschriftung %v in Textfläche %v, Schrift %v", tw, wide, drawn, text, px)
	}
	if w < lo || w > hi {
		t.Errorf("tw %v breit %v: Breite %v", tw, wide, w)
	}
}

// Bedienhinweis als Zeile: Text und Glyphen im Wechsel, der Text (plain) wie vor den Icons; am Pad bleibt B Text.
func TestBedienhinweisAlsZeile(t *testing.T) {
	kinds := func(l keyLine) string {
		var b strings.Builder
		for i := range l.n {
			if g := l.segs[i].g; g.key != "" {
				b.WriteString("<" + g.key + ">")
			} else {
				b.WriteString("[" + g.label + "]")
			}
		}
		return b.String()
	}
	cases := []struct {
		line        keyLine
		plain, segs string
	}{
		{menuLine("de", wasd), "A / D wählen · Leertaste bestätigen · K schließen",
			"<key:A>[/]<key:D>[wählen ·]<key:SPACE>[bestätigen ·]<key:K>[schließen]"},
		{menuLine("de", gp), "◄ ► wählen · A bestätigen · B schließen", "[◄ ► wählen ·]<pad:A>[bestätigen · B schließen]"},
		{seaLine("en", arrows, true), "Left arrow / Right arrow choose destination · Enter confirm",
			"<key:LEFT>[/]<key:RIGHT>[choose destination ·]<key:ENTER>[confirm]"},
		{seaLine("de", gp, false), "A bestätigen", "<pad:A>[bestätigen]"},
		{newKeyLine("{x} und {ok}", arg("ok", newGlyph("key:E", "E"))), "{x} und E", "[{x} und]<key:E>"},
	}
	for _, c := range cases {
		if c.line.plain != c.plain || kinds(c.line) != c.segs {
			t.Errorf("Zeile %q: %s, erwartet %q: %s", c.line.plain, kinds(c.line), c.plain, c.segs)
		}
	}
	parts := make([]glyph, 0, 2*lineSegs)
	for range lineSegs {
		parts = append(parts, newGlyph("key:E", "E"), textPart(" x "))
	}
	if l := newKeyLine("{k}", arg("k", parts...)); l.n != 0 || !strings.HasPrefix(l.plain, "E x E x") {
		t.Errorf("zu viele Teile: n %d, Text %q", l.n, l.plain)
	}
}

// AC-06 (B-911/AC-02): Ohne Grafik, mit nicht ladbaren oder nicht geladenen Icons zeichnet das HUD die gemalte Glyphe
// (Breite wie vor den Icons) und den Bedienhinweis als Text; nicht ladbare Icons meldet es nicht als Asset.
func TestOhneIconGemalteGlyphe(t *testing.T) {
	check := checkPainted
	check(t, "ohne Grafik", New(nil).(*hud))
	soft := &hud{}
	soft.useKeyIcons(func(a string) bool { return a != "icons/xbox-rs" })
	if soft.keyIcons || !slices.Equal(soft.Assets("forest", nil), hudAssets) {
		t.Errorf("nicht ladbares Icon: gemeldet %v", soft.Assets("forest", nil))
	}
	check(t, "Icon nicht ladbar", soft)
	soft.useKeyIcons(func(string) bool { return true })
	if got := soft.Assets("forest", nil); !slices.IsSorted(got) || len(got) != len(hudAssets)+len(keyAssets) || !slices.Contains(got, capWide) {
		t.Errorf("ladbar: Assets %v", got)
	}
	h := seaLib(t) // mit Grafik, aber Level ohne die Tasten-Icons
	if err := h.lib.SetLevels(map[string][]string{"forest": hudAssets}); err != nil {
		t.Fatal(err)
	}
	check(t, "Icons nicht geladen", h)
}

// paintedWidth ist die Breite der gemalten Glyph g bei Höhe gh wie vor den Icons: Kappe um die Beschriftung
// (Tastatur, Schultertasten), sonst ein Quadrat.
func paintedWidth(font *art.Font, g glyph, gh float64) float64 {
	_, pad, _ := strings.Cut(g.key, ":")
	label := g.label
	switch {
	case isKeyGlyph(g):
	case isShoulder(pad):
		label = pad
	default:
		return gh
	}
	tw, _ := font.Measure(label, gh*0.5)
	return max(gh, tw+gh*0.5)
}

// checkPainted prüft, dass h jede Glyph malt (kein Icon, Breite wie vor den Icons) und den Bedienhinweis als Text lässt.
func checkPainted(t *testing.T, name string, h *hud) {
	t.Helper()
	const gh = 36.0
	dst := ebiten.NewImage(256, 64)
	font, _ := art.LoadFont(repoFont, false)
	h.paint.stencils()
	allGlyphs(func(lang, p, a string, g glyph) {
		if g.key == "" {
			return
		}
		if h.glyphIcon(g) != nil {
			t.Fatalf("%s: %s hat ein Icon", name, g.key)
		}
		want := paintedWidth(font, g, gh)
		if w := h.drawGlyph(dst, font, 0, slotMenuGlyph, g, 0, 32, gh); w != want || h.glyphWidth(font, 0, slotMenuCap, g, gh) != want {
			t.Errorf("%s: %s/%s/%s gemalt %v breit, erwartet %v", name, lang, p, a, w, want)
		}
	})
	l := menuLine("de", arrows)
	if _, ok := h.lineWidth(font, &l, 24); ok || h.lineIcons(&l) || h.drawLine(dst, font, &l, 0, 0, 24, white) {
		t.Errorf("%s: Bedienhinweis mit Icons", name)
	}
}

// Mit geladenen Icons zeichnet das HUD sie: jede Glyph hat ihr Sheet, zwei Spieler mit verschiedenen Profilen je ihre
// Glyphen, die lange Beschriftung liegt auf der gestreckten Kappe; Messen und Zeilen rechnen ohne Allokation je Bild.
func TestGlyphenMitIcons(t *testing.T) {
	h := seaLib(t)
	f := h.font()
	if !h.keyIcons || f == nil {
		t.Fatal("Tasten-Icons nicht gemeldet")
	}
	allGlyphs(func(lang, p, a string, g glyph) {
		if g.key != "" && h.glyphIcon(g) == nil {
			t.Errorf("%s/%s/%s: Icon %q nicht geladen", lang, p, a, g.icon)
		}
	})
	es := menuEntries(nil, []string{"taunt"}, 1, only("heal"), true)
	slots := slotViews([]string{"fireball", "taunt", "", ""}, nil)
	h.seats[0].menuV = buildMenu(newMenuKey("de", arrows, 0, 1, es, slots), es)
	h.seats[1].menuV = buildMenu(newMenuKey("de", gp, 0, 1, es, slots), es)
	icon := func(p int) string {
		for _, r := range h.seats[p].menuV.rows {
			if r.glyph.key != "" {
				return r.glyph.icon
			}
		}
		return ""
	}
	if icon(0) != capWide || !strings.HasPrefix(icon(1), "icons/xbox-") {
		t.Errorf("zwei Spieler: Icons %q und %q", icon(0), icon(1))
	}
	const gh = 36.0
	long := glyphOf("de", "attack", arrows) // „Strg rechts“
	if w := h.glyphWidth(f, 0, slotMenuCap, long, gh); w < gh*capWideMin || h.glyphWidth(f, 0, slotMenuCap, glyphOf("de", "attack", wasd), gh) != gh {
		t.Errorf("Kappen-Breite lang %v", w)
	}
	checkLines(t, h, long, gh)
}

// checkLines prüft die Bedienhinweise beider Spieler: mit Icons, breiter als nichts, Messen ohne Allokation je Bild.
func checkLines(t *testing.T, h *hud, long glyph, gh float64) {
	t.Helper()
	f := h.font()
	for p := range 2 {
		l := &h.seats[p].menuV.line
		if w, ok := h.lineWidth(f, l, 24); !ok || w <= 0 {
			t.Errorf("Spieler %d: Zeile %v breit (%v)", p, w, ok)
		}
	}
	if n := testing.AllocsPerRun(50, func() {
		for p := range 2 {
			l := &h.seats[p].menuV.line
			_, _ = h.lineWidth(f, l, 24)
			_ = h.lineIcons(l)
			_ = h.glyphWidth(f, p, slotMenuCap, long, gh)
			_ = h.glyphIcon(long)
		}
	}); n != 0 {
		t.Errorf("%v Allokationen je Bild", n)
	}
}
