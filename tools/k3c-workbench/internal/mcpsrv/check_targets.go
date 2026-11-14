package mcpsrv

import (
	"fmt"
	"os"
	"path/filepath"
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
	pattern string   // Schalter vor dem Testmuster; leer = kein Muster erlaubt; endet er auf `=`, hängt das Muster direkt an
	// quote setzt das Muster in einfache Anführungszeichen: Task setzt eine Variable (GO_TEST_FLAGS) als Text in den
	// Befehl seiner Shell ein, `|` oder `$(…)` liefen dort sonst als Shell-Syntax (B-866). CLI_ARGS nach `--` quotet Task
	// selbst, go test startet ohne Shell; dort bleibt das Muster roh.
	quote   bool
	timeout time.Duration
	kinds   []string
	needs   []string // Pfade relativ zur Wurzel, ohne die der Lauf nicht startet (Hinweis task install, B-831)
}

const (
	defaultTimeout = 10 * time.Minute
	testTimeout    = 5 * time.Minute
	devTimeout     = 20 * time.Minute // Workbench: Frontend-Build, Vitest, go test samt Worktree-Tests, Lint (B-831)
)

// devFrontendDeps sind die Frontend-Abhängigkeiten der Workbench; ohne sie scheitert schon der Frontend-Build.
const devFrontendDeps = "tools/k3c-workbench/frontend/node_modules"

var checkTargets = []checkTarget{
	{name: "task:check", command: []string{"task", "check"}, timeout: defaultTimeout,
		kinds: []string{kindOxlint, kindTsc, kindVitest}},
	{name: "task:test", command: []string{"task", "test"}, pattern: "--", timeout: testTimeout,
		kinds: []string{kindVitest}},
	{name: "task:typecheck", command: []string{"task", "typecheck"}, timeout: defaultTimeout,
		kinds: []string{kindTsc}},
	{name: "task:lint", command: []string{"task", "lint"}, timeout: defaultTimeout,
		kinds: []string{kindOxlint}},
	{name: "task:build", command: []string{"task", "build"}, timeout: defaultTimeout,
		kinds: []string{kindTsc, kindVite}},
	{name: "task:check:go", command: []string{"task", "check:go"}, timeout: defaultTimeout,
		kinds: []string{kindGoTest, kindGolangci}},
	{name: "task:check:diff", command: []string{"task", "check:diff"}, timeout: defaultTimeout,
		kinds: []string{kindOxlint, kindTsc, kindVitest, kindGoTest, kindGolangci}},
	// Teilaufgaben von check:diff (B-788): plan zeigt nur die Schritte, js und go wiederholen eine Bahn.
	{name: "task:check:diff:plan", command: []string{"task", "check:diff:plan"}, timeout: testTimeout},
	{name: "task:check:diff:js", command: []string{"task", "check:diff:js"}, timeout: defaultTimeout,
		kinds: []string{kindOxlint, kindTsc, kindVitest}},
	{name: "task:check:diff:go", command: []string{"task", "check:diff:go"}, timeout: defaultTimeout,
		kinds: []string{kindGoTest, kindGolangci}},
	{name: "task:check:race", command: []string{"task", "check:race"}, timeout: defaultTimeout,
		kinds: []string{kindGoTest}},
	{name: "go:test", command: []string{"go", "test", "./..."}, pattern: "-run", timeout: testTimeout,
		kinds: []string{kindGoTest}},
	{name: "go:lint", command: []string{"task", "go:lint"}, timeout: defaultTimeout,
		kinds: []string{kindGolangci}},
	// Workbench (B-831): gotest baut das Frontend vorher (go:embed braucht frontend/dist) und lässt die Sim-Pakete aus;
	// check:dev ist die volle Prüfung der Review (Frontend mit Vitest, go test, golangci-lint).
	{name: "dev:test", command: []string{"task", "k3c-workbench:gotest"}, pattern: "GO_TEST_FLAGS=-run=", quote: true,
		timeout: devTimeout, kinds: []string{kindTsc, kindVite, kindGoTest}, needs: []string{devFrontendDeps}},
	{name: "task:check:dev", command: []string{"task", "check:dev"}, timeout: devTimeout,
		kinds: []string{kindTsc, kindVite, kindVitest, kindGoTest, kindGolangci}, needs: []string{devFrontendDeps}},
}

