package main

import (
	"sync"
	"testing"
	"time"

	"k3c/tools/workkit/console"
)

// B-895/AC-01: 10 000 Zeilen in kurzer Zeit werden zu wenigen Sendungen, jede höchstens max Zeilen, die jüngste Zeile
// kommt immer an.
func TestLineBatcherBegrenzt(t *testing.T) {
	var mu sync.Mutex
	var sends, maxLen int
	var last int64
	done := make(chan struct{}, 100)
	b := newLineBatcher(func(ls []console.Line) {
		mu.Lock()
		sends++
		maxLen = max(maxLen, len(ls))
		last = ls[len(ls)-1].Seq
		mu.Unlock()
		done <- struct{}{}
	})
	b.window = 20 * time.Millisecond
	for i := 1; i <= 10000; i++ {
		b.add(console.Line{Seq: int64(i)})
	}
	select {
	case <-done:
	case <-time.After(2 * time.Second):
		t.Fatal("nichts gesendet")
	}
	time.Sleep(60 * time.Millisecond)
	mu.Lock()
	defer mu.Unlock()
	if sends > 3 {
		t.Errorf("%d Sendungen für 10 000 Zeilen, erwartet höchstens 3", sends)
	}
	if maxLen > batchMax {
		t.Errorf("Sendung mit %d Zeilen, Grenze %d", maxLen, batchMax)
	}
	if last != 10000 {
		t.Errorf("letzte gesendete Zeile %d, erwartet 10000", last)
	}
}
