package mcpsrv

import (
	"context"
	"errors"
	"os"
	"regexp"
	"strings"
	"testing"
)

// B-868/AC-01: Task-Ziel mit Testmuster und zu alter Version → Ablehnung mit Mindestversion, ohne Prozessstart;
// passende Version läuft; nicht lesbar → Warnung in der Antwort, kein Abbruch; ohne Muster oder bei go:test keine Prüfung.
func TestCheckRunTaskMindestversion(t *testing.T) {
	t.Parallel()
	ctx := context.Background()
	cases := []struct {
		name, target, pattern, version string
		readErr                        error
		wantErr, wantWarn, wantRead    bool
	}{
		{"zu alt", "task:test", "planning", "Task version: v3.40.0 (h1:abc)", nil, true, false, true},
		{"passend", "task:test", "planning", "3.54.0", nil, false, false, true},
		{"neuer", "task:test", "planning", "4.0", nil, false, false, true},
		{"nicht lesbar", "task:test", "planning", "", errors.New("nicht gefunden"), false, true, true},
		{"ohne Zahl", "dev:test", "TestX", "Task", nil, false, true, true},
		{"ohne Muster", "task:test", "", "3.1.0", nil, false, false, false},
		{"go:test ohne Task", "go:test", "TestX", "3.1.0", nil, false, false, false},
	}
	for _, c := range cases {
		s, started := fakeServer(t, func(runSpec) runResult { return runResult{} })
		if err := os.MkdirAll(s.cfg.Root+"/"+devFrontendDeps, 0o755); err != nil {
			t.Fatal(err)
		}
		read := false
		s.taskVersion = func(context.Context) (string, error) { read = true; return c.version, c.readErr }
		text, err := s.checkRun(ctx, checkIn{Target: c.target, Pattern: c.pattern})
		if c.wantErr {
			if err == nil || !strings.Contains(err.Error(), "ab 3.54.0") || !strings.Contains(err.Error(), "gefunden 3.40.0") || started.Load() != 0 {
				t.Errorf("%s: %v (gestartet %d)", c.name, err, started.Load())
			}
			continue
		}
		if err != nil || started.Load() != 1 || read != c.wantRead {
			t.Errorf("%s: %v (gestartet %d, gelesen %v)", c.name, err, started.Load(), read)
		}
		if strings.Contains(text, "Task-Version nicht lesbar") != c.wantWarn {
			t.Errorf("%s: Warnung %q", c.name, text)
		}
	}
}

// B-871/AC-01: Zwei Läufe mit Testmuster fragen `task --version` einmal ab, auch wenn die Version nicht lesbar ist
// (Warnung bei beiden Läufen); ein abgebrochener erster Aufruf wird nicht gemerkt.
func TestCheckRunFragtTaskVersionEinmal(t *testing.T) {
	t.Parallel()
	for _, readErr := range []error{nil, errors.New("nicht gefunden")} {
		s, _ := fakeServer(t, func(runSpec) runResult { return runResult{} })
		if err := os.MkdirAll(s.cfg.Root+"/"+devFrontendDeps, 0o755); err != nil {
			t.Fatal(err)
		}
		reads := 0
		s.taskVersion = func(context.Context) (string, error) { reads++; return "3.54.0", readErr }
		for range 2 {
			text, err := s.checkRun(context.Background(), checkIn{Target: "task:test", Pattern: "planning"})
			if err != nil || strings.Contains(text, "Task-Version nicht lesbar") != (readErr != nil) {
				t.Fatalf("Lauf (Fehler %v): %v %q", readErr, err, text)
			}
		}
		if reads != 1 {
			t.Errorf("task --version %d-mal gelesen (Fehler %v), erwartet 1", reads, readErr)
		}
	}
	var m taskVersionMemo
	ctx, cancel := context.WithCancel(context.Background())
	reads := 0
	lesen := m.lesen(func(context.Context) (string, error) { reads++; return "3.54.0", nil })
	cancel()
	_, _ = lesen(ctx)
	_, _ = lesen(context.Background())
	_, _ = lesen(context.Background())
	if reads != 2 {
		t.Errorf("abgebrochener Aufruf gemerkt: %d Aufrufe, erwartet 2", reads)
	}
}

// B-868/AC-01: requirements.md nennt die geprüfte Mindestversion.
func TestRequirementsNenntTaskMindestversion(t *testing.T) {
	t.Parallel()
	b, err := os.ReadFile("../../../../requirements.md")
	if err != nil {
		t.Fatal(err)
	}
	row := regexp.MustCompile(`(?m)^\| Go Task \| ([^|]+)\|`).FindStringSubmatch(string(b))
	if row == nil || !strings.Contains(row[1], "ab "+versionText(minTaskVersion)) {
		t.Errorf("requirements.md: Zeile Go Task nennt nicht „ab %s“: %v", versionText(minTaskVersion), row)
	}
}
