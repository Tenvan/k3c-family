package mcpkit

import (
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
)

// WZ63.7: Der Wächter erlaubt nur die genannten Ordner und lässt sich weder mit „..“, absoluten Pfaden noch Links umgehen.
func TestWriteGuard(t *testing.T) {
	t.Parallel()
	root := t.TempDir()
	for _, d := range []string{"data/worlds", "engine/sim", "reports"} {
		if err := os.MkdirAll(filepath.Join(root, d), 0o755); err != nil {
			t.Fatal(err)
		}
	}
	g := WriteGuard{Name: "Test", Allow: []string{"data/worlds", "reports"}}
	ok := []string{"data/worlds/standard.json", "data/worlds/neu/x.json", "reports/simtest-1/simtest.md",
		filepath.Join(root, "reports", "ratings.json"), `data\worlds\standard.json`}
	if runtime.GOOS == "windows" {
		ok = append(ok, "DATA/Worlds/standard.json")
	}
	for _, p := range ok {
		if err := g.Check(root, p); err != nil {
			t.Errorf("%s abgelehnt: %v", p, err)
		}
	}
	outside := filepath.Join(filepath.Dir(root), "fremd.json")
	for _, p := range []string{"data/troops.json", "data/worldsx/a.json", "engine/sim/x.go", "rust/a.rs",
		"data/worlds/../troops.json", `data\worlds\..\troops.json`, "../fremd.json", outside, "", "."} {
		if err := g.Check(root, p); err == nil {
			t.Errorf("%s erlaubt", p)
		}
	}
	if err := g.Check("", "data/worlds/a.json"); err == nil {
		t.Error("ohne Wurzel erlaubt")
	}
}

// Ein Link (Symlink oder Junction) unter data/worlds auf engine/ darf den Schutz nicht umgehen; EvalSymlinks löst
// Junctions seit Go 1.23 nicht mehr auf, deshalb prüft der Wächter jeden Teil des Wegs.
func TestWriteGuardLink(t *testing.T) {
	t.Parallel()
	kinds := map[string]func(target, link string) error{"symlink": os.Symlink}
	if runtime.GOOS == "windows" {
		kinds["junction"] = func(target, link string) error {
			return exec.Command("cmd", "/c", "mklink", "/J", link, target).Run()
		}
	}
	for kind, mk := range kinds {
		t.Run(kind, func(t *testing.T) {
			t.Parallel()
			root := t.TempDir()
			worlds, engine := filepath.Join(root, "data", "worlds"), filepath.Join(root, "engine")
			for _, d := range []string{worlds, engine} {
				if err := os.MkdirAll(d, 0o755); err != nil {
					t.Fatal(err)
				}
			}
			link := filepath.Join(worlds, "link")
			if err := mk(engine, link); err != nil {
				t.Skipf("%s nicht anlegbar (Windows ohne Entwicklermodus oder Adminrechte): %v", kind, err)
			}
			g := WriteGuard{Name: "Test", Allow: []string{"data/worlds"}}
			for _, p := range []string{"data/worlds/link/x.go", filepath.Join(link, "x.go"), "data/worlds/link"} {
				if err := g.Check(root, p); err == nil {
					t.Errorf("%s über %s erlaubt", p, kind)
				} else if !strings.HasPrefix(err.Error(), "Test darf") {
					t.Errorf("Meldung ohne Namen: %v", err)
				}
			}
		})
	}
}
