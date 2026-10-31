package backend

import (
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"

	assets "k3c/tools/assetkit"
)

// block ist eine Antwort mit Aktionsblock (B-679).
func block(json string) string {
	return "Gern.\n\n" + actionsFence + "\n{\"aktionen\": [" + json + "]}\n```\n"
}

// cards bereitet die Aktionen der Antwort für den Chat c vor und liefert dessen Karten.
func cards(t *testing.T, a *App, c, answer string) []ChatAction {
	t.Helper()
	if note := a.applyActions(c, answer); !strings.Contains(note, "Karten") {
		t.Fatalf("Hinweis %q", note)
	}
	return a.ChatActions(c)
}

// B-679/AC-03: Block lesen, unbekannte Art und Felder sperren mit Grund; ohne Block keine Karten und kein Hinweis.
func TestChatAktionenLesenUndSperren(t *testing.T) {
	a := &App{root: t.TempDir()}
	if a.applyActions("c", "nur Text") != "" || len(a.ChatActions("c")) != 0 {
		t.Fatal("ohne Block keine Karten")
	}
	if note := a.applyActions("c", actionsFence+"\n{kaputt\n```"); !strings.Contains(note, "nicht lesbar") {
		t.Errorf("kaputter Block: %q", note)
	}
	list := cards(t, a, "c", block(`{"art": "zaubern"}, {"art": "netz", "modus": "auto", "extra": 1}, {"art": "netz", "modus": "nie"}`))
	if len(list) != 3 || !strings.Contains(list[0].Block, "unbekannte Art") || !strings.Contains(list[1].Block, "extra") ||
		!strings.Contains(list[2].Block, "unbekannt") {
		t.Fatalf("Karten %+v", list)
	}
	if _, err := a.ChatActionRun("c", list[0].ID); err == nil {
		t.Error("gesperrte Karte lief")
	}
	if got := withoutFence("A"+block(`{}`)+"B", actionsFence); strings.Contains(got, "aktionen") {
		t.Errorf("Block nicht entfernt: %q", got)
	}
}

// B-679/AC-05: Der Katalog deckt die Lücken L2–L10 ab, jede Art hat Felder, Zweck und Prüfung, keine doppelt.
func TestChatAktionenKatalog(t *testing.T) {
	want := []string{"gebaeude_aendern", "zuordnung_setzen", "kandidat_zuordnen", "recherche", "lizenz_bestaetigen", "laden",
		"vorrat_uebernehmen", "aufraeumen", "netz", "welt_waehlen", "welt_anlegen", "welt_kopieren", "welt_loeschen",
		"welt_speichern", "chunks_planen", "testspielstand_anlegen", "testspielstand_loeschen", "ticket",
		"vorschlag_uebernehmen", "vorschlag_verwerfen"}
	var arts []string
	for _, s := range actionCatalog() {
		if s.art == "" || s.fields == "" || s.help == "" || s.prepare == nil || slices.Contains(arts, s.art) {
			t.Errorf("Art %+v", s.art)
		}
		arts = append(arts, s.art)
	}
	for _, w := range want {
		if !slices.Contains(arts, w) {
			t.Errorf("Art %s fehlt", w)
		}
	}
}

// B-679/AC-04: Gebäude-Planer per Karte: geprüft, geschrieben über HubSave, höchstens einmal, auf geändertem Stand nicht.
func TestChatAktionGebaeude(t *testing.T) {
	root := planerCopy(t, hubFile)
	a := &App{root: root}
	list := cards(t, a, "c", block(`{"art": "gebaeude_aendern", "aenderung": {"islandStartStock": {"wood": 120}}},
		{"art": "gebaeude_aendern", "aenderung": {"islandStartStock": {"wood": 130}}},
		{"art": "gebaeude_aendern", "aenderung": {"levels": []}}`))
	if list[0].Block != "" || list[2].Block == "" {
		t.Fatalf("Prüfung vorab: %+v", list)
	}
	done, err := a.ChatActionRun("c", list[0].ID)
	if err != nil || done.Status != actionDone || !strings.Contains(readIn(t, root, hubFile), `"wood": 120`) {
		t.Fatalf("Ausführen: %+v %v", done, err)
	}
	if _, err := a.ChatActionRun("c", list[0].ID); err == nil {
		t.Error("Karte lief zweimal")
	}
	stale, _ := a.ChatActionRun("c", list[1].ID) // Stand von vor dem ersten Speichern
	if stale.Status != actionFailed || strings.Contains(readIn(t, root, hubFile), `"wood": 130`) {
		t.Errorf("veralteter Stand: %+v", stale)
	}
	if d, err := a.ChatActionDrop("c", list[2].ID); err != nil || d.Status != actionDropped {
		t.Errorf("Verwerfen: %+v %v", d, err)
	}
}

