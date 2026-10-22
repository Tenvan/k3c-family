package mcpsrv

import (
	"fmt"
	"regexp"
	"strings"
)

const (
	// maxErrorLines begrenzt die Fehlerzeilen einer Antwort.
	maxErrorLines = 60
	// fallbackLines sind die letzten Zeilen, wenn ein roter Lauf keine bekannte Fehlerzeile hat.
	fallbackLines = 20
)

var ansi = regexp.MustCompile(`\x1b\[[0-9;?]*[ -/]*[@-~]`)

// errorPatterns erkennen Fehlerzeilen je Werkzeug, abgelesen an echten Läufen in diesem Repo (testdata/check/).
var errorPatterns = map[string][]*regexp.Regexp{
	kindVitest:   regs(`^\s*FAIL\s`, `^\s*[A-Za-z]*Error: `, `❯ \S+:\d+:\d+`, `^\s*Test Files\s`, `^\s*Tests\s+\d`),
	kindTsc:      regs(`^\S+\(\d+,\d+\): error TS\d+`),
	kindOxlint:   regs(`^\S+:\d+:\d+: error `),
	kindGoTest:   regs(`^--- FAIL`, `^\s+\S+\.go:\d+: `, `^FAIL\s`, `^panic: `, `^\S+\.go:\d+:\d+: `),
	kindGolangci: regs(`^\S+\.go:\d+:\d+: .*\(\w+\)$`, `^\d+ issues?:`),
	kindVite:     regs(`^error during build`, `^Build failed with`, `^\[plugin `, `^Error: `),
}

func regs(patterns ...string) []*regexp.Regexp {
	out := make([]*regexp.Regexp, len(patterns))
	for i, p := range patterns {
		out[i] = regexp.MustCompile(p)
	}
	return out
}

// cleanLines entfernt ANSI-Farben und Leerzeilen.
func cleanLines(lines []string) []string {
	out := make([]string, 0, len(lines))
	for _, l := range lines {
		if l = strings.TrimRight(ansi.ReplaceAllString(l, ""), " \t"); strings.TrimSpace(l) != "" {
			out = append(out, l)
		}
	}
	return out
}

// errorLines liefert die Zeilen, die ein Muster der Werkzeuge erkennt, in ihrer Reihenfolge.
func errorLines(lines []string, kinds []string) []string {
	var out []string
	for _, l := range lines {
		if matchesAny(l, kinds) {
			out = append(out, strings.TrimSpace(l))
		}
	}
	return out
}

func matchesAny(line string, kinds []string) bool {
	for _, k := range kinds {
		for _, re := range errorPatterns[k] {
			if re.MatchString(line) {
				return true
			}
		}
	}
	return false
}

// report ist die Antwort von check_run: grün eine Zeile, sonst Kopfzeile und höchstens die Fehlerzeilen.
func report(t checkTarget, res runResult, output []string) string {
	head := fmt.Sprintf("%s · exit %d · %s", t.name, res.exit, formatMs(res.ms()))
	if res.timedOut {
		head = fmt.Sprintf("%s · Zeitlimit %s überschritten · %s", t.name, formatMs(float64(t.timeout.Milliseconds())), formatMs(res.ms()))
	}
	if res.exit == 0 && !res.timedOut {
		return head
	}
	lines := cleanLines(output)
	found := errorLines(lines, t.kinds)
	if len(found) == 0 {
		found = lines[max(0, len(lines)-fallbackLines):]
	}
	if len(found) > maxErrorLines {
		found = append(found[:maxErrorLines:maxErrorLines], fmt.Sprintf("… %d weitere", len(found)-maxErrorLines))
	}
	return strings.Join(append([]string{head}, found...), "\n")
}
