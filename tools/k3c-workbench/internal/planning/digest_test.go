package planning

import (
	"os"
	"strings"
	"testing"
)

const digestReadme = "# X5 · DEV · Erledigter Testsprint\n\n- **Status:** erledigt\n- **Projekt:** –\n- **Domäne:** DEV\n" +
	"- **Reife:** bereit\n- **Tickets:** B-001\n- **Spec:** freigegeben\n- **Revision:** 1\n- **Freigabe:** 2026-11-01 🧑\n\n" +
	"## Ziel\n\nDer Sprint zeigt, dass Zusammenfassungen entstehen.\n\n## Akzeptanzkriterien\n\n" +
	"- **AC-01** Erstes Kriterium (`B-001/AC-01`).\n- **AC-02** Zweites Kriterium.\n\n## Sessions\n\n" +
	"| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n| X5.1 | `X5.1-a.md` | Umsetzung | autonom | fertig |\n\n" +
	"## Abnahme\n\n2026-11-02 (X5.1, Claude): alles erfüllt.\n"

const digestSession = "# X5.1 · Umsetzung\n\n- **Status:** fertig\n\n## Ergebnis\n\nAC-01 geprüft: Test grün (B-001/AC-02 verschoben). AC-02 verschoben: Grund.\n"

func digestRepo(t *testing.T) string {
	t.Helper()
	root := tempRepo(t)
	for rel, text := range map[string]string{"sprints/erledigt/X5-test/README.md": digestReadme,
		"sprints/erledigt/X5-test/X5.1-a.md": digestSession} {
		if err := os.MkdirAll(docsPath(root, "sprints/erledigt/X5-test"), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(docsPath(root, rel), []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

// TestDigestDryUndZweiterLauf: B-696/AC-11, AC-12, AC-13.
func TestDigestDryUndZweiterLauf(t *testing.T) {
	t.Parallel()
	root := digestRepo(t)
	before := snapshot(t, root)
	if out := must(t)(Digest(root, DigestIn{Dry: true})); !strings.Contains(out, "neu: X5") || snapshot(t, root) != before {
		t.Fatalf("dry: %q", out)
	}
	readme, _ := os.ReadFile(docsPath(root, "sprints/erledigt/X5-test/README.md"))
	must(t)(Digest(root, DigestIn{}))
	got := doc(t, root, "sprints/erledigt/X5-test/"+DigestFile)
	for _, want := range []string{"# X5 · Zusammenfassung", "## Ziel\n\nDer Sprint zeigt", "## Ergebnis\n\n2026-11-02 (X5.1, Claude): alles erfüllt.",
		"| AC-01 | geprüft | Erstes Kriterium", "| AC-02 | verschoben |", "| B-001 | offen | AC-01 geprüft | Altes Ticket |"} {
		if !strings.Contains(got, want) {
			t.Errorf("ohne %q:\n%s", want, got)
		}
	}
	if n := strings.Count(got, "\n"); n > digestMaxLines {
		t.Errorf("%d Zeilen", n)
	}
	if after, _ := os.ReadFile(docsPath(root, "sprints/erledigt/X5-test/README.md")); string(after) != string(readme) {
		t.Error("Original geändert")
	}
	again := snapshot(t, root)
	if out := must(t)(Digest(root, DigestIn{})); !strings.Contains(out, "1 vorhanden") || snapshot(t, root) != again {
		t.Errorf("zweiter Lauf: %q", out)
	}
	d, err := Load(root)
	if err != nil || len(d.Sprints) != 1 || len(d.Sprints[0].Sessions) != 1 {
		t.Fatalf("Load: %v %+v", err, d.Sprints)
	}
	if out := must(t)(List(root, Filter{Kind: "sprint"})); !strings.Contains(out, "X5 DEV erledigt") {
		t.Errorf("plan_list: %q", out)
	}
}

// TestDigestImCheckout schreibt die Zusammenfassungen eines echten Checkouts (WZ47.4); nur mit K3C_DIGEST=<Repo-Wurzel>.
func TestDigestImCheckout(t *testing.T) {
	root := os.Getenv("K3C_DIGEST")
	if root == "" {
		t.Skip("K3C_DIGEST nicht gesetzt")
	}
	t.Log(must(t)(Digest(root, DigestIn{})))
}

// TestDigestAuswahlUndLuecken: älter_als nach Sprint-ID und Datum, fehlendes Ziel als Hinweis.
func TestDigestAuswahlUndLuecken(t *testing.T) {
	t.Parallel()
	root := digestRepo(t)
	for ref, want := range map[string]string{"X6": "1 neu", "X5": "0 neu", "Y9": "0 neu", "2026-11-03": "1 neu", "2026-11-02": "0 neu"} {
		if out := must(t)(Digest(root, DigestIn{OlderThan: ref, Dry: true})); !strings.HasPrefix(out, "Zusammenfassungen: "+want) {
			t.Errorf("älter_als %s: %q", ref, out)
		}
	}
	noGoal := strings.Replace(digestReadme, "## Ziel\n\nDer Sprint zeigt, dass Zusammenfassungen entstehen.\n\n", "", 1)
	if err := os.WriteFile(docsPath(root, "sprints/erledigt/X5-test/README.md"), []byte(noGoal), 0o644); err != nil {
		t.Fatal(err)
	}
	if out := must(t)(Digest(root, DigestIn{Renew: true})); !strings.Contains(out, "X5 (ohne Ziel)") {
		t.Errorf("Hinweis fehlt: %q", out)
	}
	if got := doc(t, root, "sprints/erledigt/X5-test/"+DigestFile); !strings.Contains(got, "## Ziel\n\n–\n") {
		t.Errorf("Platzhalter fehlt:\n%s", got)
	}
}
