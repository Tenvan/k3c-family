package mcpsrv

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

// workspace ist der Checkout, aus dem ein Aufruf kommt: die Repo-Wurzel, an der k3c-dev läuft, oder einer ihrer
// Git-Worktrees. Jedes Tool liest und schreibt nur dort.
type workspace struct {
	root string
	name string // leer: Repo-Wurzel; sonst Ordnername des Worktrees (Präfix seiner Konsolen-Quellen)
}

// source ist eine Konsolen-Quelle dieses Checkouts, z. B. Vite → <worktree>/Vite.
func (w workspace) source(name string) string {
	if w.name == "" {
		return name
	}
	return w.name + "/" + name
}

// consoleSource übersetzt eine Quelle, wie der Client sie nennt, in die eigene: check:<ziel> → check:<worktree>/<ziel>.
func (w workspace) consoleSource(name string) string {
	if rest, ok := strings.CutPrefix(name, "check:"); ok {
		return "check:" + w.source(rest)
	}
	return w.source(name)
}

// ownSource ist die Umkehrung von consoleSource: gehört die Quelle zu diesem Checkout, ihr Name aus Sicht des Clients.
// Quellen der Repo-Wurzel enthalten kein "/", die eines Worktrees beginnen mit "<worktree>/" bzw. "check:<worktree>/".
func (w workspace) ownSource(name string) (string, bool) {
	if w.name == "" {
		return name, !strings.Contains(name, "/")
	}
	if rest, ok := strings.CutPrefix(name, "check:"+w.name+"/"); ok {
		return "check:" + rest, true
	}
	return strings.CutPrefix(name, w.name+"/")
}

func (w workspace) label() string {
	if w.name == "" {
		return "Repo-Wurzel " + w.root
	}
	return "Worktree " + w.name + " (" + w.root + ")"
}

type wsKey struct{}

// ws ist der Checkout des laufenden Aufrufs; ohne Zuordnung die Repo-Wurzel.
func (s *Server) ws(ctx context.Context) workspace {
	if w, ok := ctx.Value(wsKey{}).(workspace); ok {
		return w
	}
	return workspace{root: s.cfg.Root}
}

// HeaderRoot trägt das Arbeitsverzeichnis des Clients; .mcp.json setzt ihn per headersHelper bei jeder Anfrage. Die
// MCP-roots taugen dafür nicht: deprecated (SEP-2577) und ab Protokoll 2026-07-28 nicht während eines Aufrufs abfragbar.
const HeaderRoot = "X-K3C-Root"

// resolveWorkspace ordnet einen Aufruf seinem Checkout zu. Ohne Header (Tests, andere Clients) gilt die Repo-Wurzel;
// ein genannter Ordner, der kein Checkout dieses Repos ist, ist ein Fehler.
func (s *Server) resolveWorkspace(req *mcp.CallToolRequest) (workspace, error) {
	dir := headerRoot(req)
	if s.cfg.Root == "" || dir == "" {
		return workspace{root: s.cfg.Root}, nil
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

// writeTools ändern Dateien oder Dienste. Ihre Antwort nennt immer den Checkout, auch wenn der Header fehlt und die
// Repo-Wurzel gilt (Beschluss 🧑 2026-10-06, B-275): nicht ablehnen, aber nie still in die Wurzel schreiben.
var writeTools = map[string]bool{"plan_create": true, "plan_set": true, "plan_section": true, "plan_delete": true,
	"svc_start": true, "svc_stop": true, "svc_restart": true, "svc_start_all": true, "svc_stop_all": true, "task_start": true, "task_stop": true}

// addCheckout hängt an die Antwort eines schreibenden Tools die Zeile "Checkout: …".
func addCheckout(tool string, res mcp.Result, ws workspace) {
	r, ok := res.(*mcp.CallToolResult)
	if !ok || r == nil || !writeTools[tool] {
		return
	}
	r.Content = append(r.Content, &mcp.TextContent{Text: "Checkout: " + ws.label()})
}

// workspaceOf ordnet einen Ordner einem Checkout zu: der Repo-Wurzel selbst oder einem ihrer Git-Worktrees.
func (s *Server) workspaceOf(dir string) (workspace, error) {
	root, err := FindRoot(dir)
	if err != nil {
		return workspace{}, fmt.Errorf("%s: %w", dir, err)
	}
	if samePath(root, s.cfg.Root) {
		return workspace{root: s.cfg.Root}, nil
	}
	if !isWorktreeOf(root, s.cfg.Root) {
		return workspace{}, fmt.Errorf("%s ist kein Worktree von %s", root, s.cfg.Root)
	}
	return workspace{root: root, name: filepath.Base(root)}, nil
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

// samePath vergleicht zwei Pfade; Windows unterscheidet keine Groß- und Kleinschreibung.
func samePath(a, b string) bool {
	rel, err := filepath.Rel(canon(a), canon(b))
	return err == nil && rel == "."
}

// canon macht einen Pfad vergleichbar: Windows-Kurznamen (RUNNER~1) und Links werden aufgelöst, git schreibt lange Pfade.
func canon(p string) string {
	if c, err := filepath.EvalSymlinks(p); err == nil {
		return c
	}
	return filepath.Clean(p)
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
