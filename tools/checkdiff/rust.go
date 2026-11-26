package main

// Rust (Projekt RSU, RU1.2; Werkzeuge RU66.1): Es gibt zwei Cargo-Workspaces, das Spiel rust/ und die Werkzeuge
// rust/tools/ (eigenes [workspace], Entscheidung 014). Geänderte Dateien unter <workspace>/crates/<name>/ wählen fmt,
// clippy und Tests dieser Crate; Manifest, Lockfile oder clippy.toml direkt im Workspace-Ordner wählen den ganzen
// Workspace. rust/rust-toolchain.toml und rust/rustfmt.toml erbt rust/tools/, sie wählen beide Workspaces ganz. Der
// Spike rust/tools/spike/ gehört zu keinem Workspace. Ohne Rust-Datei läuft kein cargo.

import "strings"

// rustRoot ist der Workspace-Ordner des Spiels relativ zur Repo-Wurzel; die Werkzeuge liegen darunter in tools/.
const rustRoot = "rust/"

// rustWorkspaceFiles: Dateien direkt im Workspace-Ordner, die jede Crate dieses Workspace betreffen.
var rustWorkspaceFiles = map[string]bool{"Cargo.toml": true, "Cargo.lock": true, "clippy.toml": true}

// rustInheritedFiles: Dateien in rust/, die auch rust/tools/ erbt (Suche nach oben durch rustup und rustfmt).
var rustInheritedFiles = map[string]bool{"rust-toolchain.toml": true, "rustfmt.toml": true}

// rustFile ordnet eine Datei unter rust/ zu; true = Datei gehört zu rust/ (auch rust/tools/).
func (p *plan) rustFile(f string) bool {
	rel, ok := strings.CutPrefix(f, rustRoot)
	if !ok {
		return false
	}
	if tools, ok := strings.CutPrefix(rel, "tools/"); ok {
		pickRust(tools, p.rustTools)
		return true
	}
	if rustInheritedFiles[rel] {
		p.rust().All = true
		p.rustTools().All = true
		return true
	}
	pickRust(rel, p.rust)
	return true
}

// pickRust wählt zu einem Pfad relativ zum Workspace-Ordner die Crate oder den ganzen Workspace; get legt den Plan
// erst bei einer Auswahl an.
func pickRust(rel string, get func() *rustPlan) {
	if crate, ok := strings.CutPrefix(rel, "crates/"); ok {
		if name, _, inside := strings.Cut(crate, "/"); inside && name != "" {
			get().Crates[name] = true
		}
		return
	}
	if rustWorkspaceFiles[rel] {
		get().All = true
	}
}

func (p *plan) rust() *rustPlan {
	if p.Rust == nil {
		p.Rust = &rustPlan{Crates: map[string]bool{}}
	}
	return p.Rust
}

func (p *plan) rustTools() *rustPlan {
	if p.RustTools == nil {
		p.RustTools = &rustPlan{Crates: map[string]bool{}}
	}
	return p.RustTools
}

// rustPlan ist die Auswahl für einen Rust-Workspace.
type rustPlan struct {
	All    bool            // Manifest, Lockfile oder Toolchain geändert: ganzer Workspace
	Crates map[string]bool // geänderte Crates (Ordnername = Paketname)
}

// rustSteps: je Workspace fmt und clippy in Stufe 1, Tests in Stufe 2, alle in der Bahn rust (`-only rust` nimmt beide).
func rustSteps(p plan) []step {
	return append(rustWorkspaceSteps(p.Rust, "rust", "rust"), rustWorkspaceSteps(p.RustTools, "tools-rust", "rust/tools")...)
}

// rustWorkspaceSteps: die drei Schritte eines Workspace mit Namen <prefix>-fmt|lint|test im Ordner dir.
func rustWorkspaceSteps(rp *rustPlan, prefix, dir string) []step {
	if rp == nil || (!rp.All && len(rp.Crates) == 0) {
		return nil
	}
	scope := []string{"--workspace"}
	if !rp.All {
		scope = nil
		for _, c := range sortedKeys(rp.Crates) {
			scope = append(scope, "-p", c)
		}
	}
	cargo := func(args ...string) []string { return append([]string{"cargo"}, args...) }
	clippy := append(cargo("clippy"), append(scope, "--all-targets", "--", "-D", "warnings")...)
	return []step{
		{prefix + "-fmt", "rust", 1, dir, cargo("fmt", "--all", "--check")},
		{prefix + "-lint", "rust", 1, dir, clippy},
		{prefix + "-test", "rust", 2, dir, append(cargo("test"), scope...)},
	}
}
