package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
)

// metadata ist der Teil von `cargo metadata --format-version 1`, den die Lizenzliste braucht.
type metadata struct {
	Packages []pkg `json:"packages"`
	Resolve  struct {
		Nodes []node `json:"nodes"`
	} `json:"resolve"`
}

type pkg struct {
	ID           string  `json:"id"`
	Name         string  `json:"name"`
	Version      string  `json:"version"`
	License      *string `json:"license"`
	Repository   *string `json:"repository"`
	Source       *string `json:"source"`
	ManifestPath string  `json:"manifest_path"`
}

type node struct {
	ID   string   `json:"id"`
	Deps []depRef `json:"deps"`
}

type depRef struct {
	Pkg      string `json:"pkg"`
	DepKinds []struct {
		Kind *string `json:"kind"`
	} `json:"dep_kinds"`
}

type text struct{ name, body string }

// linkedPackages liefert die fremden Pakete (mit Quelle, also keine Workspace-Mitglieder), die vom Paket root über
// normale Abhängigkeiten erreichbar sind; Build- und Dev-Abhängigkeiten gelangen nicht in die EXE. Sortiert nach Name, Version.
func linkedPackages(m metadata, root string) ([]pkg, error) {
	byID := map[string]pkg{}
	rootID := ""
	for _, p := range m.Packages {
		byID[p.ID] = p
		if p.Name == root && p.Source == nil {
			rootID = p.ID
		}
	}
	if rootID == "" {
		return nil, fmt.Errorf("paket %q fehlt in cargo metadata", root)
	}
	deps := map[string][]string{}
	for _, n := range m.Resolve.Nodes {
		for _, d := range n.Deps {
			for _, k := range d.DepKinds {
				if k.Kind == nil {
					deps[n.ID] = append(deps[n.ID], d.Pkg)
					break
				}
			}
		}
	}
	seen := map[string]bool{rootID: true}
	queue := []string{rootID}
	var res []pkg
	for len(queue) > 0 {
		id := queue[0]
		queue = queue[1:]
		for _, d := range deps[id] {
			if seen[d] {
				continue
			}
			seen[d] = true
			queue = append(queue, d)
			if p := byID[d]; p.Source != nil {
				res = append(res, p)
			}
		}
	}
	sort.Slice(res, func(i, j int) bool {
		if res[i].Name != res[j].Name {
			return res[i].Name < res[j].Name
		}
		return res[i].Version < res[j].Version
	})
	return res, nil
}

// isLicenseName meldet, ob ein Dateiname eine Lizenz- oder Hinweisdatei benennt (Groß- und Kleinschreibung egal).
func isLicenseName(name string) bool {
	up := strings.ToUpper(name)
	for _, prefix := range []string{"LICENSE", "LICENCE", "COPYING", "UNLICENSE", "NOTICE"} {
		if strings.HasPrefix(up, prefix) {
			return true
		}
	}
	return false
}

// licenseFiles liest die Lizenz- und Hinweisdateien im Ordner dir (ohne Unterordner), nach Name sortiert.
func licenseFiles(dir string) ([]text, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	var res []text
	for _, e := range entries {
		if e.IsDir() || !isLicenseName(e.Name()) {
			continue
		}
		b, err := os.ReadFile(filepath.Join(dir, e.Name()))
		if err != nil {
			return nil, err
		}
		res = append(res, text{e.Name(), strings.ReplaceAll(string(b), "\r", "")})
	}
	return res, nil
}

// spdxURLs nennt zu einem SPDX-Ausdruck („MIT OR Apache-2.0“, „Apache-2.0/MIT“) die Adressen der Standardtexte.
func spdxURLs(expr string) []string {
	var res []string
	for _, id := range strings.FieldsFunc(expr, func(r rune) bool { return strings.ContainsRune(" /()", r) }) {
		if id == "OR" || id == "AND" || id == "WITH" {
			continue
		}
		res = append(res, "https://spdx.org/licenses/"+id+".html")
	}
	return res
}

