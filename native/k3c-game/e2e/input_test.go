package e2e

// B-918: Nach beliebigem Öffnen und Schließen von Cheat-, Skill-, Pause- und Menüseiten reagiert das Spiel sofort
// wieder auf die Tasten. Der Test fährt die echte Schleife mit einer Eingabe-Quelle, die der Test je Update füllt.

import (
	"fmt"
	"testing"

	"k3c/native/k3c-game/game"
	"k3c/native/k3c-game/local"
	"k3c/native/k3c-game/view"
)

// padSource liefert je Poll die vom Test gesetzten Aktionen und den Befehl der Tastatur.
type padSource struct {
	acts []view.Action
	cmd  view.Command
	buf  [1]view.DeviceInput
}

func (p *padSource) Poll() view.Input {
	p.buf[0] = view.DeviceInput{Device: view.DeviceID{Kind: view.DeviceKeyboard}, Command: p.cmd, Actions: p.acts}
	return view.Input{Devices: p.buf[:]}
}

func newInputRun(t *testing.T) (*run, *padSource) {
	t.Helper()
	dir := t.TempDir()
	args, err := game.ParseArgs([]string{"--saves", dir}, dir)
	if err != nil {
		t.Fatal(err)
	}
	src := &padSource{}
	pk := game.Packages(nil)
	for i := range pk {
		if pk[i].Name == "input" {
			pk[i].Source = src
		}
	}
	parts, err := game.Assemble(pk)
	if err != nil {
		t.Fatal(err)
	}
	s := local.Open(local.Config{Dir: dir, Now: fixedNow, Recording: true})
	g, err := game.New(parts, s, args)
	if err != nil {
		t.Fatal(err)
	}
	return &run{t: t, g: g, sim: s, dir: dir}, src
}

// press rechnet ein Update mit den Aktionen acts, danach ein Update ohne Taste.
func (r *run) press(src *padSource, acts ...view.Action) {
	r.t.Helper()
	src.acts = acts
	r.step()
	src.acts = nil
	r.step()
}

// startKeyboardGame startet über Hauptmenü und Lobby ein Spiel mit der Tastatur.
func (r *run) startKeyboardGame(src *padSource) {
	r.t.Helper()
	r.step()
	r.press(src, view.ActionConfirm)
	for range 5 {
		r.press(src, view.ActionDown)
	}
	r.press(src, view.ActionConfirm)
	r.step()
	if r.g.Screen() != nil || !r.sim.Active() {
		r.t.Fatalf("Spiel nicht gestartet: Screen %T, aktiv %v", r.g.Screen(), r.sim.Active())
	}
}

// reacts meldet, ob das Spiel auf Laufen reagiert: Der Monarch bewegt sich in 30 Updates nach rechts.
func (r *run) reacts(src *padSource) bool {
	r.t.Helper()
	x := r.frame().Monarch(0).X
	src.cmd = view.Command{Move: 1}
	for range 30 {
		r.step()
	}
	src.cmd = view.Command{}
	return r.frame().Monarch(0).X > x+0.1
}

// closeAll schließt alles, was offen sein kann: Zurück (im Spiel ohne Wirkung; Menü würde die Pause öffnen).
func (r *run) closeAll(src *padSource) {
	for range 12 {
		r.press(src, view.ActionBack)
	}
}

func TestInputRecoversAfterMenus(t *testing.T) {
	for seed := uint32(1); seed <= 12; seed++ {
		t.Run(fmt.Sprint("seed", seed), func(t *testing.T) { inputRecovers(t, seed) })
	}
}

func inputRecovers(t *testing.T, seed uint32) {
	r, src := newInputRun(t)
	r.startKeyboardGame(src)
	if !r.reacts(src) {
		t.Fatal("Spiel reagiert schon nach dem Start nicht")
	}
	all := []view.Action{view.ActionUp, view.ActionDown, view.ActionLeft, view.ActionRight, view.ActionConfirm,
		view.ActionBack, view.ActionMenu, view.ActionHome, view.ActionView, view.ActionPrevPage, view.ActionNextPage,
		view.ActionSkillMenu, view.ActionCheats}
	seen := map[string]int{}
	for round := range 40 {
		for range 25 {
			seed = seed*1664525 + 1013904223
			a := all[int(seed>>16)%len(all)]
			seen[fmt.Sprintf("%T", r.g.Screen())]++
			src.cmd = view.Command{Move: float64(int(seed>>20)%3 - 1), Pay: seed&0x200 != 0, Attack: seed&0x400 != 0, Skill: int(seed>>24) % 5}
			if seed&0x100 != 0 {
				r.press(src, a)
			} else {
				r.press(src, a, all[int(seed>>8)%len(all)])
			}
		}
		r.closeAll(src)
		if !r.sim.Active() {
			r.t.Logf("Runde %d: Spiel beendet, Neustart", round)
			r.closeAll(src)
			r.startKeyboardGame(src)
		}
		if !r.reacts(src) {
			t.Logf("gesehen: %v", seen)
			t.Fatalf("Runde %d: Spiel reagiert nicht mehr (Screen %T, Pause %v)", round, r.g.Screen(), r.frame().Paused)
		}
	}
	t.Logf("gesehen: %v, Spiel aktiv %v", seen, r.sim.Active())
}

// B-918: Ein Halt der Entwickler-Taste F7 (Simulation steht, kein Hinweis im Bild) überlebte Pause und Fortsetzen; das
// Spiel reagierte danach auf keine Taste mehr. Fortsetzen aus dem Pause-Menü hebt den Halt auf.
func TestResumeClearsDevHalt(t *testing.T) {
	r, src := newInputRun(t)
	r.startKeyboardGame(src)
	r.sim.Halt(true)
	if r.reacts(src) {
		t.Fatal("Halt bremst das Spiel nicht")
	}
	r.press(src, view.ActionMenu) // Pause
	if r.g.Screen() == nil {
		t.Fatal("Pause-Menü nicht offen")
	}
	r.press(src, view.ActionBack) // Fortsetzen
	if r.g.Screen() != nil || r.sim.Halted() {
		t.Fatalf("nach Fortsetzen: Screen %T, Halt %v", r.g.Screen(), r.sim.Halted())
	}
	if !r.reacts(src) {
		t.Error("Spiel reagiert nach dem Fortsetzen nicht")
	}
}