// checkRunDescription ist die Beschreibung von check_run; sie nennt den Katalog oben (B-788: Basis HEAD, zwei Stufen,
// Teilaufgaben von check:diff).
const checkRunDescription = "Führt eine Prüfung aus einem festen Katalog aus (task:check, task:test, task:typecheck, task:lint, " +
	"task:build, task:check:go, task:check:diff, task:check:diff:plan, task:check:diff:js, task:check:diff:go, task:check:race, " +
	"go:test, go:lint, dev:test, task:check:dev) und antwortet mit Exit-Code, Dauer und nur den Fehlerzeilen. dev:test baut das " +
	"Workbench-Frontend bei Bedarf und fährt go test der Workbench, task:check:dev ist die volle Workbench-Prüfung (Frontend, " +
	"Vitest, go test, Lint); beide brauchen task install. task:check:diff prüft nur, was " +
	"die geänderten Dateien gegenüber der Basis HEAD brauchen (vor jedem Commit), in zwei Stufen: Stufe 1 Regel-Tests, Lint, " +
	"Typecheck, go vet; Stufe 2 nur nach grüner Stufe 1 die Tests der geänderten Pakete. task:check:diff:plan zeigt nur die " +
	"Schritte, task:check:diff:js und task:check:diff:go wiederholen nach einem roten Lauf nur diese Bahn; " +
	"task:check:race ist der langsame Race-Detector (Review, CI). " +
	"Ersetzt task check, task test und go test in der Shell. Volle Ausgabe: console_tail check:<ziel>. " +
	"Wartet bis zum Ende des Laufs und meldet dabei Fortschritt (ohne progressToken höchstens 4 min, dann Lauf-ID; " +
	"run=<ID> wartet weiter auf diesen Lauf). Nie in Schleifen nachfragen. " +
	"Nutze es bei: jeder Prüfung vor einem Abschluss. Statt: task check, task test, go test oder golangci-lint in der Shell; " +
	"volle Ausgabe über console_tail check:<ziel>."

// patternRule ist eine Positivliste: Regex-Zeichen für Alternativen und Anker (| ^ $ ( ), B-866) sind erlaubt, Shell-
// Zeichen (; & ` " ' < > % \ und Leerzeichen) nicht, und kein führendes "-", damit kein Muster zur Option wird (z. B.
// "--watch" hielte den Lauf bis zum Zeitlimit offen). Ohne `'` ist das Quoten in args immer geschlossen.
var patternRule = regexp.MustCompile(`^[A-Za-z0-9_./:|^$()][A-Za-z0-9_./:|^$()-]{0,99}$`)

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
		return nil, fmt.Errorf("%s nimmt kein Testmuster; Muster gehen nur bei task:test, go:test, dev:test", t.name)
	}
	if !patternRule.MatchString(pattern) {
		return nil, fmt.Errorf("testmuster %q abgelehnt: erlaubt sind nur A–Z, a–z, 0–9 und _ . / : - | ^ $ ( ) (kein - am Anfang, höchstens 100)", pattern)
	}
	if t.quote {
		pattern = "'" + pattern + "'"
	}
	if strings.HasSuffix(t.pattern, "=") {
		return append(args, t.pattern+pattern), nil
	}
	return append(args, t.pattern, pattern), nil
}

// preflight prüft vor dem Start, ob die nötigen Pfade da sind; sonst Abbruch mit Hinweis statt hunderter Folgefehler.
func (t checkTarget) preflight(root string) error {
	for _, p := range t.needs {
		if _, err := os.Stat(filepath.Join(root, filepath.FromSlash(p))); err != nil {
			return fmt.Errorf("%s: %s fehlt; erst task install im Checkout (holt die Frontend-Abhängigkeiten)", t.name, p)
		}
	}
	return nil
}
