package github

import (
	"context"
	"errors"
	"os/exec"
	"strings"
	"sync"
	"time"

	"k3c/tools/k3c-dev/internal/proc"
)

// TTL ist die Frist des Zwischenspeichers; GitHub soll nicht bei jedem Neuladen der Planung gefragt werden.
const TTL = 60 * time.Second

const timeout = 15 * time.Second

// Feste Argumente: nichts davon stammt aus Dateien oder Eingaben.
var (
	prArgs = []string{"gh", "pr", "list", "--state", "all", "--base", "develop", "--limit", "50", "--json",
		"number,title,state,isDraft,headRefName,mergeable,mergeStateStatus,statusCheckRollup,url"}
	runArgs = []string{"gh", "run", "list", "--branch", "develop", "--limit", "1", "--json",
		"status,conclusion,displayTitle,url,createdAt"}
)

// Client holt den Stand mit `gh` im Repo root und hält ihn TTL lang.
type Client struct {
	root string
	run  func(ctx context.Context, dir string, args []string) ([]byte, error) // Test-Naht
	mu   sync.Mutex
	last Data
	at   time.Time
}

// New baut einen Client für das Repo unter root.
func New(root string) *Client { return &Client{root: root, run: ghOutput} }

func ghOutput(ctx context.Context, dir string, args []string) ([]byte, error) {
	cmd := proc.Command(ctx, args)
	cmd.Dir = dir
	return cmd.Output()
}

// Status liefert den Stand; force umgeht den Zwischenspeicher. Scheitert `gh`, bleibt der letzte Stand mit Hinweis.
func (c *Client) Status(ctx context.Context, force bool) Data {
	c.mu.Lock()
	defer c.mu.Unlock()
	if !force && !c.at.IsZero() && time.Since(c.at) < TTL {
		return c.last
	}
	d, err := c.fetch(ctx)
	if err != nil {
		if c.at.IsZero() {
			return Data{Sprints: map[string]SprintPR{}, Error: hint(err)}
		}
		stale := c.last
		stale.Error = hint(err) + " (Stand von " + c.last.Fetched + ")"
		return stale
	}
	c.last, c.at = d, time.Now()
	return d
}

func (c *Client) fetch(ctx context.Context) (Data, error) {
	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()
	prs, err := c.run(ctx, c.root, prArgs)
	if err != nil {
		return Data{}, err
	}
	runs, err := c.run(ctx, c.root, runArgs)
	if err != nil {
		runs = nil // ohne develop-Lauf geht es auch
	}
	return Parse(prs, runs, time.Now())
}

// hint macht aus einem Fehler von gh einen Satz für Oberfläche und Tool.
func hint(err error) string {
	var exit *exec.ExitError
	switch {
	case errors.Is(err, exec.ErrNotFound):
		return "gh nicht gefunden: GitHub CLI installieren und `gh auth login` ausführen"
	case errors.Is(err, context.DeadlineExceeded):
		return "gh antwortet nicht (Netz?)"
	case errors.As(err, &exit) && strings.Contains(string(exit.Stderr), "auth login"):
		return "gh nicht angemeldet: `gh auth login` ausführen"
	case errors.As(err, &exit):
		return "gh: " + strings.TrimSpace(firstLine(string(exit.Stderr)))
	}
	return "gh: " + err.Error()
}

func firstLine(s string) string {
	line, _, _ := strings.Cut(s, "\n")
	return line
}
