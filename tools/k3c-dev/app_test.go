package main

import (
	"context"
	"net"
	"slices"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// Ein belegter Port hält das Fenster nicht auf: Info und Ereignis nennen den Grund (B-064 › Fehlerfälle).
func TestStartupPortBusy(t *testing.T) {
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = ln.Close() }()
	root := t.TempDir()
	app := newApp(root, ln.Addr().(*net.TCPAddr).Port)
	app.usage = filepath.Join(root, "usage.json")
	var events []string
	app.emit = func(_ context.Context, name string, data ...any) {
		if st, ok := data[0].(MCPState); ok && !st.Listening {
			events = append(events, name)
		}
	}
	app.startup(context.Background())
	defer app.shutdown(context.Background())
	info := app.Info()
	if info.MCP.Listening || !strings.Contains(info.MCP.Error, "nicht verfügbar") {
		t.Errorf("MCP-Zustand = %+v", info.MCP)
	}
	if len(events) != 1 || events[0] != evMCPState {
		t.Errorf("Ereignisse = %v", events)
	}
}

func TestStartupListens(t *testing.T) {
	root := t.TempDir()
	app := newApp(root, 0)
	app.usage = filepath.Join(root, "usage.json")
	app.emit = func(context.Context, string, ...any) {}
	app.startup(context.Background())
	defer app.shutdown(context.Background())
	if st := app.Info().MCP; !st.Listening || st.Error != "" {
		t.Errorf("MCP-Zustand = %+v", st)
	}
}

// Wails ruft OnStartup in einer eigenen Goroutine: ein Binding davor wartet, statt auf nil zuzugreifen.
func TestBindingWartetAufStartup(t *testing.T) {
	root := t.TempDir()
	app := newApp(root, 0)
	app.usage = filepath.Join(root, "usage.json")
	app.emit = func(context.Context, string, ...any) {}
	go func() {
		time.Sleep(50 * time.Millisecond)
		app.startup(context.Background())
	}()
	if src := app.Sources(); len(src) == 0 || !slices.ContainsFunc(src, func(s Source) bool { return s.Name == "k3c-dev" }) {
		t.Errorf("Sources vor startup = %+v", src)
	}
	app.shutdown(context.Background())
}
