package main

import (
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// area ordnet einen Go-Ordner im Tag einem Rust-Ordner im Arbeitsbaum zu (docs/standards/rust-port.md § 2).
type area struct{ name, goDir, rustDir string }

var areas = []area{
	{"hud", "native/k3c-game/hud", "rust/crates/k3c-game/src/hud"},
	{"props", "native/k3c-game/props", "rust/crates/k3c-game/src/props"},
	{"view", "native/k3c-game/view", "rust/crates/k3c-game/src/view"},
	{"world", "native/k3c-game/world", "rust/crates/k3c-game/src/world"},
	{"fx", "native/k3c-game/fx", "rust/crates/k3c-game/src/fx"},
	{"actors", "native/k3c-game/actors", "rust/crates/k3c-game/src/actors"},
	{"menu", "native/k3c-game/menu", "rust/crates/k3c-game/src/menu"},
	{"local", "native/k3c-game/local", "rust/crates/k3c-game/src/local"},
	{"game", "native/k3c-game/game", "rust/crates/k3c-game/src/game"},
	{"input", "native/k3c-game/input", "rust/crates/k3c-game/src/input"},
	{"audio", "native/k3c-game/audio", "rust/crates/k3c-game/src/audio"},
	{"art", "native/k3c-game/art", "rust/crates/k3c-game/src/art"},
	{"devtools", "native/k3c-game/devtools", "rust/crates/k3c-game/src/devtools"},
	{"e2e", "native/k3c-game/e2e", "rust/crates/k3c-game/tests"},
	{"rig", "native/k3c-rig", "rust/crates/k3c-rig/src"},
	{"sim", "engine/sim", "rust/crates/k3c-sim/src"},
	{"level", "engine/level", "rust/crates/k3c-level/src"},
	{"room", "engine/room", "rust/crates/k3c-room/src"},
	{"store", "engine/store", "rust/crates/k3c-room/src"},
	{"balance", "engine/balance", "rust/crates/k3c-balance/src"},
}

// goTest ist ein Go-Test aus dem Tag.
type goTest struct{ name, file string }

var testFunc = regexp.MustCompile(`(?m)^func (Test[A-Za-z0-9_]*)\(`)

// collectGo sammelt die Go-Tests je Bereich aus dem Tag; Dateien ohne Bereich stehen unter dem Schlüssel "" mit dem Ordner.
func collectGo(tag string, files []string) map[string][]goTest {
	res := map[string][]goTest{}
	for _, f := range files {
		if !strings.HasSuffix(f, "_test.go") {
			continue
		}
		src, err := git("show", tag+":"+f)
		if err != nil {
			continue
		}
		key := "\x00" + filepath.ToSlash(filepath.Dir(f))
		for _, a := range areas {
			if filepath.ToSlash(filepath.Dir(f)) == a.goDir || strings.HasPrefix(f, a.goDir+"/") {
				key = a.name
				break
			}
		}
		for _, m := range testFunc.FindAllStringSubmatch(src, -1) {
			res[key] = append(res[key], goTest{name: m[1], file: f})
		}
	}
	return res
}

// withoutTarget liefert die Zahl der Go-Tests je Ordner ohne Rust-Gegenstück, sortiert.
func withoutTarget(all map[string][]goTest) []string {
	var out []string
	for k, v := range all {
		if strings.HasPrefix(k, "\x00") {
			out = append(out, strings.TrimPrefix(k, "\x00")+"|"+itoa(len(v)))
		}
	}
	sort.Strings(out)
	return out
}

var rustFn = regexp.MustCompile(`(?m)\bfn\s+([A-Za-z0-9_]+)`)

// rustFns sammelt alle Funktionsnamen der Rust-Dateien unter dir (Tests liegen in *_tests.rs und in Modulen).
func rustFns(dir string) map[string]bool {
	names := map[string]bool{}
	_ = filepath.WalkDir(dir, func(p string, d os.DirEntry, err error) error {
		if err != nil || d.IsDir() || !strings.HasSuffix(p, ".rs") {
			return nil
		}
		if b, e := os.ReadFile(p); e == nil {
			for _, m := range rustFn.FindAllStringSubmatch(string(b), -1) {
				names[m[1]] = true
			}
		}
		return nil
	})
	return names
}
