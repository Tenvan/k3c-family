package paperassets

import (
	"crypto/sha256"
	"encoding/hex"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// B-707: Nach set/create/render steht der neue renderKey in golden.json der Gruppe, im Format von gen:paper;
// andere Einträge und andere Gruppen bleiben unberührt.
func TestGoldenEintragen(t *testing.T) {
	t.Parallel()
	root, _ := testRepo(t)
	schreibe(t, root, "public/grafik/gebaeude/mauer/manifest.json",
		`{"id": "mauer", "renderKey": "neu", "animationen": {"stand": {"frames": 1, "datei": "stand.png"}}}`)
	schreibe(t, root, "public/grafik/gebaeude/mauer/stand.png", "png")
	schreibe(t, root, "public/grafik/gebaeude/golden.json",
		"{\n  \"mauer\": {\n    \"renderKey\": \"alt\",\n    \"dateien\": {\n      \"stand.png\": \"x\"\n    }\n  },\n"+
			"  \"turm\": {\n    \"renderKey\": \"t\",\n    \"dateien\": {}\n  }\n}\n")
	fremd := "{\n  \"monarch\": {}\n}\n"
	schreibe(t, root, "public/grafik/figuren/golden.json", fremd)

	a := Asset{ID: "mauer", Rig: "gebaeude", Gruppe: "gebaeude", Pfad: Grafik + "/gebaeude/mauer" + endung}
	if err := goldenEintragen(root, a); err != nil {
		t.Fatal(err)
	}
	h := sha256.Sum256([]byte("png"))
	soll := "{\n  \"mauer\": {\n    \"renderKey\": \"neu\",\n    \"dateien\": {\n      \"stand.png\": \"" + hex.EncodeToString(h[:]) +
		"\"\n    }\n  },\n  \"turm\": {\n    \"renderKey\": \"t\",\n    \"dateien\": {}\n  }\n}\n"
	if b, _ := os.ReadFile(filepath.Join(root, "public/grafik/gebaeude/golden.json")); string(b) != soll {
		t.Errorf("golden.json:\n%s\nsoll:\n%s", b, soll)
	}
	if b, _ := os.ReadFile(filepath.Join(root, "public/grafik/figuren/golden.json")); string(b) != fremd {
		t.Errorf("fremde Gruppe angefasst: %s", b)
	}
}

// Ohne golden.json legt das erste Asset sie an.
func TestGoldenEintragenNeu(t *testing.T) {
	t.Parallel()
	root, _ := testRepo(t)
	schreibe(t, root, "public/grafik/gebaeude/mauer/manifest.json",
		`{"id": "mauer", "renderKey": "k", "animationen": {"stand": {"datei": "stand.png"}}}`)
	schreibe(t, root, "public/grafik/gebaeude/mauer/stand.png", "png")
	a := Asset{ID: "mauer", Rig: "gebaeude", Gruppe: "gebaeude", Pfad: Grafik + "/gebaeude/mauer" + endung}
	if err := goldenEintragen(root, a); err != nil {
		t.Fatal(err)
	}
	if b, _ := os.ReadFile(filepath.Join(root, "public/grafik/gebaeude/golden.json")); !strings.Contains(string(b), `"renderKey": "k"`) {
		t.Errorf("golden.json: %s", b)
	}
}

// Löschen nimmt den Eintrag aus golden.json.
func TestLoescheGolden(t *testing.T) {
	t.Parallel()
	root, _ := testRepo(t)
	schreibe(t, root, "public/grafik/gebaeude/mauer/manifest.json", "{}")
	schreibe(t, root, "public/grafik/gebaeude/golden.json",
		"{\n  \"mauer\": {\n    \"renderKey\": \"m\",\n    \"dateien\": {}\n  },\n  \"turm\": {\n    \"renderKey\": \"t\",\n    \"dateien\": {}\n  }\n}\n")
	if _, err := Loesche(root, "mauer"); err != nil {
		t.Fatal(err)
	}
	b, _ := os.ReadFile(filepath.Join(root, "public/grafik/gebaeude/golden.json"))
	if string(b) != "{\n  \"turm\": {\n    \"renderKey\": \"t\",\n    \"dateien\": {}\n  }\n}\n" {
		t.Errorf("golden.json: %s", b)
	}
}

// Jede golden.json im Repo (geschrieben von gen:paper) übersteht Lesen und Schreiben unverändert: gleiches Format.
func TestGoldenFormatWieGen(t *testing.T) {
	t.Parallel()
	dateien, _ := filepath.Glob(filepath.Join(repoWurzel(t), "public", "grafik", "*", goldenName))
	if len(dateien) == 0 {
		t.Skip("keine golden.json im Repo")
	}
	for _, p := range dateien {
		b, _ := os.ReadFile(p)
		b = []byte(strings.ReplaceAll(string(b), "\r\n", "\n"))
		alle, err := goldenLesen(b)
		if err != nil {
			t.Fatalf("%s: %v", p, err)
		}
		if got := goldenText(alle); string(got) != string(b) {
			t.Errorf("%s: Format weicht von gen:paper ab", p)
		}
	}
}
