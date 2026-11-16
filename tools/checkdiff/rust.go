package main

// Rust (Projekt RSU, RU1.2): Geänderte Dateien unter rust/crates/<name>/ wählen fmt, clippy und Tests dieser Crate; eine
// Änderung an Manifest, Lockfile oder Toolchain-Dateien wählt den ganzen Workspace. Der Prototyp rust/spike/ gehört nicht
// zum Workspace. Ohne Rust-Datei läuft kein cargo.

import (
	"path"
	"strings"
)

// rustRoot ist der Workspace-Ordner relativ zur Repo-Wurzel.
const rustRoot = "rust/"

// rustWorkspaceFiles: Dateien im Workspace-Ordner, die jede Crate betreffen.
var rustWorkspaceFiles = map[string]bool{
	"Cargo.toml": true, "Cargo.lock": true, "rust-toolchain.toml": true, "clippy.toml": true, "rustfmt.toml": true,
}

// rustFile ordnet eine Datei unter rust/ zu; true = Datei gehört zu rust/.
func (p *plan) rustFile(f string) bool {
	rel, ok := strings.CutPrefix(f, rustRoot)
	if !ok {
		return false
	}
	if crate, ok := strings.CutPrefix(rel, "crates/"); ok {
		name, _, inside := strings.Cut(crate, "/")
		if inside && name != "" {
			p.rust().Crates[name] = true
		}
		return true
	}
	if rustWorkspaceFiles[path.Base(rel)] && !strings.Contains(rel, "/") {
		p.rust().All = true
	}
	return true
}

func (p *plan) rust() *rustPlan {
	if p.Rust == nil {
		p.Rust = &rustPlan{Crates: map[string]bool{}}
	}
	return p.Rust
}

// rustPlan ist die Auswahl für den Rust-Workspace.
type rustPlan struct {
	All    bool            // Manifest, Lockfile oder Toolchain geändert: ganzer Workspace
	Crates map[string]bool // geänderte Crates (Ordnername = Paketname)
}

// rustSteps: fmt und clippy in Stufe 1, Tests in Stufe 2, alle in der Bahn rust.
func rustSteps(p plan) []step {
	if p.Rust == nil || (!p.Rust.All && len(p.Rust.Crates) == 0) {
		return nil
	}
	scope := []string{"--workspace"}
	if !p.Rust.All {
		scope = nil
		for _, c := range sortedKeys(p.Rust.Crates) {
			scope = append(scope, "-p", c)
		}
	}
	cargo := func(args ...string) []string { return append([]string{"cargo"}, args...) }
	clippy := append(cargo("clippy"), append(scope, "--all-targets", "--", "-D", "warnings")...)
	return []step{
		{"rust-fmt", "rust", 1, "rust", cargo("fmt", "--all", "--check")},
		{"rust-lint", "rust", 1, "rust", clippy},
		{"rust-test", "rust", 2, "rust", append(cargo("test"), scope...)},
	}
}
