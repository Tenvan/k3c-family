package planning

import (
	"slices"
	"testing"
)

// Mehrere Pfade in einem Stichpunkt zählen alle (Review WZ44.5), Backticks ohne Pfad nicht.
func TestAllowedFilesAllePfadeJeZeile(t *testing.T) {
	t.Parallel()
	text := "# X\n\n## Erlaubte Dateien\n\n- `engine/sim/**`, `engine/net/**`\n- `docs/sprints/**` (nur über `plan_*`)\n" +
		"- keine\n\n## Nicht-Ziele\n\n- `src/nicht.ts`\n"
	got := allowedFiles(text)
	want := []string{"engine/sim/**", "engine/net/**", "docs/sprints/**"}
	if !slices.Equal(got, want) {
		t.Fatalf("allowedFiles = %v, want %v", got, want)
	}
}
