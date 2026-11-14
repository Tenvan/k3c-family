package mcpsrv

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// B-866/AC-01: Muster mit Oder-Strich und Ankern laufen, Shell-Zeichen wie `;` werden mit dem Zeichenvorrat abgelehnt.
func TestCheckRunMusterMitOderUndAnkern(t *testing.T) {
	t.Parallel()
	goTest, err := findTarget("go:test")
	if err != nil {
		t.Fatal(err)
	}
	for _, bad := range []string{"TestA;rm", "TestA&TestB", "TestA`x`", `TestA\x`} {
		if _, err := goTest.args(bad); err == nil || !strings.Contains(err.Error(), "| ^ $ ( )") {
			t.Errorf("%q: %v", bad, err)
		}
	}
	if _, err := exec.LookPath("go"); err != nil {
		t.Skip("go nicht verfügbar")
	}
	dir := t.TempDir()
	write := func(name, text string) {
		if err := os.WriteFile(filepath.Join(dir, name), []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	write("go.mod", "module muster\n\ngo 1.21\n")
	write("muster_test.go", "package muster\n\nimport \"testing\"\n\n"+
		"func TestA(*testing.T)  {}\nfunc TestAB(*testing.T) {}\nfunc TestB(*testing.T)  {}\nfunc TestC(*testing.T)  {}\n")
	for pattern, want := range map[string][]string{"TestA|TestB": {"TestA", "TestAB", "TestB"}, "^TestA$": {"TestA"}} {
		args, err := goTest.args(pattern)
		if err != nil {
			t.Fatalf("%q abgelehnt: %v", pattern, err)
		}
		cmd := exec.Command(args[0], append(args[1:], "-v", "-count=1")...) // wie check_run: ohne Shell
		cmd.Dir, cmd.Env = dir, append(os.Environ(), "GOWORK=off", "GOFLAGS=")
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("%q: %v\n%s", pattern, err, out)
		}
		var ran []string
		for _, l := range strings.Split(string(out), "\n") {
			if name, ok := strings.CutPrefix(strings.TrimSpace(l), "=== RUN   "); ok {
				ran = append(ran, name)
			}
		}
		if strings.Join(ran, ",") != strings.Join(want, ",") {
			t.Errorf("%q: gelaufen %v, erwartet %v", pattern, ran, want)
		}
	}
}

// B-866: dev:test gibt das Muster als Task-Variable weiter; Task setzt sie als Text in seine Shell ein. Die einfachen
// Anführungszeichen halten `|` und `$(…)` dort wörtlich (ohne task im PATH übersprungen).
func TestCheckRunMusterInTaskVariableGequotet(t *testing.T) {
	t.Parallel()
	dev, err := findTarget("dev:test")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := exec.LookPath("task"); err != nil {
		t.Skip("task nicht verfügbar")
	}
	taskfile := filepath.Join(t.TempDir(), "Taskfile.yml")
	if err := os.WriteFile(taskfile, []byte("version: '3'\ntasks:\n  zeig:\n    cmds:\n      - echo [{{.GO_TEST_FLAGS}}]\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	for _, pattern := range []string{"TestA|TestB", "^TestA$", "x$(exit)y"} {
		args, err := dev.args(pattern)
		if err != nil {
			t.Fatalf("%q abgelehnt: %v", pattern, err)
		}
		out, err := exec.Command("task", "-s", "-t", taskfile, "zeig", args[len(args)-1]).CombinedOutput()
		if got := strings.TrimSpace(string(out)); err != nil || got != "[-run="+pattern+"]" {
			t.Errorf("%q: %q (%v)", pattern, got, err)
		}
	}
}
