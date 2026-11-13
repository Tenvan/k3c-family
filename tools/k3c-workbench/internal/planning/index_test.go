package planning

import (
	"os"
	"strings"
	"testing"
)

// TestStatuswechselZwischenIndexDateien: B-696/AC-09; Datei und Zeile wandern zwischen beiden Index-Dateien, der
// Ticket-Text bleibt bis auf das Status-Feld byte-gleich.
func TestStatuswechselZwischenIndexDateien(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	must(t)(RebuildIndex(root, false))
	open := docsPath(root, "backlog/B-002-t.md")
	before, _ := os.ReadFile(open)
	must(t)(Set(root, "B-002", map[string]string{"Status": "erledigt"}))
	moved, err := os.ReadFile(docsPath(root, "backlog/archiv/B-002-t.md"))
	if err != nil {
		t.Fatal("nicht archiviert:", err)
	}
	if strings.Replace(string(before), "- **Status:** offen", "- **Status:** erledigt", 1) != string(moved) {
		t.Errorf("Text verändert:\n%s", moved)
	}
	main, arch := doc(t, root, mainIndex), doc(t, root, archiveIndex)
	if strings.Contains(main, "[B-002]") || !strings.Contains(arch, "| [B-002](B-002-t.md) | DEV | Idee | mittel | erledigt |") {
		t.Fatalf("Zeile nicht verschoben:\n%s\n---\n%s", main, arch)
	}
	checkIndexes(t, root)
	must(t)(Set(root, "B-002", map[string]string{"Status": "offen"}))
	back, _ := os.ReadFile(open)
	if string(back) != string(before) {
		t.Errorf("zurück nicht byte-gleich:\n%s", back)
	}
	main, arch = doc(t, root, mainIndex), doc(t, root, archiveIndex)
	if strings.Contains(arch, "[B-002]") || !strings.Contains(main, "| [B-002](B-002-t.md) | DEV | Idee | mittel | offen |") {
		t.Fatalf("Zeile nicht zurück:\n%s\n---\n%s", main, arch)
	}
	checkIndexes(t, root)
}

// TestArchivIndexEntstehtBeimErstenWechsel: ohne archiv/README.md legt der erste Wechsel ins Archiv sie an; die Zeile
// verschwindet aus dem alten Abschnitt „Archiv“ des Haupt-Index.
func TestArchivIndexEntstehtBeimErstenWechsel(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	must(t)(Set(root, "B-003", map[string]string{"Status": "verworfen"}))
	arch := doc(t, root, archiveIndex)
	if !strings.HasPrefix(arch, "# Backlog · Archiv") || !strings.Contains(arch, "| [B-003](B-003-t.md) |") {
		t.Fatalf("Archiv-Index:\n%s", arch)
	}
	if strings.Contains(doc(t, root, mainIndex), "[B-003]") {
		t.Error("Zeile steht noch im Haupt-Index")
	}
	checkIndexes(t, root)
}

// TestRebuildIndex: B-696/AC-07 im Kleinen; dry schreibt nichts, ein Ticket im falschen Ordner bricht ab.
func TestRebuildIndex(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	must(t)(Set(root, "B-003", map[string]string{"Status": "erledigt"}))
	before := snapshot(t, root)
	if out := must(t)(RebuildIndex(root, true)); !strings.Contains(out, "3 offen/eingeplant") || snapshot(t, root) != before {
		t.Fatalf("dry: %q", out)
	}
	must(t)(RebuildIndex(root, false))
	main, arch := doc(t, root, mainIndex), doc(t, root, archiveIndex)
	if strings.Contains(main, "erledigt |") || !strings.Contains(main, "[`archiv/README.md`](archiv/README.md)") {
		t.Errorf("Haupt-Index:\n%s", main)
	}
	if !strings.Contains(arch, "| [B-003](B-003-t.md) |") {
		t.Errorf("Archiv-Index:\n%s", arch)
	}
	checkIndexes(t, root)
	again := snapshot(t, root)
	must(t)(RebuildIndex(root, false))
	if snapshot(t, root) != again {
		t.Error("zweiter Lauf ändert etwas")
	}
	if err := os.WriteFile(docsPath(root, "backlog/archiv/B-030-x.md"), []byte("# B-030 · X\n\n- **Status:** offen\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	before = snapshot(t, root)
	if _, err := RebuildIndex(root, false); err == nil || snapshot(t, root) != before {
		t.Errorf("falscher Ordner: %v", err)
	}
}

// checkIndexes: jedes Ticket steht genau einmal in einem der beiden Index-Dateien, im richtigen.
func checkIndexes(t *testing.T, root string) {
	t.Helper()
	seen := map[string]bool{}
	for _, rel := range indexFiles {
		text, _, err := readText(root, rel)
		if err != nil {
			continue
		}
		for _, m := range indexRow.FindAllStringSubmatch(text, -1) {
			if seen[m[1]] {
				t.Errorf("%s doppelt", m[1])
			}
			seen[m[1]] = true
			file := "backlog/" + m[2]
			if rel == archiveIndex {
				file = "backlog/archiv/" + m[2]
			}
			if st := ParseTicket(doc(t, root, file)).Status; st != m[3] {
				t.Errorf("%s: Index %s, Datei %s", m[1], m[3], st)
			}
		}
	}
	for _, x := range must2(t)(indexTickets(root)) {
		if !seen[x.t.Nr] {
			t.Errorf("%s fehlt im Index", x.t.Nr)
		}
	}
}

func must2(t *testing.T) func([]indexTicket, error) []indexTicket {
	return func(v []indexTicket, err error) []indexTicket {
		t.Helper()
		if err != nil {
			t.Fatal(err)
		}
		return v
	}
}

// TestIndexImCheckoutVerdichten wendet RebuildIndex auf einen echten Checkout an (WZ47.4, B-696/AC-07); nur mit
// K3C_REBUILD_INDEX=<Repo-Wurzel>, sonst übersprungen.
func TestIndexImCheckoutVerdichten(t *testing.T) {
	root := os.Getenv("K3C_REBUILD_INDEX")
	if root == "" {
		t.Skip("K3C_REBUILD_INDEX nicht gesetzt")
	}
	t.Log(must(t)(RebuildIndex(root, false)))
	checkIndexes(t, root)
}
