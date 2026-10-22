package main

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"time"

	"github.com/wailsapp/wails/v2/pkg/runtime"
)

// uiEventBuffer fasst die Ereignisse eines Stroms, die der Browser noch nicht abgeholt hat. Ein voller Puffer
// verwirft das Ereignis (die Oberfläche lädt Listen ohnehin beim Seitenwechsel neu); ein Ausbruch von
// console:line-Zeilen passt hinein.
const uiEventBuffer = 1024

// uiKeepAlive hält Proxys (Vite) und Browser vom Schließen eines ruhigen Stroms ab.
const uiKeepAlive = 20 * time.Second

// uiEvent ist ein Ereignis mit fertig kodierter Nutzlast.
type uiEvent struct {
	name string
	data []byte
}

// eventBus verteilt die Ereignisse von emit an die offenen SSE-Ströme.
type eventBus struct {
	mu   sync.Mutex
	subs map[chan uiEvent]struct{}
}

func newEventBus() *eventBus { return &eventBus{subs: map[chan uiEvent]struct{}{}} }

// publish kodiert die Nutzlast wie Wails: das erste Argument, ohne Argument null.
func (b *eventBus) publish(name string, data []any) {
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
		case c <- uiEvent{name: name, data: raw}:
		default:
		}
	}
}

func (b *eventBus) subscribe() (<-chan uiEvent, func()) {
	c := make(chan uiEvent, uiEventBuffer)
	b.mu.Lock()
	b.subs[c] = struct{}{}
	b.mu.Unlock()
	return c, func() { b.mu.Lock(); delete(b.subs, c); b.mu.Unlock() }
}

// emitAll ist emit der Produktion: an die Wails-Laufzeit nur im Fenster (sie beendet das Programm bei einem ctx ohne
// Wails), immer an den Bus für die Ströme der HTTP-Oberfläche.
func (a *App) emitAll(ctx context.Context, name string, data ...any) {
	if a.window {
		runtime.EventsEmit(ctx, name, data...)
	}
	a.bus.publish(name, data)
}

// uiEvents ist ein Server-Sent-Events-Strom: je Ereignis `event: <name>` und `data: <JSON>`.
func (a *App) uiEvents(w http.ResponseWriter, r *http.Request) {
	fl, ok := w.(http.Flusher)
	if !ok {
		uiFail(w, http.StatusInternalServerError, "kein Strom möglich")
		return
	}
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-store")
	ch, unsub := a.bus.subscribe()
	defer unsub()
	_, _ = fmt.Fprint(w, ": offen\n\n")
	fl.Flush()
	tick := time.NewTicker(uiKeepAlive)
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
