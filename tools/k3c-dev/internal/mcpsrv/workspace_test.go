package mcpsrv

import (
	"bytes"
	"context"
	"log/slog"
	"os"
	"net/http"
	"path/filepath"
	"strings"
	"sync"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

func writeFile(t *testing.T, path, text string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

func TestFindRoot(t *testing.T) {
	root := t.TempDir()
	sub := filepath.Join(root, "tools", "k3c-dev")
	writeFile(t, filepath.Join(root, "go.mod"), "module k3c\n\ngo 1.27.0\n")
	writeFile(t, filepath.Join(sub, "go.mod"), "module k3c/tools/k3c-dev\n")
	if got, err := FindRoot(sub); err != nil || got != filepath.Clean(root) {
		t.Errorf("FindRoot(sub) = %q, %v", got, err)
	}
	// Das Temp-Verzeichnis kann im Repo liegen (task setzt TMP nach .work/tmp); die Laufwerkswurzel hat sicher kein go.mod.
	if _, err := FindRoot(filepath.VolumeName(os.TempDir()) + string(os.PathSeparator)); err == nil {
		t.Error("ohne go.mod kein Fehler")
	}
}

// repoWithWorktree legt eine Repo-Wurzel mit einem Worktree wt1 an, wie git worktree add sie schreibt, und ein
// fremdes Repo daneben.
func repoWithWorktree(t *testing.T) (main, wt, foreign string) {
	t.Helper()
	base := t.TempDir()
	main, foreign = filepath.Join(base, "k3c"), filepath.Join(base, "anders")
	wt = filepath.Join(main, ".claude", "worktrees", "wt1")
	for _, dir := range []string{main, wt, foreign} {
		writeFile(t, filepath.Join(dir, "go.mod"), "module k3c\n")
	}
	writeFile(t, filepath.Join(wt, ".git"), "gitdir: "+filepath.ToSlash(filepath.Join(main, ".git", "worktrees", "wt1"))+"\n")
	writeFile(t, filepath.Join(foreign, ".git"), "gitdir: "+filepath.ToSlash(filepath.Join(foreign, "x", ".git", "worktrees", "y"))+"\n")
	return main, wt, foreign
}

type headerTransport struct{ dir string }

func (h headerTransport) RoundTrip(r *http.Request) (*http.Response, error) {
	r = r.Clone(r.Context())
	r.Header.Set(HeaderRoot, h.dir)
	return http.DefaultTransport.RoundTrip(r)
}

// connectWithHeader verbindet über HTTP mit dem Header X-K3C-Root, wie der headersHelper aus .mcp.json.
func connectWithHeader(t *testing.T, s *Server, dir string) *mcp.ClientSession {
	t.Helper()
	if s.URL() == "" {
		if err := s.Start(); err != nil {
			t.Fatal(err)
		}
		t.Cleanup(func() { _ = s.Stop() })
	}
	tr := &mcp.StreamableClientTransport{Endpoint: s.URL(), HTTPClient: &http.Client{Transport: headerTransport{dir}}}
	cs, err := mcp.NewClient(&mcp.Implementation{Name: "test", Version: "0"}, nil).Connect(context.Background(), tr, nil)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cs.Close() })
	return cs
}

func TestWorkspaceAusHeader(t *testing.T) {
	main, wt, foreign := repoWithWorktree(t)
	s := New(Config{Root: main, Version: "test"})
	if text, _ := callText(t, connectWithHeader(t, s, filepath.Join(wt, "tools", "k3c-dev")), "workbench_status", nil); !strings.Contains(text, "Worktree wt1") {
		t.Errorf("Header aus Unterordner: %q", text)
	}
	if text, _ := callText(t, connectWithHeader(t, s, main), "workbench_status", nil); !strings.Contains(text, "Repo-Wurzel "+main) {
		t.Errorf("Header mit der Wurzel: %q", text)
	}
	if text, isErr := callText(t, connectWithHeader(t, s, foreign), "workbench_status", nil); !isErr || !strings.Contains(text, "kein Worktree von") {
		t.Errorf("fremdes Repo nicht abgelehnt: %q", text)
	}
	if text, isErr := callText(t, connectWithHeader(t, s, filepath.Dir(main)), "workbench_status", nil); !isErr { // TMP kann im Repo liegen (task): dann „kein Worktree“, sonst „nicht gefunden“
		t.Errorf("Ordner ohne go.mod nicht abgelehnt: %q", text)
	}
	// Ohne Header (Tests, andere Clients) bleibt es bei der Repo-Wurzel.
	if text, _ := callText(t, connect(t, s), "workbench_status", nil); !strings.Contains(text, "Repo-Wurzel "+main) {
		t.Errorf("ohne Zuordnung: %q", text)
	}
}

