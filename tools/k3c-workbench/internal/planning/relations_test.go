package planning

import (
	"os"
	"strings"
	"testing"
)

// relRepo legt B-002 bis B-004 über Create an (volle Vorlage mit Relationsfeldern).
func relRepo(t *testing.T) string {
	t.Helper()
	root := tempRepo(t)
	old := "# B-001 · Altes Ticket\n\n- **Domäne:** SRV\n- **Typ:** Idee\n- **Prio:** mittel\n- **Status:** offen\n- **Sprint:** –\n"
	if err := os.WriteFile(docsPath(root, "backlog/B-001-alt.md"), []byte(old), 0o644); err != nil {
		t.Fatal(err)
	}
	for _, title := range []string{"Zweites Ticket", "Drittes Ticket", "Viertes Ticket"} {
		must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "t", Title: title,
			Fields: map[string]string{"Domäne": "DEV", "Typ": "Idee", "Prio": "mittel"}}))
	}
	return root
}

// TestRelationenUngueltigAendernNichts: B-696/AC-02.
func TestRelationenUngueltigAendernNichts(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	must(t)(Set(root, "B-003", map[string]string{fieldReplace: "B-002"}))
	must(t)(Set(root, "B-004", map[string]string{fieldReplace: "B-003"}))
	must(t)(Set(root, "B-001", map[string]string{"Status": "erledigt"}))
	before := snapshot(t, root)
	bad := map[string]struct {
		id     string
		values map[string]string
		want   string
	}{
		"unbekannte ID":         {"B-002", map[string]string{fieldReplace: "B-099"}, "B-099 gibt es nicht"},
		"Selbstverweis":         {"B-002", map[string]string{fieldReplace: "B-002"}, "auf sich selbst"},
		"Zyklus direkt":         {"B-002", map[string]string{fieldReplace: "B-003"}, "Zyklus B-002 → B-003 → B-002"},
		"Zyklus zwei Schritte":  {"B-002", map[string]string{fieldReplace: "B-004"}, "Zyklus B-002 → B-004 → B-003 → B-002"},
		"Zyklus über Duplikat":  {"B-002", map[string]string{fieldDup: "B-004"}, "Zyklus"},
		"Duplikat mehrere":      {"B-002", map[string]string{fieldDup: "B-003, B-004"}, "genau eine ID"},
		"Duplikat erledigt":     {"B-001", map[string]string{fieldDup: "B-002"}, "erledigt"},
		"Format":                {"B-002", map[string]string{fieldReplace: "B-3"}, "keine Ticket-ID"},
		"doppelt":               {"B-002", map[string]string{fieldReplace: "B-001, B-001"}, "doppelt"},
		"Duplikat beim Anlegen": {"", map[string]string{fieldDup: "B-002"}, "kein Duplikat"},
	}
	for name, c := range bad {
		var err error
		if c.id == "" {
			_, err = Create(root, NewDoc{Kind: "ticket", Slug: "d", Title: "Neu", Fields: map[string]string{
				"Domäne": "DEV", "Typ": "Idee", "Prio": "mittel", fieldDup: "B-002"}})
		} else {
			_, err = Set(root, c.id, c.values)
		}
		if err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s: Fehler %v, erwartet %q", name, err, c.want)
		}
		if snapshot(t, root) != before {
			t.Errorf("%s: Dateien geändert", name)
		}
	}
}

// TestDuplikatVerwirftUndArchiviert: B-696/AC-03, dazu der Hinweis auf die Wurzel der Kette.
func TestDuplikatVerwirftUndArchiviert(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	must(t)(Set(root, "B-003", map[string]string{fieldReplace: "B-002"}))
	out := must(t)(Set(root, "B-004", map[string]string{fieldDup: "B-002"}))
	if !strings.Contains(out, "Status verworfen") || !strings.Contains(out, "Wurzel der Kette ist B-003") {
		t.Errorf("Antwort: %q", out)
	}
	r, err := resolve(root, "B-004")
	if err != nil || r.state != "archiv" {
		t.Fatalf("nicht archiviert: %v %+v", err, r)
	}
	if f := headFields(doc(t, root, r.rel)); f["Status"] != "verworfen" || f[fieldDup] != "B-002" {
		t.Errorf("Kopf: %v", f)
	}
	all := indexText(t, root)
	if !strings.Contains(all, "| [B-004](archiv/B-004-t.md) |") && !strings.Contains(all, "| [B-004](B-004-t.md) | DEV | Idee | mittel | verworfen |") {
		t.Errorf("Index nicht nachgezogen:\n%s", all)
	}
	checkConsistent(t, root)
}

