package backend

import (
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"testing"
)

// B-686: Welt-Aufträge Schritt für Schritt, Vorabprüfung jeder Antwort mit Korrekturversuchen, Hafen ergänzt das Studio,
// vollständige Fehlerliste im Kontext. Alles in Kopien (t.TempDir()), kein echtes Modell.

// stepWorld ist eine Welt-Antwort mit Inseln insel1 … mit den Tiefen depths, ohne Hafen.
func stepWorld(name string, depths ...[]int) string {
	var islands []string
	for i, d := range depths {
		islands = append(islands, fmt.Sprintf(`{ "id": "insel%d", "name": "Insel %d", "depths": %s, "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 } }`, i+1, i+1, depthsText(d)))
	}
	return fmt.Sprintf("```json\n{ \"id\": \"stufen\", \"name\": %q, \"notes\": \"\", \"islands\": [ %s ] }\n```", name, strings.Join(islands, ", "))
}

// brokenWorld: insel-a ohne Hafen (nicht die letzte), insel-b ohne Endboss, insel-c gültig.
const brokenWorld = `{ "id": "standard", "name": "Standard", "notes": "",
  "islands": [
    { "id": "insel-a", "name": "A", "depths": [0, 4], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 } },
    { "id": "insel-b", "name": "B", "depths": [0, 2], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 } },
    { "id": "insel-c", "name": "C", "depths": [0, 4], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 } }
  ] }`

func TestWeltFehlerlisteVollstaendig(t *testing.T) {
	got := worldProblems([]byte(brokenWorld), "standard")
	if len(got) != 2 || !strings.Contains(got[0], "Insel insel-a: kein Hafen") || !strings.Contains(got[1], "Insel insel-b: kein Endboss") {
		t.Fatalf("Fehlerliste %q", got)
	}
	for _, p := range got {
		if !strings.HasPrefix(p, "data/worlds/standard.json: ") {
			t.Errorf("Datei falsch: %q", p)
		}
	}
	if worldProblems([]byte(worldFixture), "standard") != nil {
		t.Error("gültige Welt mit Befund")
	}
	a, _ := planApp(t, brokenWorld)
	if v := a.PlanLoad(); len(v.Problems) != 2 {
		t.Errorf("PlanLoad ohne Fehlerliste: %q", v.Problems)
	}
}

// Die Meldung nennt die Datei der gewählten Welt, nicht die id der Antwort; ein fehlender Hafen wird ergänzt.
func TestAntwortDateinameUndHafen(t *testing.T) {
	a, _ := worldApp(t)
	if v := a.PlanWorldAdd("Nordsee"); v.Error != "" || v.Active != "nordsee" {
		t.Fatalf("anlegen: %+v", v)
	}
	bad := a.PlanWorldAnswer(stepWorld("X", []int{0, 4}, []int{0, 2}))
	if bad.OK || !strings.HasPrefix(bad.Error, "data/worlds/nordsee.json: Insel insel2: kein Endboss") {
		t.Errorf("Meldung %q", bad.Error)
	}
	good := a.PlanWorldAnswer(stepWorld("X", []int{0, 4}, []int{0, 1, 4}))
	if !good.OK || !strings.Contains(good.Hint, "Hafen ergänzt: Insel insel1, Tiefe 0, Platz -7") {
		t.Fatalf("Hafen nicht ergänzt: %+v", good)
	}
	if strings.Contains(string(good.Draft.Islands[1].PlannedChunks), "harbor") {
		t.Error("die letzte Insel braucht keinen Hafen")
	}
}

func TestChatAktionenWeltVorabGeprueft(t *testing.T) {
	a, path := worldApp(t)
	list := cards(t, a, "c", block(`{"art": "chunks_planen", "insel": "island1", "tiefe": 0, "chunks": []},
		{"art": "welt_speichern", "welt": {"name": "X", "islands": [{"id": "a", "name": "A", "depths": [0, 2], "depthScaling": {"hp": 1, "damage": 1, "speed": 1}}]}}`))
	if list[0].Block != "" || !strings.Contains(strings.Join(list[0].Details, " "), "Hafen ergänzt: Insel island1") {
		t.Errorf("Chunks ohne Hafen: %+v", list[0])
	}
	if !strings.Contains(list[1].Block, "kein Endboss") {
		t.Errorf("ungültige Welt nicht gesperrt: %+v", list[1])
	}
	if err := os.WriteFile(path, []byte(brokenWorld), 0o644); err != nil {
		t.Fatal(err)
	}
	list = cards(t, a, "d", block(`{"art": "chunks_planen", "insel": "insel-c", "tiefe": 4, "chunks": []}, {"art": "welt_kopieren", "name": "Kopie"}`))
	for _, c := range list {
		if !strings.Contains(c.Block, "kein Hafen") {
			t.Errorf("Karte auf ungültiger Welt nicht gesperrt: %+v", c)
		}
	}
}

func TestKontextMitEntwurfUndFehlerliste(t *testing.T) {
	a, _ := worldApp(t)
	d, _ := draftOf(t, a)
	d.Islands[1].Depths = []int{0, 2}
	text := a.currentWorldText(&d, "")
	for _, want := range []string{"Entwurf im Welten Editor", "Prüfung: ungültig", "- data/worlds/standard.json: Insel island2: kein Endboss in der tiefsten Stufe 2", `"depths": [0, 2]`} {
		if !strings.Contains(text, want) {
			t.Errorf("Kontext ohne %q:\n%s", want, text)
		}
	}
	if text = a.currentWorldText(nil, ""); !strings.Contains(text, "gespeicherte Datei data/worlds/standard.json") || !strings.Contains(text, "Prüfung: gültig") {
		t.Errorf("Kontext ohne Datei:\n%s", text)
	}
}