func TestWorktreeTrenntKonsoleUndDateien(t *testing.T) {
	main, wt, _ := repoWithWorktree(t)
	writeFile(t, filepath.Join(wt, "logs", "nur-im-worktree.jsonl"), "")
	s := New(Config{Root: main, Version: "test"})
	s.console.Add("check:wt1/task:test", "stdout", "aus dem Worktree")
	s.console.Add("check:task:test", "stdout", "aus der Wurzel")
	cw, cm := connectWithHeader(t, s, wt), connectWithHeader(t, s, main)
	if text, _ := callText(t, cw, "console_tail", map[string]any{"service": "check:task:test"}); !strings.Contains(text, "aus dem Worktree") {
		t.Errorf("Worktree liest fremde Konsole: %q", text)
	}
	if text, _ := callText(t, cm, "console_tail", map[string]any{"service": "check:task:test"}); !strings.Contains(text, "aus der Wurzel") {
		t.Errorf("Wurzel liest fremde Konsole: %q", text)
	}
	if text, _ := callText(t, cw, "logs_services", nil); !strings.Contains(text, "nur-im-worktree") || strings.Count(text, "Konsole check:task:test ·") != 1 {
		t.Errorf("logs_services im Worktree: %q", text)
	}
	if text, _ := callText(t, cm, "logs_services", nil); strings.Contains(text, "nur-im-worktree") || strings.Contains(text, "wt1/") {
		t.Errorf("logs_services in der Wurzel: %q", text)
	}
}

func TestWorktreeDiensteMitEigenenPorts(t *testing.T) {
	main, wt, _ := repoWithWorktree(t)
	writeFile(t, filepath.Join(wt, "tools", "k3c-dev", "services.json"), `[
		{"name":"Vite","command":["x"],"port":5173,"health":"http","portEnv":"K3C_VITE_PORT"},
		{"name":"Spielserver","command":["x"],"port":8080,"health":"http","portEnv":"K3C_HTTP_PORT"}]`)
	t.Setenv("K3C_SERVER_URL", "http://anderswo:1") // gilt im Worktree nicht: dort zählt der eigene Spielserver
	s := New(Config{Root: main, Version: "test"})
	s.portBusy = func(port int) bool { return port == 8090 } // Versatz 10 ist belegt → 20
	t.Cleanup(func() { s.StopWorktreeServices(context.Background()) })
	cw := connectWithHeader(t, s, wt)
	text, isErr := callText(t, cw, "svc_status", nil)
	if isErr || !strings.Contains(text, "Vite · gestoppt · Port 5193") || !strings.Contains(text, "Spielserver · gestoppt · Port 8100") {
		t.Errorf("svc_status im Worktree: %q", text)
	}
	if text, _ := callText(t, cw, "server_status", nil); !strings.Contains(text, "127.0.0.1:8100") {
		t.Errorf("server_status fragt nicht den Spielserver des Worktrees: %q", text)
	}
	if text, _ := callText(t, connectWithHeader(t, s, main), "svc_status", nil); !strings.Contains(text, "keine Dienste") {
		t.Errorf("Wurzel ohne Controller sieht Worktree-Dienste: %q", text)
	}
}

// lockedLog sammelt das JSON-Log des Servers; der Server schreibt aus eigenen Goroutinen.
type lockedLog struct {
	mu  sync.Mutex
	buf bytes.Buffer
}

func (l *lockedLog) Write(p []byte) (int, error) {
	l.mu.Lock()
	defer l.mu.Unlock()
	return l.buf.Write(p)
}

func (l *lockedLog) lines(tool string) []string {
	l.mu.Lock()
	defer l.mu.Unlock()
	var out []string
	for _, line := range strings.Split(l.buf.String(), "\n") {
		if strings.Contains(line, `"tool":"`+tool+`"`) {
			out = append(out, line)
		}
	}
	return out
}

// B-275/AC-03: Ein schreibendes Tool ohne Header schreibt in die Wurzel und nennt sie; mit Header auf einen Worktree
// unter .claude/worktrees/ schreibt es nur dort. Das Log vermerkt je Aufruf, ob der Header da war.
func TestSchreibendesToolNenntCheckout(t *testing.T) {
	main, wt, _ := repoWithWorktree(t)
	ticket := filepath.Join("docs", "backlog", "B-001-x.md")
	for _, dir := range []string{main, wt} {
		writeFile(t, filepath.Join(dir, ticket), "# B-001 · X\n\n- **Status:** offen\n\n## Ausgangslage\n\nalt\n")
	}
	logs := &lockedLog{}
	s := New(Config{Root: main, Version: "test", Log: slog.New(slog.NewJSONHandler(logs, nil))})
	section := func(text string) map[string]any {
		return map[string]any{"id": "B-001", "section": "Ausgangslage", "text": text}
	}
	if text, isErr := callText(t, connect(t, s), "plan_section", section("aus der Wurzel")); isErr || !strings.Contains(text, "Checkout: Repo-Wurzel "+main) {
		t.Errorf("ohne Header: %q", text)
	}
	if text, isErr := callText(t, connectWithHeader(t, s, wt), "plan_section", section("aus dem Worktree")); isErr || !strings.Contains(text, "Checkout: Worktree wt1") {
		t.Errorf("mit Header: %q", text)
	}
	read := func(dir string) string {
		b, err := os.ReadFile(filepath.Join(dir, ticket))
		if err != nil {
			t.Fatal(err)
		}
		return string(b)
	}
	if m, w := read(main), read(wt); !strings.Contains(m, "aus der Wurzel") || strings.Contains(m, "aus dem Worktree") || !strings.Contains(w, "aus dem Worktree") {
		t.Errorf("Wurzel:\n%s\nWorktree:\n%s", m, w)
	}
	got := logs.lines("plan_section")
	if len(got) != 2 || !strings.Contains(got[0], `"header":false`) || !strings.Contains(got[0], `"checkout":"Repo-Wurzel `) ||
		!strings.Contains(got[1], `"header":true`) || !strings.Contains(got[1], `"checkout":"Worktree wt1 (`) {
		t.Errorf("Log je Aufruf:\n%s", strings.Join(got, "\n"))
	}
}
