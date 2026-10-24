package planning

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

type nextSession struct{ nr, agent, status, domain, deps, crit string }

// nextRepo: BBB (Rang 1) vor AAA (Rang 2), obwohl AAA im Ordner zuerst steht; RUH ruht, CCC hat keinen aktiven Sprint.
func nextRepo(t *testing.T) string {
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
	w("backlog/B-010-offen.md", "# B-010 · Offen\n\n- **Status:** offen\n")
	w("backlog/archiv/B-011-fertig.md", "# B-011 · Fertig\n\n- **Status:** erledigt\n")
	w("sprints/geplant/.keep", "")
	w("projekte/AAA-a.md", nextProject("AAA", "aktiv", "2", "A1"))
	w("projekte/BBB-b.md", nextProject("BBB", "aktiv", "1", "B1"))
	w("projekte/CCC-c.md", nextProject("CCC", "aktiv", "3", "C9"))
	w("projekte/RUH-r.md", nextProject("RUH", "ruht", "–", "R1"))
	nextSprint(w, "erledigt", "E1", "–", []nextSession{{"E1.1", "autonom", "fertig", "SIM", "–", "AC-01"}})
	nextSprint(w, "aktiv", "B1", "BBB", []nextSession{
		{"B1.1", "Mensch", "offen", "SIM", "–", "AC-01"},
		{"B1.2", "autonom", "offen", "SIM", "B1.1", "AC-01"},
		{"B1.3", "autonom", "offen", "DEV", "B-010", "AC-01"},
		{"B1.4", "autonom", "offen", "CLI", "E1.1, B-011", "AC-02"},
	})
	nextSprint(w, "aktiv", "A1", "AAA", []nextSession{
		{"A1.1", "autonom", "in Arbeit", "SRV", "–", "AC-01"},
		{"A1.2", "autonom", "offen", "SRV", "–", "AC-01"},
		{"A1.3", "autonom", "offen", "INF", "–", "alle"},
		{"A1.4", "autonom", "offen", "CLI", "–", "AC-01"},
		{"A1.5", "autonom", "offen", "INF", "–", "AC-01"},
	})
	nextSprint(w, "aktiv", "R1", "RUH", []nextSession{{"R1.1", "autonom", "offen", "PLAT", "–", "AC-01"}})
	return root
}

func nextProject(id, status, rang string, sprints ...string) string {
	text := "# " + id + " · Projekt " + id + "\n\n- **Status:** " + status + "\n- **Rang:** " + rang +
		"\n\n## Sprints\n\n| Sprint | Status |\n|---|---|\n"
	for _, s := range sprints {
		text += "| " + s + " | aktiv |\n"
	}
	return text
}

func nextSprint(w func(rel, text string), state, id, project string, ss []nextSession) {
	dir := "sprints/" + state + "/" + id + "-s/"
	readme := "# " + id + " · SRV · Sprint " + id + "\n\n- **Status:** " + state + "\n- **Projekt:** " + project +
		"\n\n## Akzeptanzkriterien\n\n- **AC-01** Erstes.\n- **AC-02** Zweites\n  mit Folgezeile.\n\nNachsatz.\n\n" +
		"## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n"
	for _, s := range ss {
		file := s.nr + "-x.md"
		readme += "| " + s.nr + " | `" + file + "` | Umsetzung | " + s.agent + " | " + s.status + " |\n"
		w(dir+file, "# "+s.nr+" · Titel "+s.nr+"\n\n- **Agent:** "+s.agent+"\n- **Domäne:** "+s.domain+
			"\n- **Umgebung:** offline\n- **Branch:** "+strings.ToLower(id)+"/1-x\n- **Abhängig von:** "+s.deps+
			"\n- **Kriterien:** "+s.crit+"\n\n## Ziel\n\nZiel "+s.nr+".\n")
	}
	w(dir+"README.md", readme)
}

func checkText(t *testing.T, out string, want, not []string) {
	t.Helper()
	for _, s := range want {
		if !strings.Contains(out, s) {
			t.Errorf("fehlt %q in:\n%s", s, out)
		}
	}
	for _, s := range not {
		if strings.Contains(out, s) {
			t.Errorf("unerwartet %q in:\n%s", s, out)
		}
	}
}

// TestNextNachRangUndAbhaengigkeit: BBB vor AAA; B1.1 Mensch, B1.2 und B1.3 warten, B1.4 ist frei (E1.1 fertig,
// B-011 im Archiv). Parallel nur andere Domänen, je Domäne eine, ohne gesperrte SRV und ohne ruhendes Projekt.
func TestNextNachRangUndAbhaengigkeit(t *testing.T) {
	out := must(t)(Next(nextRepo(t), NextFilter{}))
	if !strings.HasPrefix(out, "Nächste Session: B1.4 · Titel B1.4\n") {
		t.Fatalf("Auswahl:\n%s", out)
	}
	checkText(t, out, []string{
		"Pfad: docs/sprints/aktiv/B1-s/B1.4-x.md", "Sprint: B1 · Sprint B1 (Projekt BBB, Rang 1)",
		"Domäne: CLI · Umgebung: offline", "Branch: sprint/b1", "Kriterien: AC-02", "Ziel B1.4.",
		"- **AC-02** Zweites\n  mit Folgezeile.", "Stand der Sprints",
		"── Parallel möglich ──\n- INF · A1.3 · Titel A1.3 (Sprint A1, offline)",
	}, []string{"- **AC-01**", "Nachsatz", "A1.4", "A1.5", "R1.1", "SRV ·"})
}

// TestNextMitDomaene: Filter wählt A1.3 (Kriterien alle), parallel steht die freie CLI-Session aus BBB.
func TestNextMitDomaene(t *testing.T) {
	root := nextRepo(t)
	out := must(t)(Next(root, NextFilter{Domain: "inf"}))
	if !strings.HasPrefix(out, "Nächste Session: A1.3 ·") {
		t.Fatalf("Auswahl:\n%s", out)
	}
	checkText(t, out, []string{"- **AC-01** Erstes.\n- **AC-02** Zweites", "- CLI · B1.4"}, []string{"A1.5", "- INF"})
	if _, err := Next(root, NextFilter{Domain: "XYZ"}); err == nil {
		t.Error("unbekannte Domäne angenommen")
	}
}

// TestNextKeineFrei: SRV ist durch A1.1 gesperrt; die Gründe stehen je Projekt in Rang-Reihenfolge, gekürzt.
func TestNextKeineFrei(t *testing.T) {
	out := must(t)(Next(nextRepo(t), NextFilter{Domain: "SRV"}))
	want := "Keine Session frei in Domäne SRV. Gründe je Projekt:\n" +
		"- BBB: B1.1 Agent Mensch; B1.2 wartet auf B1.1 (offen); B1.3 wartet auf B-010 (offen); … 1 weitere\n" +
		"- AAA: A1.2 Domäne SRV gesperrt (A1.1 in Arbeit); A1.3 andere Domäne INF; A1.4 andere Domäne CLI; … 1 weitere\n" +
		"- CCC: kein aktiver Sprint\n"
	if !strings.HasPrefix(out, want) || strings.Contains(out, "RUH") {
		t.Fatalf("Antwort:\n%s", out)
	}
}
