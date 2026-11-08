package simexe

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// Die Test-EXE spielt bin/k3c-sim (nach <root>/bin kopiert); K3C_SIMEXE_FAKE wählt das Verhalten.
const fakeEnv = "K3C_SIMEXE_FAKE"

func TestMain(m *testing.M) {
	if mode := os.Getenv(fakeEnv); mode != "" && strings.HasPrefix(filepath.Base(os.Args[0]), "k3c-sim") {
		os.Exit(fake(mode))
	}
	os.Exit(m.Run())
}

func fake(mode string) int {
	switch mode {
	case "ok":
		raw := make([]byte, 64)
		n, _ := os.Stdin.Read(raw)
		fmt.Printf(`{"event":"result","text":%q}`+"\n", "eingabe "+string(raw[:n]))
	case "crash":
		fmt.Fprintln(os.Stderr, "panic: boom")
		return 2
	case "slow":
		time.Sleep(time.Minute)
	case "bot":
		fmt.Println(`{"event":"commands","commands":[{"player":1,"moveX":-1,"sprint":false,"pay":true,"attack":false,"skill":0}]}`)
		time.Sleep(time.Minute)
	}
	return 0
}

func install(t *testing.T, mode string) string {
	t.Helper()
	root := t.TempDir()
	self, err := os.ReadFile(os.Args[0])
	if err != nil {
		t.Fatal(err)
	}
	exe := filepath.Join(root, Rel(0))
	if err := os.MkdirAll(filepath.Dir(exe), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(exe, self, 0o755); err != nil {
		t.Fatal(err)
	}
	t.Setenv(fakeEnv, mode)
	return root
}

func TestCheckFehltUndVeraltet(t *testing.T) {
	root := t.TempDir()
	if err := Check(root); err == nil || !strings.Contains(err.Error(), "fehlt") || !strings.Contains(err.Error(), BuildHint) {
		t.Errorf("fehlt: %v", err)
	}
	root = install(t, "ok")
	if err := Check(root); err != nil {
		t.Fatal(err)
	}
	src := filepath.Join(root, "engine", "sim", "neu.go")
	if err := os.MkdirAll(filepath.Dir(src), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(src, []byte("package sim\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	future := time.Now().Add(time.Hour)
	_ = os.Chtimes(src, future, future)
	err := Check(root)
	if err == nil || !strings.Contains(err.Error(), "veraltet") || !strings.Contains(err.Error(), "engine/sim/neu.go") {
		t.Errorf("veraltet: %v", err)
	}
	test := filepath.Join(root, "engine", "sim", "neu.go")
	_ = os.Rename(test, filepath.Join(root, "engine", "sim", "neu_test.go"))
	if err := Check(root); err != nil {
		t.Errorf("Tests zählen nicht: %v", err)
	}
}

func TestCallErgebnisAbsturzZeitlimit(t *testing.T) {
	ev, err := Call(context.Background(), install(t, "ok"), []string{"run"}, []byte("abc"), 30*time.Second)
	if err != nil || ev.Text != "eingabe abc" {
		t.Errorf("ok: %+v %v", ev, err)
	}
	_, err = Call(context.Background(), install(t, "crash"), []string{"run"}, nil, 30*time.Second)
	if err == nil || !strings.Contains(err.Error(), "Exit 2") || !strings.Contains(err.Error(), "panic: boom") {
		t.Errorf("absturz: %v", err)
	}
	_, err = Call(context.Background(), install(t, "slow"), []string{"level"}, nil, 300*time.Millisecond)
	if err == nil || !strings.Contains(err.Error(), "Zeitlimit") {
		t.Errorf("zeitlimit: %v", err)
	}
}

func TestBotAntwortUndEnde(t *testing.T) {
	b, err := StartBot(context.Background(), install(t, "bot"), "saver")
	if err != nil {
		t.Fatal(err)
	}
	cmds, err := b.Decide(json.RawMessage(`{"players":[]}`), []int{1})
	if err != nil || len(cmds) != 1 || cmds[0].Player != 1 || cmds[0].MoveX != -1 || !cmds[0].Pay {
		t.Errorf("antwort: %+v %v", cmds, err)
	}
	// die falsche EXE antwortet nur einmal: die zweite Anfrage scheitert erst beim Ende (Close beendet den Prozess)
	done := make(chan error, 1)
	go func() { _, err := b.Decide(json.RawMessage(`{}`), []int{0}); done <- err }()
	time.Sleep(100 * time.Millisecond)
	b.Close()
	if err := <-done; err == nil || !strings.Contains(err.Error(), "keine Antwort") {
		t.Errorf("nach Close: %v", err)
	}
}

// Hängt k3c-sim bot, endet Decide nach decideTimeout mit Fehler; danach antwortet der Bot nur noch mit diesem Fehler.
func TestBotZeitlimit(t *testing.T) {
	old := decideTimeout
	decideTimeout = 300 * time.Millisecond
	t.Cleanup(func() { decideTimeout = old })
	b, err := StartBot(context.Background(), install(t, "bot"), "saver")
	if err != nil {
		t.Fatal(err)
	}
	defer b.Close()
	if _, err := b.Decide(json.RawMessage(`{}`), []int{1}); err != nil {
		t.Fatal(err)
	}
	if _, err := b.Decide(json.RawMessage(`{}`), []int{1}); err == nil || !strings.Contains(err.Error(), "binnen") {
		t.Errorf("zeitlimit: %v", err)
	}
	if _, err := b.Decide(json.RawMessage(`{}`), []int{1}); err == nil || !strings.Contains(err.Error(), "binnen") {
		t.Errorf("danach: %v", err)
	}
}
