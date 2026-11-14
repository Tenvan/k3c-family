package planning

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// failWritesTo ersetzt writeFile für diesen Test: Schreiben nach rel scheitert, die Reihenfolge der übrigen wird
// mitgeschrieben. Nicht parallel (Paket-Variable); parallele Tests laufen erst nach den sequentiellen weiter.
func failWritesTo(t *testing.T, root, rel string) *[]string {
	t.Helper()
	var order []string
	orig := writeFile
	t.Cleanup(func() { writeFile = orig })
	fail := docsPath(root, rel)
	writeFile = func(p, text string, crlf bool) error {
		if p == fail {
			return errors.New("erzwungener Schreibfehler")
		}
		order = append(order, p)
		return orig(p, text, crlf)
	}
	return &order
}

// indexCount zählt, in wie vielen Index-Dateien das Ticket eine Zeile hat.
func indexCount(t *testing.T, root, nr string) int {
	t.Helper()
	n := 0
	for _, f := range indexFiles {
		if strings.Contains(doc(t, root, f), "| ["+nr+"](") {
			n++
		}
	}
	return n
}

// B-862/AC-01: Scheitert der zweite Index (der die Zeile abgibt), bleibt die Zeile in genau einem Index, die
// Ticket-Datei am alten Ort, beide Index-Dateien byte-gleich; scheitert der erste, ebenso.
func TestStatuswechselSchreibfehlerVerliertKeineZeile(t *testing.T) { //nolint:paralleltest // ersetzt writeFile
	for _, broken := range []string{mainIndex, archiveIndex} {
		t.Run(broken, func(t *testing.T) { statuswechselMitSchreibfehler(t, broken) })
	}
}

func statuswechselMitSchreibfehler(t *testing.T, broken string) {
	{
		root := relRepo(t)
		must(t)(RebuildIndex(root, false))
		before := snapshot(t, root)
		failWritesTo(t, root, broken)
		if _, err := Set(root, "B-002", map[string]string{"Status": "erledigt"}); err == nil ||
			!strings.Contains(err.Error(), "zurückgenommen") {
			t.Fatalf("%s kaputt: Fehler %v", broken, err)
		}
		if n := indexCount(t, root, "B-002"); n != 1 {
			t.Errorf("%s kaputt: Zeile in %d Index-Dateien", broken, n)
		}
		if after := snapshot(t, root); after != before {
			t.Errorf("%s kaputt: Zustand geändert", broken)
		}
		if _, err := os.Stat(docsPath(root, "backlog/B-002-t.md")); err != nil {
			t.Errorf("%s kaputt: Ticket nicht zurück: %v", broken, err)
		}
	}
}

// B-862: Ohne Fehler schreibt ein Statuswechsel zuerst den Index, der die Zeile bekommt.
func TestStatuswechselSchreibtZielIndexZuerst(t *testing.T) { //nolint:paralleltest // ersetzt writeFile
	root := relRepo(t)
	must(t)(RebuildIndex(root, false))
	order := failWritesTo(t, root, "gibt-es-nicht")
	must(t)(Set(root, "B-002", map[string]string{"Status": "erledigt"}))
	pos := map[string]int{}
	for i, p := range *order {
		pos[p] = i + 1
	}
	a, m := pos[docsPath(root, archiveIndex)], pos[docsPath(root, mainIndex)]
	if a == 0 || m == 0 || a > m {
		t.Errorf("Reihenfolge Archiv %d, Haupt %d: %v", a, m, *order)
	}
	if indexCount(t, root, "B-002") != 1 {
		t.Error("Zeile nicht in genau einem Index")
	}
}

