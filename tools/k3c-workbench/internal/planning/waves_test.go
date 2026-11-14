package planning

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// waveSession: Nr., Agent, Status, Abhängig von, Erlaubte Dateien (kommagetrennt), Kontext.
type waveSession struct{ nr, agent, status, deps, files, context string }

// waveWriter schreibt Dateien unter root/docs.
func waveWriter(t *testing.T, root string) func(rel, text string) {
	return func(rel, text string) {
		t.Helper()
		p := filepath.Join(root, "docs", filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
}

// rules ist der Abschnitt „Regeln und Einschränkungen“ mit den Planungsangaben.
func rules(area, deps, needs, size, lock string) string {
	return "## Regeln und Einschränkungen\n\nDomäne DEV.\n\n- **Bereich:** " + area + "\n- **Abhängig von:** " + deps +
		"\n- **Braucht 🧑:** " + needs + "\n- **Größe:** " + size + "\n- **Sperre:** " + lock + "\n\n## Beispiele\n\n–\n"
}

func waveSprint(w func(rel, text string), id, project, state, lock string, ss []waveSession) {
	dir := "sprints/" + state + "/" + id + "-s/"
	readme := "# " + id + " · DEV · Sprint " + id + "\n\n- **Status:** " + state + "\n- **Projekt:** " + project +
		"\n- **Spec:** freigegeben\n\n" + rules("`tools/"+strings.ToLower(id)+"/**`", "–", "–", "M", lock) +
		"\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n"
	for _, s := range ss {
		file := s.nr + "-x.md"
		readme += "| " + s.nr + " | `" + file + "` | Umsetzung | " + s.agent + " | " + s.status + " |\n"
		text := "# " + s.nr + " · Titel " + s.nr + "\n\n- **Agent:** " + s.agent + "\n- **Domäne:** DEV\n- **Umgebung:** offline" +
			"\n- **Branch:** " + strings.ToLower(id) + "/1-x\n- **Abhängig von:** " + s.deps + "\n- **Kriterien:** AC-01\n\n" +
			"## Ziel\n\nZiel.\n\n## Kontext\n\n" + s.context + "\n\n## Erlaubte Dateien\n\n"
		for _, f := range strings.Split(s.files, ",") {
			if f != "" {
				text += "- `" + f + "`\n"
			}
		}
		w(dir+file, text+"\n## Nicht-Ziele\n\n–\n")
	}
	w(dir+"README.md", readme)
}

func waveTicket(w func(rel, text string), nr, project, prio, spec, rulesText string) {
	w("backlog/"+nr+"-t.md", "# "+nr+" · Ticket "+nr+"\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** "+prio+
		"\n- **Umgebung:** offline\n- **Status:** offen\n- **Sprint:** –\n- **Projekt:** "+project+"\n- **Spec:** "+spec+
		"\n\n## Ziel\n\nZiel.\n\n"+rulesText)
}

// wavesRepo: AAA (Rang 1) mit A1 (Sperre exe) und R1 (R1.1 in Arbeit), BBB (Rang 2) mit B1 (Sperre exe) und C1.
func wavesRepo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	w := waveWriter(t, root)
	w("backlog/README.md", "# Backlog\n")
	w("sprints/geplant/.keep", "")
	w("projekte/AAA-a.md", nextProject("AAA", "aktiv", "1", "A1", "R1"))
	w("projekte/BBB-b.md", nextProject("BBB", "aktiv", "2", "B1", "C1"))
	waveSprint(w, "A1", "AAA", "aktiv", "exe", []waveSession{
		{"A1.1", "autonom", "offen", "–", "tools/a/**", ""},
		{"A1.2", "autonom", "offen", "A1.1", "tools/a/x/**", ""},
		{"A1.3", "Mensch", "offen", "A1.2", "", "Abnahme am Fernseher.\n\nGerät: Xbox"},
	})
	waveSprint(w, "R1", "AAA", "aktiv", "–", []waveSession{{"R1.1", "autonom", "in Arbeit", "–", "tools/r/**", ""}})
	waveSprint(w, "B1", "BBB", "aktiv", "exe", []waveSession{{"B1.1", "autonom", "offen", "–", "tools/b/**", ""}})
	waveSprint(w, "C1", "BBB", "aktiv", "–", []waveSession{
		{"C1.1", "autonom", "offen", "–", "tools/a/sub/file.go", ""},
		{"C1.2", "autonom", "offen", "–", "tools/c/**,docs/backlog/README.md", ""},
		{"C1.3", "Mensch", "offen", "–", "", "Am Gerät prüfen."},
	})
	waveTicket(w, "B-001", "AAA", "hoch", "freigegeben", rules("`tools/d/**`", "–", "–", "S", "–"))
	waveTicket(w, "B-002", "BBB", "niedrig", "freigegeben", rules("`tools/e/**`", "–", "–", "S", "–"))
	waveTicket(w, "B-003", "BBB", "mittel", "freigegeben", rules("", "–", "–", "S", "–"))
	waveTicket(w, "B-004", "BBB", "mittel", "freigegeben", rules("`tools/f/**`", "B-999", "–", "S", "–"))
	waveTicket(w, "B-005", "BBB", "mittel", "freigegeben", rules("`tools/g/**`", "B-003", "–", "S", "–"))
	waveTicket(w, "B-006", "BBB", "mittel", "Entwurf", rules("`tools/h/**`", "–", "Freigabe", "S", "–"))
	waveTicket(w, "B-007", "BBB", "mittel", "Entwurf", rules("`tools/i/**`", "–", "Entscheidung", "S", "–"))
	waveTicket(w, "B-008", "BBB", "mittel", "freigegeben", rules("`tools/j/**`", "–", "Freigabe", "S", "–"))
	return root
}

// B-783/AC-01: Abhängigkeit (A1.2 nach A1.1), gleiche Sperre (B1.1 neben A1.1), überlappender Bereich (C1.1 unter
// tools/a/**), volle Plätze (B-002) und Knoten ohne Welle mit Grund.
func TestWavesAbhaengigkeitBereichSperre(t *testing.T) {
	t.Parallel()
	out := must(t)(Waves(wavesRepo(t), WavesFilter{}))
	want := []string{
		"Arbeitswellen: 1 in Arbeit, 7 eingeplant in 3 Wellen, 3 in keiner Welle · 4 Plätze",
		"── Welle 1, jetzt (4/4 Plätze) ──\n- R1.1 · in Arbeit · Titel R1.1 (DEV, Projekt AAA, Sprint R1)\n" +
			"- B-001 · Ticket · Ticket B-001 (DEV, Projekt AAA)\n- A1.1 · Session · Titel A1.1 (DEV, Projekt AAA, Sprint A1) · Sperre exe\n" +
			"- C1.2 · Session · Titel C1.2 (DEV, Projekt BBB, Sprint C1)\n",
		"── Welle 2 (3/4 Plätze) ──\n- B1.1 · Session · Titel B1.1 (DEV, Projekt BBB, Sprint B1) · Sperre exe — kollidiert mit A1.1: Sperre exe\n" +
			"- C1.1 · Session · Titel C1.1 (DEV, Projekt BBB, Sprint C1) — kollidiert mit A1.1: Bereich `tools/a/**` / `tools/a/sub/file.go`\n" +
			"- B-002 · Ticket · Ticket B-002 (DEV, Projekt BBB) — Plätze voll\n",
		"── Welle 3 (1/4 Plätze) ──\n- A1.2 · Session · Titel A1.2 (DEV, Projekt AAA, Sprint A1) · Sperre exe — nach A1.1; kollidiert mit B1.1: Sperre exe\n",
		"- B-003 · Ticket · Ticket B-003 — Bereich fehlt",
		"- Fehler: B-004 · Ticket · Ticket B-004 — unbekannte ID B-999 in „Abhängig von“",
		"- B-005 · Ticket · Ticket B-005 — wartet auf B-003 (nicht in einer Welle)",
	}
	checkText(t, out, want, []string{"B-006", "B-007", "B-008", "A1.3", "C1.3"})
}

// Filter grenzen nur die Anzeige ein; slots außerhalb 1–4 lehnt das Tool ab.
func TestWavesFilterUndPlaetze(t *testing.T) {
	t.Parallel()
	root := wavesRepo(t)
	out := must(t)(Waves(root, WavesFilter{Project: "aaa"}))
	checkText(t, out, []string{"A1.1", "Anzeige gefiltert"}, []string{"C1.2", "- B1.1"})
	out = must(t)(Waves(root, WavesFilter{Slots: 2}))
	checkText(t, out, []string{"── Welle 1, jetzt (2/2 Plätze) ──\n- R1.1 · in Arbeit", "- B-001 · Ticket"}, nil)
	if _, err := Waves(root, WavesFilter{Slots: 5}); err == nil {
		t.Error("slots 5 angenommen")
	}
}

func TestPlanSpecUndGlobs(t *testing.T) {
	t.Parallel()
	p := parsePlanSpec(rules("`a/**`, `docs/glossar.md`, `b.go`", "B-1, SP12.3, irgendwas", "–", "M", "`exe`, golden"))
	if strings.Join(p.Area, " ") != "a/** b.go" || strings.Join(p.Deps, " ") != "B-1 SP12.3" ||
		strings.Join(p.BadDeps, " ") != "irgendwas" || p.Needs != "–" || p.Size != "M" || strings.Join(p.Locks, " ") != "exe golden" {
		t.Errorf("Angaben: %+v", p)
	}
	cases := []struct {
		a, b string
		want bool
	}{
		{"tools/a/**", "tools/a/x.go", true},
		{"tools/assets/style/**", "tools/assets/sound/**", false},
		{"tools/paper/cli/gen.ts", "tools/paper/cli/gen.ts", true},
		{"tools/paper/cli/gen.ts", "tools/paper/cli/gen.tsx", false},
		{"x/tools_planning*.go", "x/tools_sim*.go", false},
		{"x/**", "x/tools_sim*.go", true},
		{"docs/", "docs/arbeitsweise.md", true},
	}
	for _, c := range cases {
		if got := globsOverlap(c.a, c.b); got != c.want {
			t.Errorf("globsOverlap(%q, %q) = %v", c.a, c.b, got)
		}
	}
}
