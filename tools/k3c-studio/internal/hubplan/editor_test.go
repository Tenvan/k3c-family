package hubplan

import (
	"testing"

	"k3c/tools/k3c-studio/internal/exeengine"
)

// editor ist der Planer unter root mit der Spiel-EXE der Repo-Wurzel (exeengine; ohne EXE wird der Test übersprungen).
func editor(t testing.TB, root string) Editor {
	t.Helper()
	return Editor{Root: root, Engine: exeengine.For(t, "hub", root)}
}
