package backend

import (
	"os"
	"strings"
	"testing"
)

// WZ19.3 (B-488/AC-08), WZ21.2 (B-516): Grundgerüst aus den Daten und Prüfung der Antwort des Modells. Kein echter Modellaufruf;
// geschrieben wird nichts (Kopien in t.TempDir()).

func TestPromptEnthaeltDieDaten(t *testing.T) {
	p := worldInstruction()
	for _, want := range []string{
		`"depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 }`, // Beispiel aus der Standard-Welt
		"Tiefe 0: Oberwelt (Wald)", "Tiefe 4:", // alle Tiefen mit Biom
		"hier darf eine Insel enden",
		"clearing", "tunnel", "crystals", // Chunk-Arten
		"tree", "stoneVein", // Objekt-Arten
		"sehr klein: 16", "mega groß: 160", // Größen-Tabelle
		"Prüfregeln", "kein Endboss in der tiefsten Stufe 2",
	} {
		if !strings.Contains(p, want) {
			t.Errorf("Prompt ohne %q", want)
		}
	}
	if strings.Contains(p, "MEINE WÜNSCHE") { // Instruktion, kein Text zum Ergänzen (B-516)
		t.Errorf("Instruktion mit MEINE WÜNSCHE:, Ende: %q", p[len(p)-40:])
	}
}

func TestPromptNurTiefe4HatEndboss(t *testing.T) {
	p := worldInstruction()
	if strings.Count(p, "hier darf eine Insel enden") != 1 {
		t.Errorf("genau eine Tiefe darf enden:\n%s", p)
	}
}

const answerWorld = `{ "id": "nordmeer", "name": "Nordmeer", "notes": "Entwurf",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2, 3, 4], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 } },
    { "id": "eisinsel", "name": "Eisinsel", "depths": [0, 4], "depthScaling": { "hp": 1.6, "damage": 1.4, "speed": 1.15 },
      "plannedChunks": { "0": [ { "slot": -3, "kind": "clearing", "seed": "lichtung" } ] }, "chunkCounts": { "0": 24 } }
  ] }`

func TestAntwortGueltigeWeltWirdEntwurf(t *testing.T) {
	a, path := worldApp(t)
	before, _ := os.ReadFile(path)
	for name, text := range map[string]string{
		"roh":       answerWorld,
		"Code-Block": "Hier ist deine Welt:\n```json\n" + answerWorld + "\n```\nViel Spaß!",
		"mit Text":  "Gern! " + answerWorld + " Sag Bescheid.",
	} {
		v := a.PlanWorldAnswer(text)
		if !v.OK || v.Error != "" || v.Draft == nil {
			t.Fatalf("%s: %+v", name, v)
		}
		if v.Draft.Name != "Nordmeer" || len(v.Draft.Islands) != 2 || v.Draft.Islands[1].ID != "eisinsel" || v.Draft.Islands[1].From != "" {
			t.Errorf("%s: Entwurf %+v", name, v.Draft)
		}
		if string(v.Draft.Islands[1].ChunkCounts) == "" || string(v.Draft.Islands[1].PlannedChunks) == "" {
			t.Errorf("%s: Planung und Größe gehen nicht mit", name)
		}
	}
	if after, _ := os.ReadFile(path); string(after) != string(before) {
		t.Error("die Prüfung hat die Welt-Datei geändert")
	}
}

func TestAntwortMitFehlerBleibtEntwurfMitGrund(t *testing.T) {
	a, _ := worldApp(t)
	bad := strings.Replace(answerWorld, `"depths": [0, 4]`, `"depths": [0, 2]`, 1)
	v := a.PlanWorldAnswer(bad)
	if v.OK || v.Draft == nil {
		t.Fatalf("erwartet Entwurf mit Fehler: %+v", v)
	}
	if !strings.Contains(v.Error, "Insel eisinsel: kein Endboss in der tiefsten Stufe 2") || !strings.Contains(v.Error, "nordmeer.json") {
		t.Errorf("Grund nennt Welt, Insel und Tiefe nicht: %q", v.Error)
	}
}

func TestAntwortUnlesbarOderUnbekanntesFeldHatKeinenEntwurf(t *testing.T) {
	a, _ := worldApp(t)
	cases := map[string]string{
		"kein JSON":        "Ich kann dir leider keine Welt geben.",
		"JSON kaputt":      `{"id": "x", "name": `,
		"unbekanntes Feld": strings.Replace(answerWorld, `"notes"`, `"size": 3, "notes"`, 1),
	}
	for name, text := range cases {
		v := a.PlanWorldAnswer(text)
		if v.OK || v.Draft != nil || !strings.HasPrefix(v.Error, "Antwort nicht lesbar") {
			t.Errorf("%s: %+v", name, v)
		}
	}
	if v := a.PlanWorldAnswer(cases["unbekanntes Feld"]); !strings.Contains(v.Error, `"size"`) {
		t.Errorf("unbekanntes Feld nicht genannt: %q", v.Error)
	}
}

// Der Entwurf aus der Antwort lässt sich wie jeder Entwurf prüfen; die Planung der Antwort steht in der Vorschau.
func TestAntwortEntwurfLaesstSichPruefen(t *testing.T) {
	a, path := worldApp(t)
	before, _ := os.ReadFile(path)
	d := *a.PlanWorldAnswer(answerWorld).Draft
	check := a.PlanWorldCheck(d)
	if !check.OK {
		t.Fatalf("Prüfung: %+v", check)
	}
	for _, want := range []string{`"eisinsel"`, `"clearing"`, `"chunkCounts"`} {
		if !strings.Contains(check.Preview, want) {
			t.Errorf("Vorschau ohne %s:\n%s", want, check.Preview)
		}
	}
	if after, _ := os.ReadFile(path); string(after) != string(before) {
		t.Error("PlanWorldCheck hat geschrieben")
	}
}

// B-514: Eine Insel der Antwort mit einer id aus der Datei und ohne Planfelder behält beim Speichern ihre Planung.
func TestAntwortBehaeltPlanungBestehenderInsel(t *testing.T) {
	a, path := worldApp(t)
	d := *a.PlanWorldAnswer(answerWorld).Draft
	if d.Islands[0].From != "island1" || d.Islands[1].From != "" {
		t.Fatalf("From: %q / %q", d.Islands[0].From, d.Islands[1].From)
	}
	stand := planStand(mustRead(t, path))
	if s := a.PlanWorldSave(d, stand); s.Error != "" {
		t.Fatal(s.Error)
	}
	got := string(mustRead(t, path))
	for _, want := range []string{`"kind": "clearing"`, `"0": 24`, `"eisinsel"`} {
		if !strings.Contains(got, want) {
			t.Errorf("Datei ohne %s:\n%s", want, got)
		}
	}
	if strings.Contains(got, "island2") {
		t.Error("island2 steht nicht in der Antwort und entfällt")
	}
}

func mustRead(t *testing.T, path string) []byte {
	t.Helper()
	b, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	return b
}
