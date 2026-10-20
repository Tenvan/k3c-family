package backend

import (
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"

	"k3c/engine/room"
)

// WZ9.2 (B-462/AC-03, AC-04, AC-08): Abweichung der Regeltabelle (auf einer Kopie) und Live-Vorschau des Planer-Stands.

const armory4, armory3 = `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 4 }`, `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 3 }`

func readRepo(t *testing.T, rel string) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(repoRoot, filepath.FromSlash(rel)))
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

// planerArmoryFrom3 ist data/hub.json mit der Rüstkammer ab Hub-Stufe 3 statt 4 (Beispiel 2 aus B-462), nur im Speicher.
func planerArmoryFrom3(t *testing.T) string {
	t.Helper()
	raw := readRepo(t, hubFile)
	if strings.Count(raw, armory4) != 1 {
		t.Fatalf("data/hub.json enthält %s nicht genau einmal; Test an die Datei anpassen", armory4)
	}
	return strings.Replace(raw, armory4, armory3, 1)
}

func TestHubRulesOhneAbweichung(t *testing.T) {
	v := (&App{root: repoRoot}).HubRules("")
	if v.Error != "" || !v.Same || len(v.Lines) != 0 || v.Ist != v.Soll {
		t.Fatalf("Regeltabelle weicht ab: %+v", v)
	}
	if v.Path != rulesFile || v.Task != "task gen:rules" {
		t.Errorf("Pfad %q, Task %q", v.Path, v.Task)
	}
}

// AC-03: Eine absichtlich abweichende Tabelle (Kopie) wird mit Zeile, Ist und Soll erkannt.
func TestHubRulesAbweichendeTabelle(t *testing.T) {
	root := t.TempDir()
	doc := readRepo(t, rulesFile)
	const from, to = "| 4 | Eisen | Rüstkammer +40 |", "| 4 | Eisen | Rüstkammer +44 |"
	if strings.Count(doc, from) != 1 {
		t.Fatalf("Regeltabelle enthält %q nicht genau einmal; Test an die Datei anpassen", from)
	}
	write(t, filepath.Join(root, filepath.FromSlash(rulesFile)), strings.Replace(doc, from, to, 1))
	write(t, filepath.Join(root, filepath.FromSlash(hubFile)), readRepo(t, hubFile))
	v := (&App{root: root}).HubRules("")
	if v.Error != "" || v.Same || len(v.Lines) != 1 {
		t.Fatalf("Abweichung nicht erkannt: %+v", v)
	}
	l := v.Lines[0]
	if !strings.Contains(l.Ist, "Rüstkammer +44") || !strings.Contains(l.Soll, "Rüstkammer +40") || l.Line != 7 {
		t.Errorf("Zeile %+v", l)
	}
	if readRepo(t, rulesFile) != doc {
		t.Error("die Regeltabelle des Checkouts wurde geändert")
	}
}

func TestHubRulesPlanerStandUndFehler(t *testing.T) {
	v := (&App{root: repoRoot}).HubRules(planerArmoryFrom3(t))
	if v.Error != "" || v.Same || len(v.Lines) != 2 {
		t.Fatalf("Planer-Stand Rüstkammer ab 3: %+v", v)
	}
	if !strings.Contains(v.Lines[0].Soll, "Rüstkammer +40") || strings.Contains(v.Lines[1].Soll, "Rüstkammer") {
		t.Errorf("Zeilen %+v", v.Lines)
	}
	root := t.TempDir()
	if v := (&App{root: root}).HubRules(""); !strings.Contains(v.Error, hubFile) {
		t.Errorf("fehlende hub.json: %q", v.Error)
	}
	if v := (&App{root: root}).HubRules(readRepo(t, hubFile)); !strings.Contains(v.Error, rulesFile) {
		t.Errorf("fehlende Regeltabelle: %q", v.Error)
	}
	write(t, filepath.Join(root, filepath.FromSlash(rulesFile)), "ohne Marken")
	if v := (&App{root: root}).HubRules(readRepo(t, hubFile)); !strings.Contains(v.Error, "nicht vergleichbar") {
		t.Errorf("ohne Marken: %q", v.Error)
	}
}

func builtSites(t *testing.T, v WorldPreviewView, kind string) int {
	t.Helper()
	if v.Error != "" {
		t.Fatal(v.Error)
	}
	n := 0
	for _, s := range v.State["sites"].([]any) {
		site := s.(map[string]any)
		if site["kind"] == kind && site["state"] == "built" {
			n++
		}
	}
	return n
}

// AC-04: Die Live-Vorschau folgt dem Planer-Stand (ungespeichert) und der gewählten Hub-Stufe.
func TestWorldPreviewPlanerStand(t *testing.T) {
	a := &App{}
	args := previewArgs()
	args.HubLevel = 3
	if n := builtSites(t, a.WorldPreview(args), "armory"); n != 0 {
		t.Errorf("ohne Kandidat: Rüstkammer %d in Hub-Stufe 3", n)
	}
	args.HubCandidate = planerArmoryFrom3(t)
	if n := builtSites(t, a.WorldPreview(args), "armory"); n != 1 {
		t.Errorf("mit Kandidat: Rüstkammer %d in Hub-Stufe 3, erwartet 1", n)
	}
	args.HubLevel = 2
	if n := builtSites(t, a.WorldPreview(args), "armory"); n != 0 {
		t.Errorf("mit Kandidat: Rüstkammer %d in Hub-Stufe 2", n)
	}
	args.HubCandidate = strings.Replace(args.HubCandidate, `"kind": "workshop"`, `"kind": "smithy"`, 1)
	if v := a.WorldPreview(args); !strings.Contains(v.Error, "Befunden") || v.State != nil {
		t.Errorf("ungültiger Kandidat: %q", v.Error)
	}
}

// AC-08: Mit der 7-Stufen-Fixture als Planer-Stand ist Hub-Stufe 7 wählbar, 8 nicht.
func TestWorldPreviewSiebenStufen(t *testing.T) {
	args := previewArgs()
	args.HubCandidate = readRepo(t, "testdata/hub/hub7.json")
	args.HubLevel = 7
	v := (&App{}).WorldPreview(args)
	if builtSites(t, v, "armory") != 1 || v.State["hubLevel"] != float64(7) {
		t.Errorf("Hub-Stufe 7: %v", v.State["hubLevel"])
	}
	args.HubLevel = 8
	if v := (&App{}).WorldPreview(args); v.Error == "" {
		t.Error("Hub-Stufe 8 mit 7 Stufen ohne Fehler")
	}
	keys := []string{}
	for _, r := range hubPlanFrom(repoRoot, "testdata/hub/hub7.json").Matrix.Rows {
		keys = append(keys, r.Key)
	}
	if !slices.Contains(keys, room.PreviewSiteKey("armory", 40)) {
		t.Errorf("Schlüssel der Matrix passen nicht zur Vorschau: %v", keys)
	}
}
