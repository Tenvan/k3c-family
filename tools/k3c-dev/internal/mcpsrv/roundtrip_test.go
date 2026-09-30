package mcpsrv

import (
	"context"
	"slices"
	"strings"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// connect verbindet einen Client über den In-Memory-Transport des SDK mit s.
func connect(t *testing.T, s *Server) *mcp.ClientSession {
	t.Helper()
	ctx := context.Background()
	ct, st := mcp.NewInMemoryTransports()
	if _, err := s.mcp.Connect(ctx, st, nil); err != nil {
		t.Fatal(err)
	}
	cs, err := mcp.NewClient(&mcp.Implementation{Name: "test", Version: "0"}, nil).Connect(ctx, ct, nil)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cs.Close() })
	return cs
}

func callText(t *testing.T, cs *mcp.ClientSession, name string, args any) (string, bool) {
	t.Helper()
	res, err := cs.CallTool(context.Background(), &mcp.CallToolParams{Name: name, Arguments: args})
	if err != nil {
		t.Fatal(err)
	}
	return resultText(res), res.IsError
}

func TestAlleToolsMitBeschreibungUndAnnotations(t *testing.T) {
	s := New(Config{Version: "test"})
	res, err := connect(t, s).ListTools(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	var names []string
	for _, tool := range res.Tools {
		names = append(names, tool.Name)
		if tool.Description == "" || tool.Annotations == nil {
			t.Errorf("%s: Beschreibung oder Annotations fehlen", tool.Name)
		}
	}
	var catalog []string
	for _, ts := range s.Stats().Tools {
		catalog = append(catalog, ts.Name)
	}
	slices.Sort(catalog) // ListTools liefert nach Namen sortiert
	if strings.Join(names, ",") != strings.Join(catalog, ",") {
		t.Errorf("gelistet %v, Katalog %v", names, catalog)
	}
}

type echoIn struct {
	Text  string `json:"text"`
	Times int    `json:"times,omitempty"`
}

func TestUnbekannterParameterNenntGueltige(t *testing.T) {
	s := New(Config{Version: "test"})
	add(s, &mcp.Tool{Name: "echo", Description: "Test"}, func(_ context.Context, in echoIn) (string, error) {
		return in.Text, nil
	})
	cs := connect(t, s)
	text, isErr := callText(t, cs, "echo", map[string]any{"txt": "x"})
	if !isErr || !strings.Contains(text, "gültige Parameter: text, times") {
		t.Errorf("Hinweis fehlt: %q", text)
	}
	text, _ = callText(t, cs, "workbench_status", map[string]any{"foo": 1})
	if !strings.Contains(text, "gültige Parameter: keine") {
		t.Errorf("Hinweis ohne Parameter fehlt: %q", text)
	}
}

func TestPanikWirdZumFehler(t *testing.T) {
	s := New(Config{Version: "test"})
	add(s, &mcp.Tool{Name: "boom", Description: "Test"}, func(context.Context, struct{}) (string, error) {
		panic("kaputt")
	})
	cs := connect(t, s)
	text, isErr := callText(t, cs, "boom", nil)
	if !isErr || !strings.Contains(text, panicText) {
		t.Errorf("Panik nicht als Fehler: %q", text)
	}
	if text, isErr := callText(t, cs, "workbench_status", nil); isErr {
		t.Errorf("Server antwortet nach Panik nicht mehr: %q", text)
	}
	if st := s.Stats(); st.Errors != 1 || st.TotalCalls != 2 {
		t.Errorf("Zähler nach Panik: %+v", st)
	}
}

func TestHTTPNurAnLocalhost(t *testing.T) {
	s := New(Config{Version: "test"})
	if err := s.Start(); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = s.Stop() })
	url := s.URL()
	if !strings.HasPrefix(url, "http://127.0.0.1:") || !strings.HasSuffix(url, "/mcp") {
		t.Fatalf("Adresse %q", url)
	}
	ctx := context.Background()
	cs, err := mcp.NewClient(&mcp.Implementation{Name: "test", Version: "0"}, nil).
		Connect(ctx, &mcp.StreamableClientTransport{Endpoint: url}, nil)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = cs.Close() }()
	text, isErr := callText(t, cs, "workbench_status", nil)
	if isErr || !strings.Contains(text, "Clients 1") {
		t.Errorf("Status über HTTP: %q", text)
	}
	if err := s.Restart(); err != nil {
		t.Errorf("Neustart: %v", err)
	}
}

func TestResolvePort(t *testing.T) {
	for raw, want := range map[string]int{"": DefaultPort, " 6000 ": 6000} {
		if got, err := ResolvePort(raw); err != nil || got != want {
			t.Errorf("ResolvePort(%q) = %d, %v", raw, got, err)
		}
	}
	for _, raw := range []string{"0", "70000", "abc"} {
		if _, err := ResolvePort(raw); err == nil {
			t.Errorf("ResolvePort(%q) ohne Fehler", raw)
		}
	}
}
