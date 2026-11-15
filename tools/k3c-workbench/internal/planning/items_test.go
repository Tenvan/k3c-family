package planning

import (
	"encoding/json"
	"os"
	"path/filepath"
	"sort"
	"testing"
)

const (
	listLimit = 1536 << 10 // B-909/AC-01: Listen-Daten unter 1,5 MiB
	itemLimit = 500 << 10  // B-909/AC-01: eine ID unter 500 KiB
)

func jsonSize(t *testing.T, v any) int {
	t.Helper()
	b, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return len(b)
}

// realRoot ist die Wurzel dieses Checkouts (tools/k3c-workbench/internal/planning, vier Ebenen hoch); ohne Planung
// wird übersprungen.
func realRoot(t *testing.T) string {
	t.Helper()
	root, _ := filepath.Abs("../../../..")
	if _, err := os.Stat(filepath.Join(root, "docs", "sprints")); err != nil {
		t.Skip("keine Planung im Checkout")
	}
	return root
}

// textStats zählt die Sessions und ihre rohen Texte (KiB) eines Standes mit Texten.
func textStats(d Data) (sessions, textKiB int) {
	for _, s := range d.Sprints {
		for _, x := range s.Sessions {
			textKiB += len(x.Text)
			sessions++
		}
	}
	return sessions, textKiB >> 10
}

// candidateIDs nennt die Sprints, die 40 längsten Sessions, alle Tickets und Projekte: die Kandidaten für das größte
// einzelne Dokument (alle 1400 Sessions zu laden dauert zu lang).
func candidateIDs(full Data) []string {
	type sized struct {
		id string
		n  int
	}
	var all []sized
	var ids []string
	for _, s := range full.Sprints {
		ids = append(ids, s.ID)
		for _, x := range s.Sessions {
			all = append(all, sized{x.Nr, len(x.Text)})
		}
	}
	sort.Slice(all, func(i, j int) bool { return all[i].n > all[j].n })
	for _, x := range all[:min(40, len(all))] {
		ids = append(ids, x.id)
	}
	for _, tk := range full.Tickets {
		ids = append(ids, tk.Nr)
	}
	for _, p := range full.Projects {
		ids = append(ids, p.ID)
	}
	return ids
}

// biggestItem lädt die IDs (in Paketen) und liefert das größte Dokument als JSON-Größe.
func biggestItem(t *testing.T, root string, ids []string) (id string, size int) {
	t.Helper()
	for start := 0; start < len(ids); start += maxItems {
		items, err := Items(root, ids[start:min(len(ids), start+maxItems)])
		if err != nil {
			t.Fatal(err)
		}
		for _, it := range items {
			if n := jsonSize(t, it); it.Error == "" && n > size {
				id, size = it.ID, n
			}
		}
	}
	return id, size
}

// TestPlanungsdatenGroesse misst Load und LoadList gegen die echte Planung (B-909/AC-01; AC-03: Tabelle mit -v).
func TestPlanungsdatenGroesse(t *testing.T) {
	if testing.Short() {
		t.Skip("liest die ganze Planung zweimal (etwa 30 s)")
	}
	root := realRoot(t)
	full, err := Load(root)
	if err != nil {
		t.Fatal(err)
	}
	list, err := LoadList(root)
	if err != nil {
		t.Fatal(err)
	}
	sessions, texts := textStats(full)
	if n, kib := textStats(list); n != sessions || kib != 0 {
		t.Errorf("LoadList: %d Sessions mit %d KiB Text, voll %d Sessions", n, kib, sessions)
	}
	t.Logf("Größe (JSON) in KiB: voll %d | Liste %d (Projekte %d, Sprints und Sessions %d, Tickets %d) | Session-Texte roh %d bei %d Sessions",
		jsonSize(t, full)>>10, jsonSize(t, list)>>10, jsonSize(t, list.Projects)>>10, jsonSize(t, list.Sprints)>>10,
		jsonSize(t, list.Tickets)>>10, texts, sessions)
	if n := jsonSize(t, list); n >= listLimit {
		t.Errorf("Listen-Daten %d KiB, Grenze %d KiB", n>>10, listLimit>>10)
	}
	id, size := biggestItem(t, root, candidateIDs(full))
	t.Logf("größtes einzelnes Dokument: %s mit %d KiB", id, size>>10)
	if size >= itemLimit {
		t.Errorf("%s: %d KiB, Grenze %d KiB", id, size>>10, itemLimit>>10)
	}
}

func TestItemsEinzeln(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	write(t, root, "docs/sprints/aktiv/X1-a/README.md", "# X1 · DEV · A\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n| X1.1 | X1.1-s.md | Umsetzung | autonom | offen |\n")
	write(t, root, "docs/sprints/aktiv/X1-a/X1.1-s.md", "# X1.1 · Session\n\n- **Status:** offen\n")
	write(t, root, "docs/sprints/geplant/Y1-b/README.md", "# Y1 · DEV · B\n")
	write(t, root, "docs/backlog/B-001-a.md", "# B-001 · Ticket\n\n- **Status:** offen\n")
	items, err := Items(root, []string{"X1.1", "B-001", "X1", "B-001", "nix", "B-777"})
	if err != nil {
		t.Fatal(err)
	}
	if len(items) != 5 {
		t.Fatalf("Doppelte zählen einmal, bekam %d: %+v", len(items), items)
	}
	if it := items[0]; it.Kind != "session" || it.Path != "docs/sprints/aktiv/X1-a/X1.1-s.md" || it.Text != "# X1.1 · Session\n\n- **Status:** offen\n" {
		t.Errorf("Session: %+v", it)
	}
	if it := items[1]; it.Kind != "ticket" || it.Path != "docs/backlog/B-001-a.md" {
		t.Errorf("Ticket: %+v", it)
	}
	if it := items[2]; it.Kind != "sprint" || it.Path != "docs/sprints/aktiv/X1-a/README.md" {
		t.Errorf("Sprint: %+v", it)
	}
	if items[3].Error == "" || items[4].Error == "" {
		t.Errorf("unbekannte IDs brauchen Error: %+v %+v", items[3], items[4])
	}
	if _, err := Items(root, make([]string, maxItems+1)); err == nil {
		t.Error("zu viele IDs")
	}
}

func TestLoadListOhneText(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	write(t, root, "docs/sprints/aktiv/X1-a/README.md", "# X1 · DEV · A\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n| X1.1 | X1.1-s.md | Umsetzung | autonom | offen |\n")
	write(t, root, "docs/sprints/aktiv/X1-a/X1.1-s.md", "# X1.1 · Session\n\n- **Umgebung:** offline\n- **Domäne:** DEV\n")
	write(t, root, "docs/sprints/geplant/Y1-b/README.md", "# Y1 · DEV · B\n")
	write(t, root, "docs/backlog/B-001-a.md", "# B-001 · Ticket\n")
	list, err := LoadList(root)
	if err != nil {
		t.Fatal(err)
	}
	x := list.Sprints[0].Sessions[0]
	if x.Text != "" || x.Titel != "Session" || x.Env != "offline" || x.Domain != "DEV" {
		t.Errorf("Text weg, Kopf bleibt: %+v", x)
	}
}
