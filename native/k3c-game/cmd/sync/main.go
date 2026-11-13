// Command sync kopiert Dateien und Ordner ins Spiel-Paket (dist/k3c-game, `task native:game:build`): je Argument
// `quelle=ziel` eine Datei oder einen Ordner (rekursiv, zusammengeführt). Eine Datei wird nur kopiert, wenn sie im Ziel
// fehlt oder Größe bzw. Änderungszeit abweichen; so ersetzt ein neuer Build geänderte Grafik und lässt den Rest liegen.
// Eigenes Werkzeug statt cp: Task nutzt unter Windows ein eingebautes cp ohne -u und ohne Zusammenführen von Ordnern.
package main

import (
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	n := 0
	for _, arg := range os.Args[1:] {
		src, dst, ok := strings.Cut(arg, "=")
		if !ok {
			fail(fmt.Errorf("argument %q: erwartet quelle=ziel", arg))
		}
		c, err := syncPath(src, dst)
		if err != nil {
			fail(err)
		}
		n += c
	}
	fmt.Printf("✅ %d Dateien aktualisiert\n", n)
}

func fail(err error) {
	fmt.Fprintln(os.Stderr, "❌", err)
	os.Exit(1)
}

// syncPath bringt dst auf den Stand von src (Datei oder Ordner) und liefert die Zahl kopierter Dateien.
func syncPath(src, dst string) (int, error) {
	n := 0
	err := filepath.WalkDir(src, func(p string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, _ := filepath.Rel(src, p)
		to := filepath.Join(dst, rel)
		if d.IsDir() {
			return os.MkdirAll(to, 0o755)
		}
		copied, err := syncFile(p, to)
		if copied {
			n++
		}
		return err
	})
	return n, err
}

// syncFile kopiert from nach to, wenn to fehlt oder Größe bzw. Änderungszeit abweichen; die Zeit wird übernommen.
func syncFile(from, to string) (bool, error) {
	fi, err := os.Stat(from)
	if err != nil {
		return false, err
	}
	if ti, err := os.Stat(to); err == nil && ti.Size() == fi.Size() && ti.ModTime().Equal(fi.ModTime()) {
		return false, nil
	}
	if err := os.MkdirAll(filepath.Dir(to), 0o755); err != nil {
		return false, err
	}
	if err := copyFile(from, to); err != nil {
		return false, fmt.Errorf("%s → %s: %w (läuft das Spiel dort noch?)", from, to, err)
	}
	return true, os.Chtimes(to, fi.ModTime(), fi.ModTime())
}

func copyFile(from, to string) error {
	in, err := os.Open(from)
	if err != nil {
		return err
	}
	defer func() { _ = in.Close() }() // nur gelesen: Fehler beim Schließen ändert nichts am Ziel
	out, err := os.Create(to)
	if err != nil {
		return err
	}
	if _, err := io.Copy(out, in); err != nil {
		_ = out.Close() // der Kopierfehler ist der eigentliche Fehler
		return err
	}
	return out.Close()
}
