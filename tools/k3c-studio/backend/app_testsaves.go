package backend

import (
	"errors"
	"fmt"
	"net/url"

	"k3c/engine/store"
)

// Testspielstände des Studios (B-507/AC-02, TS1.2): anlegen, laden und löschen in der eingebundenen Engine, nur im Ordner
// testSavesDir, nie in saves/ des Servers. Methoden ohne eigene Oberfläche; „Im Spiel öffnen“ legt sie ebenfalls an.

// TestSavesView sind die Testspielstände nach Name oder ein Hinweis in Error.
type TestSavesView struct {
	Saves []store.SaveInfo `json:"saves"`
	Error string           `json:"error"`
}

// testSavesView liefert die Liste, mit err als Hinweis.
func (a *App) testSavesView(err error) TestSavesView {
	if a.engine == nil {
		return TestSavesView{Saves: []store.SaveInfo{}, Error: errNoEngine.Error()}
	}
	v := TestSavesView{Saves: a.engine.Saves.List()}
	if err != nil {
		v.Error = err.Error()
	}
	return v
}

// WorldTestSaves listet die Testspielstände (Binding).
func (a *App) WorldTestSaves() TestSavesView { return a.testSavesView(nil) }

// WorldTestSaveCreate legt den Testspielstand „voll ausgebaut“ `voll-<seed>` an (Binding); ein gleichnamiger wird
// überschrieben, außer ein Gerät spielt ihn gerade.
func (a *App) WorldTestSaveCreate(seed string) TestSavesView {
	if a.engine == nil {
		return a.testSavesView(nil)
	}
	if !gameFullSeed.MatchString(seed) {
		return a.testSavesView(hint("Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 27 Zeichen)"))
	}
	if _, err := a.engine.Rooms.FullSave(seed); err != nil {
		return a.testSavesView(fullSaveHint(err))
	}
	return a.testSavesView(nil)
}

// WorldTestSaveOpen lädt einen vorhandenen Testspielstand im Spiel (Binding): die Landingpage unter base öffnet
// `game.html` mit ihm.
func (a *App) WorldTestSaveOpen(base, name string) OpenGameResult {
	root, err := loopbackBase(base)
	if err != nil {
		return OpenGameResult{Error: err.Error()}
	}
	if a.engine == nil {
		return OpenGameResult{Error: errNoEngine.Error()}
	}
	if !gameSaveName.MatchString(name) {
		return OpenGameResult{Error: fmt.Sprintf("Name %q ungültig", name)}
	}
	_, err = a.engine.Saves.Load(name)
	switch {
	case errors.Is(err, store.ErrNotFound):
		return OpenGameResult{Error: fmt.Sprintf("Testspielstand %q nicht gefunden", name)}
	case err != nil:
		return OpenGameResult{Error: err.Error()}
	}
	return OpenGameResult{URL: gameURL(root, url.Values{"autostart": {"1"}, "save": {name}}), Save: name}
}

// WorldTestSaveDelete löscht einen Testspielstand samt Sicherungen (Binding). Ein offener Raum mit dem Stand würde ihn
// beim Schließen neu schreiben: dann bleibt er.
func (a *App) WorldTestSaveDelete(name string) TestSavesView {
	if a.engine == nil {
		return a.testSavesView(nil)
	}
	if !gameSaveName.MatchString(name) {
		return a.testSavesView(hint(fmt.Sprintf("Name %q ungültig", name)))
	}
	// Race: ein Gerät kann den Raum zwischen dieser Prüfung und dem Löschen öffnen; dann schreibt der Raum den Stand neu.
	// Hingenommen (Testspielstand, nur lokal).
	for _, r := range a.engine.Rooms.Rooms() {
		if r.Name == name {
			return a.testSavesView(hint(fmt.Sprintf("Testspielstand %q ist gerade in einem Raum offen", name)))
		}
	}
	if err := a.engine.Saves.Delete(name); err != nil {
		return a.testSavesView(err)
	}
	return a.testSavesView(nil)
}
