package mcpsrv

import (
	"fmt"
	"regexp"
	"strings"
	"time"
)

// Werkzeuge, deren Fehlerzeilen check_run erkennt (check_compact.go).
const (
	kindVitest   = "vitest"
	kindTsc      = "tsc"
	kindOxlint   = "oxlint"
	kindGoTest   = "gotest"
	kindGolangci = "golangci"
	kindVite     = "vite"
)

// checkTarget ist ein Ziel von check_run. Der Katalog ist fest im Code und bleibt fail-closed: ein neues Ziel
// kommt nur per Code-Änderung dazu, nie aus einer Datei (B-046).
type checkTarget struct {
	name    string
	dir     string   // relativ zur Repo-Wurzel
	command []string // ohne Shell gestartet
	pattern string   // Schalter vor dem Testmuster; leer = kein Muster erlaubt
	timeout time.Duration
	kinds   []string
}

const (
	defaultTimeout = 10 * time.Minute
	testTimeout    = 5 * time.Minute
)

var checkTargets = []checkTarget{
	{name: "npm:check", command: []string{"npm", "run", "check"}, timeout: defaultTimeout,
		kinds: []string{kindOxlint, kindTsc, kindVitest}},
	{name: "npm:test", command: []string{"npm", "test"}, pattern: "--", timeout: testTimeout,
		kinds: []string{kindVitest}},
	{name: "npm:typecheck", command: []string{"npm", "run", "typecheck"}, timeout: defaultTimeout,
		kinds: []string{kindTsc}},
	{name: "npm:lint", command: []string{"npm", "run", "lint"}, timeout: defaultTimeout,
		kinds: []string{kindOxlint}},
	{name: "npm:build", command: []string{"npm", "run", "build"}, timeout: defaultTimeout,
		kinds: []string{kindTsc, kindVite}},
	{name: "go:test", command: []string{"go", "test", "./..."}, pattern: "-run", timeout: testTimeout,
		kinds: []string{kindGoTest}},
	{name: "go:lint", command: []string{"golangci-lint", "run"}, timeout: defaultTimeout,
		kinds: []string{kindGolangci}},
	{name: "dev:test", dir: "tools/k3c-dev", command: []string{"go", "test", "./..."}, pattern: "-run",
		timeout: testTimeout, kinds: []string{kindGoTest}},
}

// patternRule ist eine Positivliste: sie lehnt Shell-Zeichen (;&|$`"' < > % ^) und Leerzeichen ab.
var patternRule = regexp.MustCompile(`^[A-Za-z0-9_./:-]{1,100}$`)

func targetNames() string {
	names := make([]string, len(checkTargets))
	for i, t := range checkTargets {
		names[i] = t.name
	}
	return strings.Join(names, ", ")
}

func findTarget(name string) (checkTarget, error) {
	for _, t := range checkTargets {
		if t.name == name {
			return t, nil
		}
	}
	return checkTarget{}, fmt.Errorf("unbekanntes Ziel %q; gültige Ziele: %s", name, targetNames())
}

// args liefert die Befehlszeile; ein Muster wird geprüft und abgelehnt, nie bereinigt.
func (t checkTarget) args(pattern string) ([]string, error) {
	args := append([]string(nil), t.command...)
	if pattern == "" {
		return args, nil
	}
	if t.pattern == "" {
		return nil, fmt.Errorf("%s nimmt kein Testmuster; Muster gehen nur bei npm:test, go:test, dev:test", t.name)
	}
	if !patternRule.MatchString(pattern) {
		return nil, fmt.Errorf("testmuster %q abgelehnt: erlaubt sind nur A–Z, a–z, 0–9 und _ . / : - (höchstens 100)", pattern)
	}
	return append(args, t.pattern, pattern), nil
}
