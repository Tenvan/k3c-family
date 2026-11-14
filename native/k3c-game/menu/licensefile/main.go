// Command licensefile schreibt die Lizenzdatei des nativen Spiels nach stdout; der Task native:game:paket legt sie als
// LIZENZEN.txt ins Paket (B-808). Oben steht die Liste der Lizenz-Seite (menu.LicenseText), darunter die Volltexte aller
// Module, die in die EXE gelinkt sind (LICENSE-, COPYING- und NOTICE-Dateien aus dem Modul-Cache; Apache-2.0 §4, BSD und
// MIT verlangen sie bei Weitergabe). Aufruf im Modul-Ordner des Spiels; -version setzt die Version der Datei.
package main

import (
	"flag"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sort"
	"strings"

	"k3c/native/k3c-game/menu"
)

func main() {
	flag.StringVar(&menu.Version, "version", menu.Version, "Version für die Lizenzdatei (leer = unbekannt)")
	full := flag.Bool("full", true, "Volltexte der gelinkten Module anhängen")
	flag.Parse()
	fmt.Print(menu.LicenseText())
	if !*full {
		return
	}
	mods, err := linkedModules()
	if err != nil {
		fmt.Fprintln(os.Stderr, "❌ licensefile:", err)
		os.Exit(1)
	}
	if root, err := exec.Command("go", "env", "GOROOT").Output(); err == nil { // Go-Laufzeit und Standardbibliothek
		mods = append([]module{{"Go (Laufzeit, Standardbibliothek)", runtime.Version(), strings.TrimSpace(string(root))}}, mods...)
	}
	for _, m := range mods {
		texts := licenseFiles(m.dir)
		if len(texts) == 0 {
			fmt.Fprintln(os.Stderr, "❌ licensefile: keine Lizenzdatei in", m.path)
			os.Exit(1)
		}
		fmt.Printf("\n\n==== %s %s ====\n", m.path, m.version)
		for _, t := range texts {
			fmt.Printf("\n-- %s --\n%s", filepath.Base(t.name), t.body)
		}
	}
}

type module struct{ path, version, dir string }

// linkedModules: fremde Module der Pakete, die das Spiel (Paket .) linkt; die eigenen (k3c…) entfallen.
func linkedModules() ([]module, error) {
	out, err := exec.Command("go", "list", "-deps", "-f",
		`{{with .Module}}{{.Path}}|{{.Version}}|{{.Dir}}{{end}}`, ".").Output()
	if err != nil {
		return nil, fmt.Errorf("go list: %w", err)
	}
	seen := map[string]module{}
	for _, l := range strings.Split(strings.ReplaceAll(string(out), "\r", ""), "\n") {
		f := strings.SplitN(l, "|", 3)
		if len(f) != 3 || f[0] == "k3c" || strings.HasPrefix(f[0], "k3c/") {
			continue
		}
		seen[f[0]] = module{f[0], f[1], f[2]}
	}
	res := make([]module, 0, len(seen))
	for _, m := range seen {
		res = append(res, m)
	}
	sort.Slice(res, func(i, j int) bool { return res[i].path < res[j].path })
	return res, nil
}

type text struct{ name, body string }

// licenseFiles liest die Lizenz- und Hinweisdateien im Modul-Ordner (ohne Unterordner).
func licenseFiles(dir string) []text {
	entries, _ := os.ReadDir(dir)
	var res []text
	for _, e := range entries {
		up := strings.ToUpper(e.Name())
		if e.IsDir() || !(strings.HasPrefix(up, "LICENSE") || strings.HasPrefix(up, "LICENCE") ||
			strings.HasPrefix(up, "COPYING") || strings.HasPrefix(up, "NOTICE")) {
			continue
		}
		if b, err := os.ReadFile(filepath.Join(dir, e.Name())); err == nil {
			res = append(res, text{e.Name(), strings.ReplaceAll(string(b), "\r", "")})
		}
	}
	return res
}
