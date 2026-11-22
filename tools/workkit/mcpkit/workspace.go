package mcpkit

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// Workspace ist der Checkout, aus dem ein Aufruf kommt: die Repo-Wurzel, an der der Server läuft, oder einer ihrer
// Git-Worktrees. Jedes Tool liest und schreibt nur dort.
type Workspace struct {
	Root string
	Name string // leer: Repo-Wurzel; sonst Ordnername des Worktrees (Präfix seiner Konsolen-Quellen)
}

// Source ist eine Konsolen-Quelle dieses Checkouts, z. B. Vite → <worktree>/Vite.
func (w Workspace) Source(name string) string {
	if w.Name == "" {
		return name
	}
	return w.Name + "/" + name
}

// ConsoleSource übersetzt eine Quelle, wie der Client sie nennt, in die eigene: check:<ziel> → check:<worktree>/<ziel>.
func (w Workspace) ConsoleSource(name string) string {
	if rest, ok := strings.CutPrefix(name, "check:"); ok {
		return "check:" + w.Source(rest)
	}
	return w.Source(name)
}

// OwnSource ist die Umkehrung von ConsoleSource: gehört die Quelle zu diesem Checkout, ihr Name aus Sicht des Clients.
// Quellen der Repo-Wurzel enthalten kein "/", die eines Worktrees beginnen mit "<worktree>/" bzw. "check:<worktree>/".
func (w Workspace) OwnSource(name string) (string, bool) {
	if w.Name == "" {
		return name, !strings.Contains(name, "/")
	}
	if rest, ok := strings.CutPrefix(name, "check:"+w.Name+"/"); ok {
		return "check:" + rest, true
	}
	return strings.CutPrefix(name, w.Name+"/")
}

// Label nennt den Checkout ausführlich: Repo-Wurzel mit Pfad oder Worktree mit Name und Pfad.
func (w Workspace) Label() string {
	if w.Name == "" {
		return "Repo-Wurzel " + w.Root
	}
	return "Worktree " + w.Name + " (" + w.Root + ")"
}

// Short nennt den Checkout kurz: „Repo-Wurzel“ oder den Worktree-Namen.
func (w Workspace) Short() string {
	if w.Name == "" {
		return "Repo-Wurzel"
	}
	return w.Name
}

type wsKey struct{}

// WithWorkspace hängt den Checkout eines Aufrufs an ctx (Middleware observe, Tests).
func WithWorkspace(ctx context.Context, ws Workspace) context.Context {
	return context.WithValue(ctx, wsKey{}, ws)
}

// Workspace ist der Checkout des laufenden Aufrufs; ohne Zuordnung die Repo-Wurzel.
func (s *Server) Workspace(ctx context.Context) Workspace {
	if w, ok := ctx.Value(wsKey{}).(Workspace); ok {
		return w
	}
	return Workspace{Root: s.opts.Root}
}

// OwnCheckout sagt, ob der Aufruf im Checkout dieses Servers läuft (B-924, B-925). Tools, die Code und Daten des
// eigenen Prozesses rechnen (Engine), lehnen sonst ab.
func (s *Server) OwnCheckout(ctx context.Context) bool {
	root := s.Workspace(ctx).Root
	return root == "" || s.opts.Root == "" || SameDir(root, s.opts.Root)
}

// HeaderRoot trägt das Arbeitsverzeichnis des Clients; .mcp.json setzt ihn per headersHelper bei jeder Anfrage. Die
// MCP-roots taugen dafür nicht: deprecated (SEP-2577) und ab Protokoll 2026-07-28 nicht während eines Aufrufs abfragbar.
const HeaderRoot = "X-K3C-Root"

// resolveWorkspace ordnet einen Aufruf seinem Checkout zu. Ohne Header (Tests, andere Clients) gilt die Repo-Wurzel;
// ein genannter Ordner, der kein Checkout dieses Repos ist, ist ein Fehler.
func (s *Server) resolveWorkspace(req *mcp.CallToolRequest) (Workspace, error) {
	dir := headerRoot(req)
	if s.opts.Root == "" || dir == "" {
		return Workspace{Root: s.opts.Root}, nil
	}
	return s.workspaceOf(filepath.Clean(dir))
}

// headerRoot ist der Wert von X-K3C-Root; leer, wenn der Client ihn nicht schickt (In-Memory-Transport, fremde Clients).
func headerRoot(req *mcp.CallToolRequest) string {
	if extra := req.GetExtra(); extra != nil {
		return extra.Header.Get(HeaderRoot)
	}
	return ""
}

