package planning

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// laneSession ist eine Session im Lane-Fixture; jede liegt in einem eigenen aktiven Sprint (Sessions eines Sprints
// laufen nacheinander, PV1.1).
type laneSession struct{ nr, status, domain, lane, model string }

// laneRepo: ein aktives Projekt LAN mit je einem aktiven Sprint pro Session, in der Reihenfolge der Eingabe.
func laneRepo(t *testing.T, ss ...laneSession) string {
	t.Helper()
	root := t.TempDir()
	w := func(rel, text string) {
		p := filepath.Join(root, "docs", filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	w("backlog/README.md", "# Backlog\n")
	w("sprints/geplant/.keep", "")
	var ids []string
	for _, s := range ss {
		id, _, _ := strings.Cut(s.nr, ".")
		ids = append(ids, id)
		dir := "sprints/aktiv/" + id + "-s/"
		w(dir+"README.md", "# "+id+" · "+s.domain+" · Sprint "+id+"\n\n- **Status:** aktiv\n- **Projekt:** LAN\n\n"+
			"## Akzeptanzkriterien\n\n- **AC-01** Eins.\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n"+
			"| "+s.nr+" | `"+s.nr+"-x.md` | Umsetzung | autonom | "+s.status+" |\n")
		head := "- **Agent:** autonom\n- **Domäne:** " + s.domain + "\n"
		if s.lane != "" {
			head += "- **Lane:** " + s.lane + "\n"
		}
		if s.model != "" {
			head += "- **Modell:** " + s.model + "\n"
		}
		w(dir+s.nr+"-x.md", "# "+s.nr+" · Titel "+s.nr+"\n\n"+head+"- **Umgebung:** offline\n- **Branch:** "+
			strings.ToLower(id)+"/1-x\n- **Abhängig von:** –\n- **Kriterien:** AC-01\n\n## Ziel\n\nZiel.\n")
	}
	w("projekte/LAN-l.md", nextProject("LAN", "aktiv", "1", ids...))
	return root
}

// TestNextLanesParallel: zwei offene DEV-Sessions mit verschiedenen Lanes laufen parallel.
func TestNextLanesParallel(t *testing.T) {
	root := laneRepo(t, laneSession{"L1.1", "offen", "DEV", "mcp", ""}, laneSession{"L2.1", "offen", "DEV", "ui", ""})
	out := must(t)(Next(root, NextFilter{}))
	if !strings.HasPrefix(out, "Nächste Session: L1.1 ·") {
		t.Fatalf("Auswahl:\n%s", out)
	}
	checkText(t, out, []string{"Domäne: DEV · Lane: mcp · Umgebung: offline", "── Parallel möglich ──\n- DEV/ui · L2.1 · Titel L2.1"}, nil)
}

// TestNextSameLaneLocked: DEV/mcp in Arbeit sperrt eine offene DEV/mcp, nicht DEV/ui.
func TestNextSameLaneLocked(t *testing.T) {
	root := laneRepo(t, laneSession{"L1.1", "in Arbeit", "DEV", "mcp", ""}, laneSession{"L2.1", "offen", "DEV", "mcp", ""},
		laneSession{"L3.1", "offen", "DEV", "ui", ""})
	out := must(t)(Next(root, NextFilter{}))
	if !strings.HasPrefix(out, "Nächste Session: L3.1 ·") {
		t.Fatalf("Auswahl:\n%s", out)
	}
	out = must(t)(Next(laneRepo(t, laneSession{"L1.1", "in Arbeit", "DEV", "mcp", ""}, laneSession{"L2.1", "offen", "DEV", "mcp", ""}), NextFilter{}))
	checkText(t, out, []string{"L2.1 Domäne DEV, Lane mcp gesperrt (L1.1 in Arbeit)"}, []string{"Nächste Session"})
}

// TestNextNoLaneLocksDomain: eine Session ohne Lane sperrt die Domäne und wird von jeder Lane gesperrt.
func TestNextNoLaneLocksDomain(t *testing.T) {
	out := must(t)(Next(laneRepo(t, laneSession{"L1.1", "in Arbeit", "DEV", "", ""}, laneSession{"L2.1", "offen", "DEV", "ui", ""}), NextFilter{}))
	checkText(t, out, []string{"L2.1 Domäne DEV gesperrt (L1.1 in Arbeit)"}, []string{"Nächste Session"})
	out = must(t)(Next(laneRepo(t, laneSession{"L1.1", "in Arbeit", "DEV", "ui", ""}, laneSession{"L2.1", "offen", "DEV", "–", ""}), NextFilter{}))
	checkText(t, out, []string{"L2.1 Domäne DEV gesperrt (L1.1 in Arbeit)"}, []string{"Nächste Session"})
	// Parallel: eine Session ohne Lane steht nie neben einer anderen derselben Domäne.
	out = must(t)(Next(laneRepo(t, laneSession{"L1.1", "offen", "DEV", "", ""}, laneSession{"L2.1", "offen", "DEV", "ui", ""}), NextFilter{}))
	checkText(t, out, []string{"Nächste Session: L1.1", "Parallel möglich ──\nkeine"}, []string{"- DEV/ui"})
}

// TestNextShowsModel: Modell aus dem Feld, ohne Feld „sonnet (Standard)“, auch in der Parallel-Liste.
func TestNextShowsModel(t *testing.T) {
	out := must(t)(Next(laneRepo(t, laneSession{"L1.1", "offen", "PAP", "renderer", "opus"}, laneSession{"L2.1", "offen", "DEV", "", ""}), NextFilter{}))
	checkText(t, out, []string{"Domäne: PAP · Lane: renderer · Umgebung: offline · Modell: opus",
		"- DEV · L2.1 · Titel L2.1 (Sprint L2, offline) · Modell: sonnet (Standard)"}, nil)
	out = must(t)(Next(laneRepo(t, laneSession{"L1.1", "offen", "DEV", "", "–"}), NextFilter{}))
	checkText(t, out, []string{"Domäne: DEV · Umgebung: offline · Modell: sonnet (Standard)"}, []string{"Lane:"})
}

// TestNextMaxFourParallel: fünf freie Lanes → neben der Auswahl höchstens drei weitere.
func TestNextMaxFourParallel(t *testing.T) {
	var ss []laneSession
	for i, lane := range []string{"a", "b", "c", "d", "e"} {
		ss = append(ss, laneSession{"L" + string(rune('1'+i)) + ".1", "offen", "PAP", lane, ""})
	}
	out := must(t)(Next(laneRepo(t, ss...), NextFilter{}))
	_, par, _ := strings.Cut(out, "── Parallel möglich ──\n")
	par, _, _ = strings.Cut(par, "\n\n")
	if n := strings.Count(par, "- PAP/"); n != 3 || strings.Contains(par, "L5.1") {
		t.Fatalf("%d parallel statt 3:\n%s", n, out)
	}
}

// TestDomainPAPValid: Domäne PAP beim Anlegen, Lane und Modell setzen (auch in Dateien ohne die Felder), ungültige
// Werte mit Wertebereich abgelehnt.
func TestDomainPAPValid(t *testing.T) {
	root := laneRepo(t, laneSession{"L1.1", "offen", "DEV", "", ""})
	tpl := "# SP00.1 · Titel\n\n- **Status:** offen\n- **Typ:** Umsetzung\n- **Agent:** autonom\n- **Domäne:** REG | DEV | PAP\n" +
		"- **Lane:** – (oder Kurzname)\n- **Modell:** – | haiku | sonnet | opus | fable (Empfehlung)\n- **Umgebung:** offline | live | ?\n" +
		"- **Branch:** sp00/1-x\n- **Abhängig von:** –\n- **Tickets:** B-000\n- **Kriterien:** AC-01\n\n## Ziel\n\nZiel.\n"
	if err := os.MkdirAll(filepath.Join(root, "docs", "vorlagen"), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, "docs", "vorlagen", "session.md"), []byte(tpl), 0o644); err != nil {
		t.Fatal(err)
	}
	roadmap := "# Fahrplan\n\n## Aktiv\n\n| Sprint | Domäne | Projekt | Thema | Am Ende sichtbar | Ordner |\n|---|---|---|---|---|---|\n" +
		"| L1 | DEV | LAN | Sprint L1 | – | `aktiv/L1-s/` |\n"
	if err := os.WriteFile(filepath.Join(root, "docs", "sprints", "README.md"), []byte(roadmap), 0o644); err != nil {
		t.Fatal(err)
	}
	must(t)(Create(root, NewDoc{Kind: "session", ID: "L1.2", Slug: "neu", Title: "Neu", Fields: map[string]string{"Domäne": "PAP"}}))
	created := readDoc(t, root, "sprints/aktiv/L1-s/L1.2-neu.md")
	checkText(t, created, []string{"- **Domäne:** PAP\n- **Lane:** –\n- **Modell:** –\n- **Umgebung:** ?"}, nil)
	must(t)(Set(root, "L1.1", map[string]string{"Lane": "mcp", "Modell": "opus"}))
	checkText(t, readDoc(t, root, "sprints/aktiv/L1-s/L1.1-x.md"), []string{"- **Domäne:** DEV\n- **Lane:** mcp\n- **Modell:** opus\n- **Umgebung:** offline"}, nil)
	for _, bad := range []map[string]string{{"Lane": "MCP!"}, {"Modell": "gpt"}, {"Domäne": "XYZ"}} {
		if _, err := Set(root, "L1.2", bad); err == nil || !strings.Contains(err.Error(), "ungültig") {
			t.Errorf("%v: %v", bad, err)
		}
	}
	if _, err := Set(root, "L1.2", map[string]string{"Modell": "gpt"}); err == nil || !strings.Contains(err.Error(), "haiku, sonnet, opus, fable") {
		t.Errorf("Wertebereich fehlt: %v", err)
	}
}

func readDoc(t *testing.T, root, rel string) string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(root, "docs", filepath.FromSlash(rel)))
	if err != nil {
		t.Fatal(err)
	}
	return strings.ReplaceAll(string(b), "\r\n", "\n")
}
