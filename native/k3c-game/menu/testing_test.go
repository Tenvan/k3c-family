package menu

import (
	"errors"
	"slices"
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// starterSession ist eine Session mit der Rolle view.StressStarter, die den Start mitschreibt.
type starterSession struct {
	fakeSession
	started []view.StressTest
	by      []view.DeviceID
	err     error
}

func (s *starterSession) StartStress(t view.StressTest, by view.DeviceID) error {
	s.started, s.by = append(s.started, t), append(s.by, by)
	return s.err
}

// keysKeyboard ist die Tastatur (ein Gerät, Profil wasd).
var keysKeyboard = view.DeviceID{Kind: view.DeviceKeyboard}

// B-896/AC-01: Die Seite zeigt alle Einstellungen von B-896 (rollend, maxRows sichtbar), Start und Zurück.
func TestTestingShowsAllRows(t *testing.T) {
	sc := newTesting(newMenus())
	s := &starterSession{}
	sc.Update(view.Input{}, s)
	var labels []string
	for range tstRows {
		sc.fill(view.DefaultLanguage)
		labels = append(labels, sc.pg.rows[sc.pg.sel].label)
		sc.Update(press(pad1, view.ActionDown), s)
	}
	want := []string{"Vorgabe", "Spieler", "Stufe (Tiefe)", "Seed", "Gegner je Welle", "Anstieg", "Faktor je Welle",
		"Ausbau vorab", "Truppen vorab", "Zeitraffer", "Ende nach Wellen", "Ende nach Minuten", "Modus", "Test starten", "Tasten-Test", "Zurück"}
	if !slices.Equal(labels, want) {
		t.Errorf("Zeilen %v", labels)
	}
	if sc.pg.n > maxRows {
		t.Errorf("%d sichtbare Zeilen", sc.pg.n)
	}
}

// B-896/AC-01: Pad und Tastatur ändern Werte (links/rechts begrenzt, A im Kreis); eine Änderung macht „eigene Werte“.
func TestTestingChangesValues(t *testing.T) {
	sc := newTesting(newMenus())
	s := &starterSession{}
	sc.Update(view.Input{}, s)
	sc.Update(press(pad1, view.ActionRight), s) // Vorgabe 1
	if sc.cur.Preset != "vieleGegner" || sc.cur.Enemies != 40 {
		t.Fatalf("Vorgabe übernommen: %+v", sc.cur)
	}
	sc.sel = tstEnemies
	sc.Update(press(keysKeyboard, view.ActionRight), s)
	if sc.cur.Enemies != 45 || sc.cur.Preset != "" || sc.preset != -1 {
		t.Errorf("Tastatur rechts: %+v", sc.cur)
	}
	sc.sel = tstGrowth
	sc.Update(press(pad1, view.ActionConfirm), s)
	if sc.cur.Growth != view.GrowthExp {
		t.Errorf("Anstieg nicht gekippt: %q", sc.cur.Growth)
	}
	sc.sel = tstPlayers
	for range 5 {
		sc.Update(press(pad1, view.ActionRight), s)
	}
	if sc.cur.Players != view.MaxPlayers {
		t.Errorf("Spieler über dem Rand: %d", sc.cur.Players)
	}
	sc.sel = tstTimescale
	sc.Update(press(pad1, view.ActionConfirm), s)
	if sc.cur.Timescale == 4 {
		t.Errorf("Zeitraffer unverändert")
	}
	sc.sel = tstMode // B-921: Modus kippt zwischen normal und rein
	sc.Update(press(pad1, view.ActionConfirm), s)
	if !sc.cur.Pure || sc.valueText("de", tstMode) != "rein (Gegner an der Burg)" {
		t.Errorf("Modus nicht rein: %+v", sc.cur)
	}
	sc.Update(press(pad1, view.ActionConfirm), s)
	if sc.cur.Pure {
		t.Errorf("Modus nicht zurück auf normal")
	}
}

// B-896/AC-01: Starten merkt die Werte in den Einstellungen und startet über view.StressStarter; ohne die Rolle oder
// bei Fehler bleibt die Seite mit Meldung.
func TestTestingStart(t *testing.T) {
	sc := newTesting(newMenus())
	s := &starterSession{}
	sc.Update(view.Input{}, s)
	sc.Update(press(pad1, view.ActionRight), s)
	sc.sel = tstStart
	st := sc.Update(press(pad1, view.ActionConfirm), s)
	if st.Kind != view.StepPlay || len(s.started) != 1 || s.started[0].Preset != "vieleGegner" || s.by[0] != pad1 {
		t.Fatalf("Start: %+v, %+v", st, s.started)
	}
	if len(s.settings) != 1 || s.settings[0].Stress != s.started[0] {
		t.Errorf("Werte nicht gemerkt: %+v", s.settings)
	}
	s.err = errors.New("kaputt")
	if st := sc.Update(press(pad1, view.ActionConfirm), s); st.Kind != view.StepStay || !sc.note.bad {
		t.Errorf("Fehler beim Start: %+v", st)
	}
	plain := newTesting(newMenus())
	plain.Update(view.Input{}, &fakeSession{})
	plain.sel = tstStart
	if st := plain.Update(press(pad1, view.ActionConfirm), &fakeSession{}); st.Kind != view.StepStay || plain.note.key != txtTstNoRunner {
		t.Errorf("ohne Rolle: %+v", st)
	}
}

// Die Werte folgen den Kurven: linear + Faktor, exponentiell × Faktor, gedeckelt.
func TestTestingValueTexts(t *testing.T) {
	sc := newTesting(newMenus())
	sc.Update(view.Input{}, &fakeSession{})
	sc.cur = view.StressTest{Players: 1, Growth: view.GrowthExp, Factor: 1.5, Timescale: 4}.Normalized()
	got := []string{sc.valueText("de", tstGrowth), sc.valueText("de", tstFactor), sc.valueText("de", tstTimescale),
		sc.valueText("de", tstWaves), sc.valueText("de", tstSeed)}
	if strings.Join(got, "|") != "exponentiell|× 1.5|4×|keine|Name des Spielstands" {
		t.Errorf("Werte %v", got)
	}
}

// Die Karte Testing gibt es nur im Entwickler-Modus; die Navigation überspringt sie sonst.
func TestMainTestingOnlyForDevelopers(t *testing.T) {
	sc := newMain(newMenus())
	sc.dev = false
	sc.fill(view.DefaultLanguage)
	for i := range sc.pg.n {
		if sc.pg.rows[i].label == "Testing" {
			t.Fatal("Testing ohne Entwickler-Modus sichtbar")
		}
	}
	if !sc.off(mainTesting) {
		t.Error("Testing ohne Entwickler-Modus wählbar")
	}
	s := &fakeSession{}
	sc = newMain(newMenus())
	st := stepThrough(sc, s, press(pad1, view.ActionDown), press(pad1, view.ActionDown), press(pad1, view.ActionDown),
		press(pad1, view.ActionConfirm))
	if _, ok := st.Next.(*testingScreen); st.Kind != view.StepPush || !ok {
		t.Errorf("Hauptmenü › Testing: %+v", st)
	}
}

// Kaputte Vorgaben: keine Vorgaben, keine ohne Id oder Name, Werte begrenzt.
func TestParseStressPresets(t *testing.T) {
	if _, err := parseStressPresets([]byte("{")); err == nil {
		t.Error("kaputte Datei ohne Fehler")
	}
	ps, err := parseStressPresets([]byte(`{"presets":[{"id":"a","name":"A","enemies":999,"players":9},{"name":"ohne Id"}]}`))
	if err != nil || len(ps) != 1 || ps[0].Enemies != view.StressMaxEnemies || ps[0].Players != view.MaxPlayers || ps[0].Preset != "a" {
		t.Errorf("Vorgaben %+v, %v", ps, err)
	}
	real, err := loadStressPresets()
	if err != nil || len(real) < 3 {
		t.Errorf("data/stresstests.json: %d Vorgaben, %v", len(real), err)
	}
}
