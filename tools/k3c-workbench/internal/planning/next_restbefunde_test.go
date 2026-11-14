package planning

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// Restbefunde Review WZ44.5 (B-839): Plätze, Ordner-Pfade, laufende Sessions ohne Domäne, Branch-Zeile.

// TestNextParallelCountsRunning: Drei Sessions in Arbeit → neben der Auswahl kein weiterer Vorschlag (vier Agenten).
func TestNextParallelCountsRunning(t *testing.T) {
	t.Parallel()
	root := laneRepo(t, laneSession{"L1.1", "in Arbeit", "SIM", "", ""}, laneSession{"L2.1", "in Arbeit", "SRV", "", ""},
		laneSession{"L3.1", "in Arbeit", "CLI", "", ""}, laneSession{"L4.1", "offen", "DEV", "", ""},
		laneSession{"L5.1", "offen", "INF", "", ""}, laneSession{"L6.1", "offen", "PAP", "", ""})
	out := must(t)(Next(root, NextFilter{}))
	checkText(t, out, []string{"Nächste Session: L4.1 ·", "── Parallel möglich ──\nkeine"}, []string{"L5.1 · Titel", "L6.1 · Titel"})

	root = laneRepo(t, laneSession{"L1.1", "in Arbeit", "SIM", "", ""}, laneSession{"L4.1", "offen", "DEV", "", ""},
		laneSession{"L5.1", "offen", "INF", "", ""}, laneSession{"L6.1", "offen", "PAP", "", ""},
		laneSession{"L7.1", "offen", "REG", "", ""})
	out = must(t)(Next(root, NextFilter{}))
	checkText(t, out, []string{"- INF · L5.1", "- PAP · L6.1"}, []string{"L7.1 · Titel"})
}

// TestGlobsOverlapFolderWithoutSlash: Ein fester Pfad ohne `/` am Ende gilt auch als Ordner.
func TestGlobsOverlapFolderWithoutSlash(t *testing.T) {
	t.Parallel()
	cases := []struct {
		a, b string
		want bool
	}{
		{"tools/paper", "tools/paper/cli/**", true},
		{"tools/paper/cli/**", "tools/paper", true},
		{"tools/paper", "tools/paper/cli/main.ts", true},
		{"tools/paper", "tools/paper2/**", false},
		{"tools/paper", "tools/paper.go", false},
		{"a/b.go", "a/b.go", true},
	}
	for _, c := range cases {
		if got := globsOverlap(c.a, c.b); got != c.want {
			t.Errorf("globsOverlap(%q, %q) = %v, will %v", c.a, c.b, got, c.want)
		}
	}
}

// appendSession hängt Text an eine Session-Datei des Lane-Fixtures.
func appendSession(t *testing.T, root, nr, text string) {
	t.Helper()
	id, _, _ := strings.Cut(nr, ".")
	p := filepath.Join(root, "docs", "sprints", "aktiv", id+"-s", nr+"-x.md")
	b, err := os.ReadFile(p)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, append(b, []byte(text)...), 0o644); err != nil {
		t.Fatal(err)
	}
}

// TestNextRunningWithoutDomainBlocks: Eine Session in Arbeit ohne Domäne sperrt mit ihren Dateien.
func TestNextRunningWithoutDomainBlocks(t *testing.T) {
	t.Parallel()
	root := laneRepo(t, laneSession{"L1.1", "in Arbeit", "", "", ""}, laneSession{"L2.1", "offen", "DEV", "", ""})
	files := "\n## Erlaubte Dateien\n\n- `tools/x/**`\n"
	appendSession(t, root, "L1.1", files)
	appendSession(t, root, "L2.1", files)
	out := must(t)(Next(root, NextFilter{}))
	checkText(t, out, []string{"L2.1 Bereich `tools/x/**` gesperrt (L1.1 in Arbeit)"}, []string{"Nächste Session"})
}

// TestNextBranchMatchesOverlay: Die Branch-Zeile kommt aus derselben Session wie der Ref des origin-Overlays (die
// erste mit Feld Branch), nicht aus der gewählten Session.
func TestNextBranchMatchesOverlay(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	w := func(rel, text string) {
		p := filepath.Join(root, "docs", filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	w("backlog/README.md", "# Backlog\n")
	w("sprints/geplant/.keep", "")
	w("projekte/DDD-d.md", nextProject("DDD", "aktiv", "1", "D1"))
	nextSprint(w, "aktiv", "D1", "DDD", []nextSession{
		{"D1.1", "autonom", "fertig", "DEV", "–", "AC-01"},
		{"D1.2", "autonom", "offen", "DEV", "D1.1", "AC-01"},
	})
	w("sprints/aktiv/D1-s/D1.1-x.md", "# D1.1 · Titel D1.1\n\n- **Agent:** autonom\n- **Domäne:** DEV\n"+
		"- **Branch:** sprint/dd\n- **Abhängig von:** –\n- **Kriterien:** AC-01\n\n## Ziel\n\nZiel.\n")
	out := must(t)(Next(root, NextFilter{}))
	checkText(t, out, []string{"Nächste Session: D1.2 ·", "Branch: sprint/dd", "D1: kein origin/sprint/dd"}, []string{"Branch: sprint/d1"})
}
