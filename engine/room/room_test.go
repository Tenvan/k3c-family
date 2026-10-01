package room

import (
	"encoding/json"
	"fmt"
	"slices"
	"strings"
	"testing"
	"time"

	"k3c/engine/level"
	"k3c/engine/sim"
	"k3c/engine/store"
)

// peer schreibt mit, was der Raum schickt.
type peer struct {
	log      []string
	you      []Seat
	monarchs []string
	world    *sim.World
}

func (p *peer) Joined(room, name string, you []Seat) {
	p.you = you
	p.log = append(p.log, fmt.Sprintf("joined %s %s %v", room, name, you))
}
func (p *peer) Level(depth int, _ level.Layout) { p.log = append(p.log, fmt.Sprintf("level %d", depth)) }
func (p *peer) State(tick int, w *sim.World)    { p.world = w; p.log = append(p.log, fmt.Sprintf("state %d", tick)) }
func (p *peer) Seats(you []Seat, monarchs []string) {
	p.you, p.monarchs = you, monarchs
	p.log = append(p.log, "seats")
}
func (p *peer) Replaced() { p.log = append(p.log, "replaced") }
func (p *peer) Closed(bool) { p.log = append(p.log, "closed") }

func (p *peer) has(entry string) bool { return slices.Contains(p.log, entry) }

type memStore struct {
	data  map[string][]byte
	saves int
}

func (s *memStore) Load(name string) ([]byte, error) {
	if d, ok := s.data[name]; ok {
		return d, nil
	}
	return nil, store.ErrNotFound
}

func (s *memStore) Store(name string, data []byte) (string, error) {
	s.data[name] = data
	s.saves++
	return "", nil
}

type fixture struct {
	m     *Manager
	store *memStore
	now   time.Time
}

func newFixture() *fixture {
	f := &fixture{store: &memStore{data: map[string][]byte{}}, now: time.Date(2026, 10, 1, 12, 0, 0, 0, time.UTC)}
	f.m = NewManager(f.store)
	f.m.Now = func() time.Time { return f.now }
	codes := []string{"KRNZ", "BWTQ", "MXPL", "HJKA", "ZZZZ"}
	f.m.NewCode = func() string { c := codes[0]; codes = codes[1:]; return c }
	return f
}

func (f *fixture) wait(d time.Duration) { f.now = f.now.Add(d); f.m.Sweep() }

// need(f())(t) bricht bei einem Fehler ab und liefert sonst den Wert.
func need[T any](v T, err error) func(*testing.T) T {
	return func(t *testing.T) T {
		t.Helper()
		if err != nil {
			t.Fatal(err)
		}
		return v
	}
}

func ok(t *testing.T, err error) {
	t.Helper()
	if err != nil {
		t.Fatal(err)
	}
}

func ticks(r *Room, n int) {
	for range n {
		_ = r.Tick()
	}
}

func xs(r *Room) []float64 {
	var out []float64
	for _, p := range r.camp.CurrentWorld().Players {
		out = append(out, p.X)
	}
	return out
}

// ownMovement: Xbox-Slot 0 läuft rechts, Slot 1 links, das Handy (Monarch 2) steht. Jeder steuert seinen eigenen Monarchen.
func ownMovement(t *testing.T, r *Room) {
	t.Helper()
	ok(t, r.Input("xbox", 0, sim.PlayerCommand{MoveX: 1}))
	ok(t, r.Input("xbox", 1, sim.PlayerCommand{MoveX: -1}))
	before := xs(r)
	ticks(r, 30)
	after := xs(r)
	if !(after[0] > before[0] && after[1] < before[1] && after[2] == before[2]) {
		t.Fatalf("Bewegung vorher %v, nachher %v", before, after)
	}
}