// B-863/AC-01: Mit B-999 im Backlog vergibt Create B-1000 und B-1001; Auflösen, Relationen und Index (numerisch
// sortiert) gehen mit vierstelligen Nummern.
func TestTicketNummernAbB1000(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	head := "# B-999 · Neunhundert\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Status:** offen\n- **Sprint:** –\n"
	if err := os.WriteFile(docsPath(root, "backlog/B-999-neun.md"), []byte(head), 0o644); err != nil {
		t.Fatal(err)
	}
	must(t)(RebuildIndex(root, false)) // B-999 bekommt seine Zeile
	for _, want := range []string{"B-1000 angelegt", "B-1001 angelegt"} {
		if out := newTicket(t, root); !strings.Contains(out, want) {
			t.Fatalf("Nummer: %q", out)
		}
	}
	must(t)(Set(root, "B-1001", map[string]string{fieldReplace: "B-999, B-1000", "Prio": "hoch"}))
	if r, err := resolve(root, "B-1000"); err != nil || !strings.HasPrefix(filepath.Base(r.rel), "B-1000-") {
		t.Fatalf("resolve B-1000: %+v %v", r, err)
	}
	must(t)(RebuildIndex(root, false))
	main := doc(t, root, mainIndex)
	i999, i1000, i1001 := strings.Index(main, "[B-999]"), strings.Index(main, "[B-1000]"), strings.Index(main, "[B-1001]")
	if i999 < 0 || i999 > i1000 || i1000 > i1001 {
		t.Errorf("Index nicht numerisch sortiert:\n%s", main)
	}
	must(t)(Set(root, "B-1000", map[string]string{"Status": "erledigt"}))
	if indexCount(t, root, "B-1000") != 1 || !strings.Contains(doc(t, root, archiveIndex), "| [B-1000](B-1000-") {
		t.Errorf("B-1000 nicht im Archiv-Index:\n%s", doc(t, root, archiveIndex))
	}
	if !strings.Contains(strings.Join(reverseLines(root, "B-1000"), "\n"), "Ersetzt durch:** B-1001") {
		t.Errorf("Relation B-1001 → B-1000 fehlt: %v", reverseLines(root, "B-1000"))
	}
}

// B-863: ticketLess und less (Index) ordnen nach Nummer, nicht nach Text.
func TestTicketNummernNumerischSortiert(t *testing.T) {
	t.Parallel()
	for _, c := range [][2]string{{"B-999", "B-1000"}, {"B-1000", "B-1001"}, {"B-002", "B-010"}} {
		if !ticketLess(c[0], c[1]) || ticketLess(c[1], c[0]) || !less(nil, c[0], c[1]) {
			t.Errorf("ticketLess(%s, %s)", c[0], c[1])
		}
	}
}

// B-865/AC-01: Eine Ticket-Änderung lässt den Fahrplan byte-gleich, eine Sprint- oder Session-Änderung heilt ihn.
func TestFahrplanHeilungNurBeiSprintAenderung(t *testing.T) {
	t.Parallel()
	root := refreshRepo(t)
	before := doc(t, root, roadmapRel)
	c := newChangeSet(root)
	c.write("backlog/README.md", "# Backlog\n\nneu\n")
	if err := c.apply(); err != nil {
		t.Fatal(err)
	}
	if doc(t, root, roadmapRel) != before {
		t.Fatal("Ticket-Änderung hat den Fahrplan geschrieben")
	}
	session := "sprints/aktiv/A1-a/A1.3-x.md"
	c = newChangeSet(root)
	c.write(session, doc(t, root, session))
	if err := c.apply(); err != nil {
		t.Fatal(err)
	}
	if after := doc(t, root, roadmapRel); after == before || strings.Contains(after, "`aktiv/NG15-ton/`") {
		t.Errorf("Session-Änderung hat den Fahrplan nicht geheilt:\n%s", after)
	}

	// Über Set: plan_set B-002 Prio hoch lässt einen veralteten Fahrplan stehen.
	repo := relRepo(t)
	stale := strings.Replace(doc(t, repo, roadmapRel), "|---|---|---|---|---|\n\n## Offen am Gerät",
		"|---|---|---|---|---|\n| ZZ9 | DEV | Weg | – | `aktiv/ZZ9-weg/` |\n\n## Offen am Gerät", 1)
	if err := os.WriteFile(docsPath(repo, roadmapRel), []byte(stale), 0o644); err != nil {
		t.Fatal(err)
	}
	must(t)(Set(repo, "B-002", map[string]string{"Prio": "hoch"}))
	if doc(t, repo, roadmapRel) != stale {
		t.Error("plan_set am Ticket hat den Fahrplan geändert")
	}
}
