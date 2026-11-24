package gameexe

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"net"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

// TestHelperProcess ist kein Test: Das Test-Binary spielt die EXE (K3C_HELPER wählt das Verhalten).
func TestHelperProcess(t *testing.T) {
	mode := os.Getenv("K3C_HELPER")
	switch mode {
	case "":
		return
	case "lines": // Eingabedatei lesen und als result zurückgeben
		in := argAfter("--input")
		b, err := os.ReadFile(in)
		if err != nil {
			fmt.Println(`{"event":"error","message":"keine Eingabe"}`)
			os.Exit(2)
		}
		text, _ := json.Marshal(string(b))
		fmt.Println("kein JSON")
		fmt.Println(`{"event":"progress","done":1,"total":2}`)
		fmt.Printf("{\"event\":\"result\",\"text\":%s}\n", text)
		fmt.Fprintln(os.Stderr, "❌ Meldung")
		os.Exit(0)
	case "fail":
		fmt.Println(`{"event":"error","message":"kaputt"}`)
		os.Exit(1)
	case "wait": // wartet auf das Ende von stdin wie `--stop-stdin`
		fmt.Println(`{"event":"progress","done":1,"total":9}`)
		_, _ = bufio.NewReader(os.Stdin).ReadString(0)
		fmt.Println(`{"event":"aborted"}`)
		os.Exit(130)
	case "hang": // ignoriert stdin, startet einen Enkel mit offenem Port
		child := exec.Command(os.Args[0], "-test.run=^TestHelperProcess$")
		child.Env = append(os.Environ(), "K3C_HELPER=grandchild")
		if err := child.Start(); err != nil {
			os.Exit(2)
		}
		for range 100 { // erst melden, wenn der Enkel seinen Port geschrieben hat
			if b, err := os.ReadFile(os.Getenv("K3C_PORTFILE")); err == nil && len(b) > 0 {
				break
			}
			time.Sleep(50 * time.Millisecond)
		}
		fmt.Println(`{"event":"progress","done":1,"total":9}`)
		time.Sleep(time.Minute)
	case "grandchild":
		ln, err := net.Listen("tcp", "127.0.0.1:0")
		if err != nil {
			os.Exit(2)
		}
		_ = os.WriteFile(os.Getenv("K3C_PORTFILE"), []byte(ln.Addr().String()), 0o644)
		time.Sleep(time.Minute)
	}
	os.Exit(0)
}

func argAfter(flag string) string {
	for i, a := range os.Args {
		if a == flag && i+1 < len(os.Args) {
			return os.Args[i+1]
		}
	}
	return ""
}

// helperArgv ist die Befehlszeile, mit der das Test-Binary die EXE spielt.
func helperArgv(mode string, t *testing.T, extra ...string) []string {
	t.Helper()
	t.Setenv("K3C_HELPER", mode)
	return append([]string{os.Args[0], "-test.run=^TestHelperProcess$", "--"}, extra...)
}

func TestLinesUndInputDatei(t *testing.T) {
	root := t.TempDir()
	in := filepath.Join(root, "in.json")
	if err := os.WriteFile(in, []byte(`{"seed":1}`), 0o600); err != nil {
		t.Fatal(err)
	}
	var mu sync.Mutex
	var evs []Event
	var errLines []string
	o := Options{
		OnEvent:  func(ev Event) { mu.Lock(); evs = append(evs, ev); mu.Unlock() },
		OnStderr: func(l string) { mu.Lock(); errLines = append(errLines, l); mu.Unlock() },
	}
	err := run(context.Background(), helperArgv("lines", t, "--input", in), root, o)
	if err != nil {
		t.Fatal(err)
	}
	if len(evs) != 2 || evs[0].Event != "progress" || evs[0].Done != 1 || evs[1].Text != `{"seed":1}` {
		t.Errorf("Events: %+v", evs)
	}
	if len(errLines) != 1 || errLines[0] != "❌ Meldung" {
		t.Errorf("stderr: %v", errLines)
	}
}

func TestFehlerzeileUndExitCode(t *testing.T) {
	err := run(context.Background(), helperArgv("fail", t), t.TempDir(), Options{})
	if err == nil || err.Error() != "kaputt" {
		t.Errorf("Fehler = %v, will „kaputt“", err)
	}
}

func TestAbbruchUeberStdin(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	var evs []string
	var mu sync.Mutex
	o := Options{OnEvent: func(ev Event) {
		mu.Lock()
		evs = append(evs, ev.Event)
		mu.Unlock()
		if ev.Event == "progress" {
			cancel()
		}
	}}
	start := time.Now()
	err := run(ctx, helperArgv("wait", t), t.TempDir(), o)
	if err != context.Canceled {
		t.Fatalf("Fehler = %v, will context.Canceled", err)
	}
	if time.Since(start) > 4*time.Second {
		t.Errorf("Abbruch über stdin dauerte %v", time.Since(start))
	}
	mu.Lock()
	defer mu.Unlock()
	if len(evs) != 2 || evs[1] != "aborted" {
		t.Errorf("Events %v, will progress und aborted", evs)
	}
}

func TestHarterAbbruchNachFristRaeumtBaum(t *testing.T) {
	portFile := filepath.Join(t.TempDir(), "port")
	t.Setenv("K3C_PORTFILE", portFile)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	o := Options{Grace: 300 * time.Millisecond, OnEvent: func(Event) { cancel() }}
	err := run(ctx, helperArgv("hang", t), t.TempDir(), o)
	if err != context.Canceled {
		t.Fatalf("Fehler = %v, will context.Canceled", err)
	}
	var addr string
	for range 100 {
		if b, err := os.ReadFile(portFile); err == nil && len(b) > 0 {
			addr = strings.TrimSpace(string(b))
			break
		}
		time.Sleep(50 * time.Millisecond)
	}
	if addr == "" {
		t.Fatal("Enkel hat keinen Port gemeldet")
	}
	for range 50 {
		conn, err := net.DialTimeout("tcp", addr, 200*time.Millisecond)
		if err != nil {
			return // Enkel beendet
		}
		_ = conn.Close()
		time.Sleep(100 * time.Millisecond)
	}
	t.Errorf("Enkel an %s läuft nach dem Abbruch noch", addr)
}

func TestStreamOhneExeNenntBauBefehl(t *testing.T) {
	err := Stream(context.Background(), Options{Root: t.TempDir(), Sub: "level", Input: []byte("{}")})
	if err == nil || !strings.Contains(err.Error(), BuildCommand) {
		t.Errorf("Fehler = %v, will Hinweis auf %q", err, BuildCommand)
	}
}

// TestEchteExe rechnet `sim level` mit der echten EXE, wenn sie frisch im Checkout liegt.
func TestEchteExe(t *testing.T) {
	root, err := filepath.Abs("../../../..")
	if err != nil {
		t.Fatal(err)
	}
	if _, _, err := Find(root); err != nil {
		t.Skip("keine frische Spiel-EXE: ", err)
	}
	ev, err := Call(context.Background(), Options{Root: root, Sub: "level", Input: []byte(`{"biome":"forest","seed":1}`)})
	if err != nil {
		t.Fatal(err)
	}
	if ev.Event != "result" || ev.Text == "" {
		t.Errorf("Ergebnis: %+v", ev)
	}
}
