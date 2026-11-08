package watch

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func waitSignal(t *testing.T, c <-chan struct{}) bool {
	t.Helper()
	select {
	case <-c:
		return true
	case <-time.After(3 * time.Second):
		return false
	}
}

func TestNotifyMeldetAenderungUndNeuenOrdner(t *testing.T) {
	dir := t.TempDir()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	c, err := Notify(ctx, []Root{{Name: "data", FS: os.DirFS(dir), Dir: dir}})
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "a.json"), []byte("{}"), 0o644); err != nil {
		t.Fatal(err)
	}
	if !waitSignal(t, c) {
		t.Fatal("kein Signal für neue Datei")
	}
	sub := filepath.Join(dir, "neu")
	if err := os.Mkdir(sub, 0o755); err != nil {
		t.Fatal(err)
	}
	if !waitSignal(t, c) {
		t.Fatal("kein Signal für neuen Ordner")
	}
	time.Sleep(200 * time.Millisecond)
	for len(c) > 0 {
		<-c
	}
	if err := os.WriteFile(filepath.Join(sub, "b.png"), []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}
	if !waitSignal(t, c) {
		t.Fatal("kein Signal für Datei im neuen Ordner")
	}
}

func TestNotifyIgnoriertTempDateien(t *testing.T) {
	dir := t.TempDir()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	c, err := Notify(ctx, []Root{{Name: "data", FS: os.DirFS(dir), Dir: dir}})
	if err != nil {
		t.Fatal(err)
	}
	_ = os.WriteFile(filepath.Join(dir, ".tmp-x"), []byte("x"), 0o644)
	_ = os.WriteFile(filepath.Join(dir, "a.paper.json"), []byte("x"), 0o644)
	select {
	case <-c:
		t.Fatal("Temp-Datei oder Paper-Spec hat ein Signal ausgelöst")
	case <-time.After(500 * time.Millisecond):
	}
}

func TestNotifyOhneOrdnerMeldetFehler(t *testing.T) {
	if _, err := Notify(context.Background(), []Root{{Name: "x"}}); err == nil {
		t.Fatal("Fehler erwartet")
	}
}

// Mit wake wird im Leerlauf nicht abgefragt: kein Poll, also keine Änderung erkannt, bis ein Signal kommt.
func TestRunMitWakeFragtImLeerlaufNichtAb(t *testing.T) {
	dir := t.TempDir()
	w := New([]Root{{Name: "d", FS: os.DirFS(dir)}}, 0, time.Time{})
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	tick, wake := make(chan time.Time), make(chan struct{}, 1)
	fired := make(chan []string, 1)
	go Run(ctx, w, tick, wake, func(p []string) { fired <- p })
	now := time.Now()
	tick <- now // erster Takt: Grundabfrage (dirty startet true)
	tick <- now // ungepuffert: kehrt erst zurück, wenn die Grundabfrage fertig ist (sonst sieht sie schon a.json)
	_ = os.WriteFile(filepath.Join(dir, "a.json"), []byte("{}"), 0o644)
	tick <- now.Add(time.Second)
	select {
	case p := <-fired:
		t.Fatalf("ohne Signal gemeldet: %v", p)
	case <-time.After(200 * time.Millisecond):
	}
	wake <- struct{}{}
	time.Sleep(50 * time.Millisecond)
	tick <- now.Add(2 * time.Second)
	select {
	case p := <-fired:
		if len(p) != 1 || p[0] != "d/a.json" {
			t.Fatalf("Pfade: %v", p)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("nach Signal nicht gemeldet")
	}
}

func BenchmarkPollLeerlaufAllokation(b *testing.B) {
	dir := b.TempDir()
	for i := 0; i < 200; i++ {
		_ = os.WriteFile(filepath.Join(dir, "f"+string(rune('a'+i%26))+string(rune('a'+i/26))+".json"), []byte("{}"), 0o644)
	}
	w := New([]Root{{Name: "d", FS: os.DirFS(dir)}}, 0, time.Time{})
	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		w.Poll(time.Now())
	}
}

// Leerlauf mit Ereignissen: Takte ohne Signal fragen nicht ab und allokieren nicht (0 allocs/op).
func BenchmarkRunLeerlaufMitWake(b *testing.B) {
	dir := b.TempDir()
	for i := 0; i < 200; i++ {
		_ = os.WriteFile(filepath.Join(dir, "f"+string(rune('a'+i%26))+string(rune('a'+i/26))+".json"), []byte("{}"), 0o644)
	}
	w := New([]Root{{Name: "d", FS: os.DirFS(dir)}}, 0, time.Time{})
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	tick, wake := make(chan time.Time), make(chan struct{})
	go Run(ctx, w, tick, wake, func([]string) {})
	now := time.Now()
	tick <- now // Grundabfrage
	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		tick <- now.Add(time.Millisecond) // höchstens fullScanEvery nach dem Grundstand
	}
}