// render schreibt den Kopf und je Paket Name, Version, SPDX-Ausdruck und die Lizenztexte. Liegt dem Paket keine
// Lizenzdatei bei (einige Crates veröffentlichen sie nicht), stehen SPDX-Ausdruck, Quelle und die Adresse der
// Standardtexte da; fehlt auch der SPDX-Ausdruck, bricht der Aufruf ab.
func render(w io.Writer, head string, pkgs []pkg, read func(dir string) ([]text, error)) error {
	if _, err := io.WriteString(w, head); err != nil {
		return err
	}
	_, _ = fmt.Fprintf(w, "\n\nRust-Crates im Spiel (%d), Lizenztexte aus den Paketen. Für die gelinkten Bibliotheken ist diese Liste maßgeblich. · Rust crates linked into the game; this list is authoritative for the linked libraries.\n", len(pkgs))
	for _, p := range pkgs {
		texts, err := read(filepath.Dir(p.ManifestPath))
		if err != nil {
			return fmt.Errorf("%s %s: %w", p.Name, p.Version, err)
		}
		if len(texts) == 0 && (p.License == nil || *p.License == "") {
			return fmt.Errorf("weder Lizenzdatei noch SPDX-Ausdruck im Crate %s %s", p.Name, p.Version)
		}
		spdx := "?"
		if p.License != nil {
			spdx = *p.License
		}
		_, _ = fmt.Fprintf(w, "\n\n==== %s %s (%s) ====\n", p.Name, p.Version, spdx)
		if len(texts) == 0 {
			_, _ = fmt.Fprintf(w, "\nDas Crate enthält keine Lizenzdatei. Lizenz: %s\n", spdx)
			if p.Repository != nil {
				_, _ = fmt.Fprintf(w, "Quelle: %s\n", *p.Repository)
			}
			for _, u := range spdxURLs(spdx) {
				_, _ = fmt.Fprintf(w, "Text: %s\n", u)
			}
		}
		for _, t := range texts {
			_, _ = fmt.Fprintf(w, "\n-- %s --\n%s", t.name, t.body)
		}
	}
	return nil
}

// cargoMetadata ruft `cargo metadata` im Rust-Ordner auf (nur das Windows-Ziel, Cargo.lock bleibt unverändert).
func cargoMetadata(rustDir, target string) (metadata, error) {
	cmd := exec.Command("cargo", "metadata", "--format-version", "1", "--locked", "--filter-platform", target)
	cmd.Dir = rustDir
	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	out, err := cmd.Output()
	if err != nil {
		return metadata{}, fmt.Errorf("cargo metadata: %w: %s", err, strings.TrimSpace(stderr.String()))
	}
	var m metadata
	if err := json.Unmarshal(out, &m); err != nil {
		return metadata{}, fmt.Errorf("cargo metadata: %w", err)
	}
	return m, nil
}

// cmdLizenzen: der Kopf kommt von `k3c-game licenses` (Liste der Lizenz-Seite im Menü), darunter die Crates.
func cmdLizenzen(args []string, out io.Writer) error {
	fs := flag.NewFlagSet("lizenzen", flag.ContinueOnError)
	exe := fs.String("exe", "", "k3c-game.exe, gibt mit `licenses` den Kopf aus")
	rust := fs.String("rust", "rust", "Ordner des Rust-Workspace")
	target := fs.String("target", "x86_64-pc-windows-msvc", "Zielplattform der EXE")
	if err := fs.Parse(args); err != nil {
		return err
	}
	if *exe == "" {
		return errors.New("-exe fehlt")
	}
	head, err := exec.Command(*exe, "licenses").Output()
	if err != nil {
		return fmt.Errorf("%s licenses: %w", *exe, err)
	}
	m, err := cargoMetadata(*rust, *target)
	if err != nil {
		return err
	}
	// Spiel und Cheat-Fenster (RU56, B-1135) liegen im Paket; das Fenster bindet das `windows`-Crate
	pkgs, err := linkedPackages(m, "k3c-game")
	if err != nil {
		return err
	}
	win, err := linkedPackages(m, "k3c-devwin")
	if err != nil {
		return err
	}
	pkgs = mergePackages(pkgs, win)
	return render(out, strings.TrimRight(strings.ReplaceAll(string(head), "\r", ""), "\n"), pkgs, licenseFiles)
}

// mergePackages vereinigt zwei sortierte Listen ohne doppelte Name-Version-Paare, sortiert nach Name, Version.
func mergePackages(a, b []pkg) []pkg {
	seen := map[string]bool{}
	var res []pkg
	for _, p := range append(append([]pkg{}, a...), b...) {
		k := p.Name + " " + p.Version
		if !seen[k] {
			seen[k] = true
			res = append(res, p)
		}
	}
	sort.Slice(res, func(i, j int) bool {
		if res[i].Name != res[j].Name {
			return res[i].Name < res[j].Name
		}
		return res[i].Version < res[j].Version
	})
	return res
}
