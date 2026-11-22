package mcpkit

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"path/filepath"
	"strings"
	"time"

	"github.com/google/jsonschema-go/jsonschema"

	"k3c/tools/workkit/proc"
)

// Argument checkout (B-388): Der Client der Desktop-App meldet im Header aus jedem Worktree die Repo-Wurzel (B-341).
// Tools mit Options.TakesCheckout nehmen deshalb den Checkout als Argument; es gilt vor dem Header.

// CheckoutArg ist der Name des Arguments.
const CheckoutArg = "checkout"

// withCheckout ergänzt das aus In abgeleitete Schema um checkout. Der Handler sieht das Feld nie (splitCheckout).
func withCheckout[In any](names []string) (*jsonschema.Schema, []string, error) {
	schema, err := jsonschema.For[In](nil)
	if err != nil {
		return nil, nil, err
	}
	if schema.Properties == nil {
		schema.Properties = map[string]*jsonschema.Schema{}
	}
	schema.Properties[CheckoutArg] = &jsonschema.Schema{Type: "string", Description: "Worktree-Ordnername oder absoluter " +
		"Pfad des eigenen Checkouts (git rev-parse --show-toplevel); gilt vor dem Header. Im Worktree immer setzen."}
	return schema, append(names, CheckoutArg), nil
}

// splitCheckout nimmt checkout aus den rohen Argumenten; leer, wenn es fehlt.
func splitCheckout(raw json.RawMessage) (string, json.RawMessage, error) {
	var args map[string]json.RawMessage
	if len(raw) == 0 || json.Unmarshal(raw, &args) != nil || args[CheckoutArg] == nil {
		return "", raw, nil
	}
	var value string
	if err := json.Unmarshal(args[CheckoutArg], &value); err != nil {
		return "", raw, errors.New("checkout muss ein Text sein (Worktree-Name oder Pfad)")
	}
	delete(args, CheckoutArg)
	rest, err := json.Marshal(args)
	return value, rest, err
}

// resolveCheckout: zuerst Ordnername eines Worktrees aus git worktree list, sonst ein absoluter Pfad (workspaceOf).
func (s *Server) resolveCheckout(value string) (Workspace, error) {
	paths, gitErr := s.Worktrees()
	var hits []string
	for _, p := range paths {
		if strings.EqualFold(filepath.Base(p), value) {
			hits = append(hits, p)
		}
	}
	if len(hits) > 1 {
		return Workspace{}, fmt.Errorf("checkout %q ist mehrdeutig: %s", value, strings.Join(hits, ", "))
	}
	if len(hits) == 1 {
		return s.workspaceOf(hits[0])
	}
	valid := "gültige Worktrees: " + worktreeNames(paths)
	if gitErr != nil {
		valid = "git worktree list: " + gitErr.Error()
	}
	if !filepath.IsAbs(value) {
		return Workspace{}, fmt.Errorf("checkout %q: kein Worktree dieses Namens; %s", value, valid)
	}
	ws, err := s.workspaceOf(filepath.Clean(value))
	if err != nil {
		return Workspace{}, fmt.Errorf("checkout: %v; %s", err, valid)
	}
	return ws, nil
}

// Worktrees sind die Pfade aus git worktree list --porcelain der Repo-Wurzel, die Wurzel zuerst.
func (s *Server) Worktrees() ([]string, error) {
	if s.opts.Root == "" {
		return nil, errors.New(s.opts.Name + " kennt keine Repo-Wurzel")
	}
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()
	// proc.Command: ohne Konsolenfenster (B-416)
	out, err := proc.Command(ctx, []string{"git", "-C", s.opts.Root, "worktree", "list", "--porcelain"}).Output()
	if err != nil {
		return nil, err
	}
	var paths []string
	for line := range strings.Lines(string(out)) {
		if p, ok := strings.CutPrefix(strings.TrimSpace(line), "worktree "); ok {
			paths = append(paths, filepath.Clean(filepath.FromSlash(p)))
		}
	}
	return paths, nil
}

func worktreeNames(paths []string) string {
	if len(paths) <= 1 {
		return "keine"
	}
	names := make([]string, 0, len(paths)-1)
	for _, p := range paths[1:] {
		names = append(names, filepath.Base(p))
	}
	return strings.Join(names, ", ")
}