// TestGegenrichtungBerechnet: B-696/AC-04; die Datei des Gegenstücks bleibt byte-gleich.
func TestGegenrichtungBerechnet(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	must(t)(Set(root, "B-003", map[string]string{fieldReplace: "B-002"}))
	must(t)(Set(root, "B-004", map[string]string{fieldDup: "B-002"}))
	p := docsPath(root, "backlog/B-002-t.md")
	before, _ := os.ReadFile(p)
	out := must(t)(Get(root, "B-002"))
	after, _ := os.ReadFile(p)
	if string(before) != string(after) {
		t.Error("Get hat die Datei geändert")
	}
	for _, want := range []string{"> **Ersetzt durch:** B-003 (berechnet)", "> **Duplikate:** B-004 (berechnet)"} {
		if !strings.Contains(out, want) {
			t.Errorf("Get ohne %q:\n%s", want, out)
		}
	}
	if strings.Index(out, "Ersetzt durch") > strings.Index(out, "## Ausgangslage") {
		t.Error("Gegenrichtung steht nicht unter dem Kopf")
	}
	if out := must(t)(Get(root, "B-003")); strings.Contains(out, "(berechnet)") {
		t.Errorf("B-003 ohne Gegenstück:\n%s", out)
	}
	if list := must(t)(List(root, Filter{Kind: "ticket", Archive: true})); !strings.Contains(list, "Drittes Ticket · ersetzt B-002") {
		t.Errorf("plan_list ohne Relation:\n%s", list)
	}
}

// TestAltticketBekommtFelder: B-696/AC-05; die Felder kommen hinter Freigabe, CRLF bleibt.
func TestAltticketBekommtFelder(t *testing.T) {
	t.Parallel()
	root := relRepo(t)
	old := "# B-010 · Altes Ticket mit CRLF\r\n\r\n- **Domäne:** DEV\r\n- **Typ:** Idee\r\n- **Prio:** mittel\r\n" +
		"- **Umgebung:** offline\r\n- **Status:** offen\r\n- **Sprint:** –\r\n- **Projekt:** –\r\n- **Erstellt:** 2026-01-01\r\n" +
		"- **Spec:** Entwurf\r\n- **Revision:** 1\r\n- **Freigabe:** –\r\n\r\n## Ausgangslage\r\n\r\nText\r\n"
	p := docsPath(root, "backlog/B-010-alt.md")
	if err := os.WriteFile(p, []byte(old), 0o644); err != nil {
		t.Fatal(err)
	}
	must(t)(Set(root, "B-010", map[string]string{fieldReplace: "B-002, B-003"}))
	b, _ := os.ReadFile(p)
	got := string(b)
	want := "- **Freigabe:** –\r\n- **Ersetzt:** B-002, B-003\r\n\r\n## Ausgangslage"
	if !strings.Contains(got, want) || strings.Count(got, "\n") != strings.Count(got, "\r\n") {
		t.Fatalf("Datei:\n%q", got)
	}
	must(t)(Set(root, "B-010", map[string]string{fieldDup: "B-004"}))
	b, _ = os.ReadFile(docsPath(root, "backlog/archiv/B-010-alt.md"))
	if !strings.Contains(string(b), "- **Freigabe:** –\r\n- **Duplikat von:** B-004\r\n- **Ersetzt:** B-002, B-003\r\n") {
		t.Fatalf("Reihenfolge laut Vorlage:\n%q", b)
	}
}

// TestAehnlicheTitel: B-696/AC-06.
func TestAehnlicheTitel(t *testing.T) {
	t.Parallel()
	root := tempRepo(t)
	index := strings.Replace(indexMD, "| Altes Ticket |\n", "| Altes Ticket |\n"+
		"| [B-020](B-020-a.md) | DEV | Idee | mittel | offen | – | Balancing-Seite zeigt eine Liste mit Varianten |\n", 1) +
		"| [B-021](archiv/B-021-b.md) | DEV | Idee | mittel | erledigt | – | Die Varianten stehen auf der Balancing-Seite als Liste |\n" +
		"| [B-022](archiv/B-022-c.md) | SRV | Idee | mittel | erledigt | – | Server speichert Spielstände schneller |\n"
	if err := os.WriteFile(docsPath(root, "backlog/README.md"), []byte(index), 0o644); err != nil {
		t.Fatal(err)
	}
	fields := map[string]string{"Domäne": "DEV", "Typ": "Idee", "Prio": "mittel"}
	out := must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "a", Title: "Die Balancing-Seite listet Varianten in einer Liste", Fields: fields}))
	if !strings.Contains(out, "B-002 angelegt") || !strings.Contains(out, "B-020 (offen)") || !strings.Contains(out, "B-021 (archiv, erledigt)") ||
		strings.Contains(out, "B-022") {
		t.Errorf("ähnlicher Titel:\n%s", out)
	}
	out = must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "b", Title: "Monarch reitet über die Brücke", Fields: fields}))
	if strings.Contains(out, "ähnlich") {
		t.Errorf("unähnlicher Titel mit Warnung:\n%s", out)
	}
	if err := os.MkdirAll(docsPath(root, archiveIndex), 0o755); err != nil {
		t.Fatal(err)
	}
	out = must(t)(Create(root, NewDoc{Kind: "ticket", Slug: "c", Title: "Balancing-Seite zeigt Varianten", Fields: fields}))
	if !strings.Contains(out, "B-004 angelegt") || !strings.Contains(out, "Ähnlichkeitsprüfung übersprungen") {
		t.Errorf("Prüfung kaputt, Ticket trotzdem angelegt:\n%s", out)
	}
}

// indexText sind beide Index-Dateien hintereinander (die Archiv-Datei darf fehlen).
func indexText(t *testing.T, root string) string {
	t.Helper()
	text := doc(t, root, "backlog/README.md")
	if b, err := os.ReadFile(docsPath(root, archiveIndex)); err == nil {
		text += "\n" + string(b)
	}
	return text
}
