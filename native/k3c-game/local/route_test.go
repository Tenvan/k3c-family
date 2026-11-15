package local

import "testing"

// Die Standard-Welt liefert ihre Inseln mit Etappe (>= 1) für die Seekarte; eine unbekannte Welt-Datei ist ein Fehler.
func TestLoadWorldDefRoute(t *testing.T) {
	def, err := loadWorldDef("")
	if err != nil || len(def.Islands) == 0 {
		t.Fatalf("Standard-Welt: %v, %d Inseln", err, len(def.Islands))
	}
	t.Setenv(WorldsDirEnv, t.TempDir())
	if _, err := loadWorldDef("gibt-es-nicht"); err == nil {
		t.Error("unbekannte Welt: Fehler")
	}
}
