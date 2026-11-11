package main

// Zugriff auf Git und die Go-Module des Checkouts: geänderte Dateien, Modul-Ordner, Abnehmer geänderter Pakete.

import (
	"bufio"
	"bytes"
	"os"
	"os/exec"
	"path"
	"path/filepath"
	"strings"
	"sync"
)

// gitLines ruft git auf und liefert eine Zeile je Eintrag.
func gitLines(args ...string) ([]string, error) {
	out, err := exec.Command("git", append([]string{"-c", "core.quotePath=false"}, args...)...).Output()
	if err != nil {
		return nil, err
	}
	var res []string
	sc := bufio.NewScanner(bytes.NewReader(out))
	for sc.Scan() {
		if l := strings.TrimSpace(sc.Text()); l != "" {
			res = append(res, l)
		}
	}
	return res, nil
}

// changedFiles liefert Basis und geänderte Dateien. Ohne Vorgabe ist die Basis HEAD (der Commit, der gerade entsteht:
// geänderte und neue Dateien); ist der Checkout sauber, der letzte Commit (HEAD~1).
func changedFiles(base string) (string, []string, error) {
	untracked, err := gitLines("ls-files", "--others", "--exclude-standard")
	if err != nil {
		return "", nil, err
	}
	if base == "" {
		base = "HEAD"
		dirty, err := gitLines("diff", "--name-only", "--no-renames", "HEAD")
		if err != nil {
			return "", nil, err
		}
		if len(dirty)+len(untracked) == 0 {
			base = "HEAD~1"
		}
	}
	if mb, err := gitLines("merge-base", "HEAD", base); err == nil && len(mb) == 1 {
		base = mb[0] // wie der eigene Stand vom Ziel abweicht, ohne dessen neuere Commits
	}
	files, err := gitLines("diff", "--name-only", "--no-renames", base)
	if err != nil {
		return "", nil, err
	}
	return base, append(files, untracked...), nil
}

// loadRepo liest die Modul-Ordner (eingecheckte go.mod) und liefert die Abfrage nach Go-Dateien je Ordner.
func loadRepo() (repo, error) {
	mods, err := gitLines("ls-files", "*go.mod")
	if err != nil {
		return repo{}, err
	}
	r := repo{}
	for _, m := range mods {
		r.modules = append(r.modules, path.Dir(m))
	}
	cache := map[string]bool{}
	r.hasGo = func(dir string) bool {
		if v, ok := cache[dir]; ok {
			return v
		}
		found, _ := filepath.Glob(filepath.Join(filepath.FromSlash(dir), "*.go"))
		cache[dir] = len(found) > 0
		return cache[dir]
	}
	return r, nil
}

// modulePath liest den Modulnamen aus go.mod.
func modulePath(mod string) string {
	data, err := os.ReadFile(filepath.Join(filepath.FromSlash(mod), "go.mod"))
	if err != nil {
		return ""
	}
	for _, l := range strings.Split(string(data), "\n") {
		if name, ok := strings.CutPrefix(strings.TrimSpace(l), "module "); ok {
			return strings.TrimSpace(name)
		}
	}
	return ""
}

// usesRoot: Bindet das Modul das Wurzel-Modul per replace ein?
func usesRoot(mod string) bool {
	data, err := os.ReadFile(filepath.Join(filepath.FromSlash(mod), "go.mod"))
	return err == nil && mod != "." && strings.Contains(string(data), "k3c => ")
}

const listFormat = `{{.ImportPath}} {{join .Deps " "}} {{join .TestImports " "}} {{join .XTestImports " "}}`

// dependentsIn liefert die Pakete eines Moduls, die eines der geänderten Pakete (Importpfade) importieren.
func dependentsIn(mod string, changed map[string]bool) []string {
	cmd := exec.Command("go", "list", "-e", "-f", listFormat, "./...")
	cmd.Dir = filepath.FromSlash(mod)
	out, err := cmd.Output()
	if err != nil {
		say("⚠ go list in %s scheitert (%v): Abnehmer dort ungeprüft", mod, err)
		return nil
	}
	var res []string
	for _, l := range strings.Split(string(out), "\n") {
		f := strings.Fields(l)
		if len(f) < 2 || changed[f[0]] {
			continue
		}
		for _, dep := range f[1:] {
			if changed[dep] {
				res = append(res, f[0])
				break
			}
		}
	}
	return res
}

// changedImports: Importpfade aller geänderten Pakete über alle Module.
func changedImports(p plan) map[string]bool {
	res := map[string]bool{}
	for mod, mp := range p.Mods {
		name := modulePath(mod)
		for d := range mp.Dirs {
			res[path.Join(name, d)] = true
		}
		for d := range mp.Gone {
			res[path.Join(name, d)] = true
		}
	}
	return res
}

// allTargets bildet die Go-Ziele je Modul; die Abnehmer sucht es in allen Modulen nebeneinander.
func allTargets(r repo, p plan, sim bool) []goTargets {
	changed := changedImports(p)
	deps := make([][]string, len(r.modules))
	if len(changed) > 0 {
		var wg sync.WaitGroup
		for i, m := range r.modules {
			wg.Add(1)
			go func() {
				defer wg.Done()
				deps[i] = dependentsIn(m, changed)
			}()
		}
		wg.Wait()
	}
	var res []goTargets
	rootAll := p.Mods["."] != nil && p.Mods["."].All
	for i, m := range r.modules {
		t := targetsOf(m, p.Mods[m], deps[i], sim)
		if rootAll && usesRoot(m) {
			t.Vet = []string{"./..."} // go.mod der Wurzel wirkt über replace auch hier
		}
		res = append(res, t)
	}
	return res
}
