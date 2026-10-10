package mcpsrv

import (
	"encoding/json"
	"errors"
	"fmt"
	"os/exec"
	"path/filepath"
	"strings"

	"github.com/google/jsonschema-go/jsonschema"
)

// Argument checkout (B-388): Der Client der Desktop-App meldet im Header aus jedem Worktree die Repo-Wurzel (B-341).
// Schreibende Tools und check_run nehmen deshalb den Checkout als Argument; es gilt vor dem Header.

const checkoutArg = "checkout"

// takesCheckout: Tools mit dem Argument checkout im Schema.
func takesCheckout(tool string) bool { return writeTools[tool] || tool == "check_run" }

// withCheckout ergänzt das aus In abgeleitete Schema um checkout. Der Handler sieht das Feld nie (splitCheckout).
func withCheckout[In any](names []string) (*jsonschema.Schema, []string, error) {
	schema, err := jsonschema.For[In](nil)
	if err != nil {
		return nil, nil, err
	}
	if schema.Properties == nil {
		schema.Properties = map[string]*jsonschema.Schema{}
	}
	schema.Properties[checkoutArg] = &jsonschema.Schema{Type: "string", Description: "Worktree-Ordnername oder absoluter " +
		"Pfad des eigenen Checkouts (git rev-parse --show-toplevel); gilt vor dem Header. Im Worktree immer setzen."}
	return schema, append(names, checkoutArg), nil
}

// splitCheckout nimmt checkout aus den rohen Argumenten; leer, wenn es fehlt.
func splitCheckout(raw json.RawMessage) (string, json.RawMessage, error) {
	var args map[string]json.RawMessage
	if len(raw) == 0 || json.Unmarshal(raw, &args) != nil || args[checkoutArg] == nil {
		return "", raw, nil
	}
	var value string
	if err := json.Unmarshal(args[checkoutArg], &value); err != nil {
		return "", raw, errors.New("checkout muss ein Text sein (Worktree-Name oder Pfad)")
	}
	delete(args, checkoutArg)
	rest, err := json.Marshal(args)
	return value, rest, err
}

// resolveCheckout: zuerst Ordnername eines Worktrees aus git worktree list, sonst ein absoluter Pfad (workspaceOf).
func (s *Server) resolveCheckout(value string) (workspace, error) {
	paths, gitErr := s.worktrees()
	var hits []string
	for _, p := range paths {
		if strings.EqualFold(filepath.Base(p), value) {
			hits = append(hits, p)
		}
	}
	if len(hits) > 1 {
		return workspace{}, fmt.Errorf("checkout %q ist mehrdeutig: %s", value, strings.Join(hits, ", "))
	}
	if len(hits) == 1 {
		return s.workspaceOf(hits[0])
	}
	valid := "gültige Worktrees: " + worktreeNames(paths)
	if gitErr != nil {
		valid = "git worktree list: " + gitErr.Error()
	}
	if !filepath.IsAbs(value) {
		return workspace{}, fmt.Errorf("checkout %q: kein Worktree dieses Namens; %s", value, valid)
	}
	ws, err := s.workspaceOf(filepath.Clean(value))
	if err != nil {
		return workspace{}, fmt.Errorf("checkout: %v; %s", err, valid)
	}
	return ws, nil
}

// worktrees sind die Pfade aus git worktree list --porcelain der Repo-Wurzel, die Wurzel zuerst.
func (s *Server) worktrees() ([]string, error) {
	if s.cfg.Root == "" {
		return nil, errors.New("k3c-dev kennt keine Repo-Wurzel")
	}
	out, err := exec.Command("git", "-C", s.cfg.Root, "worktree", "list", "--porcelain").Output()
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
