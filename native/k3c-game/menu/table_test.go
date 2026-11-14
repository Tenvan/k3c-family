package menu

import (
	"image"
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// fakeKeys ist eine Tastenbelegung mit einer Zeile je view.KeyActions (wie game.KeyRows).
func fakeKeys(lang string) []view.KeyRow {
	out := make([]view.KeyRow, 0, len(view.KeyActions))
	for _, a := range view.KeyActions {
		out = append(out, view.KeyRow{Action: a, Left: "L-" + a, Right: "R-" + a, Pad: "P-" + lang})
	}
	return out
}

// Seite Tasten (B-807 AC-01): erreichbar über das Pause-Menü, eine Zeile je Aktion der Belegung mit beschrifteter
// Aktion, B, Esc (Back und Menu zugleich) und Home führen zurück ins Pause-Menü.
func TestKeysPage(t *testing.T) {
	m := New(nil, fakeKeys).(*menus)
	pause := newPause(m)
	s := &fakeSession{active: true, seats: []view.Seat{{Player: 0, Device: keys1}}}
	st := stepThrough(pause, s, press(keys1, view.ActionDown), press(keys1, view.ActionDown), press(keys1, view.ActionConfirm))
	keys, ok := st.Next.(*tableScreen)
	if st.Kind != view.StepPush || !ok || keys.kind != tableKeys {
		t.Fatalf("Pause › Tasten: %+v", st)
	}
	keys.fill("de")
	if keys.tb.n != len(view.KeyActions) {
		t.Fatalf("%d Zeilen, erwartet je Aktion eine (%d)", keys.tb.n, len(view.KeyActions))
	}
	for i, a := range view.KeyActions {
		r := keys.tb.rows[i]
		if r[0] == a || r[0] == "" || r[1] != "L-"+a || r[2] != "R-"+a || r[3] != "P-de" {
			t.Errorf("Zeile %d (%s): %q", i, a, r)
		}
	}
	back := []view.Input{
		press(pad1, view.ActionBack),
		{Devices: []view.DeviceInput{{Device: keys1, Actions: []view.Action{view.ActionBack, view.ActionMenu}}}},
		press(pad1, view.ActionHome),
	}
	for _, in := range back {
		if st := newKeys(m).Update(in, s); st.Kind != view.StepPop || !hasCue(st, view.CueBack) {
			t.Errorf("zurück ins Pause-Menü: %+v", st)
		}
	}
	if st := newKeys(m).Update(press(pad1, view.ActionDown), s); st.Kind != view.StepStay {
		t.Errorf("Hoch/Runter bleibt: %+v", st)
	}
}

// Lizenz-Seite (B-808 AC-01): nennt Ebitengine, die Schrift (Go Regular) und Audio; die Lizenzdatei kommt aus
// derselben Liste.
func TestLicensesPage(t *testing.T) {
	sc := newLicenses(newMenus())
	for _, lang := range []string{"de", "en"} {
		sc.fill(lang)
		var all strings.Builder
		for _, r := range sc.tb.rows[:sc.tb.n] {
			all.WriteString(strings.Join(r[:], " ") + "\n")
		}
		text := all.String()
		for _, want := range []string{"Ebitengine", "Go Regular", tr(lang, txtLicFont), tr(lang, txtLicAudio), "go-mp3", "CC0"} {
			if !strings.Contains(text, want) {
				t.Errorf("%s: Lizenz-Seite ohne %q:\n%s", lang, want, text)
			}
		}
	}
	file := LicenseText()
	for _, l := range licenses {
		if !strings.Contains(file, l.work+" – "+l.holder+" – "+l.license) {
			t.Errorf("Lizenzdatei ohne %s", l.work)
		}
	}
	st := stepThrough(newMenus().Main(), &fakeSession{}, press(pad1, view.ActionDown), press(pad1, view.ActionDown),
		press(pad1, view.ActionDown), press(pad1, view.ActionConfirm))
	if l, ok := st.Next.(*tableScreen); st.Kind != view.StepPush || !ok || l.kind != tableLicenses {
		t.Errorf("Hauptmenü › Lizenzen: %+v", st)
	}
}

// Version im Hauptmenü (B-808 AC-02): unter dem Titel, ohne Angabe „unbekannt“.
func TestMainShowsVersion(t *testing.T) {
	old := Version
	defer func() { Version = old }()
	for _, c := range []struct{ v, want string }{{"", "Version unbekannt"}, {"v0.9-3-gabc", "Version v0.9-3-gabc"}} {
		Version = c.v
		sc := newMenus().Main().(*mainScreen)
		if sc.fill("de"); sc.pg.sub != c.want {
			t.Errorf("Version %q: %q, erwartet %q", c.v, sc.pg.sub, c.want)
		}
	}
	if Version = ""; !strings.Contains(LicenseText(), "Version unbekannt") {
		t.Error("Lizenzdatei ohne Version")
	}
}

// Erstes Bild nach „Spiel verlassen“ (B-813 AC-03): Das Hauptmenü kennt den Spielstand schon vor seinem ersten Update;
// ein Hauptmenü ohne gelesene Liste zeigt „lädt“, nie „Kein Spielstand vorhanden“.
func TestMainFirstDrawAfterLeave(t *testing.T) {
	s := &fakeSession{active: true, saves: []view.SaveInfo{save("burg", 3, true)}, seats: []view.Seat{{Player: 0, Device: pad1}}}
	pause := newPause(newMenus())
	pause.sel = pauseLeave
	st := pause.Update(press(pad1, view.ActionConfirm), s)
	main, ok := st.Next.(*mainScreen)
	if st.Kind != view.StepReplace || !ok {
		t.Fatalf("Verlassen: %+v", st)
	}
	main.fill("de")
	if v := main.pg.rows[mainContinue].value; !strings.Contains(v, "burg") || main.pg.rows[mainContinue].off {
		t.Errorf("erstes Bild: Weiter %q (aus %v)", v, main.pg.rows[mainContinue].off)
	}
	fresh := newMenus().Main().(*mainScreen)
	fresh.fill("de")
	if v := fresh.pg.rows[mainContinue].value; v != tr("de", txtContinueLoading) || fresh.pg.rows[mainContinue].off {
		t.Errorf("vor dem ersten Update: %q", v)
	}
}

// Die Tafel liegt im Fenster unter dem Titel und über der Meldung; die Zeilen überlappen nicht.
func TestTableGeometry(t *testing.T) {
	for _, win := range []image.Rectangle{image.Rect(0, 0, 1280, 720), image.Rect(0, 0, 1920, 1080), image.Rect(0, 0, 640, 360)} {
		g := geomOf(win)
		tg := tableOf(g, len(view.KeyActions))
		prevBottom := -1.0
		for i := range len(view.KeyActions) + 1 {
			x, y, w, h := tg.row(i)
			if x < 0 || x+w > float64(win.Dx()) || y < g.titleY+g.titlePx || y+h > g.noteY+1e-9 || y < prevBottom-1e-9 || tg.px > h {
				t.Errorf("%v: Zeile %d bei %v/%v %vx%v (Schrift %v)", win, i, x, y, w, h, tg.px)
			}
			prevBottom = y + h
		}
	}
}
