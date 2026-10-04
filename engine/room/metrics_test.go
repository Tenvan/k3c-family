package room

import (
	"bytes"
	"encoding/json"
	"errors"
	"log/slog"
	"slices"
	"strings"
	"testing"
	"time"

	"k3c/engine/sim"
)

// Spielmetrik-Report je Raumlauf (S2.3, B-150): Inhalt, kein Einfluss auf den Spielverlauf, keine Namen.

// sessionSink ist eine Ablage im Speicher; mit fail scheitert sie.
type sessionSink struct {
	data [][]byte
	fail error
}

func (s *sessionSink) StoreSession(d []byte) (string, error) {
	if s.fail != nil {
		return "", s.fail
	}
	s.data = append(s.data, d)
	return "mem", nil
}

// Lange Geräte-IDs, damit der Test erkennt, ob nur das Kürzel (8 Zeichen) im Report steht.
const (
	xboxID  = "xbox-wohnzimmer-0001"
	handyID = "handy-kinderzimmer-02"
)

// finishBuild lässt einen Bauplatz der Stufe 0 im nächsten Schritt fertig werden (wie TestEventsObergrenzeImTick).
func finishBuild(t *testing.T, r *Room) {
	t.Helper()
	w := r.isl.Stages[0]
	if len(w.Sites) == 0 {
		t.Fatal("Stufe 0 ohne Bauplatz")
	}
	s := w.Sites[0]
	id := 90000
	s.State, s.HP, s.BuildProgress, s.WorkerID = "waitingWorker", 0, 0.9999, &id
	w.Troops = append(w.Troops, &sim.Troop{ID: id, Kind: "peasant", X: s.X, AnchorX: s.X, HP: 1, MaxHP: 1, Job: &sim.Job{Type: "build", SiteID: s.ID}})
}

// downByGoblin setzt neben Monarch idx (1 HP, ohne Gold) einen Goblin.
func downByGoblin(r *Room, idx int) {
	p := r.isl.Players()[idx]
	p.HP, p.Gold = 1, 0
	w := r.isl.Stages[r.isl.StageOf(idx)]
	w.Enemies = append(w.Enemies, &sim.Enemy{ID: 90001, Kind: "goblin", X: p.X - 0.5, HP: 50, MaxHP: 50, Damage: 5, Speed: 1, Range: 1, HomeX: p.X - 60})
}

// setCycleSpeed ändert die Beschleunigung des Zyklus, ohne die Stelle im Zyklus zu verschieben (Time × CycleSpeed bleibt).
func setCycleSpeed(r *Room, v float64) {
	for _, w := range r.isl.Stages {
		w.Time, w.CycleSpeed = w.Time*w.CycleSpeed/v, v
	}
}

// tickToDay tickt, bis Stufe 0 den Tag day erreicht hat.
func tickToDay(t *testing.T, r *Room, day int) {
	t.Helper()
	for i := 0; r.isl.Stages[0].Cycle.Day < day; i++ {
		if i > 3000 {
			t.Fatalf("Tag %d nicht erreicht, Zyklus %+v", day, r.isl.Stages[0].Cycle)
		}
		_ = r.Tick()
	}
}