// Ablauf aus docs/protocol.md › Beispiel: 2 Controller an der Xbox + 1 Handy.
func TestXboxUndHandy(t *testing.T) {
	f := newFixture()
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "familie", true, 0, []int{0}))(t)
	if r.Code != "KRNZ" || !x.has("joined KRNZ familie [{0 0}]") || !x.has("level 0") || !x.has("state 0") {
		t.Fatalf("Erstellen: %v", x.log)
	}
	ok(t, r.AddSlot("xbox", 1))
	need(f.m.Join("handy", h, "KRNZ", []int{0}))(t)
	if fmt.Sprint(h.you) != "[{0 2}]" || fmt.Sprint(x.monarchs) != "[taken taken taken]" {
		t.Fatalf("Handy %v, Plätze %v", h.you, x.monarchs)
	}
	ownMovement(t, r)
	// Handy verliert WLAN: Monarch 2 wartet und steht still, die Xbox spielt weiter.
	r.Drop("handy", h)
	if fmt.Sprint(x.monarchs) != "[taken taken waiting]" || !r.camp.CurrentWorld().Players[2].Free {
		t.Fatalf("nach Abbruch: %v", x.monarchs)
	}
	f.wait(10 * time.Second)
	h2 := &peer{}
	need(f.m.Join("handy", h2, "KRNZ", []int{0}))(t)
	if fmt.Sprint(h2.you) != "[{0 2}]" || !h2.has("level 0") {
		t.Fatalf("Wiederverbinden: %v", h2.log)
	}
	// Controller 2 geht: Monarch 1 frei; ein neues Gerät bekommt ihn.
	ok(t, r.RemoveSlot("xbox", 1))
	if fmt.Sprint(x.monarchs) != "[taken free taken]" {
		t.Fatalf("nach removeSlot: %v", x.monarchs)
	}
	z := &peer{}
	need(f.m.Join("tablet", z, "KRNZ", []int{0}))(t)
	if fmt.Sprint(z.you) != "[{0 1}]" {
		t.Fatalf("neues Gerät bekommt %v", z.you)
	}
}

func TestWiederverbindenMitGeaendertenSlots(t *testing.T) {
	f := newFixture()
	x := &peer{}
	r := need(f.m.Create("xbox", x, "slots", true, 0, []int{0, 1}))(t)
	r.Drop("xbox", x)
	x2 := &peer{}
	need(f.m.Join("xbox", x2, r.Code, []int{1, 2}))(t)
	// Slot 1 behält Monarch 1, Slot 0 fällt weg (Monarch 0 frei), Slot 2 bekommt den kleinsten freien: 0.
	if fmt.Sprint(x2.you) != "[{1 1} {2 0}]" || fmt.Sprint(x2.monarchs) != "[taken taken]" {
		t.Fatalf("mehr/weniger Slots: %v %v", x2.you, x2.monarchs)
	}
	r.Drop("xbox", x2)
	x3 := &peer{}
	need(f.m.Join("xbox", x3, r.Code, []int{1}))(t)
	if fmt.Sprint(x3.you) != "[{1 1}]" || fmt.Sprint(x3.monarchs) != "[free taken]" {
		t.Fatalf("weniger Slots: %v %v", x3.you, x3.monarchs)
	}
}

func TestNach60SekundenFreiUndBevorzugt(t *testing.T) {
	f := newFixture()
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "frist", true, 0, []int{0}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0, 1}))(t)
	r.Drop("handy", h)
	f.wait(59 * time.Second)
	if fmt.Sprint(x.monarchs) != "[taken waiting waiting]" {
		t.Fatalf("nach 59 s: %v", x.monarchs)
	}
	f.wait(time.Second)
	if fmt.Sprint(x.monarchs) != "[taken free free]" {
		t.Fatalf("nach 60 s: %v", x.monarchs)
	}
	// Rückkehr nach der Frist: normales Beitreten, die alten Monarchen werden bevorzugt.
	h2 := &peer{}
	need(f.m.Join("handy", h2, r.Code, []int{1, 0}))(t)
	if fmt.Sprint(h2.you) != "[{0 1} {1 2}]" {
		t.Fatalf("Rückkehr: %v", h2.you)
	}
}

func TestPausierterRaumFristenUndAufraeumen(t *testing.T) {
	f := newFixture()
	x := &peer{}
	r := need(f.m.Create("xbox", x, "pause", true, 0, []int{0}))(t)
	ticks(r, 3)
	saves := f.store.saves
	r.Drop("xbox", x)
	if f.store.saves != saves+1 {
		t.Fatal("leerer Raum speichert nicht sofort")
	}
	ticks(r, 30)
	if got := r.camp.CurrentWorld().Time; got > 0.11 {
		t.Fatalf("pausierter Raum tickt: time %v", got)
	}
	f.wait(61 * time.Second)
	if r.monarchs[0].state != Free {
		t.Fatal("60-s-Frist läuft im pausierten Raum nicht")
	}
	f.wait(8 * time.Minute)
	if len(f.m.Rooms()) != 1 {
		t.Fatal("Raum vor 10 min aufgeräumt")
	}
	f.wait(time.Minute)
	if len(f.m.Rooms()) != 0 || f.store.saves != saves+2 {
		t.Fatalf("Aufräumen: %d Räume, %d Speicherungen", len(f.m.Rooms()), f.store.saves-saves)
	}
	if _, err := f.m.Join("xbox", &peer{}, r.Code, []int{0}); err != ErrRoomNotFound {
		t.Fatalf("Code nach Aufräumen: %v", err)
	}
}

