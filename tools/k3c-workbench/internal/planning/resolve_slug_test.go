package planning

import (
	"os"
	"strings"
	"testing"
)

// Doppelt vergebene Nummer (zwei Rechner legen dieselbe Nummer an): B-1287-kurzname wählt genau eine Datei, die
// bloße Nummer bleibt bei der ersten; Set ändert nur die gewählte Datei.
func TestTicketMitKurznameTrenntDoppelteNummer(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	head := "# B-1287 · %s\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Status:** offen\n- **Sprint:** –\n"
	for _, slug := range []string{"aaa-erster", "bbb-zweiter"} {
		text := strings.Replace(head, "%s", slug, 1)
		if err := os.WriteFile(docsPath(root, "backlog/B-1287-"+slug+".md"), []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	r, err := resolve(root, "B-1287-bbb-zweiter")
	if err != nil || r.rel != "backlog/B-1287-bbb-zweiter.md" || r.id != "B-1287" {
		t.Fatalf("resolve mit Kurzname: %+v %v", r, err)
	}
	if _, err := resolve(root, "B-1287-fehlt"); err == nil {
		t.Error("unbekannter Kurzname: Fehler erwartet")
	}
	must(t)(Set(root, "B-1287-bbb-zweiter", map[string]string{"Prio": "hoch"}))
	if !strings.Contains(doc(t, root, "backlog/B-1287-bbb-zweiter.md"), "**Prio:** hoch") {
		t.Error("gewählte Datei nicht geändert")
	}
	if strings.Contains(doc(t, root, "backlog/B-1287-aaa-erster.md"), "**Prio:** hoch") {
		t.Error("andere Datei mit gleicher Nummer geändert")
	}
}
