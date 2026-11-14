package hud

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// settingsSession erfüllt view.Session für die Anleitung: Einstellungen lesen und schreiben, der Rest ist unbenutzt.
type settingsSession struct {
	view.Session
	cur  view.Settings
	sets int
}

func (s *settingsSession) Settings() view.Settings { return s.cur }

func (s *settingsSession) SetSettings(v view.Settings) error {
	s.cur, s.sets = v.Normalized(), s.sets+1
	return nil
}

// guideFrame ist ein Spieler mit einer Münze in Reichweite des Preisschilds.
func guideFrame(t *testing.T) (*sim.World, *view.Frame) {
	t.Helper()
	w, f := twoPlayers(t, 0)
	f.Seats = f.Seats[:1]
	w.Coins = []*sim.Coin{{ID: 900, X: w.Players[0].X + 2}}
	return w, f
}

// Anleitung aus (Settings.GuideOff, B-809 AC-02): kein Hinweis; an: der Münz-Hinweis.
func TestAnleitungAus(t *testing.T) {
	_, f := guideFrame(t)
	h := New(nil).(*hud)
	h.Listen(f)
	if h.seats[0].guideS == "" || h.seats[0].guide.id != guideCoin {
		t.Fatalf("Anleitung an: Münz-Hinweis erwartet, %+v %q", h.seats[0].guide, h.seats[0].guideS)
	}
	f.Settings.GuideOff = true
	h.Listen(f)
	if h.seats[0].guideS != "" || h.seats[0].guide.id != guideNone {
		t.Errorf("Anleitung aus: kein Hinweis, %+v %q", h.seats[0].guide, h.seats[0].guideS)
	}
}

// „Anleitung gesehen“ (B-809 AC-03): Ein erledigter Hinweis geht über das Overlay in Settings.GuideSeen; aus den
// Einstellungen gelesen kommt er nach einem Neustart (neues hud) nicht wieder.
func TestAnleitungGesehenGespeichert(t *testing.T) {
	w, f := guideFrame(t)
	h := New(nil).(*hud)
	h.Listen(f)
	w.Coins = nil
	h.Listen(f)
	s := &settingsSession{cur: view.DefaultSettings()}
	h.Update(view.Input{}, f, s)
	if s.sets != 1 || guideSeen(s.cur.GuideSeen) != 1<<guideCoin {
		t.Fatalf("gesehen gespeichert: %d Aufrufe, Bits %b", s.sets, s.cur.GuideSeen)
	}
	if h.Update(view.Input{}, f, s); s.sets != 1 {
		t.Error("nur neue Hinweise speichern")
	}
	w.Coins = []*sim.Coin{{ID: 901, X: w.Players[0].X + 1}}
	f.Settings = s.cur
	again := New(nil).(*hud)
	if again.Listen(f); again.seats[0].guide.id == guideCoin {
		t.Error("nach Neustart: gesehener Hinweis kommt nicht wieder")
	}
	f.Settings.GuideSeen = 0
	if again.Listen(f); again.seats[0].guide.id != guideCoin {
		t.Error("zurückgesetzt (Anleitung wieder an): Hinweis kommt wieder")
	}
}

// Münze und Mond als Bild (B-811 AC-03): Münz-Hinweis mit Münze, Nacht mit Mond, Landstreicher ohne; fehlen die
// Vorlagen zum Zeichnen, steht der Text allein. Lage: Bild links neben dem Text, beide zusammen mittig.
func TestAnleitungMitBild(t *testing.T) {
	cases := []struct {
		id    guideID
		ready bool
		want  guideIcon
	}{
		{guideCoin, true, iconCoin}, {guideDusk, true, iconMoon}, {guideRecruit, true, iconNone},
		{guideCoin, false, iconNone}, {guideDusk, false, iconNone},
	}
	for _, c := range cases {
		if got := guideImage(c.id, c.ready); got != c.want {
			t.Errorf("Hinweis %d, Vorlagen %v: Bild %d, erwartet %d", c.id, c.ready, got, c.want)
		}
	}
	img, tx, ty := guideLayout(500, 300, 200, 30, true, 1)
	if img.W != 30 || img.H != 30 || img.Y != ty || img.X+img.W > tx || ty != 270 {
		t.Errorf("Bild links neben dem Text: %+v, Text bei %v/%v", img, tx, ty)
	}
	if mid := (img.X + tx + 200) / 2; mid != 500 {
		t.Errorf("Bild und Text mittig um x: %v", mid)
	}
	if img, tx, _ := guideLayout(500, 300, 200, 30, false, 1); img.W != 0 || tx != 400 {
		t.Errorf("ohne Bild nur der Text mittig: %+v %v", img, tx)
	}
}