func TestGrenzen(t *testing.T) {
	f := newFixture()
	for i := range MaxRooms {
		need(f.m.Create(fmt.Sprint("g", i), &peer{}, fmt.Sprint("raum-", i), true, 0, []int{0}))(t)
	}
	if _, err := f.m.Create("g9", &peer{}, "raum-9", true, 0, []int{0}); err != ErrTooManyRooms {
		t.Fatalf("5. Raum: %v", err)
	}
	r := f.m.rooms["KRNZ"]
	if _, err := f.m.Join("voll", &peer{}, r.Code, []int{0, 1, 2, 3}); err != ErrRoomFull {
		t.Fatalf("5. Monarch: %v", err)
	}
	if len(r.monarchs) != 1 {
		t.Fatal("Beitreten war nicht alles oder nichts")
	}
	need(f.m.Join("drei", &peer{}, r.Code, []int{0, 1, 2}))(t)
	if err := r.AddSlot("drei", 3); err != ErrRoomFull {
		t.Fatalf("addSlot im vollen Raum: %v", err)
	}
	if _, err := f.m.Join("x", &peer{}, r.Code, []int{4}); err != ErrTooManySlots {
		t.Fatalf("Slot 4: %v", err)
	}
	if err := r.AddSlot("drei", 4); err != ErrTooManySlots {
		t.Fatalf("addSlot 4: %v", err)
	}
	for _, bad := range [][]int{{}, {-1}, {1, 1}} {
		if _, err := f.m.Join("x", &peer{}, r.Code, bad); err != ErrBadRequest {
			t.Errorf("Slots %v: %v", bad, err)
		}
	}
}

func TestErsetzteVerbindung(t *testing.T) {
	f := newFixture()
	a, b := &peer{}, &peer{}
	r := need(f.m.Create("tab", a, "tabs", true, 0, []int{0}))(t)
	need(f.m.Join("tab", b, r.Code, []int{0}))(t)
	if !a.has("replaced") || fmt.Sprint(b.you) != "[{0 0}]" {
		t.Fatalf("alte %v, neue %v", a.log, b.you)
	}
	r.Drop("tab", a) // die alte Verbindung bricht ab: ohne Wirkung
	if r.monarchs[0].state != Taken {
		t.Fatal("ersetzte Verbindung hat den Monarchen freigegeben")
	}
}

func TestCreateReihenfolge(t *testing.T) {
	f := newFixture()
	if _, err := f.m.Create("a", &peer{}, "Falsch!", true, 0, []int{4}); err != ErrTooManySlots {
		t.Errorf("Slot 4 zuerst: %v", err)
	}
	if _, err := f.m.Create("a", &peer{}, "Falsch!", true, 0, []int{0}); err != ErrBadRequest {
		t.Errorf("Name: %v", err)
	}
	if _, err := f.m.Create("a", &peer{}, "tief", true, 7, []int{0}); err != ErrBadRequest {
		t.Errorf("Startstufe 7: %v", err)
	}
	r := need(f.m.Create("a", &peer{}, "offen", true, 0, []int{0}))(t)
	if _, err := f.m.Create("b", &peer{}, "offen", true, 0, []int{0}); err != ErrSaveExists {
		t.Errorf("neu, aber offen: %v", err)
	}
	if again := need(f.m.Create("b", &peer{}, "offen", false, 0, []int{0}))(t); again != r {
		t.Error("gespeichert und offen: nicht demselben Raum beigetreten")
	}
	f.store.data["alt"] = []byte(`{}`)
	if _, err := f.m.Create("c", &peer{}, "alt", true, 0, []int{0}); err != ErrSaveExists {
		t.Errorf("neu, aber gespeichert: %v", err)
	}
	if _, err := f.m.Create("c", &peer{}, "fehlt", false, 0, []int{0}); err != ErrSaveNotFound {
		t.Errorf("gespeichert, aber fehlt: %v", err)
	}
	for i := range MaxRooms - 1 {
		need(f.m.Create("d", &peer{}, fmt.Sprint("voll-", i), true, 0, []int{0}))(t)
	}
	if _, err := f.m.Create("e", &peer{}, "alt", true, 0, []int{0}); err != ErrSaveExists {
		t.Errorf("Spielstand vor Raumzahl prüfen: %v", err)
	}
	if _, err := f.m.Create("e", &peer{}, "noch-einer", true, 0, []int{0}); err != ErrTooManyRooms {
		t.Errorf("5. Raum: %v", err)
	}
}

