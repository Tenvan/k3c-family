package room

import (
	"testing"
	"time"
	"unsafe"
)

// B-281/AC-01: Der Ring wird über seine Größe gefüllt, die Länge bleibt fest, die ältesten Punkte fallen raus.
func TestRingFestBegrenzt(t *testing.T) {
	r := newRing[RoomPoint](Window)
	for i := range Window + 10 {
		r.add(RoomPoint{T: int64(i + 1)})
	}
	all := r.since(0)
	if len(all) != Window || len(r.buf) != Window || cap(r.buf) != Window {
		t.Fatalf("Länge %d, Puffer %d/%d, erwartet %d", len(all), len(r.buf), cap(r.buf), Window)
	}
	if all[0].T != 11 || all[Window-1].T != Window+10 || r.last() != Window+10 {
		t.Errorf("ältester %d, neuester %d: die ersten 10 Punkte müssen raus sein", all[0].T, all[Window-1].T)
	}
	if got := r.since(Window + 5); len(got) != 5 || got[0].T != Window+6 {
		t.Errorf("since: %v", got)
	}
}

// Der Tick zählt nur; der Punkt je Sekunde liest max, p99 und Ticks über Budget des Intervalls aus den Tick-Dauern.
func TestRaumPunktAusIntervall(t *testing.T) {
	f := newFixture()
	r := need(f.m.Create("a", &peer{}, "eins", true, 0, []int{0}, Options{}))(t)
	r.mu.Lock()
	r.durations = []time.Duration{50 * time.Millisecond, time.Millisecond, 2 * time.Millisecond, 40 * time.Millisecond}
	r.secTicks = 3 // die erste Dauer gehört zum vorigen Intervall
	r.mu.Unlock()
	f.m.Sample()
	f.m.Sample() // ohne Tick kein neuer Punkt
	got := f.m.Monitor.Since(0, f.now).Rooms[r.Code]
	if len(got) != 1 || got[0].MaxMs != 40 || got[0].P99Ms != 40 || got[0].Over != 1 || got[0].T != f.now.UnixMilli() {
		t.Fatalf("Punkt %+v", got)
	}
	if s := f.m.Monitor.Since(0, f.now).Server; len(s) != 2 || s[0].Goroutines <= 0 || s[0].HeapMB <= 0 || s[0].CPU != -1 {
		t.Errorf("Server-Punkte %+v", s)
	}
}

// Reihen eines geschlossenen Raums oder getrennten Geräts bleiben bis zum Ende des Fensters, danach fallen sie weg.
func TestReiheEndetMitDemFenster(t *testing.T) {
	mon := NewMonitor(time.UnixMilli(0))
	mon.sample(1000, map[string]RoomPoint{"ABCD": {T: 1000}})
	mon.Device("geraet01", "ABCD", DevicePoint{T: 1000})
	mon.sample(1000+Window*1000, nil)
	if m := mon.Since(0, time.UnixMilli(0)); len(m.Rooms) != 1 || len(m.Devices) != 1 {
		t.Fatalf("Reihen nach genau einem Fenster: %d Räume, %d Geräte", len(m.Rooms), len(m.Devices))
	}
	mon.sample(1001+Window*1000, nil)
	if m := mon.Since(0, time.UnixMilli(0)); len(m.Rooms) != 0 || len(m.Devices) != 0 || len(m.Server) != 3 {
		t.Errorf("nach dem Fenster: %d Räume, %d Geräte, %d Server-Punkte", len(m.Rooms), len(m.Devices), len(m.Server))
	}
}

// B-281 › Regeln: Alle Puffer zusammen bleiben bei 4 Räumen × 4 Geräten unter 2 MB.
func TestMonitorSpeicherbudget(t *testing.T) {
	bytes := Window * (unsafe.Sizeof(ServerPoint{}) + 4*unsafe.Sizeof(RoomPoint{}) + 16*unsafe.Sizeof(DevicePoint{}))
	if bytes >= 2<<20 {
		t.Errorf("Puffer %d Bytes, erlaubt < 2 MB", bytes)
	}
}
