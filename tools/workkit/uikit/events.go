package uikit

import (
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"time"
)

// eventBuffer fasst die Ereignisse eines Stroms, die der Browser noch nicht abgeholt hat. Ein voller Puffer verwirft
// das Ereignis (die Oberfläche lädt Listen ohnehin beim Seitenwechsel neu); ein Ausbruch von console:line-Zeilen passt
// hinein.
const eventBuffer = 1024

// keepAlive hält Proxys (Vite) und Browser vom Schließen eines ruhigen Stroms ab.
const keepAlive = 20 * time.Second

// event ist ein Ereignis mit fertig kodierter Nutzlast.
type event struct {
	name string
	data []byte
}

// Bus verteilt die Ereignisse der App an die offenen SSE-Ströme.
type Bus struct {
	mu   sync.Mutex
	subs map[chan event]struct{}
}

// NewBus legt einen leeren Bus an.
func NewBus() *Bus { return &Bus{subs: map[chan event]struct{}{}} }

// Publish kodiert die Nutzlast wie Wails: das erste Argument, ohne Argument null.
func (b *Bus) Publish(name string, data []any) {
	var payload any
	if len(data) > 0 {
		payload = data[0]
	}
	raw, err := json.Marshal(payload)
	if err != nil {
		raw = []byte("null")
	}
	b.mu.Lock()
	defer b.mu.Unlock()
	for c := range b.subs {
		select {
		case c <- event{name: name, data: raw}:
		default:
		}
	}
}

func (b *Bus) subscribe() (<-chan event, func()) {
	c := make(chan event, eventBuffer)
	b.mu.Lock()
	b.subs[c] = struct{}{}
	b.mu.Unlock()
	return c, func() { b.mu.Lock(); delete(b.subs, c); b.mu.Unlock() }
}

// serve ist ein Server-Sent-Events-Strom: je Ereignis `event: <name>` und `data: <JSON>`.
func (b *Bus) serve(w http.ResponseWriter, r *http.Request) {
	fl, ok := w.(http.Flusher)
	if !ok {
		Fail(w, http.StatusInternalServerError, "kein Strom möglich")
		return
	}
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-store")
	ch, unsub := b.subscribe()
	defer unsub()
	_, _ = fmt.Fprint(w, ": offen\n\n")
	fl.Flush()
	tick := time.NewTicker(keepAlive)
	defer tick.Stop()
	for {
		select {
		case <-r.Context().Done():
			return
		case ev := <-ch:
			_, _ = fmt.Fprintf(w, "event: %s\ndata: %s\n\n", ev.name, ev.data)
		case <-tick.C:
			_, _ = fmt.Fprint(w, ": halten\n\n")
		}
		fl.Flush()
	}
}
