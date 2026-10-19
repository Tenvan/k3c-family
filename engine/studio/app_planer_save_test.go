package studio

import (
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"

	assets "k3c/tools/assetkit"
	"k3c/tools/ruletable/table"
)

// WZ9.3 (B-462/AC-02, AC-03, AC-05, AC-06): Speichern im Gebäude-Planer, ausschließlich auf Kopien in t.TempDir().
// data/hub.json, die Regeltabelle und testdata/golden des Checkouts werden nur gelesen.

// planerCopy legt hubRel (Repo) als data/hub.json und die Regeltabelle des Checkouts in ein temporäres Verzeichnis.
func planerCopy(t *testing.T, hubRel string) string {
	t.Helper()
	root := t.TempDir()
	write(t, filepath.Join(root, filepath.FromSlash(hubFile)), readRepo(t, hubRel))
	write(t, filepath.Join(root, filepath.FromSlash(rulesFile)), readRepo(t, rulesFile))
	return root
}

func readIn(t *testing.T, root, rel string) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(rel)))
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

// saveEdit lädt den Planer unter root, ändert den Kandidaten mit edit (kompakt, sortiert) und speichert.
func saveEdit(t *testing.T, root string, edit func(m map[string]any)) (HubPlanView, HubSaveResult) {
	t.Helper()
	plan := hubPlanFrom(root, hubFile)
	if plan.Error != "" {
		t.Fatal(plan.Error)
	}
	return plan, hubSaveIn(root, []byte(remarshal(t, plan.Raw, edit)), plan.Stand)
}

// wantSaved prüft: gespeichert, REG-Hinweis, kein Tabellenfehler, neuer Stand, Regeltabelle passt zur hub.json und
// ist außerhalb der Marken byte-gleich zum Stand vorher.
func wantSaved(t *testing.T, root, rulesBefore string, res HubSaveResult) {
	t.Helper()
	if !res.Saved || res.Error != "" || res.Stale != "" || res.RulesError != "" || len(res.Findings) != 0 {
		t.Fatalf("nicht gespeichert: %+v", res)
	}
	if res.Notice != regNotice || !strings.Contains(res.Notice, "task golden:update") {
		t.Errorf("REG-Hinweis %q", res.Notice)
	}
	hub, doc := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	if res.Stand != (HubStand{Hub: stamp([]byte(hub)), Rules: stamp([]byte(doc))}) {
		t.Errorf("Stand nach dem Speichern %+v passt nicht zu den Dateien", res.Stand)
	}
	if same, _, _, err := table.Gegenprobe(doc, []byte(hub)); err != nil || !same {
		t.Errorf("Regeltabelle nicht zur gespeicherten hub.json erzeugt: %v", err)
	}
	from, to := strings.Index(doc, table.Start), strings.Index(doc, table.End)+len(table.End)
	bf, bt := strings.Index(rulesBefore, table.Start), strings.Index(rulesBefore, table.End)+len(table.End)
	if doc[:from] != rulesBefore[:bf] || doc[to:] != rulesBefore[bt:] {
		t.Error("Regeltabelle außerhalb der Marken verändert")
	}
	leftovers(t, root)
}

// leftovers: keine Temp-Datei des atomaren Schreibens bleibt liegen.
func leftovers(t *testing.T, root string) {
	t.Helper()
	for _, dir := range []string{"data", "docs/rules"} {
		entries, err := os.ReadDir(filepath.Join(root, dir))
		if err != nil {
			t.Fatal(err)
		}
		if len(entries) != 1 {
			t.Errorf("%s enthält %d Dateien, erwartet 1 (Temp-Datei liegen geblieben?)", dir, len(entries))
		}
	}
}

// wantOneLine: hub.json unterscheidet sich vom Stand vorher in genau einer Zeile, von from nach to.
func wantOneLine(t *testing.T, before, after, from, to string) {
	t.Helper()
	removed, added := lineDiff(before, after)
	if len(removed) != 1 || len(added) != 1 || strings.TrimSpace(removed[0]) != from || strings.TrimSpace(added[0]) != to {
		t.Fatalf("kein Ein-Zeilen-Diff %q → %q: -%q +%q", from, to, removed, added)
	}
}

// Beispiel B-462: Rüstkammer auf Stufe 3 → eine Zeile in hub.json, neu erzeugter Block in § 3, REG-Hinweis.
func TestHubSaveRuestkammerStufe3(t *testing.T) {
	root := planerCopy(t, hubFile)
	hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	_, res := saveEdit(t, root, func(m map[string]any) { site(m, "sites", 8)["hubLevel"] = 3 })
	wantSaved(t, root, rulesBefore, res)
	wantOneLine(t, hubBefore, readIn(t, root, hubFile), armory4+",", armory3+",")
	if readIn(t, root, rulesFile) == rulesBefore {
		t.Error("Block der Regeltabelle nicht neu geschrieben")
	}
}

