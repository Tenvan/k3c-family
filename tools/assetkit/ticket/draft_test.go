package ticket

import (
	"strings"
	"testing"
	"time"
)

// TestDraftOptionaleFelder: Relationsfelder (B-696) bekommen `–`, wenn die Vorlage sie kennt; eine ältere Vorlage ohne
// sie bleibt gültig.
func TestDraftOptionaleFelder(t *testing.T) {
	tpl := "# B-000 · Titel\n\n- **Status:** x\n- **Sprint:** x\n- **Projekt:** x\n- **Erstellt:** x\n- **Umgebung:** x\n" +
		"- **Spec:** x\n- **Revision:** x\n- **Freigabe:** x\n- **Duplikat von:** – (Erklärung)\n- **Ersetzt:** – (Erklärung)\n\n## A\n"
	got, err := Draft(tpl, "B-001", "Neu", Defaults(time.Now()))
	if err != nil || !strings.Contains(got, "- **Duplikat von:** –\n- **Ersetzt:** –\n") {
		t.Fatalf("mit Feldern: %v\n%s", err, got)
	}
	old := strings.Replace(tpl, "- **Duplikat von:** – (Erklärung)\n- **Ersetzt:** – (Erklärung)\n", "", 1)
	if got, err = Draft(old, "B-001", "Neu", Defaults(time.Now())); err != nil || strings.Contains(got, "Ersetzt") {
		t.Fatalf("alte Vorlage: %v\n%s", err, got)
	}
}
