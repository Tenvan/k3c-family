package mcpsrv

import (
	"flag"
	"os"
	"path/filepath"
	"testing"

	"k3c/tools/k3c-studio/internal/exeengine"
	"k3c/tools/k3c-studio/internal/hubplan"
	"k3c/tools/k3c-studio/internal/world"
)

func TestMain(m *testing.M) {
	// Welten- und Hub-Tools rechnen über die Spiel-EXE; die Tests nehmen die EXE der Repo-Wurzel mit den Fixture-Daten (exeengine).
	worldEngine = func(root string) world.Engine { return exeengine.Engine("world", root) }
	hubEngine = func(root string) hubplan.Engine { return exeengine.Engine("hub", root) }
	// Die Asset-Tests rufen die CLI-Attrappe; einmal für den Lauf statt t.Setenv je Test, damit sie parallel laufen (B-830).
	if os.Getenv("K3C_PAPER_CLI") == "" {
		stub, _ := filepath.Abs(filepath.Join("..", "..", "..", "paper", "fixtures", "cli-stub.ts"))
		_ = os.Setenv("K3C_PAPER_CLI", stub)
	}
	// Höchstens sechs parallele Tests, wenn -parallel nicht gesetzt ist: Mit GOMAXPROCS warten git-, bun- und
	// Render-Prozesse aufeinander und ein einzelner Test zeigt über 3 s (B-830).
	flag.Parse()
	set := false
	flag.Visit(func(f *flag.Flag) { set = set || f.Name == "test.parallel" })
	if !set {
		_ = flag.Set("test.parallel", "6")
	}
	code := m.Run()
	if paperDir != "" {
		_ = os.RemoveAll(paperDir)
	}
	os.Exit(code)
}
