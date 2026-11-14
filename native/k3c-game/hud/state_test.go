package hud

import (
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func TestListenJeSpielerGetrennt(t *testing.T) {
	w, f := twoPlayers(t, 0)
	f.Settings = view.DefaultSettings()
	h := New(nil).(*hud)
	f.Events = []view.Event{{Stage: 0, Data: sim.Event{"type": "chest", "player": 1, "gold": 5}}}
	f.Cues = []view.Cue{{Kind: view.CueDenied, Player: 1, Text: "abgelehnt"}}
	h.Listen(f)
	a, b := &h.seats[0], &h.seats[1]
	if !a.active || !b.active || a.goldS == "" || a.clockS == "" || a.sharedS == "" {
		t.Fatalf("beide Spieler mit Werten: %+v", a)
	}
	if a.banner.showing != "P2 findet 5 Gold" || len(b.banner.queue) != 1 || b.banner.queue[0] != "P2 findet 5 Gold" {
		t.Errorf("Meldung der Stufe in beiden Zellen: %q %v", a.banner.showing, b.banner.queue)
	}
	if len(a.banner.queue) != 0 || b.banner.showing != "abgelehnt" {
		t.Errorf("Ablehnung nur beim betroffenen Spieler: %v %q", a.banner.queue, b.banner.showing)
	}
	if a.placeS != "P1 · Oberwelt (Wald)" || b.placeS != "P2 · Oberwelt (Wald)" {
		t.Errorf("Ort je Spieler: %q %q", a.placeS, b.placeS)
	}
	goldUndGesichert(t, h, w, f)
}

// goldUndGesichert: Goldtopf nur beim Spieler mit geändertem Gold, „gesichert“ läuft ab, ein gehender Spieler fällt weg.
func goldUndGesichert(t *testing.T, h *hud, w *sim.World, f *view.Frame) {
	t.Helper()
	a, b := &h.seats[0], &h.seats[1]
	w.Players[1].Gold += 3
	f.Events, f.Cues, f.Clock = nil, []view.Cue{{Kind: view.CueSaved, Player: -1}}, 1
	h.Listen(f)
	if a.flash || !b.flash {
		t.Error("Goldtopf nur beim Spieler mit geändertem Gold")
	}
	if !strings.HasSuffix(a.clockS, "gesichert") || !strings.HasSuffix(b.clockS, "gesichert") {
		t.Errorf("gesichert nach CueSaved: %q", a.clockS)
	}
	f.Cues, f.Clock = nil, 1+savedSeconds
	h.Listen(f)
	if strings.HasSuffix(a.clockS, "gesichert") {
		t.Error("gesichert läuft ab")
	}
	f.Seats = f.Seats[1:]
	h.Listen(f)
	if h.seats[0].active || h.cells != 1 {
		t.Error("Spieler 0 gegangen: Spieler 1 allein")
	}
}

func TestListenEnglischUndBeitritt(t *testing.T) {
	_, f := twoPlayers(t, 0)
	f.Settings = view.Settings{Language: view.LanguageEnglish}
	f.Unseated = []view.DeviceID{{Kind: view.DeviceGamepad, Index: 4}}
	h := New(nil).(*hud)
	h.Listen(f)
	if h.joinS != "Press A to join" || !strings.HasPrefix(h.seats[0].clockS, "Day 1") {
		t.Errorf("englisch: %q %q", h.joinS, h.seats[0].clockS)
	}
}

// Gleicher Frame, gleicher Zustand: zwei HUDs mit derselben Folge liefern dieselben Texte (Bildgleichheit).
func TestListenDeterministisch(t *testing.T) {
	run := func() [2]seatState {
		w, f := twoPlayers(t, 1)
		h := New(nil).(*hud)
		cmds := []sim.PlayerCommand{{MoveX: 1}, {MoveX: -1, Sprint: true}}
		for tick := range 120 {
			sim.Step(w, cmds, 1.0/30)
			f.Tick, f.Clock, f.Events = uint64(tick), float64(tick)/30, f.Events[:0]
			for _, e := range w.Events {
				f.Events = append(f.Events, view.Event{Stage: 0, Data: e})
			}
			h.Listen(f)
		}
		return h.seats
	}
	a, b := run(), run()
	for p := range a {
		x, y := a[p], b[p]
		if x.goldS != y.goldS || x.clockS != y.clockS || x.banner.showing != y.banner.showing || x.x != y.x || x.guideS != y.guideS {
			t.Errorf("Spieler %d verschieden", p)
		}
	}
}

// Textregel (textRule.test.ts): Spieltexte stehen nur in texts.go; kein anderes Literal mit Umlaut oder zwei Wörtern.
func TestKeineTexteAusserhalbDerTextdatei(t *testing.T) {
	german := regexp.MustCompile(`[äöüßÄÖÜ]|[A-Za-z]{2,}\s+[A-Za-z]{2,}`)
	files, _ := filepath.Glob("*.go")
	for _, name := range files {
		if strings.HasSuffix(name, "_test.go") || name == "texts.go" {
			continue
		}
		src, err := os.ReadFile(name)
		if err != nil {
			t.Fatal(err)
		}
		f, err := parser.ParseFile(token.NewFileSet(), name, src, 0)
		if err != nil {
			t.Fatal(err)
		}
		ast.Inspect(f, func(n ast.Node) bool {
			if lit, ok := n.(*ast.BasicLit); ok && lit.Kind == token.STRING {
				if s, _ := strconv.Unquote(lit.Value); german.MatchString(s) && !strings.Contains(s, "🚫") && !strings.Contains(s, "💥") {
					t.Errorf("%s: Text-Literal %q gehört nach texts.go", name, s)
				}
			}
			return true
		})
	}
}
