package room

import (
	"fmt"
	"maps"
	"runtime"
	"slices"
	"strings"
	"sync"
	"time"
)

// Messreihen des Servers (B-281, Glossar „Messreihe“): ein Punkt je Sekunde und Kennzahl in einem Ring-Puffer fester
// Größe. Der Tick zählt nur seine Ticks (safeTick); ein eigener 1-s-Ticker in Manager.Run liest außerhalb des Raum-Ticks
// die Tick-Dauern der Räume und die Laufzeitwerte. Geräte schreiben ihre Punkte selbst (engine/net/ping.go). Punkte sind
// kompakt (float32, Zeit in Unix-ms), damit 4 Räume × 4 Geräte unter 2 MB bleiben (TestMonitorSpeicherbudget).

// Window ist die Zahl der Punkte je Reihe: 1 h bei 1 s (Beschluss 🧑 2026-10-05). Eine Reihe, deren letzter Punkt älter
// als das Fenster ist, fällt weg.
const Window = 3600

// sampleEvery ist der Abstand der Punkte.
const sampleEvery = time.Second

// ServerPoint sind die Laufzeitwerte des Prozesses in einem Intervall.
type ServerPoint struct {
	T          int64   `json:"t"` // Unix-ms
	HeapMB     float32 `json:"heapMB"`
	GCPauseMs  float32 `json:"gcPauseMs"` // Summe der GC-Pausen im Intervall
	Goroutines int32   `json:"goroutines"`
	CPU        float32 `json:"cpu"`    // Prozent einer CPU; -1 = unbekannt (nur Linux misst)
	SaveMs     float32 `json:"saveMs"` // längste Speicherung eines Spielstands im Intervall, 0 = keine
}

// RoomPoint sind die Tick-Dauern eines Raums in einem Intervall.
type RoomPoint struct {
	T     int64   `json:"t"`
	MaxMs float32 `json:"maxMs"`
	P99Ms float32 `json:"p99Ms"`
	Over  int32   `json:"over"` // Ticks über Budget (länger als ein Takt)
}

// DevicePoint ist die Verbindung eines Geräts zum Zeitpunkt eines Pings.
type DevicePoint struct {
	T       int64   `json:"t"`
	RTTMs   float32 `json:"rttMs"` // -1 = kein Pong
	Queue   uint16  `json:"queue"`
	Dropped uint16  `json:"dropped"` // verworfene Zustände seit dem letzten Punkt
}

// Event ist ein Diagnose-Ereignis (Glossar): Absturz, 🐢-Tick, Trennung oder Client-Fehler.
type Event struct {
	T    int64  `json:"t"`
	Room string `json:"room"` // leer = ohne Raum (Client-Fehler)
	Kind string `json:"kind"` // crash, slow, drop, client
	Text string `json:"text"` // höchstens eventText Bytes
}

// Diagnose-Ereignisse: die letzten Events im Ring, Text gekürzt.
const (
	Events    = 200
	eventText = 200
)

func (p ServerPoint) at() int64 { return p.T }
func (e Event) at() int64       { return e.T }
func (p RoomPoint) at() int64   { return p.T }
func (p DevicePoint) at() int64 { return p.T }

type stamped interface{ at() int64 }

// ring ist ein Ring-Puffer fester Größe; ist er voll, ersetzt ein neuer Punkt den ältesten.
type ring[T stamped] struct {
	buf  []T
	next int // ältester Punkt, sobald der Puffer voll ist
}

func newRing[T stamped](n int) *ring[T] { return &ring[T]{buf: make([]T, 0, n)} }

func (r *ring[T]) add(p T) {
	if len(r.buf) < cap(r.buf) {
		r.buf = append(r.buf, p)
		return
	}
	r.buf[r.next] = p
	r.next = (r.next + 1) % len(r.buf)
}

// since sind die Punkte mit Zeit nach t, älteste zuerst.
func (r *ring[T]) since(t int64) []T {
	out := []T{}
	for i := range r.buf {
		if p := r.buf[(r.next+i)%len(r.buf)]; p.at() > t {
			out = append(out, p)
		}
	}
	return out
}

// last ist die Zeit des neuesten Punkts; 0 = leer.
func (r *ring[T]) last() int64 {
	if len(r.buf) == 0 {
		return 0
	}
	return r.buf[(r.next+len(r.buf)-1)%len(r.buf)].at()
}

// DeviceSeries ist die Reihe eines Geräts (Kürzel) mit dem Raum seines letzten Punkts.
type DeviceSeries struct {
	Room   string        `json:"room"`
	Points []DevicePoint `json:"points"`
}

type deviceRing struct {
	room   string
	points *ring[DevicePoint]
}

// Metrics sind alle Punkte nach einem Zeitpunkt (Delta für /api/metrics). Reihen ohne neuen Punkt fehlen.
type Metrics struct {
	StartedAt int64                   `json:"startedAt"` // Start des Sammlers (Unix-ms)
	Now       int64                   `json:"now"`
	Server    []ServerPoint           `json:"server"`
	Rooms     map[string][]RoomPoint  `json:"rooms"`
	Devices   map[string]DeviceSeries `json:"devices"`
	Events    []Event                 `json:"events"`
}

// Monitor hält die Messreihen; alle Methoden sind sicher für mehrere Goroutinen. Eigene Sperre, nie über einer anderen.
type Monitor struct {
	// CPU liefert die CPU-Last seit dem letzten Aufruf (engine/net setzt einen eigenen Messer); nil = unbekannt.
	CPU func() (float64, bool)

	mu      sync.Mutex
	started int64
	server  *ring[ServerPoint]
	rooms   map[string]*ring[RoomPoint]
	devices map[string]*deviceRing
	events  *ring[Event]
	saveMax time.Duration // längste Speicherung seit dem letzten Punkt
	gcPause uint64        // PauseTotalNs beim letzten Punkt
	gcSeen  bool          // gcPause gesetzt (erster Punkt ohne GC-Pause)
}