// addCheckout hängt an die Antwort eines schreibenden Tools (Options.WriteTool) die Zeile "Checkout: …". Sie nennt den
// Checkout immer, auch wenn der Header fehlt und die Repo-Wurzel gilt (Beschluss 🧑 2026-10-06, B-275): nicht ablehnen,
// aber nie still in die Wurzel schreiben.
func (s *Server) addCheckout(tool string, res mcp.Result, ws Workspace) {
	r, ok := res.(*mcp.CallToolResult)
	if !ok || r == nil || s.opts.WriteTool == nil || !s.opts.WriteTool(tool) {
		return
	}
	r.Content = append(r.Content, &mcp.TextContent{Text: "Checkout: " + ws.Label()})
}

// workspaceOf ordnet einen Ordner einem Checkout zu: der Repo-Wurzel selbst oder einem ihrer Git-Worktrees.
func (s *Server) workspaceOf(dir string) (Workspace, error) {
	root, err := FindRoot(dir)
	if err != nil {
		return Workspace{}, fmt.Errorf("%s: %w", dir, err)
	}
	if SamePath(root, s.opts.Root) {
		return Workspace{Root: s.opts.Root}, nil
	}
	if !isWorktreeOf(root, s.opts.Root) {
		return Workspace{}, fmt.Errorf("%s ist kein Worktree von %s", root, s.opts.Root)
	}
	return Workspace{Root: root, Name: filepath.Base(root)}, nil
}

// isWorktreeOf: dir/.git ist eine Datei "gitdir: <main>/.git/worktrees/<name>", wie git worktree add sie schreibt.
func isWorktreeOf(dir, main string) bool {
	data, err := os.ReadFile(filepath.Join(dir, ".git"))
	if err != nil {
		return false
	}
	gitdir, ok := strings.CutPrefix(strings.TrimSpace(string(data)), "gitdir:")
	if !ok {
		return false
	}
	gitdir = filepath.Clean(filepath.FromSlash(strings.TrimSpace(gitdir)))
	if !filepath.IsAbs(gitdir) {
		gitdir = filepath.Join(dir, gitdir)
	}
	rel, err := filepath.Rel(canon(filepath.Join(main, ".git", "worktrees")), canon(gitdir))
	return err == nil && rel != "." && !strings.HasPrefix(rel, "..")
}

// SamePath vergleicht zwei Pfade; Windows unterscheidet keine Groß- und Kleinschreibung.
func SamePath(a, b string) bool {
	rel, err := filepath.Rel(canon(a), canon(b))
	return err == nil && rel == "."
}

// canon macht einen Pfad vergleichbar: Windows-Kurznamen (RUNNER~1) und Links werden aufgelöst, git schreibt lange Pfade.
// Fehlt der Pfad (noch), gilt das für seinen längsten vorhandenen Anfang; der Rest bleibt angehängt (B-628).
func canon(p string) string {
	p = filepath.Clean(p)
	rest := ""
	for cur := p; ; {
		if c, err := filepath.EvalSymlinks(cur); err == nil {
			return filepath.Join(c, rest)
		}
		parent := filepath.Dir(cur)
		if parent == cur {
			return p
		}
		rest, cur = filepath.Join(filepath.Base(cur), rest), parent
	}
}

// SameDir sagt, ob a und b derselbe Ordner sind (Schreibweise, Links).
func SameDir(a, b string) bool {
	ia, errA := os.Stat(a)
	ib, errB := os.Stat(b)
	if errA != nil || errB != nil {
		return filepath.Clean(a) == filepath.Clean(b)
	}
	return os.SameFile(ia, ib)
}

// FindRoot sucht ab dir aufwärts das go.mod des Spiels (module k3c): die Repo-Wurzel bzw. den Worktree.
func FindRoot(dir string) (string, error) {
	for {
		if isGameModule(filepath.Join(dir, "go.mod")) {
			return dir, nil
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return "", errors.New("repo-wurzel nicht gefunden: kein go.mod mit \"module k3c\" über dem Arbeitsverzeichnis")
		}
		dir = parent
	}
}

func isGameModule(path string) bool {
	f, err := os.Open(path)
	if err != nil {
		return false
	}
	defer func() { _ = f.Close() }()
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		if line := strings.TrimSpace(sc.Text()); strings.HasPrefix(line, "module ") {
			return strings.TrimSpace(strings.TrimPrefix(line, "module ")) == "k3c"
		}
	}
	return false
}
