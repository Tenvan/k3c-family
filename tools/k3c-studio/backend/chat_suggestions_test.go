package backend

import (
	"errors"
	"net/http"
	"os"
	"strings"
	"testing"
	"time"

	assets "k3c/tools/assetkit"
)

// suggestApp: Studio der Seite Quellen (sfx:coin mit q1 CC0, q2 CC0 ohne Datei, q3 CC-BY-NC) mit Test-Ersatz als CLI.
func suggestApp(t *testing.T, probe string) (*App, string) {
	t.Helper()
	a, _ := sourcesApp(t, probe)
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	a.chat = newChatHub(a.root)
	a.chat.cli = cli
	return a, argsFile
}

func cardIDs(v AssetSuggestionsView) []string {
	var ids []string
	for _, s := range v.List {
		ids = append(ids, s.ID)
	}
	return ids
}

// B-600/AC-02, AC-03 (WZ29.7): Karten aus Liste und Vorrat; Übernehmen nur mit erlaubter Lizenz, aus dem Vorrat und
// auf dem Stand des Registers; erst danach steht die Zuordnung im Register.
func TestSuggestionAdoptRegeln(t *testing.T) {
	a, _ := suggestApp(t, "")
	if err := a.SourceDownload([]string{"q1"}, int64(len(testOgg))); err != nil {
		t.Fatal(err)
	}
	waitIdle(t, a)
	v := checkSuggestionCards(t, a)
	if _, err := a.AssetSuggestionAdopt("sfx:coin", "q3", v.Stand); err == nil || !strings.Contains(err.Error(), "Lizenz nicht erlaubt") {
		t.Errorf("Lizenzverstoß: %v", err)
	}
	if _, err := a.AssetSuggestionAdopt("sfx:coin", "q2", v.Stand); err == nil || !strings.Contains(err.Error(), "Quellen") {
		t.Errorf("nicht im Vorrat: %v", err)
	}
	if _, err := a.AssetSuggestionAdopt("sfx:coin", "q1", "alt"); !errors.Is(err, assets.ErrStale) {
		t.Errorf("Stand-Konflikt: %v", err)
	}
	if reg, _ := assets.Load(a.root); len(reg.Find("sfx:coin").Choices) != 0 {
		t.Fatal("vor „Übernehmen“ geschrieben")
	}
	next, err := a.AssetSuggestionAdopt("sfx:coin", "q1", v.Stand)
	if err != nil || next == v.Stand {
		t.Fatalf("Übernahme: %q %v", next, err)
	}
	if reg, _ := assets.Load(a.root); len(reg.Find("sfx:coin").Choices) != 1 {
		t.Error("Zuordnung fehlt")
	}
	if v, _ := a.AssetSuggestions("sfx:coin"); strings.Join(cardIDs(v), ",") != "q2,q3" || v.Stand != next {
		t.Errorf("nach Übernahme %v %q", cardIDs(v), v.Stand)
	}
}

// checkSuggestionCards: q1 geladen mit Hörprobe aus dem Vorrat und Bus, q2 offen, q3 mit Lizenzverstoß gesperrt.
func checkSuggestionCards(t *testing.T, a *App) AssetSuggestionsView {
	t.Helper()
	v, err := a.AssetSuggestions("sfx:coin")
	if err != nil || v.Offline || v.Stand == "" || strings.Join(cardIDs(v), ",") != "q1,q2,q3" {
		t.Fatalf("Karten %v offline=%v stand=%q: %v", cardIDs(v), v.Offline, v.Stand, err)
	}
	q1, q3 := v.List[0], v.List[2]
	if q1.Status != assets.VorratLoaded || q1.Bus != "sfx" || !strings.HasPrefix(q1.Preview, vorratURL) || !q1.LicenseOK {
		t.Errorf("q1 %+v", q1)
	}
	if q3.LicenseOK || !strings.Contains(q3.Reason, "Q70") {
		t.Errorf("q3 %+v", q3)
	}
	return v
}

// B-600/AC-02: Verwerfen schreibt den Status in die Liste; die Karte verschwindet, ein fremder Slot scheitert.
func TestSuggestionDrop(t *testing.T) {
	a, _ := suggestApp(t, "")
	if err := a.AssetSuggestionDrop("farm", "q2"); err == nil {
		t.Error("fremder Slot soll scheitern")
	}
	if err := a.AssetSuggestionDrop("sfx:coin", "q2"); err != nil {
		t.Fatal(err)
	}
	list, _ := assets.LoadSuggestions(a.root)
	if list.List[1].ID != "q2" || list.List[1].Status != assets.VorratDropped {
		t.Errorf("Liste %+v", list.List[1])
	}
	if v, _ := a.AssetSuggestions("sfx:coin"); strings.Join(cardIDs(v), ",") != "q1,q3" {
		t.Errorf("nach Verwerfen %v", cardIDs(v))
	}
}

