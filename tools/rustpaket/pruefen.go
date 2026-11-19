package main

import (
	"errors"
	"flag"
	"fmt"
	"io"
	"io/fs"
	"os"
	"path"
	"slices"
	"strings"
)

// Pflichtdateien des Rust-Pakets (relativ zum Paketordner).
var requiredFiles = []string{
	"k3c-game.exe", "README.txt", "LICENSE", "LICENSE-ASSETS", "LIZENZEN.txt", "public/schrift/LICENSE",
}

// requiredDirs müssen mindestens eine Datei enthalten; das Suffix (falls nicht leer) muss auf mindestens eine zutreffen.
var requiredDirs = []struct{ dir, suffix string }{
	{"data", ".json"}, {"public/grafik", "index.paper.json"}, {"public/audio", ""}, {"native/k3c-rig/assets", ""},
}

var (
	forbiddenExt   = []string{".go", ".rs", ".pdb", ".tmp"}
	forbiddenNames = []string{"go.mod", "go.sum", "assets.txt", "Cargo.toml", "Cargo.lock"}
	forbiddenDirs  = []string{"saves", "reports", ".git", ".work", "target", "trace"}
	// runtimeDirs entstehen beim Spielen aus dem Paket (Spielstände, Berichte, Aufzeichnung): im Entwicklungsordner
	// erlaubt (-lokal), im Paket zum Weitergeben nicht.
	runtimeDirs = []string{"saves", "reports", "trace"}
)

// pruefe prüft den Paketordner fsys und liefert je Mangel eine Zeile (leer = vollständig); n ist die Zahl der Dateien.
func pruefe(fsys fs.FS, maxExeBytes int64, lokal bool) (mangel []string, n int) {
	files := map[string]int64{}
	err := fs.WalkDir(fsys, ".", func(p string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		base := path.Base(p)
		if d.IsDir() {
			if lokal && p != "." && slices.Contains(runtimeDirs, p) {
				return fs.SkipDir
			}
			for _, f := range forbiddenDirs {
				if base == f {
					mangel = append(mangel, "verbotener Ordner: "+p)
					return fs.SkipDir
				}
			}
			return nil
		}
		info, err := d.Info()
		if err != nil {
			return err
		}
		files[p] = info.Size()
		if reason := forbiddenFile(p, base); reason != "" {
			mangel = append(mangel, "verbotene Datei: "+p+" ("+reason+")")
		}
		return nil
	})
	if err != nil {
		return []string{"Paketordner nicht lesbar: " + err.Error()}, 0
	}
	mangel = append(mangel, missing(files, maxExeBytes)...)
	if _, ok := files["LIZENZEN.txt"]; ok && !lizenzenOK(fsys) {
		mangel = append(mangel, "LIZENZEN.txt enthält keinen Crate-Abschnitt")
	}
	return mangel, len(files)
}

// forbiddenFile nennt den Grund, warum eine Datei nicht ins Paket gehört (Go-Reste, Quellen, Entwickler-Dateien), sonst "".
func forbiddenFile(p, base string) string {
	for _, e := range forbiddenExt {
		if strings.HasSuffix(base, e) {
			return "Endung " + e
		}
	}
	for _, n := range forbiddenNames {
		if base == n {
			return "Entwickler-Datei"
		}
	}
	if strings.HasPrefix(p, "data/") && strings.HasSuffix(base, ".md") {
		return "Doku unter data/"
	}
	return ""
}

// missing prüft Pflichtdateien, Pflichtordner, EXE-Größe und den Inhalt der Lizenzdatei gegen die Dateiliste.
func missing(files map[string]int64, maxExeBytes int64) []string {
	var mangel []string
	for _, f := range requiredFiles {
		if _, ok := files[f]; !ok {
			mangel = append(mangel, "Pflichtdatei fehlt: "+f)
		}
	}
	if size := files["k3c-game.exe"]; maxExeBytes > 0 && size > maxExeBytes {
		mangel = append(mangel, fmt.Sprintf("k3c-game.exe zu groß: %.1f MiB, erlaubt %.1f MiB", float64(size)/(1<<20), float64(maxExeBytes)/(1<<20)))
	}
	for _, r := range requiredDirs {
		found := false
		for f := range files {
			if strings.HasPrefix(f, r.dir+"/") && strings.HasSuffix(f, r.suffix) {
				found = true
				break
			}
		}
		if !found {
			mangel = append(mangel, "Pflichtordner leer oder fehlt: "+r.dir+" (erwartet *"+r.suffix+")")
		}
	}
	return mangel
}

// lizenzenOK meldet, ob LIZENZEN.txt mindestens einen Crate-Abschnitt enthält.
func lizenzenOK(fsys fs.FS) bool {
	b, err := fs.ReadFile(fsys, "LIZENZEN.txt")
	return err == nil && strings.Contains(string(b), "\n==== ")
}

// cmdPruefen: `pruefen [-max-exe-mib 40] [-lokal] <paketordner>`; Exit 1 mit der Mängelliste.
func cmdPruefen(args []string, out io.Writer) error {
	fl := flag.NewFlagSet("pruefen", flag.ContinueOnError)
	maxMiB := fl.Int("max-exe-mib", 40, "größte erlaubte EXE in MiB (Schwelle G1; 0 = keine Grenze)")
	lokal := fl.Bool("lokal", false, "saves/, reports/ und trace/ im Paketordner dulden (Entwicklungsordner nach dem Spielen)")
	if err := fl.Parse(args); err != nil {
		return err
	}
	if fl.NArg() != 1 {
		return errors.New("genau ein Paketordner erwartet")
	}
	dir := fl.Arg(0)
	if st, err := os.Stat(dir); err != nil || !st.IsDir() {
		return fmt.Errorf("paketordner %s fehlt (erst task rust:game:build)", dir)
	}
	fsys := os.DirFS(dir)
	mangel, n := pruefe(fsys, int64(*maxMiB)<<20, *lokal)
	if len(mangel) > 0 {
		for _, m := range mangel {
			_, _ = fmt.Fprintln(out, "❌", m)
		}
		return fmt.Errorf("paket %s unvollständig (%d Mängel)", dir, len(mangel))
	}
	_, _ = fmt.Fprintf(out, "✅ Paket %s vollständig (%d Dateien)\n", dir, n)
	return nil
}
