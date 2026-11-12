package planning

import (
	"strings"
	"testing"
)

// B-783/AC-02: Mensch-Sessions nach Gerät (Zeile „Gerät:“ im Kontext, sonst unbekannt), Specs im Entwurf mit
// „Braucht 🧑: Freigabe“, Entscheidungen; ein freigegebenes Ticket steht nicht bei den Freigaben.
func TestQueueNachGeraet(t *testing.T) {
	root := wavesRepo(t)
	out := must(t)(Queue(root, QueueFilter{}))
	want := "Ralf-Queue: 1 Freigaben · 1 Entscheidungen · 2 Abnahmen an 2 Geräten\n" +
		"── Freigaben ──\n- B-006 · Ticket B-006 (Ticket, DEV, Spec Entwurf, Braucht 🧑: Freigabe)\n" +
		"── Entscheidungen ──\n- B-007 · Ticket B-007 (Ticket, DEV, Spec Entwurf, Braucht 🧑: Entscheidung)\n" +
		"── Abnahmen je Gerät ──\n" +
		"Xbox/TV (1):\n- A1.3 · Titel A1.3 (Sprint A1, offen, Agent Mensch)\n" +
		"unbekannt (1):\n- C1.3 · Titel C1.3 (Sprint C1, offen, Agent Mensch)"
	if out != want {
		t.Errorf("Antwort:\n%s\nwill:\n%s", out, want)
	}
	out = must(t)(Queue(root, QueueFilter{Device: "xbox"}))
	checkText(t, out, []string{"Xbox/TV (1):\n- A1.3"}, []string{"C1.3", "── Freigaben", "B-006"})
}

func TestNormDevice(t *testing.T) {
	for in, want := range map[string]string{"Xbox": "Xbox/TV", "TV im Wohnzimmer": "Xbox/TV", "pi-gaming Pi": "Pi",
		"PC": "PC", "Mini-PC": "Mini-PC", "Workshop mit Kindern": "Workshop", "Veröffentlichung": "Veröffentlichung",
		"Tablet": "Tablet"} {
		if got := normDevice(in); got != want {
			t.Errorf("normDevice(%q) = %q, will %q", in, got, want)
		}
	}
	if d := deviceOf("# X\n\n- **Agent:** Mensch\n\n## Kontext\n\n- **Gerät:** Pi\n"); d != "Pi" {
		t.Errorf("deviceOf: %q", d)
	}
	if d := deviceOf("# X\n\n## Kontext\n\nkein Gerät genannt\n"); !strings.EqualFold(d, unknownDevice) {
		t.Errorf("deviceOf ohne Zeile: %q", d)
	}
}
