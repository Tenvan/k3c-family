package balance

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// B-632/AC-02: Die eingebettete Richtwert-Datei lädt; jeder Hebel zeigt auf eine Zahl in data/ (ein umbenannter Pfad
// fällt hier auf, nicht erst im Bericht).
func TestRichtwerteLaden(t *testing.T) {
	h := testHints(t)
	for _, k := range figureKeys {
		if _, ok := h.Figures[k]; !ok {
			t.Errorf("Richtwert für %s fehlt", k)
		}
	}
	for measure, l := range h.Levers {
		for _, lv := range append(append([]Lever{}, l.Low...), l.High...) {
			if _, err := ParseTuneParam(lv.Path + "=0..1"); err != nil {
				t.Errorf("hebel %s: %v", measure, err)
			}
		}
	}
}

// AC-02: Fehler beim Laden nennen die Stelle.
func TestRichtwerteFehler(t *testing.T) {
	cases := map[string]string{
		`{"version":1,"figures":{},"levers":{},"x":1}`:                                                        "unknown field",
		`{"version":2,"figures":{},"levers":{}}`:                                                              "version 2",
		`{"version":1,"figures":{"fehlt":{"name":"a","green":[0,1],"yellow":[0,1]}},"levers":{}}`:            `kennzahl "fehlt"`,
		`{"version":1,"figures":{"idleShare":{"name":"a","green":[0,30],"yellow":[0,20]}},"levers":{}}`:       "yellow muss green",
		`{"version":1,"figures":{"idleShare":{"name":"a","green":[5,1],"yellow":[null,null]}},"levers":{}}`:   "von ≤ bis",
		`{"version":1,"figures":{},"levers":{"fehlt":{"low":[],"high":[]}}}`:                                 `"fehlt" unbekannt`,
		`{"version":1,"figures":{},"levers":{"castleHeld":{"low":[{"path":"a","direction":"+","step":0.1,"label":"x"}]}}}`: "erwartet datei.json",
		`{"version":1,"figures":{},"levers":{"castleHeld":{"low":[{"path":"a.json:b","direction":"*","step":0.1,"label":"x"}]}}}`: "direction",
		`{"version":1,"figures":{},"levers":{"castleHeld":{"low":[{"path":"a.json:b","direction":"+","step":2,"label":"x"}]}}}`: "step",
	}
	for raw, want := range cases {
		if _, err := LoadHints([]byte(raw)); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: Fehler %v, erwartet %q", raw, err, want)
		}
	}
}

// B-632/AC-04: Der HTML-Bericht ist eine Datei ohne externe Ressourcen, mit eingebetteten Spielgrafiken und den Daten.
func TestHTMLEigenstaendig(t *testing.T) {
	in := BuildInsights(fixture(t), nil, testHints(t), Meta{Run: "simtest-run-7"})
	page, err := in.HTML(repoRoot(t))
	if err != nil {
		t.Fatal(err)
	}
	html := string(page)
	external := regexp.MustCompile(`(?i)(src|href)\s*=\s*["']?\s*(https?:)?//|url\(\s*["']?(https?:)?//|@import|<link\b`)
	if m := external.FindString(html); m != "" {
		t.Errorf("externe Ressource: %q", m)
	}
	for _, key := range []string{"castle", "wall", "tower", "coin", "enemy"} {
		if !strings.Contains(html, `"`+key+`":"data:image/png;base64,`) {
			t.Errorf("Grafik %s fehlt", key)
		}
	}
	for _, ph := range []string{"__CSS__", "__JS__", "__DATA__", "__ASSETS__", "__TITLE__"} {
		if strings.Contains(html, ph) {
			t.Errorf("Platzhalter %s nicht ersetzt", ph)
		}
	}
	if strings.Count(html, "</script>") != 3 || !strings.Contains(html, "Hub-Stufe 2") {
		t.Error("Skript-Blöcke oder Daten fehlen")
	}
}

// AC-04 Fehlerfall: Ohne public/ fehlen die Grafiken (Platzhalter im Bericht), der Bericht entsteht trotzdem.
func TestHTMLOhneGrafiken(t *testing.T) {
	page, err := BuildInsights(fixture(t), nil, testHints(t), Meta{}).HTML(t.TempDir())
	if err != nil || !strings.Contains(string(page), `id="report-assets">{}</script>`) {
		t.Errorf("ohne Grafiken: %v", err)
	}
}

// AC-06: Für einen vorhandenen Bericht entstehen .md und .html nachträglich.
func TestReevaluate(t *testing.T) {
	dir := filepath.Join(t.TempDir(), "simtest-run-9")
	if err := os.MkdirAll(dir, 0o755); err != nil {
		t.Fatal(err)
	}
	raw, _ := os.ReadFile("testdata/insights-run7.json")
	src := filepath.Join(dir, "balance-20261025-062117.json")
	if err := os.WriteFile(src, raw, 0o644); err != nil {
		t.Fatal(err)
	}
	if _, err := Reevaluate(filepath.Join(dir, "tuning-1.json"), repoRoot(t)); err == nil {
		t.Error("tuning-*.json darf nicht ausgewertet werden")
	}
	page, err := Reevaluate(src, repoRoot(t))
	if err != nil {
		t.Fatal(err)
	}
	html, _ := os.ReadFile(page)
	md, _ := os.ReadFile(filepath.Join(dir, "balance-20261025-062117.md"))
	if !strings.Contains(string(html), `"run":"simtest-run-9"`) || !strings.Contains(string(html), `"generated":"25.10.2026 06:21"`) ||
		!strings.Contains(string(md), "## Kurzauswertung") {
		t.Error("Auswertung nicht geschrieben")
	}
}

// repoRoot: Wurzel des Checkouts (hier liegen public/ und testdata/).
func repoRoot(t *testing.T) string {
	t.Helper()
	root, err := filepath.Abs("../../../..")
	if err != nil {
		t.Fatal(err)
	}
	return root
}
