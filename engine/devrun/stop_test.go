package devrun

import (
	"context"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// startChild baut das Testprogramm und startet es als Kind eines Runners (ohne Run-Schleife).
func startChild(t *testing.T, grace time.Duration, env ...string) (*runner, *recorder, string) {
	t.Helper()
	src, state := newAppDirs(t)
	rec := &recorder{}
	r, err := newRunner(Config{Name: "test", Dir: src, Package: ".", Child: filepath.Join("bin", "app-dev.exe"),
		Args: []string{state}, Env: env, Grace: grace, Log: slog.New(rec)})
	if err != nil {
		t.Fatal(err)
	}
	if err := r.build(context.Background(), r.child); err != nil {
		t.Fatal(err)
	}
	r.start()
	t.Cleanup(func() { r.stop(nil) })
	waitUntil(t, "Kind gestartet", func() bool {
		b, _ := os.ReadFile(filepath.Join(state, "log.txt"))
		return strings.Contains(string(b), "a start 1")
	})
	return r, rec, state
}

// Ein zweites Signal während der Frist beendet das Kind sofort hart, ohne die Frist abzuwarten.
func TestStopBrichtBeiZweitemSignalAb(t *testing.T) {
	r, rec, _ := startChild(t, time.Minute, "IGNORE_INT=1")
	abort := make(chan os.Signal, 1)
	go func() {
		time.Sleep(300 * time.Millisecond)
		abort <- os.Interrupt
	}()
	began := time.Now()
	r.stop(abort)
	if d := time.Since(began); d > 10*time.Second {
		t.Fatalf("stop dauerte %v trotz zweitem Signal", d)
	}
	if rec.count("🛑 Kind hart beendet") != 1 {
		t.Fatalf("Kind nicht hart beendet: %q", rec.all())
	}
}

// Ein schon beendetes Kind stoppt sofort, ohne Signal und ohne hartes Beenden.
func TestStopKehrtBeiBeendetemKindSofortZurueck(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	r, rec, state := startChild(t, time.Minute)
	ch := r.cur
	write(t, filepath.Join(state, "exit"), "0")
	<-ch.done
	r.stop(nil)
	if n := rec.count("🛑"); n != 0 {
		t.Fatalf("stop hat ein beendetes Kind behandelt: %q", rec.all())
	}
}

// Nach loopMax schnellen Neustarts mit RestartCode wartet Run auf die nächste Änderung; ein Build setzt die Bremse zurück.
func TestRunBremstNeustartSchleife(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	a := startApp(t, 75, "EXIT_AT_START=75")
	a.waitFor("🚫-Log", func() bool { return a.rec.count("🚫 zu viele Neustarts") > 0 })
	time.Sleep(restartPause + 500*time.Millisecond)
	if n := a.count("a start 1x"); n != loopMax+1 {
		t.Fatalf("%d Starts, will %d (Start und %d Neustarts): %q", n, loopMax+1, loopMax, a.lines())
	}
	a.version("bb")
	a.waitLine("bb start 1x", 1)
}

// Das Kind merkt sich K3C_DEV_CHILD und entfernt es, damit seine eigenen Kinder nicht als devrun-Kind gelten.
func TestTakeChildEnv(t *testing.T) {
	t.Setenv(childEnv, "1")
	if !takeChildEnv() {
		t.Fatal("K3C_DEV_CHILD=1 nicht erkannt")
	}
	if _, ok := os.LookupEnv(childEnv); ok {
		t.Fatal("K3C_DEV_CHILD noch in der Umgebung")
	}
	if takeChildEnv() {
		t.Fatal("ohne K3C_DEV_CHILD als Kind erkannt")
	}
}
