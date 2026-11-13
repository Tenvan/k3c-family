package planning

import (
	"strings"
	"testing"
)

// B-784/AC-01: plan_check meldet Bereich leer (B-003) und an B-020 Umgebung ?, Größe L, unbekannte ID und die
// Überlappung mit dem offenen Ticket B-001 (tools/d/**).
func TestCheckMeldetMaengel(t *testing.T) {
	t.Parallel()
	root := wavesRepo(t)
	w := waveWriter(t, root)
	w("backlog/B-020-t.md", "# B-020 · Ticket B-020\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n"+
		"- **Umgebung:** ?\n- **Status:** offen\n- **Sprint:** –\n- **Projekt:** BBB\n- **Spec:** Entwurf\n\n## Ziel\n\nZiel.\n\n"+
		rules("`tools/d/x.go`", "B-999, A1.1, irgendwann", "–", "L", "–"))
	out := must(t)(Check(root, CheckIn{ID: "B-020"}))
	want := "B-020: 5 Mängel\n- Umgebung: ?\n- Größe: L (vor der Freigabe in Tickets je Teil zerlegen)\n" +
		"- unbekannte ID B-999 in „Abhängig von“\n- keine ID in „Abhängig von“: irgendwann\n" +
		"- Bereich überlappt mit B-001 (`tools/d/x.go` / `tools/d/**`)"
	if out != want {
		t.Errorf("B-020:\n%s\nwill:\n%s", out, want)
	}
	checkText(t, must(t)(Check(root, CheckIn{ID: "B-003"})), []string{"B-003: 1 Mängel\n- Bereich leer"}, nil)
	checkText(t, must(t)(Check(root, CheckIn{ID: "B-002"})), []string{"B-002: keine Mängel"}, nil)
	checkText(t, must(t)(Check(root, CheckIn{ID: "B-001"})), []string{"B-001: 1 Mängel\n- Bereich überlappt mit B-020"}, nil)
	checkText(t, must(t)(Check(root, CheckIn{ID: "A1"})), []string{"A1: keine Mängel"}, nil)
}

// Fehlende Zeilen, archivierte Tickets und falsche IDs.
func TestCheckZeilenArchivUndFehler(t *testing.T) {
	t.Parallel()
	root := wavesRepo(t)
	w := waveWriter(t, root)
	w("backlog/B-021-t.md", "# B-021 · Ohne Angaben\n\n- **Umgebung:** offline\n- **Status:** offen\n\n## Ziel\n\nZiel.\n")
	w("backlog/B-022-t.md", "# B-022 · Halb\n\n- **Umgebung:** offline\n- **Status:** offen\n\n"+
		"## Regeln und Einschränkungen\n\n- **Bereich:** `x/**`\n- **Sperre:** –\n")
	w("backlog/archiv/B-023-t.md", "# B-023 · Alt\n\n- **Status:** erledigt\n")
	checkText(t, must(t)(Check(root, CheckIn{ID: "B-021"})), []string{"Planungsangaben fehlen"}, []string{"Zeile"})
	checkText(t, must(t)(Check(root, CheckIn{ID: "B-022"})),
		[]string{"Zeile „Abhängig von“ fehlt", "Zeile „Braucht 🧑“ fehlt", "Zeile „Größe“ fehlt"}, []string{"Zeile „Bereich“", "Zeile „Sperre“"})
	if out := must(t)(Check(root, CheckIn{ID: "B-023"})); !strings.Contains(out, "archiviert, nicht geprüft") {
		t.Errorf("Archiv: %s", out)
	}
	for _, id := range []string{"B-404", "A1.1", "quatsch"} {
		if _, err := Check(root, CheckIn{ID: id}); err == nil {
			t.Errorf("%s angenommen", id)
		}
	}
}