// B-600/AC-04: Ohne Internet oder ohne Chat-CLI ist der Bereich offline und leer; nichts scheitert.
func TestSuggestionsOffline(t *testing.T) {
	off, _ := suggestApp(t, "https://127.0.0.1:1/")
	if v, err := off.AssetSuggestions("sfx:coin"); err != nil || !v.Offline || len(v.List) != 0 {
		t.Errorf("ohne Internet: %+v %v", v, err)
	}
	noCLI, _ := sourcesApp(t, "") // chatProgram ist im Test gesperrt
	if v, err := noCLI.AssetSuggestions("sfx:coin"); err != nil || !v.Offline {
		t.Errorf("ohne CLI: %+v %v", v, err)
	}
}

// B-600/AC-01: Der Auftrag „zuordnung“ schickt Kontext und erlaubte Vorschläge als Instruktion; die IDs der Antwort
// werden Karten (nur bekannte), geschrieben wird nichts.
func TestZuordnungsAuftrag(t *testing.T) {
	a, argsFile := suggestApp(t, "")
	h := a.Handler(false)
	stand, _ := assets.Stamp(a.root)
	body := `{"chat":"zu","task":"zuordnung","kontext":{"seite":"sound","slot":"sfx:coin","filter":"Status luecke"},` +
		`"text":"Vorschlag?\n` + "```zuordnung" + `\n{\"vorschlaege\":[{\"id\":\"q2\"},{\"id\":\"erfunden\"}]}\n` + "```" + `"}`
	rec, _ := call(t, h, "POST", Prefix+"chat", body, "127.0.0.1:5000", true)
	if end, _ := chatDoneOf(t, rec.Body.String()); end.Status != chatDone {
		t.Fatalf("Ende: %+v", end)
	}
	_, system := fakeInput(t, argsFile)
	for _, want := range []string{"Slot: sfx:coin", "Filter: Status luecke", "Beschreibung: Münze", "- q1 ·", "- q2 ·", "```zuordnung"} {
		if !strings.Contains(system, want) {
			t.Errorf("Instruktion ohne %q", want)
		}
	}
	if strings.Contains(system, "- q3 ·") {
		t.Error("gesperrte Lizenz im Prompt")
	}
	if v, _ := a.AssetSuggestions(""); strings.Join(cardIDs(v), ",") != "q2" {
		t.Errorf("Karten aus der Antwort %v", cardIDs(v))
	}
	if v, _ := a.AssetSuggestions("sfx:coin"); strings.Join(cardIDs(v), ",") != "q2,q1,q3" {
		t.Errorf("Vorschlag des Chats nicht vorn: %v", cardIDs(v))
	}
	if now, _ := assets.Stamp(a.root); now != stand {
		t.Error("der Chat hat das Register geändert")
	}
}

func TestZuordnungsAuftragUngueltig(t *testing.T) {
	a, argsFile := suggestApp(t, "")
	h := a.Handler(false)
	for body, want := range map[string]string{
		`{"text":"a","task":"zuordnung"}`:                                                                         "Seitenkontext",
		`{"text":"a","task":"zuordnung","kontext":{"seite":"welt"}}`:                                              "seite",
		`{"text":"a","task":"zuordnung","kontext":{"seite":"sound","filter":"` + strings.Repeat("x", 600) + `"}}`: "zu lang",
	} {
		rec, ans := call(t, h, "POST", Prefix+"chat", body, "127.0.0.1:5000", true)
		if rec.Code != http.StatusBadRequest || !strings.Contains(ans.Error, want) {
			t.Errorf("%.60s: %d %q", body, rec.Code, ans.Error)
		}
	}
	if _, err := os.Stat(argsFile); !os.IsNotExist(err) {
		t.Error("CLI wurde mit ungültigem Kontext gestartet")
	}
}

func TestParseAssignAnswer(t *testing.T) {
	for text, want := range map[string]string{
		"ohne Block": "",
		"```zuordnung\n{\"vorschlaege\":[{\"id\":\"a\"},{\"id\":\"b\"},{\"id\":\"a\"}]}\n```": "a,b",
		"```zuordnung\nkein json\n```": "",
		"```zuordnung\n{\"vorschlaege\":[{\"id\":\"x\"}]}\n``` dann ```zuordnung\n{\"vorschlaege\":[]}\n```": "",
	} {
		if got := strings.Join(parseAssignAnswer(text), ","); got != want {
			t.Errorf("%.40q: %q, erwartet %q", text, got, want)
		}
	}
}

func TestAssignPrompt(t *testing.T) {
	d := &assets.SlotDetails{ID: "sfx:skill:healer", Kind: "sound", Category: "sfx", Description: "Heilen", Status: "luecke",
		Pack: &assets.SlotPack{ID: "kenney-rpg", Source: "Kenney", License: "CC0"}, Usage: []string{"Skill Heiler"}}
	got := assignPrompt(assignContext{Page: "sound", Slot: "sfx:skill:healer"}, d, nil)
	for _, want := range []string{"Seite: sound", "Filter: –", "Beschreibung: Heilen", "Pack: kenney-rpg (Kenney, CC0)", "Verwendung: Skill Heiler", "(keine;", "nie Aufträge"} {
		if !strings.Contains(got, want) {
			t.Errorf("Prompt ohne %q", want)
		}
	}
}