// Ein geladener Stand hat keine Monarchen; sie entstehen beim Beitreten mit dem Gold ihres Index.
func TestGeladenerStand(t *testing.T) {
	f := newFixture()
	c := sim.CreateCampaign("gold", "g", 1)
	c.JoinPlayer().Gold = 33
	c.JoinPlayer().Gold = 44
	f.store.data["gold"] = need(json.Marshal(c.ToSave("2026-01-01T00:00:00.000Z")))(t)
	x := &peer{}
	r := need(f.m.Create("xbox", x, "gold", false, 0, []int{1}))(t)
	players := r.camp.CurrentWorld().Players
	if len(players) != 1 || players[0].Gold != 33 || fmt.Sprint(x.you) != "[{1 0}]" {
		t.Fatalf("Monarchen nach Laden: %d, Gold %v", len(players), players[0].Gold)
	}
}

func TestSpeicherzeitpunkte(t *testing.T) {
	f := newFixture()
	x := &peer{}
	r := need(f.m.Create("xbox", x, "reise", true, 0, []int{0}))(t)
	saves := f.store.saves
	w := r.camp.CurrentWorld()
	for _, e := range w.Level.Entities {
		if e.Kind == "exit" {
			w.Players[0].X = e.X
		}
	}
	ticks(r, 70) // 2 s am Tiefen-Eingang
	if r.camp.Depth != 1 || f.store.saves != saves+1 || !x.has("level 1") {
		t.Fatalf("Stufenwechsel: Tiefe %d, %d Speicherungen, %v", r.camp.Depth, f.store.saves-saves, x.log[len(x.log)-3:])
	}
	var s sim.SaveGame
	ok(t, json.Unmarshal(f.store.data["reise"], &s))
	if s.Depth != 1 {
		t.Fatalf("gespeicherte Tiefe %d", s.Depth)
	}
	h := &peer{}
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	f.m.Close()
	if f.store.saves != saves+2 || !x.has("closed") || !h.has("closed") || len(f.m.Rooms()) != 0 {
		t.Fatalf("Beenden: %d Speicherungen, %v", f.store.saves-saves, x.log)
	}
}

func TestEingabeNurEigeneSlots(t *testing.T) {
	f := newFixture()
	r := need(f.m.Create("xbox", &peer{}, "eingabe", true, 0, []int{0}))(t)
	if err := r.Input("xbox", 1, sim.PlayerCommand{}); err != ErrBadRequest {
		t.Errorf("fremder Slot: %v", err)
	}
	if err := r.Input("fremd", 0, sim.PlayerCommand{}); err != ErrBadRequest {
		t.Errorf("fremdes Gerät: %v", err)
	}
	ok(t, r.Input("xbox", 0, sim.PlayerCommand{MoveX: 7}))
	if r.monarchs[0].input.MoveX != 1 {
		t.Errorf("moveX nicht begrenzt: %v", r.monarchs[0].input.MoveX)
	}
	if err := r.RemoveSlot("xbox", 3); err != ErrBadRequest {
		t.Errorf("removeSlot fremd: %v", err)
	}
}

func TestRaumlisteUndChanged(t *testing.T) {
	f := newFixture()
	calls := 0
	f.m.Changed = func() { calls++; f.m.Rooms() } // ohne Sperre aufgerufen: Rooms() darf nicht hängen
	x := &peer{}
	r := need(f.m.Create("xbox", x, "liste", true, 0, []int{0, 1}))(t)
	r.Drop("xbox", x)
	got := f.m.Rooms()
	if len(got) != 1 || got[0] != (Info{"KRNZ", "liste", 0, 2, 2, false}) || calls != 2 {
		t.Fatalf("Raumliste %+v, Changed %d", got, calls)
	}
	if !strings.HasPrefix(x.log[0], "joined") {
		t.Fatal("joined kommt nicht zuerst")
	}
}
