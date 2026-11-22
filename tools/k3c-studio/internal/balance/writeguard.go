package balance

import (
	"os"
	"path/filepath"

	"k3c/tools/workkit/mcpkit"
)

// Schreibschutz des Balancings (WZ63.7, Beschluss Ralf 2026-11-22): Quelldateien nur Welten-Dateien unter data/worlds/,
// Laufausgaben (Berichte, Baselines, simtest.md, ratings.json, result.json, Auswertung) nur unter reports/. Alles
// andere (data/*.json, engine/, rust/, tools/, docs/, cmd/, public/, saves/) lehnt der Wächter ab. Jede Schreibstelle in
// balance und simrun geht über die Helfer unten; TestBalancingSchreibtNurUeberWaechter prüft das im Quelltext.

// Guard ist der Wächter aller Schreibzugriffe des Balancings.
var Guard = mcpkit.WriteGuard{
	Name:  "Balancing",
	Allow: []string{"data/worlds", "reports"},
	Hint:  "Code oder andere Daten nur nach Plan im Chat und Freigabe von Ralf ändern",
}

// WriteFile schreibt name wie os.WriteFile, wenn der Wächter den Pfad unter root erlaubt.
func WriteFile(root, name string, data []byte) error {
	name = inRoot(root, name)
	if err := Guard.Check(root, name); err != nil {
		return err
	}
	return os.WriteFile(name, data, 0o644)
}

// MkdirAll legt dir wie os.MkdirAll an, wenn der Wächter den Pfad erlaubt.
func MkdirAll(root, dir string) error {
	dir = inRoot(root, dir)
	if err := Guard.Check(root, dir); err != nil {
		return err
	}
	return os.MkdirAll(dir, 0o755)
}

// Mkdir legt dir wie os.Mkdir an (scheitert, wenn es ihn gibt), wenn der Wächter den Pfad erlaubt.
func Mkdir(root, dir string) error {
	dir = inRoot(root, dir)
	if err := Guard.Check(root, dir); err != nil {
		return err
	}
	return os.Mkdir(dir, 0o755)
}

// Rename benennt from in to um, wenn der Wächter beide Pfade erlaubt.
func Rename(root, from, to string) error {
	from, to = inRoot(root, from), inRoot(root, to)
	if err := Guard.Check(root, from); err != nil {
		return err
	}
	if err := Guard.Check(root, to); err != nil {
		return err
	}
	return os.Rename(from, to)
}

// Remove löscht name wie os.Remove, wenn der Wächter den Pfad erlaubt.
func Remove(root, name string) error {
	name = inRoot(root, name)
	if err := Guard.Check(root, name); err != nil {
		return err
	}
	return os.Remove(name)
}

// RemoveAll löscht dir samt Inhalt wie os.RemoveAll, wenn der Wächter den Pfad erlaubt.
func RemoveAll(root, dir string) error {
	dir = inRoot(root, dir)
	if err := Guard.Check(root, dir); err != nil {
		return err
	}
	return os.RemoveAll(dir)
}

// inRoot macht einen relativen Pfad zu einem unter root, damit Wächter und Schreibaufruf dieselbe Datei meinen.
func inRoot(root, name string) string {
	if name == "" || filepath.IsAbs(name) {
		return name
	}
	return filepath.Join(root, name)
}
