package devrun

import (
	"io/fs"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// fileSig ist, woran sich eine Änderung erkennen lässt.
type fileSig struct {
	mod  int64
	size int64
}

// skipDirs sind Ordner ohne Go-Quellen der App (Oberfläche, Abhängigkeiten, Build-Ergebnisse).
var skipDirs = []string{"frontend", "node_modules", "bin", "build"}

// snapshot liest Änderungszeit und Größe aller beobachteten Dateien. Ein fehlender Pfad ist nicht schlimm (er kann
// später entstehen), nicht lesbare Pfade zählen nicht.
func snapshot(paths, exts []string) map[string]fileSig {
	out := map[string]fileSig{}
	add := func(path string, info fs.FileInfo) {
		out[filepath.ToSlash(path)] = fileSig{mod: info.ModTime().UnixNano(), size: info.Size()}
	}
	for _, p := range paths {
		info, err := os.Stat(p)
		switch {
		case err != nil:
			continue
		case !info.IsDir():
			add(p, info) // eine einzeln genannte Datei zählt immer
			continue
		}
		_ = filepath.WalkDir(p, func(path string, d fs.DirEntry, err error) error {
			if err != nil {
				return nil
			}
			if d.IsDir() {
				if name := d.Name(); path != p && (strings.HasPrefix(name, ".") || slices.Contains(skipDirs, name)) {
					return filepath.SkipDir
				}
				return nil
			}
			if watched(d.Name(), exts) {
				if info, err := d.Info(); err == nil {
					add(path, info)
				}
			}
			return nil
		})
	}
	return out
}

// watched sagt, ob eine Datei aus einem Ordner beobachtet wird: go.mod, go.sum oder passende Endung, keine Testdatei.
func watched(name string, exts []string) bool {
	if name == "go.mod" || name == "go.sum" {
		return true
	}
	return !strings.HasSuffix(name, "_test.go") && slices.Contains(exts, strings.ToLower(filepath.Ext(name)))
}

// changed nennt eine Datei, die neu, geändert oder weg ist (die erste in Namensreihenfolge); "" heißt: nichts hat sich
// geändert.
func changed(before, after map[string]fileSig) string {
	var names []string
	for p, sig := range after {
		if old, ok := before[p]; !ok || old != sig {
			names = append(names, p)
		}
	}
	for p := range before {
		if _, ok := after[p]; !ok {
			names = append(names, p)
		}
	}
	if len(names) == 0 {
		return ""
	}
	slices.Sort(names)
	return names[0]
}