// NewMonitor legt einen leeren Sammler an; started ist der Start des Servers.
func NewMonitor(started time.Time) *Monitor {
	return &Monitor{started: started.UnixMilli(), server: newRing[ServerPoint](Window), rooms: map[string]*ring[RoomPoint]{},
		devices: map[string]*deviceRing{}, events: newRing[Event](Events)}
}

// Event nimmt ein Diagnose-Ereignis auf; der Text wird auf eventText Bytes gekürzt.
func (mon *Monitor) Event(at time.Time, room, kind, text string) {
	if len(text) > eventText {
		text = strings.ToValidUTF8(text[:eventText], "") + "…"
	}
	mon.mu.Lock()
	defer mon.mu.Unlock()
	mon.events.add(Event{at.UnixMilli(), room, kind, text})
}

// Device hängt einen Punkt an die Reihe des Geräts (Kürzel) an.
func (mon *Monitor) Device(id, room string, p DevicePoint) {
	mon.mu.Lock()
	defer mon.mu.Unlock()
	d := mon.devices[id]
	if d == nil {
		d = &deviceRing{points: newRing[DevicePoint](Window)}
		mon.devices[id] = d
	}
	d.room = room
	d.points.add(p)
}

// saved merkt die Dauer einer Speicherung für den nächsten Punkt (saver.go).
func (mon *Monitor) saved(d time.Duration) {
	mon.mu.Lock()
	defer mon.mu.Unlock()
	mon.saveMax = max(mon.saveMax, d)
}

// Since liefert alle Punkte mit Zeit nach t (Unix-ms); t vor dem Start = alles.
func (mon *Monitor) Since(t int64, now time.Time) Metrics {
	mon.mu.Lock()
	defer mon.mu.Unlock()
	out := Metrics{StartedAt: mon.started, Now: now.UnixMilli(), Server: mon.server.since(t), Rooms: map[string][]RoomPoint{},
		Devices: map[string]DeviceSeries{}, Events: mon.events.since(t)}
	for code, r := range mon.rooms {
		if p := r.since(t); len(p) > 0 {
			out.Rooms[code] = p
		}
	}
	for id, d := range mon.devices {
		if p := d.points.since(t); len(p) > 0 {
			out.Devices[id] = DeviceSeries{d.room, p}
		}
	}
	return out
}

// sample nimmt einen Punkt für Server und Räume auf und entfernt Reihen ohne Punkt im Fenster.
func (mon *Monitor) sample(now int64, rooms map[string]RoomPoint) {
	var mem runtime.MemStats
	runtime.ReadMemStats(&mem)
	cpu := float32(-1)
	if mon.CPU != nil {
		if pct, ok := mon.CPU(); ok {
			cpu = float32(pct)
		}
	}
	mon.mu.Lock()
	defer mon.mu.Unlock()
	p := ServerPoint{T: now, HeapMB: float32(mem.HeapAlloc) / (1 << 20), Goroutines: int32(runtime.NumGoroutine()), CPU: cpu,
		SaveMs: ms32(mon.saveMax)}
	if mon.gcSeen {
		p.GCPauseMs = ms32(time.Duration(mem.PauseTotalNs - mon.gcPause))
	}
	mon.gcPause, mon.gcSeen, mon.saveMax = mem.PauseTotalNs, true, 0
	mon.server.add(p)
	for code, rp := range rooms {
		if mon.rooms[code] == nil {
			mon.rooms[code] = newRing[RoomPoint](Window)
		}
		mon.rooms[code].add(rp)
	}
	old := now - Window*sampleEvery.Milliseconds()
	maps.DeleteFunc(mon.rooms, func(_ string, r *ring[RoomPoint]) bool { return r.last() < old })
	maps.DeleteFunc(mon.devices, func(_ string, d *deviceRing) bool { return d.points.last() < old })
}

func ms32(d time.Duration) float32 { return float32(d) / float32(time.Millisecond) }

// interval sind max, p99 und Ticks über Budget seit dem letzten Punkt (unter Raum-Sperre); setzt den Zähler zurück.
// false: kein Tick im Intervall.
func (r *Room) interval(now int64) (RoomPoint, bool) {
	n := min(r.secTicks, len(r.durations))
	r.secTicks = 0
	if n == 0 {
		return RoomPoint{}, false
	}
	tail := slices.Clone(r.durations[len(r.durations)-n:])
	slices.Sort(tail)
	p := RoomPoint{T: now, MaxMs: ms32(tail[n-1]), P99Ms: ms32(tail[n*99/100])}
	for _, d := range tail {
		if d > slowTick {
			p.Over++
		}
	}
	return p, true
}

// Sample liest je Raum das Intervall und nimmt einen Punkt auf (Manager.Run einmal je sampleEvery; exportiert für den
// Benchmark in engine/net).
func (m *Manager) Sample() {
	defer func() {
		if p := recover(); p != nil {
			m.log().Error("💥 Messreihe abgestürzt", "ns", "room", "err", fmt.Sprint(p))
		}
	}()
	now := m.now().UnixMilli()
	rooms := map[string]RoomPoint{}
	m.mu.Lock()
	for code, r := range m.rooms {
		r.mu.Lock()
		if p, ok := r.interval(now); ok {
			rooms[code] = p
		}
		r.mu.Unlock()
	}
	m.mu.Unlock()
	m.Monitor.sample(now, rooms)
}
