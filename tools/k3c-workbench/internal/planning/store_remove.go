package planning

// Rücknahme eines gescheiterten Entfernens in changeSet.apply (B-867): Datei oder Ordner wird vorher vollständig
// gelesen (Planungsdateien sind klein) und bei einem Fehler byte-gleich wiederhergestellt.

import (
	"bytes"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
)

// removeAll entfernt eine Datei oder einen Ordner des changeSets; Tests ersetzen es, um einen Fehler zu erzwingen.
var removeAll = os.RemoveAll

// removeWithBackup sichert p und entfernt es; undo stellt den Stand wieder her (fehlt p, ist nichts zu tun, wie
// os.RemoveAll). Scheitert das Entfernen, kommt undo mit dem Fehler zurück, weil p teilweise weg sein kann.
func removeWithBackup(p string) (undo func() error, err error) {
	snap, err := backupTree(p)
	if errors.Is(err, fs.ErrNotExist) {
		return func() error { return nil }, nil
	}
	if err != nil {
		return func() error { return nil }, err
	}
	return func() error { return restoreTree(snap) }, removeAll(p)
}

// backupTree liest Datei oder Ordner p vollständig: Pfad → Inhalt, Ordner mit nil. Fehlt p, ist der Fehler fs.ErrNotExist.
func backupTree(p string) (map[string][]byte, error) {
	out := map[string][]byte{}
	err := filepath.WalkDir(p, func(q string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			out[q] = nil
			return nil
		}
		b, err := os.ReadFile(q)
		if b == nil {
			b = []byte{} // leere Datei, nie mit einem Ordner verwechseln
		}
		out[q] = b
		return err
	})
	return out, err
}

// restoreTree stellt den Stand aus backupTree wieder her: fehlende Ordner anlegen, fehlende oder geänderte Dateien schreiben.
func restoreTree(snap map[string][]byte) error {
	keys := make([]string, 0, len(snap))
	for k := range snap {
		keys = append(keys, k)
	}
	sort.Strings(keys) // Eltern vor Kindern
	for _, k := range keys {
		b := snap[k]
		if b == nil {
			if err := os.MkdirAll(k, 0o755); err != nil {
				return err
			}
			continue
		}
		if cur, err := os.ReadFile(k); err == nil && bytes.Equal(cur, b) {
			continue
		}
		if err := os.MkdirAll(filepath.Dir(k), 0o755); err != nil {
			return err
		}
		if err := os.WriteFile(k, b, 0o644); err != nil {
			return err
		}
	}
	return nil
}