func TestHubSaveGateHubLevel(t *testing.T) {
	root := planerCopy(t, hubFile)
	hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	_, res := saveEdit(t, root, func(m map[string]any) { m["wallLines"].(map[string]any)["gateHubLevel"] = 3 })
	wantSaved(t, root, rulesBefore, res)
	wantOneLine(t, hubBefore, readIn(t, root, hubFile), `"gateHubLevel": 2`, `"gateHubLevel": 3`)
}

func TestHubSaveHaendler(t *testing.T) {
	root := planerCopy(t, hubFile)
	hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	_, res := saveEdit(t, root, func(m map[string]any) {
		mer := m["merchant"].(map[string]any)
		mer["buyOffsetUnits"], mer["sellOffsetUnits"] = 20, 16
	})
	wantSaved(t, root, rulesBefore, res)
	wantOneLine(t, hubBefore, readIn(t, root, hubFile),
		`"merchant": { "buyOffsetUnits": 16, "sellOffsetUnits": 20, "onlyDepth": 0 },`,
		`"merchant": { "buyOffsetUnits": 20, "sellOffsetUnits": 16, "onlyDepth": 0 },`)
}

func TestHubSaveWallUnits(t *testing.T) {
	root := planerCopy(t, hubFile)
	hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	_, res := saveEdit(t, root, func(m map[string]any) { m["wallLines"].(map[string]any)["wallUnits"].([]any)[4] = 126 })
	wantSaved(t, root, rulesBefore, res)
	wantOneLine(t, hubBefore, readIn(t, root, hubFile), `"wallUnits": [44, 64, 84, 104, 124],`, `"wallUnits": [44, 64, 84, 104, 126],`)
}

// Platz entfernen (Heilplatz, optional) und wieder hinzufügen: je nur die Zeilen des Platzes und das Komma davor.
func TestHubSavePlatzEntfernenUndHinzufuegen(t *testing.T) {
	root := planerCopy(t, hubFile)
	hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	const healer = `{ "kind": "healer", "offsetUnits": -40, "hubLevel": 3 }`
	_, res := saveEdit(t, root, func(m map[string]any) {
		m["sites"] = slices.Delete(m["sites"].([]any), 4, 5)
	})
	wantSaved(t, root, rulesBefore, res)
	removed, added := lineDiff(hubBefore, readIn(t, root, hubFile))
	if len(removed) != 1 || strings.TrimSpace(removed[0]) != healer+"," || len(added) != 0 {
		t.Fatalf("Entfernen: -%q +%q", removed, added)
	}
	afterRemove, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	plan := hubPlanFrom(root, hubFile)
	const farm = `    { "kind": "farm", "offsetUnits": 52, "hubLevel": 1 }`
	nl := "\n"
	if strings.Contains(plan.Raw, "\r\n") {
		nl = "\r\n"
	}
	if strings.Count(plan.Raw, farm+nl) != 1 {
		t.Fatalf("data/hub.json enthält %s nicht genau einmal; Test an die Datei anpassen", farm)
	}
	cand := strings.Replace(plan.Raw, farm+nl, farm+","+nl+"    "+healer+nl, 1)
	res = hubSaveIn(root, []byte(cand), plan.Stand) // Reihenfolge wie im Frontend (JSON.stringify)
	wantSaved(t, root, rulesBefore, res)
	removed, added = lineDiff(afterRemove, readIn(t, root, hubFile))
	if !slices.Equal(removed, []string{farm}) || !slices.Equal(added, []string{farm + ",", "    " + healer}) {
		t.Fatalf("Hinzufügen: -%q +%q", removed, added)
	}
}

// Ungültige Kandidaten (Pflichtplatz entfernt, Stufe außerhalb, Platz unbekannter Art) werden nicht geschrieben.
func TestHubSaveUngueltigerKandidat(t *testing.T) {
	for name, edit := range map[string]func(m map[string]any){
		"Farm entfernt": func(m map[string]any) { m["sites"] = slices.Delete(m["sites"].([]any), 3, 4) },
		"Stufe 9":       func(m map[string]any) { site(m, "sites", 8)["hubLevel"] = 9 },
		"Tor Stufe 0":   func(m map[string]any) { m["wallLines"].(map[string]any)["gateHubLevel"] = 0 },
		"unbekannte Art": func(m map[string]any) {
			m["sites"] = append(m["sites"].([]any), map[string]any{"kind": "mill", "offsetUnits": 30, "hubLevel": 1})
		},
		"Mauer fallend": func(m map[string]any) { m["wallLines"].(map[string]any)["wallUnits"].([]any)[4] = 90 },
	} {
		t.Run(name, func(t *testing.T) {
			root := planerCopy(t, hubFile)
			hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
			_, res := saveEdit(t, root, edit)
			if res.Saved || len(res.Findings) == 0 || res.Notice != "" {
				t.Fatalf("ungültiger Kandidat gespeichert: %+v", res)
			}
			if readIn(t, root, hubFile) != hubBefore || readIn(t, root, rulesFile) != rulesBefore {
				t.Error("Dateien nicht byte-gleich")
			}
		})
	}
}