// metrikRun ist der Lauf aus Test (a): zwei Geräte, ein Bau, eine Nacht mit einem Sturz durch einen Goblin, ein Abbruch.
func metrikRun(t *testing.T) (*fixture, *sessionSink, *Room) {
	t.Helper()
	f := newFixture()
	sink := &sessionSink{}
	f.m.Sessions = sink
	x, h := &peer{}, &peer{}
	r := need(f.m.Create(xboxID, x, "metrik-seed", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join(handyID, h, r.Code, []int{0}))(t)
	finishBuild(t, r)
	ticks(r, 3)
	tickToNight(t, r) // Zyklus × 1000 wie in S2.2
	setCycleSpeed(r, 1) // die Nacht dauert, bis der Goblin trifft
	downByGoblin(r, 1)
	for i := 0; len(r.met.deaths) == 0; i++ {
		if i > 300 {
			t.Fatal("Monarch 1 fällt nicht")
		}
		_ = r.Tick()
	}
	setCycleSpeed(r, 1000)
	tickToDay(t, r, 2)
	r.Drop(handyID, h)
	f.m.Close()
	return f, sink, r
}

// (a) AC-05: ein Report mit allen Feldern aus Schema 1 und den erwarteten Werten.
func TestMetrikReportNachRaumlauf(t *testing.T) {
	_, sink, r := metrikRun(t)
	if len(sink.data) != 1 {
		t.Fatalf("%d Reports, erwartet 1", len(sink.data))
	}
	var keys map[string]any
	need(0, json.Unmarshal(sink.data[0], &keys))(t)
	for _, k := range []string{"schema", "room", "startedAt", "endedAt", "playSeconds", "grade", "goal", "defeat", "monarchs",
		"devices", "reached", "nights", "deaths", "firstBuildSeconds", "goldPerDay", "goldEnd", "disconnects"} {
		if _, ok := keys[k]; !ok {
			t.Errorf("Feld %q fehlt", k)
		}
	}
	var rep sessionReport
	need(0, json.Unmarshal(sink.data[0], &rep))(t)
	if rep.Schema != 1 || rep.Room != r.Code || rep.StartedAt == "" || rep.EndedAt == "" || rep.PlaySeconds <= 0 || rep.Grade == "" || rep.Monarchs != 2 {
		t.Errorf("Kopf: %+v", rep)
	}
	checkRun(t, rep)
	t.Logf("Report:\n%s", sink.data[0])
}

// checkRun prüft die Werte des Laufs aus metrikRun: Geräte, Abbruch, Nacht, Bau, Sturz und Gold.
func checkRun(t *testing.T, rep sessionReport) {
	t.Helper()
	if !slices.Equal(rep.Devices, []string{"handy-ki", "xbox-woh"}) || len(rep.Disconnects) != 1 || rep.Disconnects[0] != (disconnect{"handy-ki", 1}) {
		t.Errorf("Geräte %v, Abbrüche %v", rep.Devices, rep.Disconnects)
	}
	if len(rep.Nights) == 0 || rep.Nights[0] != (nightResult{1, true}) || rep.Reached.Day < 2 {
		t.Errorf("Nächte %v, erreicht %+v", rep.Nights, rep.Reached)
	}
	checkBuildDeathGold(t, rep)
}

func checkBuildDeathGold(t *testing.T, rep sessionReport) {
	t.Helper()
	if rep.FirstBuildSeconds == nil || *rep.FirstBuildSeconds <= 0 {
		t.Errorf("firstBuildSeconds %v", rep.FirstBuildSeconds)
	}
	if !slices.ContainsFunc(rep.Deaths, func(d deathEntry) bool { return d.Monarch == 1 && d.Cause == "goblin" && d.Phase == "night" && d.Day == 1 }) {
		t.Errorf("Tode %+v", rep.Deaths)
	}
	if len(rep.GoldPerDay) == 0 || rep.GoldPerDay[0].Day != 2 || len(rep.GoldPerDay[0].Gold) != 2 || len(rep.GoldEnd) != 2 {
		t.Errorf("Gold je Tag %+v, am Ende %v", rep.GoldPerDay, rep.GoldEnd)
	}
}

// (c) AC-05: weder Spielstand-Name noch volle Geräte-ID, nur Index und Kürzel.
func TestMetrikOhneNamen(t *testing.T) {
	_, sink, _ := metrikRun(t)
	text := string(sink.data[0])
	for _, bad := range []string{"metrik-seed", xboxID, handyID, "campaignId"} {
		if strings.Contains(text, bad) {
			t.Errorf("Report enthält %q", bad)
		}
	}
}

// (b) AC-05: Gleicher Lauf mit und ohne Sammler ergibt nach N Ticks dieselben Bytes aller Stufen.
func TestMetrikAendertSpielverlaufNicht(t *testing.T) {
	run := func(collect bool) []byte {
		f := newFixture()
		x := &peer{}
		r := need(f.m.Create(xboxID, x, "hash", true, 0, []int{0, 1}, Options{}))(t)
		if collect {
			f.m.Sessions = &sessionSink{}
		} else {
			r.met = nil
		}
		tickToNight(t, r)
		for i := range 300 {
			need(0, r.Input(xboxID, x, map[int]sim.PlayerCommand{0: {MoveX: 1}, 1: {MoveX: float64(i%3 - 1)}}))(t)
			_ = r.Tick()
		}
		return need(json.Marshal(r.isl.Stages))(t)
	}
	if with, without := run(true), run(false); !bytes.Equal(with, without) {
		t.Fatal("Zustand mit Sammler weicht ab")
	}
}

// (d) Ein Raum ohne Tick schreibt keinen Report.
func TestMetrikOhneTickKeinReport(t *testing.T) {
	f := newFixture()
	sink := &sessionSink{}
	f.m.Sessions = sink
	need(f.m.Create(xboxID, &peer{}, "leer", true, 0, []int{0}, Options{}))(t)
	f.m.Close()
	if len(sink.data) != 0 {
		t.Fatalf("%d Reports ohne Tick", len(sink.data))
	}
}

// (e) Scheitert die Ablage, steht der Fehler im Log und der leere Raum ist nach EmptyFor trotzdem aufgeräumt.
func TestMetrikAblageFehlerRaumSchliesst(t *testing.T) {
	f := newFixture()
	buf := &bytes.Buffer{}
	f.m.Log = slog.New(slog.NewTextHandler(buf, nil))
	f.m.Sessions = &sessionSink{fail: errors.New("platte voll")}
	x := &peer{}
	r := need(f.m.Create(xboxID, x, "voll", true, 0, []int{0}, Options{}))(t)
	ticks(r, 5)
	r.Leave(xboxID, x)
	f.wait(EmptyFor + time.Minute)
	if !r.Closed() || len(f.m.Rooms()) != 0 {
		t.Fatal("Raum nach Ablagefehler nicht aufgeräumt")
	}
	if !strings.Contains(buf.String(), "Spielmetrik-Report nicht geschrieben") || !strings.Contains(buf.String(), "platte voll") {
		t.Fatalf("Log ohne Fehler:\n%s", buf.String())
	}
}
