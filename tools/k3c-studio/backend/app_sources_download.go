package backend

import (
	"context"
	"errors"
	"net/http"
	"net/url"
	"sync"
	"time"

	assets "k3c/tools/assetkit"
)

const (
	// sourceTimeout begrenzt einen Download (Liste: höchstens 50 MB je Datei).
	sourceTimeout = 5 * time.Minute
	// sourceProbeTimeout und probeCache: Erreichbarkeitsprobe für „offline“, höchstens alle 30 s. OpenGameArt braucht
	// für die Antwort oft über 2 s, deshalb 5 s.
	sourceProbeTimeout = 5 * time.Second
	probeCache         = 30 * time.Second
)

// probeURLs sind die Ziele der Probe, parallel gefragt; eine Antwort genügt (die Quellen der Vorschlagsliste).
var probeURLs = []string{"https://opengameart.org/", "https://kenney.nl/", "https://freesound.org/"}

// sourceState hält Downloads, Fortschritt und die Offline-Probe eines Studios.
type sourceState struct {
	client *http.Client // Test-Naht: httptest-Server statt Internet
	probes []string     // Adressen der Erreichbarkeitsprobe
	mode   string       // NetworkAuto (Probe), NetworkOnline oder NetworkOffline (Schalter, app_network.go)

	mu       sync.Mutex
	busy     bool
	order    []string
	progress map[string]*SourceProgress
	faults   map[string]string // letzter Fehler je Eintrag (Eintrag bleibt offen)
	checked  time.Time         // Zeit der letzten Probe
	down     bool              // Ergebnis der letzten Probe oder Netzfehler des letzten Downloads
}

// sourceStates hält den Zustand je App (App selbst bleibt unverändert, angelegt beim ersten Gebrauch).
var sourceStates sync.Map

func (a *App) sourceState() *sourceState {
	v, _ := sourceStates.LoadOrStore(a, &sourceState{
		client:   &http.Client{Timeout: sourceTimeout, CheckRedirect: assets.VorratRedirect},
		probes:   probeURLs,
		progress: map[string]*SourceProgress{},
		faults:   map[string]string{},
	})
	return v.(*sourceState)
}

// offline: die Probe (HEAD, 2 s) scheitert oder der letzte Download scheiterte am Netz; das Ergebnis gilt 30 s.
func (st *sourceState) offline(ctx context.Context) bool {
	st.mu.Lock()
	if st.mode == NetworkOnline || st.mode == NetworkOffline {
		off := st.mode == NetworkOffline
		st.mu.Unlock()
		return off
	}
	if !st.checked.IsZero() && time.Since(st.checked) < probeCache {
		down := st.down
		st.mu.Unlock()
		return down
	}
	st.mu.Unlock()
	down := !st.reachable(ctx)
	st.mu.Lock()
	st.checked, st.down = time.Now(), down
	st.mu.Unlock()
	return down
}

// reachable fragt alle Probe-Ziele parallel: jede HTTP-Antwort zählt, nur Netzfehler oder Frist bei allen nicht.
func (st *sourceState) reachable(ctx context.Context) bool {
	ctx, cancel := context.WithTimeout(ctx, sourceProbeTimeout)
	defer cancel()
	ok := make(chan bool, len(st.probes))
	for _, u := range st.probes {
		go func() { ok <- st.answers(ctx, u) }()
	}
	for range st.probes {
		if <-ok {
			return true
		}
	}
	return false
}

// answers: Antwortet u innerhalb der Frist von ctx mit irgendeinem HTTP-Status?
func (st *sourceState) answers(ctx context.Context, u string) bool {
	req, err := http.NewRequestWithContext(ctx, http.MethodHead, u, nil)
	if err != nil {
		return false
	}
	res, err := st.client.Do(req)
	if err != nil {
		return false
	}
	_ = res.Body.Close()
	return true
}

// begin legt den Fortschritt der Einträge an; false, wenn schon ein Download läuft.
func (st *sourceState) begin(todo []assets.Suggestion) bool {
	st.mu.Lock()
	defer st.mu.Unlock()
	if st.busy {
		return false
	}
	st.busy, st.order, st.progress = true, nil, map[string]*SourceProgress{}
	for _, s := range todo {
		st.order = append(st.order, s.ID)
		st.progress[s.ID] = &SourceProgress{ID: s.ID, Status: "wartet", Total: s.Size}
	}
	return true
}

// run lädt die Einträge nacheinander; ein Fehler bleibt am Eintrag stehen, ein Netzfehler meldet offline.
func (st *sourceState) run(ctx context.Context, root string, todo []assets.Suggestion) {
	defer func() { st.mu.Lock(); st.busy = false; st.mu.Unlock() }()
	for _, s := range todo {
		st.set(s.ID, func(p *SourceProgress) { p.Status = "laedt" })
		_, err := assets.VorratDownload(ctx, root, s, st.client, time.Now(), func(n int64) {
			st.set(s.ID, func(p *SourceProgress) { p.Loaded = n })
		})
		st.finish(s.ID, err)
	}
}

func (st *sourceState) set(id string, change func(*SourceProgress)) {
	st.mu.Lock()
	defer st.mu.Unlock()
	if p := st.progress[id]; p != nil {
		change(p)
	}
}

func (st *sourceState) finish(id string, err error) {
	st.mu.Lock()
	defer st.mu.Unlock()
	p := st.progress[id]
	if err == nil {
		delete(st.faults, id)
		p.Status, p.Loaded = "fertig", p.Total
		return
	}
	p.Status, p.Error = "fehler", err.Error()
	st.faults[id] = err.Error()
	var ue *url.Error
	if errors.As(err, &ue) {
		st.checked, st.down = time.Now(), true
	}
}

func (st *sourceState) snapshot() []SourceProgress {
	st.mu.Lock()
	defer st.mu.Unlock()
	out := make([]SourceProgress, 0, len(st.order))
	for _, id := range st.order {
		out = append(out, *st.progress[id])
	}
	return out
}

func (st *sourceState) errors() map[string]string {
	st.mu.Lock()
	defer st.mu.Unlock()
	out := make(map[string]string, len(st.faults))
	for k, v := range st.faults {
		out[k] = v
	}
	return out
}
