package mcpsrv

import (
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// gitRepo legt mit git ein Repo des Spiels an und hängt die Worktrees unter den relativen Pfaden wts an (B-388).
func gitRepo(t *testing.T, wts ...string) (main string, paths []string) {
	t.Helper()
	if _, err := exec.LookPath("git"); err != nil {
		t.Skip("git fehlt")
	}
	main = filepath.Join(t.TempDir(), "k3c")
	ticket := filepath.Join("docs", "backlog", "B-001-x.md")
	writeFile(t, filepath.Join(main, "go.mod"), "module k3c\n")
	writeFile(t, filepath.Join(main, ticket), "# B-001 · X\n\n- **Status:** offen\n\n## Ausgangslage\n\nalt\n")
	writeFile(t, filepath.Join(main, "docs", "backlog", "README.md"),
		"# Backlog\n\n"+indexHead("Offen")+"| [B-001](B-001-x.md) | DEV | Idee | mittel | offen | – | X |\n\n"+indexHead("Archiv"))
	wd, _ := os.Getwd()
	repo, err := FindRoot(wd)
	if err != nil {
		t.Fatal(err)
	}
	vorlage, err := os.ReadFile(filepath.Join(repo, "docs", "vorlagen", "ticket.md"))
	if err != nil {
		t.Fatal(err)
	}
	writeFile(t, filepath.Join(main, "docs", "vorlagen", "ticket.md"), string(vorlage))
	git := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", append([]string{"-C", main, "-c", "user.name=t", "-c", "user.email=t@t"}, args...)...)
		if out, err := cmd.CombinedOutput(); err != nil {
			t.Fatalf("git %v: %v\n%s", args, err, out)
		}
	}
	git("init", "-q")
	git("add", ".")
	git("commit", "-qm", "start")
	for i, wt := range wts {
		p := filepath.Join(main, filepath.FromSlash(wt))
		git("worktree", "add", "-q", "-b", "b"+string(rune('a'+i)), p)
		paths = append(paths, p)
	}
	return main, paths
}

func ticketText(t *testing.T, dir string) string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(dir, "docs", "backlog", "B-001-x.md"))
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

func section(text, checkout string) map[string]any {
	args := map[string]any{"id": "B-001", "section": "Ausgangslage", "text": text}
	if checkout != "" {
		args["checkout"] = checkout
	}
	return args
}

// AC-01: checkout (Name oder Pfad) gilt vor dem Header; geschrieben wird nur im genannten Worktree.
func TestCheckoutVorHeader(t *testing.T) {
	main, wts := gitRepo(t, ".claude/worktrees/wt-a")
	logs := &lockedLog{}
	s := New(Config{Root: main, Version: "test", Log: slog.New(slog.NewJSONHandler(logs, nil))})
	cs := connectWithHeader(t, s, main)
	for _, arg := range []string{"wt-a", wts[0]} {
		text, isErr := callText(t, cs, "plan_section", section("per "+arg, arg))
		if isErr || !strings.Contains(text, "Checkout: Worktree wt-a") {
			t.Fatalf("checkout %q: %q", arg, text)
		}
		if m, w := ticketText(t, main), ticketText(t, wts[0]); strings.Contains(m, "per ") || !strings.Contains(w, "per "+arg) {
			t.Fatalf("checkout %q schrieb falsch:\nWurzel:\n%s\nWorktree:\n%s", arg, m, w)
		}
	}
	set := map[string]any{"id": "B-001", "fields": map[string]string{"Status": "verworfen"}, "checkout": "wt-a"}
	if text, isErr := callText(t, cs, "plan_set", set); isErr || !strings.Contains(text, "Checkout: Worktree wt-a") {
		t.Fatalf("plan_set: %q", text)
	}
	if _, err := os.Stat(filepath.Join(main, "docs", "backlog", "B-001-x.md")); err != nil {
		t.Errorf("plan_set änderte die Wurzel: %v", err)
	}
	if _, err := os.Stat(filepath.Join(wts[0], "docs", "backlog", "archiv", "B-001-x.md")); err != nil {
		t.Errorf("plan_set archivierte nicht im Worktree: %v", err)
	}
	// AC-03: ohne checkout gilt weiter der Header.
	if text, isErr := callText(t, cs, "plan_section", section("ohne", "")); isErr || !strings.Contains(text, "Checkout: Repo-Wurzel") {
		t.Fatalf("ohne checkout: %q", text)
	}
	// AC-04: Das Log führt checkout_arg je Aufruf.
	got := logs.lines("plan_section")
	if len(got) != 3 || !strings.Contains(got[0], `"checkout_arg":true`) || !strings.Contains(got[2], `"checkout_arg":false`) {
		t.Errorf("Log:\n%s", strings.Join(got, "\n"))
	}
}

// AC-02: Unbekannter Name, Pfad außerhalb und mehrdeutiger Name werden abgelehnt, ohne zu schreiben; die Meldung nennt die Worktrees.
func TestCheckoutUngueltig(t *testing.T) {
	main, wts := gitRepo(t, "a/doppelt", "b/doppelt", "c/einzeln")
	s := New(Config{Root: main, Version: "test"})
	cs := connect(t, s)
	cases := map[string]string{
		"gibtsnicht":              "einzeln",
		t.TempDir():               "einzeln",
		"doppelt":                 "mehrdeutig",
		filepath.Join("rel", "x"): "einzeln",
	}
	for arg, want := range cases {
		text, isErr := callText(t, cs, "plan_section", section("falsch", arg))
		if !isErr || !strings.Contains(text, want) {
			t.Errorf("checkout %q: %q (erwartet %q)", arg, text, want)
		}
	}
	for _, dir := range append(wts, main) {
		if strings.Contains(ticketText(t, dir), "falsch") {
			t.Errorf("geschrieben in %s", dir)
		}
	}
}

// check_run nimmt checkout; Lese-Tools kennen es nicht (Schema lehnt ab, Hinweis nennt die Parameter).
func TestCheckoutNurBeiSchreibendenTools(t *testing.T) {
	main, _ := gitRepo(t, ".claude/worktrees/wt-a")
	s := New(Config{Root: main, Version: "test"})
	cs := connect(t, s)
	if text, _ := callText(t, cs, "check_run", map[string]any{"target": "gibtsnicht", "checkout": "wt-a"}); strings.Contains(text, "additional") {
		t.Errorf("check_run ohne checkout im Schema: %q", text)
	}
	if text, isErr := callText(t, cs, "plan_get", map[string]any{"id": "B-001", "checkout": "wt-a"}); !isErr || !strings.Contains(text, "gültige Parameter: id") {
		t.Errorf("plan_get mit checkout: %q", text)
	}
	if text, isErr := callText(t, cs, "task_start", map[string]any{"task": "check", "checkout": "wt-a"}); !isErr || strings.Contains(text, "additional") {
		t.Errorf("task_start mit checkout: %q", text)
	}
}

func indexHead(section string) string {
	return "## " + section + "\n\n| Nr. | Domäne | Typ | Prio | Status | Sprint | Titel |\n|---|---|---|---|---|---|---|\n"
}
