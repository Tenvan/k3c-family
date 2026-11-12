package art

import (
	"path/filepath"
	"testing"
)

// Die eingecheckte Schrift lädt im strengen Modus (Pfad relativ zur Repo-Wurzel, Test läuft in art/).
func TestDefaultConfigFont(t *testing.T) {
	cfg := DefaultConfig()
	if !cfg.StrictFont {
		t.Fatal("StrictFont ist aus")
	}
	f, err := LoadFont(filepath.Join("..", "..", "..", cfg.FontPath), true)
	if err != nil || f.Fallback {
		t.Fatalf("Schrift %s: %+v %v", cfg.FontPath, f, err)
	}
}