func wantAll(t *testing.T, label, text string, wants ...string) {
	t.Helper()
	for _, w := range wants {
		if !strings.Contains(text, w) {
			t.Errorf("%s ohne %q:\n%s", label, w, text)
		}
	}
}

// postWorld schickt einen Welt-Auftrag im Chat chat und liefert die gespeicherte Antwort.
func postWorld(t *testing.T, a *App, chat, text string) string {
	t.Helper()
	body, _ := json.Marshal(map[string]string{"chat": chat, "text": text, "task": "welt", "seite": "welt"})
	rec, _ := call(t, a.Handler(false), "POST", Prefix+"chat", string(body), "127.0.0.1:5000", true)
	if end, _ := chatDoneOf(t, rec.Body.String()); end.Status != chatDone {
		t.Fatalf("Ende: %+v", end)
	}
	log, err := a.chat.store.load(chat)
	if err != nil {
		t.Fatal(err)
	}
	return log.Messages[len(log.Messages)-1].Text
}

// Mehrschrittiger Verlauf: Wunsch → ungültige Antwort → Fehler zurück an den Chat → gültige Antwort; der nächste
// Auftrag setzt auf dem Vorschlag auf (nicht auf dem Chat-Text), die Stufenzahlen bleiben.
func TestChatWeltSchrittFuerSchritt(t *testing.T) {
	a, _ := worldApp(t)
	cli, calls := scriptCLI(t,
		stepWorld("Stufen", []int{0, 4}, []int{0, 1, 4}, []int{0, 1, 2}),
		stepWorld("Stufen", []int{0, 4}, []int{0, 1, 4}, []int{0, 1, 2, 4}),
		stepWorld("Stufen Neu", []int{0, 4}, []int{0, 1, 4}, []int{0, 1, 2, 4}))
	a.chat = newChatHub(a.root)
	a.chat.cli = cli
	answer := postWorld(t, a, "stufen", "3 Inseln, jede eine Stufe mehr")
	c := calls()
	if len(c) != 2 {
		t.Fatalf("%d Aufrufe, erwartet Wunsch und eine Korrektur", len(c))
	}
	wantAll(t, "Instruktion", c[0].System, "## Aktuelle Welt", "gespeicherte Datei data/worlds/standard.json", "Hafen (Pflicht)", "Änderungsmodus")
	wantAll(t, "Korrektur", c[1].Stdin, "Insel insel3: kein Endboss in der tiefsten Stufe 2", `"depths": [0, 1, 2]`)
	wantAll(t, "Instruktion der Korrektur", c[1].System, "letzter Vorschlag dieses Chats", "Prüfung: ungültig")
	v := a.PlanWorldAnswer(answer)
	if !strings.Contains(answer, chatFixMark+"1 von 2") || !v.OK || len(v.Draft.Islands[2].Depths) != 4 || !strings.Contains(v.Hint, "Hafen ergänzt") {
		t.Fatalf("Antwort nach Korrektur: %+v\n%s", v, answer)
	}
	postWorld(t, a, "stufen", "Benenne die Welt um")
	c = calls()
	if len(c) != 3 || !strings.Contains(c[2].System, "letzter Vorschlag dieses Chats") || !strings.Contains(c[2].System, `"depths": [0, 1, 2, 4]`) ||
		!strings.Contains(c[2].System, "Prüfung: gültig") {
		t.Errorf("Schritt 2 setzt nicht auf dem Vorschlag auf:\n%s", c[len(c)-1].System)
	}
}

func TestChatWeltKorrekturHoechstensZweimal(t *testing.T) {
	a, _ := worldApp(t)
	cli, calls := scriptCLI(t, stepWorld("X", []int{0, 2}))
	a.chat = newChatHub(a.root)
	a.chat.cli = cli
	answer := postWorld(t, a, "fehler", "Welt")
	if n := len(calls()); n != 1+chatFixMax {
		t.Errorf("%d Aufrufe", n)
	}
	if !strings.Contains(answer, "auch nach 2 Korrekturversuchen ungültig") || a.PlanWorldAnswer(answer).OK {
		t.Errorf("Fehler nicht gezeigt:\n%s", answer)
	}
}

// Eine Korrektur ohne neuen Vorschlag gilt nicht als behoben; Karten aus verworfenen Versuchen entstehen nicht.
func TestChatWeltKorrekturOhneVorschlag(t *testing.T) {
	a, _ := worldApp(t)
	cli, calls := scriptCLI(t, stepWorld("X", []int{0, 2})+"\n"+block(`{"art": "welt_loeschen", "id": "standard"}`), "Erledigt.")
	a.chat = newChatHub(a.root)
	a.chat.cli = cli
	answer := postWorld(t, a, "leer", "Welt")
	if n := len(calls()); n != 1+chatFixMax || !strings.Contains(answer, noFixedWorld) {
		t.Errorf("%d Aufrufe:\n%s", n, answer)
	}
	if list := a.ChatActions("leer"); len(list) != 0 {
		t.Errorf("Karten trotz ungültiger Antwort: %+v", list)
	}
}