// B-679/AC-04: Zuordnung eines Slots per Karte über AssetsSet; unbekannter Slot gesperrt.
func TestChatAktionZuordnung(t *testing.T) {
	a, _ := sourcesApp(t, "")
	write(t, filepath.Join(a.root, "public", "audio", "coin.ogg"), string(testOgg))
	list := cards(t, a, "c", block(`{"art": "zuordnung_setzen", "slot": "sfx:coin", "wahl": [{"datei": "audio/coin.ogg"}]},
		{"art": "zuordnung_setzen", "slot": "sfx:weg", "wahl": []}`))
	if list[0].Block != "" || !strings.Contains(list[1].Block, "unbekannter Asset-Slot") {
		t.Fatalf("Karten %+v", list)
	}
	if done, err := a.ChatActionRun("c", list[0].ID); err != nil || done.Status != actionDone {
		t.Fatalf("Ausführen %+v %v", done, err)
	}
	if reg, _ := assets.Load(a.root); len(reg.Find("sfx:coin").Choices) != 1 {
		t.Error("Zuordnung fehlt")
	}
}

// B-679/AC-04: Welt-Dateien per Karte; Standard-Welt und unbekannte Welten gesperrt.
func TestChatAktionWelten(t *testing.T) {
	a, _ := worldApp(t)
	list := cards(t, a, "c", block(`{"art": "welt_anlegen", "name": "Nordsee"}, {"art": "welt_loeschen", "id": "standard"},
		{"art": "welt_waehlen", "id": "mond"}, {"art": "chunks_planen", "insel": "island1", "tiefe": 0, "chunks": [{"slot": -7, "kind": "harbor"}], "anzahl": 0}`))
	if list[0].Block != "" || list[1].Block == "" || !strings.Contains(list[2].Block, "unbekannte Welt") || list[3].Block != "" {
		t.Fatalf("Karten %+v", list)
	}
	if done, _ := a.ChatActionRun("c", list[0].ID); done.Status != actionDone || a.activeWorld() == standardWorld {
		t.Fatalf("anlegen: %+v aktiv %s", done, a.activeWorld())
	}
	// die Chunk-Karte ist für die Standard-Welt geprüft; nach dem Wechsel schreibt sie nicht in die neue Welt
	if done, _ := a.ChatActionRun("c", list[3].ID); done.Status != actionFailed || !strings.Contains(done.Result, "geprüft für Welt") {
		t.Errorf("andere Welt: %+v", done)
	}
}

// B-679/AC-03: Ticket wird vorab wie beim Anlegen geprüft.
func TestChatAktionTicketVorab(t *testing.T) {
	a := &App{root: t.TempDir()}
	list := cards(t, a, "c", block(`{"art": "ticket", "titel": "X", "beschreibung": "", "typ": "Schuld", "prio": "mittel", "domaene": "DEV"}`))
	if !strings.Contains(list[0].Block, "Typ") {
		t.Errorf("Karte %+v", list[0])
	}
}

// B-679/AC-02, AC-03: Mit Seite geht der Katalog samt hub.json als Instruktion mit; der Block der Antwort wird zur Karte.
func TestChatStromAktionen(t *testing.T) {
	root := planerCopy(t, hubFile)
	cli, argsFile := fakeCLI(t, "echo", 10*time.Second)
	a := chatApp(t, root, cli)
	h := a.Handler(false)
	text := strings.ReplaceAll(block(`{"art": "netz", "modus": "auto"}`), "\n", `\n`)
	text = strings.ReplaceAll(text, `"`, `\"`)
	rec, _ := call(t, h, "POST", Prefix+"chat", `{"text":"`+text+`","seite":"planer"}`, "127.0.0.1:5000", true)
	end, id := chatDoneOf(t, rec.Body.String())
	if end.Status != chatDone || !strings.Contains(rec.Body.String(), "Aktionen als Karten") {
		t.Fatalf("Strom: %s", rec.Body)
	}
	if _, system := fakeInput(t, argsFile); !strings.Contains(system, "## Aktionen im Studio") || !strings.Contains(system, "data/hub.json") ||
		!strings.Contains(system, "gebaeude_aendern") {
		t.Errorf("Instruktion: %.300s", system)
	}
	if list := a.ChatActions(id); len(list) != 1 || list[0].Art != "netz" || list[0].Block != "" {
		t.Errorf("Karten %+v", list)
	}
	if rec, _ := call(t, h, "POST", Prefix+"chat", `{"text":"a","seite":"keller"}`, "127.0.0.1:5000", true); rec.Code != 400 {
		t.Errorf("unbekannte Seite: %d", rec.Code)
	}
}