// Schreibregel: Hat sich eine der beiden Dateien seit dem Laden geändert, wird nichts geschrieben.
func TestHubSaveExternGeaendert(t *testing.T) {
	for _, rel := range []string{hubFile, rulesFile} {
		t.Run(rel, func(t *testing.T) {
			root := planerCopy(t, hubFile)
			plan := hubPlanFrom(root, hubFile)
			path := filepath.Join(root, filepath.FromSlash(rel))
			write(t, path, readIn(t, root, rel)+"\n")
			hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
			res := hubSaveIn(root, []byte(strings.Replace(plan.Raw, armory4, armory3, 1)), plan.Stand)
			if res.Saved || !strings.Contains(res.Stale, rel) || !strings.Contains(res.Stale, "neu laden") || res.Notice != "" {
				t.Fatalf("extern geänderte Datei überschrieben: %+v", res)
			}
			if readIn(t, root, hubFile) != hubBefore || readIn(t, root, rulesFile) != rulesBefore {
				t.Error("Dateien nicht byte-gleich")
			}
		})
	}
}

// Scheitert der Tabellenschritt (Marken fehlen), bleibt hub.json gespeichert: „Regeltabelle veraltet“.
func TestHubSaveFehlerImTabellenschritt(t *testing.T) {
	root := planerCopy(t, hubFile)
	write(t, filepath.Join(root, filepath.FromSlash(rulesFile)), strings.Replace(readRepo(t, rulesFile), table.Start, "", 1))
	hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	_, res := saveEdit(t, root, func(m map[string]any) { site(m, "sites", 8)["hubLevel"] = 3 })
	if !res.Saved || !strings.HasPrefix(res.RulesError, "Regeltabelle veraltet, task rules:update") || res.Notice != regNotice {
		t.Fatalf("Tabellenfehler nicht gemeldet: %+v", res)
	}
	wantOneLine(t, hubBefore, readIn(t, root, hubFile), armory4+",", armory3+",")
	if readIn(t, root, rulesFile) != rulesBefore {
		t.Error("Regeltabelle trotz Fehler verändert")
	}
}

// Atomar: Scheitert das Umbenennen (Ziel ist ein Ordner), bleibt das Ziel unverändert und keine Temp-Datei liegen.
func TestWriteAtomicHinterlaesstNichts(t *testing.T) {
	dir := t.TempDir()
	target := filepath.Join(dir, "hub.json")
	write(t, filepath.Join(target, "drin.txt"), "x")
	if err := assets.WriteAtomic(target, []byte("{}")); err == nil {
		t.Fatal("Schreiben auf einen Ordner ohne Fehler")
	}
	entries, err := os.ReadDir(dir)
	if err != nil || len(entries) != 1 || !entries[0].IsDir() {
		t.Fatalf("Ordner nach gescheitertem Schreiben: %v %v", entries, err)
	}
}

// 7-Stufen-Fixture (mehrzeiliges Layout): Rüstkammer von 7 auf 5 ist eine Zeile, der Block bekommt 7 Stufen.
func TestHubSaveSiebenStufen(t *testing.T) {
	root := planerCopy(t, "testdata/hub/hub7.json")
	hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	plan, res := saveEdit(t, root, func(m map[string]any) { site(m, "sites", 8)["hubLevel"] = 5 })
	if plan.Matrix.Levels != 7 {
		t.Fatalf("%d Stufen, erwartet 7", plan.Matrix.Levels)
	}
	wantSaved(t, root, rulesBefore, res)
	wantOneLine(t, hubBefore, readIn(t, root, hubFile), `"hubLevel": 7`, `"hubLevel": 5`)
	if doc := readIn(t, root, rulesFile); !strings.Contains(doc, "| 7 |") {
		t.Error("Block ohne Stufe 7")
	}
}

// Kandidat gleich der Datei (andere Form, gleicher Inhalt): „unverändert“, nichts geschrieben, kein REG-Hinweis.
func TestHubSaveUnveraendert(t *testing.T) {
	root := planerCopy(t, hubFile)
	hubBefore, rulesBefore := readIn(t, root, hubFile), readIn(t, root, rulesFile)
	plan, res := saveEdit(t, root, func(map[string]any) {})
	if res.Saved || !res.Unchanged || res.Notice != "" || res.Error != "" || res.Stand != plan.Stand {
		t.Fatalf("unveränderter Kandidat: %+v", res)
	}
	if readIn(t, root, hubFile) != hubBefore || readIn(t, root, rulesFile) != rulesBefore {
		t.Error("Dateien nicht byte-gleich")
	}
}
