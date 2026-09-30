package mcpsrv

import (
	"sort"
	"sync"
	"time"
)

const (
	// callRingSize ist die Länge des Aufruf-Logs.
	callRingSize = 200
	// textRunes kürzt Argumente, Fehler und Zusammenfassungen.
	textRunes = 120
	// timeLayout ist das Zeitformat aller Zeitstempel für die Oberfläche.
	timeLayout = "2006-01-02T15:04:05.000Z07:00"
)

// ToolStats sind die Zähler eines Tools seit dem Programmstart.
type ToolStats struct {
	Name        string  `json:"name"`
	Description string  `json:"description"`
	Calls       int     `json:"calls"`
	Errors      int     `json:"errors"`
	AvgMs       float64 `json:"avgMs"`
	LastCall    string  `json:"lastCall"`
}

// Call ist ein Eintrag im Aufruf-Log. StartSeq und EndSeq stammen aus einer gemeinsamen Folge über alle Start-
// und End-Ereignisse; damit verbindet die Oberfläche Start- und Endzeile eines Aufrufs.
type Call struct {
	ID         int64   `json:"id"`
	Ref        string  `json:"ref"`
	StartedAt  string  `json:"startedAt"`
	TS         string  `json:"ts"`
	StartSeq   int64   `json:"startSeq"`
	EndSeq     int64   `json:"endSeq"`
	Running    bool    `json:"running"`
	Tool       string  `json:"tool"`
	Args       string  `json:"args"`
	DurationMs float64 `json:"durationMs"`
	OK         bool    `json:"ok"`
	Error      string  `json:"error,omitempty"`
	Summary    string  `json:"summary"`
}

// Snapshot sind alle Zähler für die Oberfläche; Tools in Katalog-Reihenfolge.
type Snapshot struct {
	StartedAt    string      `json:"startedAt"`
	Clients      int         `json:"clients"`
	PeakClients  int         `json:"peakClients"`
	InFlight     int         `json:"inFlight"`
	PeakInFlight int         `json:"peakInFlight"`
	TotalCalls   int         `json:"totalCalls"`
	Errors       int         `json:"errors"`
	Tools        []ToolStats `json:"tools"`
}

// outcome ist das Ergebnis eines beendeten Aufrufs.
type outcome struct {
	ok      bool
	err     string
	summary string
}

type toolCounter struct {
	desc     string
	calls    int
	errors   int
	sumMs    float64
	lastCall string
}

type runningCall struct {
	call  Call
	start time.Time
}

// stats zählt Aufrufe und hält das Aufruf-Log; alle Methoden sind nebenläufig sicher.
type stats struct {
	mu        sync.Mutex
	now       func() time.Time
	startedAt time.Time
	order     []string
	tools     map[string]*toolCounter
	ring      []Call
	next      int
	running   map[int64]runningCall
	nextID    int64
	seq       int64

	total, errors          int
	inFlight, peakInFlight int
	clients, peakClients   int

	onStart, onCall func(Call)
}

func newStats(now func() time.Time) *stats {
	return &stats{now: now, startedAt: now(), tools: map[string]*toolCounter{}, running: map[int64]runningCall{}}
}

// register nimmt ein Tool aus dem Katalog auf, damit es auch ohne Aufruf in den Zählern steht.
func (st *stats) register(name, desc string) {
	st.mu.Lock()
	defer st.mu.Unlock()
	st.order = append(st.order, name)
	st.tools[name] = &toolCounter{desc: desc}
}

// begin trägt einen laufenden Aufruf ein und liefert seine ID.
func (st *stats) begin(tool, args string) int64 {
	st.mu.Lock()
	now := st.now()
	st.nextID++
	st.seq++
	c := Call{ID: st.nextID, Ref: ref(st.nextID), StartedAt: now.Format(timeLayout), StartSeq: st.seq,
		Running: true, Tool: tool, Args: clip(args)}
	st.running[c.ID] = runningCall{call: c, start: now}
	st.inFlight++
	st.peakInFlight = max(st.peakInFlight, st.inFlight)
	fn := st.onStart
	st.mu.Unlock()
	if fn != nil {
		fn(c)
	}
	return c.ID
}

// end schließt einen Aufruf ab, zählt ihn, schiebt ihn ins Aufruf-Log und liefert ihn zurück.
func (st *stats) end(id int64, o outcome) (Call, bool) {
	st.mu.Lock()
	r, found := st.running[id]
	if !found {
		st.mu.Unlock()
		return Call{}, false
	}
	delete(st.running, id)
	now := st.now()
	st.seq++
	c := r.call
	c.TS, c.EndSeq, c.Running = now.Format(timeLayout), st.seq, false
	c.DurationMs = float64(now.Sub(r.start).Microseconds()) / 1000
	c.OK, c.Error, c.Summary = o.ok, clip(o.err), clip(o.summary)
	st.count(c)
	st.push(c)
	fn := st.onCall
	st.mu.Unlock()
	if fn != nil {
		fn(c)
	}
	return c, true
}

// count zählt einen beendeten Aufruf; ein Tool außerhalb des Katalogs zählt nur in den Summen.
func (st *stats) count(c Call) {
	st.inFlight--
	st.total++
	if !c.OK {
		st.errors++
	}
	t, known := st.tools[c.Tool]
	if !known {
		return
	}
	t.calls++
	t.sumMs += c.DurationMs
	t.lastCall = c.TS
	if !c.OK {
		t.errors++
	}
}

func (st *stats) push(c Call) {
	if len(st.ring) < callRingSize {
		st.ring = append(st.ring, c)
		return
	}
	st.ring[st.next] = c
	st.next = (st.next + 1) % callRingSize
}

func (st *stats) observeClients(n int) {
	st.mu.Lock()
	defer st.mu.Unlock()
	st.clients = n
	st.peakClients = max(st.peakClients, n)
}

// calls liefert beendete und laufende Aufrufe, neueste zuerst (nach StartSeq).
func (st *stats) calls() []Call {
	st.mu.Lock()
	out := make([]Call, 0, len(st.ring)+len(st.running))
	out = append(out, st.ring...)
	for _, r := range st.running {
		out = append(out, r.call)
	}
	st.mu.Unlock()
	sort.Slice(out, func(i, j int) bool { return out[i].StartSeq > out[j].StartSeq })
	return out
}

func (st *stats) snapshot() Snapshot {
	st.mu.Lock()
	defer st.mu.Unlock()
	out := Snapshot{StartedAt: st.startedAt.Format(timeLayout), Clients: st.clients, PeakClients: st.peakClients,
		InFlight: st.inFlight, PeakInFlight: st.peakInFlight, TotalCalls: st.total, Errors: st.errors,
		Tools: make([]ToolStats, 0, len(st.order))}
	for _, name := range st.order {
		t := st.tools[name]
		ts := ToolStats{Name: name, Description: t.desc, Calls: t.calls, Errors: t.errors, LastCall: t.lastCall}
		if t.calls > 0 {
			ts.AvgMs = t.sumMs / float64(t.calls)
		}
		out.Tools = append(out.Tools, ts)
	}
	return out
}

func (st *stats) uptime() time.Duration { return st.now().Sub(st.startedAt) }
