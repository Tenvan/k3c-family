package planning

import (
	"errors"
	"os"
	"strings"
	"testing"
)

// failRemove ersetzt removeAll für diesen Test: Es entfernt erst einen Teil (die Datei README.md im Ordner bzw. die
// Datei selbst) und scheitert dann. Nicht parallel (Paket-Variable).
func failRemove(t *testing.T) {
	t.Helper()
	orig := removeAll
	t.Cleanup(func() { removeAll = orig })
	removeAll = func(p string) error {
		if st, err := os.Stat(p); err == nil && st.IsDir() {
			_ = os.Remove(p + string(os.PathSeparator) + "README.md")
		} else {
			_ = os.Remove(p)
		}
		return errors.New("erzwungener Löschfehler")
	}
}

// B-867/AC-01: Ein Entfernen scheitert nach zwei Schreibschritten (eine Datei geändert, eine neu) → alle drei
// Schritte zurückgenommen, der Stand ist byte-gleich (Ordner und einzelne Datei).
func TestEntfernenScheitertNimmtAllesZurueck(t *testing.T) { //nolint:paralleltest // ersetzt removeAll
	for _, ziel := range []string{"sprints/erledigt/NG15-ton", "sprints/aktiv/A1-a/A1.3-x.md"} {
		t.Run(ziel, func(t *testing.T) {
			root := refreshRepo(t)
			before := snapshot(t, root)
			failRemove(t)
			c := newChangeSet(root)
			c.write("backlog/README.md", "# Backlog\n\ngeändert\n")
			c.write("backlog/B-001-neu.md", "# B-001 · neu\n")
			c.remove = append(c.remove, ziel)
			err := c.apply()
			if err == nil || !strings.Contains(err.Error(), "erzwungener Löschfehler") || !strings.Contains(err.Error(), "zurückgenommen") {
				t.Fatalf("Fehler: %v", err)
			}
			if after := snapshot(t, root); after != before {
				t.Error("Stand nicht byte-gleich")
			}
			if _, err := os.Stat(docsPath(root, "backlog/B-001-neu.md")); !errors.Is(err, os.ErrNotExist) {
				t.Errorf("neue Datei liegt noch: %v", err)
			}
		})
	}
}

// B-867: Ohne Fehler entfernt apply wie bisher; ein fehlendes Ziel ist kein Fehler.
func TestEntfernenOhneFehler(t *testing.T) {
	t.Parallel()
	root := refreshRepo(t)
	c := newChangeSet(root)
	c.remove = append(c.remove, "sprints/erledigt/NG15-ton", "sprints/erledigt/gibt-es-nicht")
	if err := c.apply(); err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(docsPath(root, "sprints/erledigt/NG15-ton")); !errors.Is(err, os.ErrNotExist) {
		t.Errorf("Ordner liegt noch: %v", err)
	}
}
