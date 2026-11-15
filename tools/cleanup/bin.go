package main

import (
	"fmt"
	"os"
	"path/filepath"
)

// removedPrograms sind die entfernten Programme (B-850, B-887); nur diese Namen räumt das Werkzeug in bin/ ab.
// Bewusst fest: keine Muster, keine Eingaben. Vorhandene Programme (k3c-workbench, k3c-sim, soundatlas,
// aufraeumen) stehen nie hier.
var removedPrograms = []string{"k3c-server", "k3c-load", "k3c-tui", "k3c-starter", "k3c-workbench-web"}

// binSuffixes sind die Formen je Programm: ohne Endung, EXE, Sicherung des Linkers.
var binSuffixes = []string{"", ".exe", ".exe~"}

// binCandidates nennt die Dateinamen, die cleanBin anfassen darf (Name ohne Pfad).
func binCandidates() []string {
	var names []string
	for _, p := range removedPrograms {
		for _, s := range binSuffixes {
			names = append(names, p+s)
		}
	}
	return names
}

// cleanBin löscht in binDir die alten Programme und zeigt sie mit o.Dry nur an. Es trifft ausschließlich
// reguläre Dateien direkt in binDir (kein Ordner, kein Link, kein Pfad außerhalb); gesperrte Dateien werden
// gemeldet, der Lauf geht weiter. Fehlt binDir, ist das kein Fehler.
func cleanBin(o options, binDir string) {
	info, err := os.Lstat(binDir)
	if err != nil {
		return
	}
	if !info.IsDir() { // Link auf einen Ordner oder Datei: nicht anfassen
		say(o, "🚫 bin/ ist kein echter Ordner, übersprungen: %s\n", binDir)
		return
	}
	for _, name := range binCandidates() {
		cleanBinFile(o, filepath.Join(binDir, name), name)
	}
}

func cleanBinFile(o options, path, name string) {
	fi, err := os.Lstat(path)
	if err != nil {
		return // nicht vorhanden
	}
	if !fi.Mode().IsRegular() {
		say(o, "🚫 bin/%s übersprungen: keine reguläre Datei\n", name)
		return
	}
	size := fmt.Sprintf("%.1f MiB", float64(fi.Size())/(1<<20))
	if o.Dry {
		say(o, "🧹 bin/%s (%s) würde gelöscht (--dry)\n", name, size)
		return
	}
	if err := os.Remove(path); err != nil {
		say(o, "🚫 bin/%s (%s) nicht gelöscht, vermutlich in Benutzung oder gesperrt: %v\n", name, size, err)
		return
	}
	say(o, "🧹 bin/%s (%s) gelöscht\n", name, size)
}
