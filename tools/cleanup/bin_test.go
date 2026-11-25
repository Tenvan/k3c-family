package main

import (
	"bytes"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strings"
	"testing"
)

func binFixture(t *testing.T) (root, bin string) {
	t.Helper()
	root = t.TempDir()
	bin = filepath.Join(root, "bin")
	if err := os.MkdirAll(bin, 0o755); err != nil {
		t.Fatal(err)
	}
	files := []string{
		"k3c-server.exe", "k3c-load.exe", "k3c-tui.exe", "k3c-starter.exe", "k3c-workbench-web.exe",
		"k3c-workbench-web.exe~", "k3c-workbench.exe", "k3c-studio.exe", "k3c-sim.exe", "soundatlas.exe", "aufraeumen.exe",
	}
	for _, f := range files {
		if err := os.WriteFile(filepath.Join(bin, f), []byte("x"), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	if err := os.Mkdir(filepath.Join(bin, "k3c-tui"), 0o755); err != nil { // Ordner mit altem Namen
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(bin, "k3c-tui", "keep.txt"), []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}
	// Datei außerhalb von bin/ mit altem Namen darf nie berührt werden
	if err := os.WriteFile(filepath.Join(root, "k3c-server.exe"), []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}
	return root, bin
}

func names(t *testing.T, dir string) []string {
	t.Helper()
	es, err := os.ReadDir(dir)
	if err != nil {
		t.Fatal(err)
	}
	var out []string
	for _, e := range es {
		out = append(out, e.Name())
	}
	sort.Strings(out)
	return out
}

func TestCleanBinDeletesExactlyTheOldOnes(t *testing.T) {
	root, bin := binFixture(t)
	var out bytes.Buffer
	cleanBin(options{Root: root, Out: &out}, bin)
	got := strings.Join(names(t, bin), ",")
	want := "aufraeumen.exe,k3c-studio.exe,k3c-tui,k3c-workbench.exe,soundatlas.exe"
	if got != want {
		t.Fatalf("bin/ = %s, erwartet %s", got, want)
	}
	if _, err := os.Stat(filepath.Join(bin, "k3c-tui", "keep.txt")); err != nil {
		t.Fatal("Inhalt des Ordners mit altem Namen wurde angefasst")
	}
	if _, err := os.Stat(filepath.Join(root, "k3c-server.exe")); err != nil {
		t.Fatal("Datei außerhalb von bin/ wurde gelöscht")
	}
	if !strings.Contains(out.String(), "🧹 bin/k3c-server.exe") || !strings.Contains(out.String(), "keine reguläre Datei") {
		t.Fatalf("Ausgabe unvollständig:\n%s", out.String())
	}
}

func TestCleanBinDryDeletesNothing(t *testing.T) {
	root, bin := binFixture(t)
	before := names(t, bin)
	var out bytes.Buffer
	cleanBin(options{Root: root, Dry: true, Out: &out}, bin)
	if after := names(t, bin); strings.Join(after, ",") != strings.Join(before, ",") {
		t.Fatalf("--dry hat Dateien verändert: %v -> %v", before, after)
	}
	if n := strings.Count(out.String(), "würde gelöscht"); n != 7 {
		t.Fatalf("erwartet 7 Anzeigen, %d:\n%s", n, out.String())
	}
}

func TestCleanBinLockedFileIsReportedAndRunContinues(t *testing.T) {
	root, bin := binFixture(t)
	// Eine geöffnete Datei ist unter Windows gesperrt; sonst (POSIX) bleibt der Test ohne Sperre gültig.
	f, err := os.Open(filepath.Join(bin, "k3c-load.exe"))
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = f.Close() }()
	var out bytes.Buffer
	cleanBin(options{Root: root, Out: &out}, bin)
	if _, err := os.Stat(filepath.Join(bin, "k3c-load.exe")); err == nil {
		if !strings.Contains(out.String(), "nicht gelöscht") {
			t.Fatalf("gesperrte Datei nicht gemeldet:\n%s", out.String())
		}
	}
	if _, err := os.Stat(filepath.Join(bin, "k3c-starter.exe")); err == nil {
		t.Fatal("Lauf wurde nach gesperrter Datei abgebrochen")
	}
}

func TestCleanBinMissingDirIsNoError(t *testing.T) {
	var out bytes.Buffer
	cleanBin(options{Out: &out}, filepath.Join(t.TempDir(), "bin"))
	if out.Len() != 0 {
		t.Fatalf("unerwartete Ausgabe: %s", out.String())
	}
}

func TestCleanBinSymlinkDirIsRefused(t *testing.T) {
	root, bin := binFixture(t)
	link := filepath.Join(root, "link")
	if err := os.Symlink(bin, link); err != nil {
		t.Skip("keine Symlinks möglich:", err)
	}
	var out bytes.Buffer
	cleanBin(options{Root: root, Out: &out}, link)
	if _, err := os.Stat(filepath.Join(bin, "k3c-server.exe")); err != nil {
		t.Fatal("über Link gelöscht")
	}
}

func TestBinCandidatesAreFixed(t *testing.T) {
	for _, n := range binCandidates() {
		if strings.ContainsAny(n, `*?[/\`) || strings.Contains(n, "..") {
			t.Fatalf("unzulässiger Name %q", n)
		}
		for _, keep := range []string{"k3c-workbench.exe", "k3c-studio.exe", "soundatlas.exe", "aufraeumen.exe"} {
			if n == keep {
				t.Fatalf("lebendes Programm in der Liste: %s", n)
			}
		}
	}
}

// Eine Junction (Windows, ohne Rechte anlegbar) als bin/ wird wie ein Link abgelehnt: nichts dahinter wird gelöscht.
func TestCleanBinJunctionDirIsRefused(t *testing.T) {
	if runtime.GOOS != "windows" {
		t.Skip("Junctions gibt es nur unter Windows")
	}
	root, bin := binFixture(t)
	junction := filepath.Join(root, "junction")
	if out, err := exec.Command("cmd", "/c", "mklink", "/J", junction, bin).CombinedOutput(); err != nil {
		t.Skip("keine Junction möglich:", err, string(out))
	}
	var out bytes.Buffer
	cleanBin(options{Root: root, Out: &out}, junction)
	if _, err := os.Stat(filepath.Join(bin, "k3c-server.exe")); err != nil {
		t.Fatalf("über Junction gelöscht:\n%s", out.String())
	}
}

// Aus einem Worktree räumt das Werkzeug das bin/ der Repo-Wurzel (git nennt sie zuerst), nicht das des Worktrees.
func TestMainCheckout(t *testing.T) {
	trees := []worktree{{path: "C:/repo"}, {path: "C:/repo/.claude/worktrees/x"}}
	if got := mainCheckout(trees, "C:/repo/.claude/worktrees/x"); got != "C:/repo" {
		t.Errorf("Wurzel erwartet, bekommen %q", got)
	}
	if got := mainCheckout(nil, "C:/top"); got != "C:/top" {
		t.Errorf("ohne Liste der aktuelle Checkout erwartet, bekommen %q", got)
	}
	if got := mainCheckout([]worktree{{path: ""}}, "C:/top"); got != "C:/top" {
		t.Errorf("leerer Pfad: aktueller Checkout erwartet, bekommen %q", got)
	}
}
