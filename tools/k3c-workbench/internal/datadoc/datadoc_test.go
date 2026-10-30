package datadoc

import (
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strings"
	"testing"
)

// root ist die Repo-Wurzel (Tests laufen im Paketordner).
const root = "../../../.."

// WZ28.10/AC-11: Die Liste nennt jede Datei, die engine/ lädt (Dateiname als Literal, Ordner biomes/ und worlds/).
func TestListeWieLoader(t *testing.T) {
	lit := regexp.MustCompile(`"([a-z-]+\.json)"`)
	loaded := map[string]bool{"biomes/*.json": true, "worlds/*.json": true}
	err := filepath.WalkDir(filepath.Join(root, "engine"), func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() || !strings.HasSuffix(p, ".go") || strings.HasSuffix(p, "_test.go") {
			return err
		}
		src, err := os.ReadFile(p)
		for _, m := range lit.FindAllSubmatch(src, -1) {
			loaded[string(m[1])] = true
		}
		return err
	})
	if err != nil {
		t.Fatal(err)
	}
	for name := range loaded {
		if !slices.ContainsFunc(files, func(e entry) bool { return e.Name == name }) {
			t.Errorf("engine/ lädt %s, die Liste kennt die Datei nicht", name)
		}
	}
	for _, e := range files {
		if !loaded[e.Name] && !slices.Contains(e.ReadBy, ByTester) {
			t.Errorf("%s steht in der Liste, engine/ lädt die Datei nicht", e.Name)
		}
		if m, _ := filepath.Glob(filepath.Join(root, "data", e.Name)); len(m) == 0 {
			t.Errorf("%s fehlt in data/", e.Name)
		}
	}
}

// WZ28.10/AC-11: Doc liest nur docs/data/<name>.md der Liste.
func TestDocPfadpruefung(t *testing.T) {
	for _, bad := range []string{"", "../README", "economy.md", "Economy", "sprites", "..", "economy/../hub", `..\x`} {
		if _, err := Doc(root, bad); err == nil {
			t.Errorf("Doc(%q): Fehler erwartet", bad)
		}
	}
	dir := t.TempDir()
	if _, err := Doc(dir, "economy"); err == nil || !strings.Contains(err.Error(), "fehlt") {
		t.Errorf("fehlende Beschreibung: %v", err)
	}
	if err := os.MkdirAll(filepath.Join(dir, "docs", "data"), 0o755); err != nil {
		t.Fatal(err)
	}
	md := "# economy.json\n\n- **Zweck:** Börse und Steuer.\n- **Wichtigste Werte:** `purse.startGold`\n"
	if err := os.WriteFile(filepath.Join(dir, "docs", "data", "economy.md"), []byte(md), 0o644); err != nil {
		t.Fatal(err)
	}
	list := List(dir)
	if len(list) != len(files) || list[0].Purpose != "Börse und Steuer." || list[0].Key != "`purse.startGold`" || list[0].Missing || !list[1].Missing {
		t.Errorf("List: %+v", list[:2])
	}
}

// WZ28.10/AC-11: Values liefert Zahlen mit Pfad in Dateireihenfolge, nur für variierbare Dateien.
func TestValues(t *testing.T) {
	dir := t.TempDir()
	if err := os.MkdirAll(filepath.Join(dir, "data"), 0o755); err != nil {
		t.Fatal(err)
	}
	raw := `{"version":1,"purse":{"startGold":100,"name":"x"},"levels":[{"cost":2.5},{"cost":3}]}`
	if err := os.WriteFile(filepath.Join(dir, "data", "economy.json"), []byte(raw), 0o644); err != nil {
		t.Fatal(err)
	}
	got, err := Values(dir, "economy.json")
	want := []Value{{"version", 1}, {"purse.startGold", 100}, {"levels.0.cost", 2.5}, {"levels.1.cost", 3}}
	if err != nil || !slices.Equal(got, want) {
		t.Errorf("Values = %v, %v", got, err)
	}
	for _, bad := range []string{"biomes/*.json", "balance-targets.json", "../x.json", "sprites.json", "guards.json"} {
		if _, err := Values(dir, bad); err == nil {
			t.Errorf("Values(%q): Fehler erwartet", bad)
		}
	}
}

// WZ28.10/AC-11: Als variierbar gelten genau die Dateien, die sim.UseData neu lädt (nur dort wirken Varianten).
func TestVaryWieUseData(t *testing.T) {
	src, err := os.ReadFile(filepath.Join(root, "engine", "sim", "data.go"))
	if err != nil {
		t.Fatal(err)
	}
	body := regexp.MustCompile(`(?s)func UseData\(.*?\n}`).Find(src)
	if body == nil {
		t.Fatal("engine/sim/data.go: func UseData fehlt")
	}
	want := map[string]bool{}
	for _, m := range regexp.MustCompile(`loadFrom\(fsys, "([a-z-]+\.json)"`).FindAllSubmatch(body, -1) {
		want[string(m[1])] = true
	}
	if strings.Contains(string(body), "loadEnemies(") {
		want["enemies.json"] = true
	}
	for _, e := range files {
		if e.Vary != want[e.Name] {
			t.Errorf("%s: Vary = %v, sim.UseData lädt sie %v", e.Name, e.Vary, want[e.Name])
		}
	}
}

// B-662/AC-05: AllValues liefert die Werte aller variierbaren Dateien in einem Aufruf, mit Datei und Pfad.
func TestAllValues(t *testing.T) {
	got, err := AllValues(root)
	if err != nil {
		t.Fatal(err)
	}
	seen := map[string]int{}
	for _, v := range got {
		seen[v.File]++
	}
	for _, e := range files {
		if e.Vary != (seen[e.Name] > 0) {
			t.Errorf("%s (vary %v): %d Werte", e.Name, e.Vary, seen[e.Name])
		}
	}
	eco, err := Values(root, "economy.json")
	if err != nil || len(eco) != seen["economy.json"] {
		t.Fatalf("economy.json: %d Werte in AllValues, %d in Values (%v)", seen["economy.json"], len(eco), err)
	}
	if i := slices.IndexFunc(got, func(v FileValue) bool { return v.File == "economy.json" }); got[i].Path != eco[0].Path || got[i].Value != eco[0].Value {
		t.Errorf("erster Wert: %+v, Values: %+v", got[i], eco[0])
	}
	if _, err := AllValues(t.TempDir()); err == nil {
		t.Error("fehlende Dateien ohne Fehler")
	}
}
