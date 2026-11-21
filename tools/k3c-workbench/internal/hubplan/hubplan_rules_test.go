package hubplan

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// WZ9.2 (B-462/AC-03, AC-04, AC-08): Abweichung der Regeltabelle (auf einer Kopie) und Live-Vorschau des Planer-Stands.

const armory4, armory3 = `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 4, "topOnly": true }`, `{ "kind": "armory", "offsetUnits": 40, "hubLevel": 3, "topOnly": true }`

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
	v := editor(repoRoot).Rules("")
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
	v := editor(root).Rules("")
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
	v := editor(repoRoot).Rules(planerArmoryFrom3(t))
	if v.Error != "" || v.Same || len(v.Lines) != 2 {
		t.Fatalf("Planer-Stand Rüstkammer ab 3: %+v", v)
	}
	if !strings.Contains(v.Lines[0].Soll, "Rüstkammer +40") || strings.Contains(v.Lines[1].Soll, "Rüstkammer") {
		t.Errorf("Zeilen %+v", v.Lines)
	}
	root := t.TempDir()
	if v := editor(root).Rules(""); !strings.Contains(v.Error, hubFile) {
		t.Errorf("fehlende hub.json: %q", v.Error)
	}
	if v := editor(root).Rules(readRepo(t, hubFile)); !strings.Contains(v.Error, rulesFile) {
		t.Errorf("fehlende Regeltabelle: %q", v.Error)
	}
	write(t, filepath.Join(root, filepath.FromSlash(rulesFile)), "ohne Marken")
	if v := editor(root).Rules(readRepo(t, hubFile)); !strings.Contains(v.Error, "nicht vergleichbar") {
		t.Errorf("ohne Marken: %q", v.Error)
	}
}
