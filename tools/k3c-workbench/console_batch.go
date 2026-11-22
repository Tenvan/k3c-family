package main

import (
	"sync"
	"time"

	"k3c/tools/workkit/console"
)

// Bündelung von console:line (B-895): Jede Zeile einzeln an die Oberfläche zu schicken, brachte den WebView2-Renderer
// bei parallelen Prüfläufen auf über 1 GiB, bis das Fenster leer blieb. Der Bündler sammelt Zeilen höchstens
// batchWindow lang und schickt dann eine Liste; mehr als batchMax Zeilen je Fenster schickt er nur die jüngsten. Die
// Lücke in `seq` erkennt die Oberfläche selbst und lädt den Rest über consoleTail nach (logs/lines.ts › mergeLines).
const (
	batchWindow = 200 * time.Millisecond
	batchMax    = 400
)

type lineBatcher struct {
	mu      sync.Mutex
	lines   []console.Line
	pending bool
	window  time.Duration
	max     int
	send    func([]console.Line)
}

func newLineBatcher(send func([]console.Line)) *lineBatcher {
	return &lineBatcher{window: batchWindow, max: batchMax, send: send}
}

// add nimmt eine Zeile auf; die erste Zeile eines Fensters plant das Senden.
func (b *lineBatcher) add(l console.Line) {
	b.mu.Lock()
	defer b.mu.Unlock()
	b.lines = append(b.lines, l)
	if len(b.lines) > b.max {
		b.lines = append(b.lines[:0], b.lines[len(b.lines)-b.max:]...)
	}
	if !b.pending {
		b.pending = true
		time.AfterFunc(b.window, b.flush)
	}
}

// flush schickt die gesammelten Zeilen als eine Liste.
func (b *lineBatcher) flush() {
	b.mu.Lock()
	out := b.lines
	b.lines, b.pending = nil, false
	b.mu.Unlock()
	if len(out) > 0 {
		b.send(out)
	}
}
