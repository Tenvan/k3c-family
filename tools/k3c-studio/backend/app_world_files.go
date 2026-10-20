package backend

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"k3c/engine/sim"
)

// Welten als Dateien (Welten Editor, WZ18.6): Jede Welt ist eine JSON-Datei mit allen Inseln und ihren Anpassungen
// (Planung, Inselgrößen), im Format der Welt-Datei (docs/rules/welten.md). Die Welt „standard“ ist
// data/worlds/standard.json (islandsPath, die das Spiel eingebettet lädt), weitere liegen daneben unter data/worlds/.
// Angelegt (leer oder als Kopie) wird nur eine Welt, die die Prüfung des Ladens besteht.
// Das Studio bearbeitet immer die gewählte Welt; die Wahl gilt für alle Aufrufe der Planung (Laden, Prüfen, Speichern,
// Inseln, Quelle der Vorschauen).

const (
	standardWorld = "standard"
	worldsDir     = "data/worlds"
)

var worldID = regexp.MustCompile(`^[a-z0-9][a-z0-9-]{0,31}$`)

// WorldInfo ist eine wählbare Welt.
type WorldInfo struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

// WorldsView sind die Welten und die gewählte; Error nennt einen Fehler (Wahl bleibt).
type WorldsView struct {
	Worlds []WorldInfo `json:"worlds"`
	Active string      `json:"active"`
	Error  string      `json:"error"`
}

// activeWorld ist die gewählte Welt (Standard: standard).
func (a *App) activeWorld() string {
	a.mu.Lock()
	defer a.mu.Unlock()
	if a.world == "" {
		return standardWorld
	}
	return a.world
}

// worldFile ist der Pfad der Datei einer Welt; id kommt immer aus worldID oder ist „standard“.
func (a *App) worldFile(id string) string {
	if id == standardWorld {
		return filepath.Join(a.root, islandsPath)
	}
	return filepath.Join(a.root, worldsDir, id+".json")
}

func nameOfWorld(raw []byte, id string) string {
	var head struct {
		Name string `json:"name"`
	}
	if json.Unmarshal(raw, &head) == nil && head.Name != "" {
		return head.Name
	}
	if id == standardWorld {
		return "Standard"
	}
	return id
}

func (a *App) listWorlds() []WorldInfo {
	raw, _ := os.ReadFile(a.worldFile(standardWorld))
	out := []WorldInfo{{ID: standardWorld, Name: nameOfWorld(raw, standardWorld)}}
	entries, _ := os.ReadDir(filepath.Join(a.root, worldsDir))
	var more []WorldInfo
	for _, e := range entries {
		id := strings.TrimSuffix(e.Name(), ".json")
		if e.IsDir() || id == e.Name() || !worldID.MatchString(id) || id == standardWorld {
			continue
		}
		raw, _ := os.ReadFile(a.worldFile(id))
		more = append(more, WorldInfo{ID: id, Name: nameOfWorld(raw, id)})
	}
	sort.Slice(more, func(i, j int) bool { return more[i].ID < more[j].ID })
	return append(out, more...)
}

func (a *App) worldsView(err error) WorldsView {
	v := WorldsView{Worlds: a.listWorlds(), Active: a.activeWorld()}
	if err != nil {
		v.Error = err.Error()
	}
	return v
}

// PlanWorlds liefert die Welten und die gewählte (Binding).
func (a *App) PlanWorlds() WorldsView { return a.worldsView(nil) }

// PlanWorldSelect wählt die Welt id (Binding); danach liest PlanLoad ihre Datei.
func (a *App) PlanWorldSelect(id string) WorldsView {
	for _, w := range a.listWorlds() {
		if w.ID == id {
			a.mu.Lock()
			a.world = id
			a.mu.Unlock()
			return a.worldsView(nil)
		}
	}
	return a.worldsView(fmt.Errorf("die Welt %q gibt es nicht", id))
}

// slugOf macht aus einem Namen die Welt-ID: Kleinbuchstaben, Ziffern und Bindestriche.
func slugOf(name string) string {
	var b strings.Builder
	dash := false
	for _, r := range strings.ToLower(strings.TrimSpace(name)) {
		switch {
		case r >= 'a' && r <= 'z', r >= '0' && r <= '9':
			b.WriteRune(r)
			dash = false
		case r == 'ä':
			b.WriteString("ae")
		case r == 'ö':
			b.WriteString("oe")
		case r == 'ü':
			b.WriteString("ue")
		case r == 'ß':
			b.WriteString("ss")
		case !dash && b.Len() > 0:
			b.WriteByte('-')
			dash = true
		}
	}
	s := strings.Trim(b.String(), "-")
	if len(s) > 32 {
		s = strings.Trim(s[:32], "-")
	}
	return s
}

// newWorldFile ist der Inhalt einer neuen, leeren Welt: id, Name und eine Insel nach dem Muster der ersten Insel der
// Standard-Welt (Tiefen, Skalierung) ohne Planung und Inselgrößen.
func newWorldFile(standard []byte, id, name string) ([]byte, error) {
	w, err := sim.ParseWorld(standard, standardWorld)
	if err != nil {
		return nil, err
	}
	first := w.Islands[0]
	scaling := DraftScaling{HP: first.DepthScaling.HP, Damage: first.DepthScaling.Damage, Speed: first.DepthScaling.Speed}
	return fmt.Appendf(nil, newWorldText, jsonText(id), jsonText(name), depthsText(first.Depths), scalingText(scaling)), nil
}

// newWorldText ist das Muster einer neuen Welt-Datei (Felder in der Reihenfolge aus welten.md § 2).
const newWorldText = `{
  "id": %s,
  "name": %s,
  "notes": "",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": %s, "depthScaling": %s, "plannedChunks": {} }
  ]
}
`

// copyWorldFile ist die Kopie einer Welt-Datei mit neuer id und neuem Namen; alle übrigen Bytes bleiben.
func copyWorldFile(raw []byte, id, name string) ([]byte, error) {
	out, err := setKey(raw, span{0, len(raw)}, "id", jsonText(id))
	if err == nil {
		out, err = setKey(out, span{0, len(out)}, "name", jsonText(name))
	}
	return out, err
}

// newWorldID prüft den Namen einer neuen Welt und liefert ihre id (aus dem Namen, noch nicht vergeben).
func (a *App) newWorldID(name string) (string, error) {
	id := slugOf(name)
	switch {
	case name == "" || len([]rune(name)) > 60:
		return "", errors.New("der Name braucht 1 bis 60 Zeichen")
	case !worldID.MatchString(id) || id == standardWorld:
		return "", fmt.Errorf("aus %q wird keine gültige Welt-ID (a–z, 0–9, -)", name)
	}
	for _, w := range a.listWorlds() {
		if w.ID == id {
			return "", fmt.Errorf("die Welt %q gibt es schon", id)
		}
	}
	return id, nil
}

// addWorld legt die Welt name mit dem Inhalt aus content an: nur gültig (Prüfung des Ladens), atomar; danach ist sie
// gewählt.
func (a *App) addWorld(name string, content func(id string) ([]byte, error)) WorldsView {
	name = strings.TrimSpace(name)
	id, err := a.newWorldID(name)
	if err != nil {
		return a.worldsView(err)
	}
	raw, err := content(id)
	if err == nil {
		_, err = sim.ParseWorld(raw, id)
	}
	if err == nil {
		err = os.MkdirAll(filepath.Join(a.root, worldsDir), 0o755)
	}
	if err == nil {
		err = writeAtomic(a.worldFile(id), raw)
	}
	if err != nil {
		return a.worldsView(err)
	}
	return a.PlanWorldSelect(id)
}

// PlanWorldAdd legt eine leere Welt mit dem Namen name an (Binding, siehe newWorldFile); sie ist danach gewählt.
func (a *App) PlanWorldAdd(name string) WorldsView {
	return a.addWorld(name, func(id string) ([]byte, error) {
		standard, err := os.ReadFile(a.worldFile(standardWorld))
		if err != nil {
			return nil, err
		}
		return newWorldFile(standard, id, strings.TrimSpace(name))
	})
}

// PlanWorldCopy legt eine Kopie der gewählten Welt mit dem Namen name an (Binding): alle Inseln mit Planung und
// Inselgrößen; die Kopie ist danach gewählt.
func (a *App) PlanWorldCopy(name string) WorldsView {
	return a.addWorld(name, func(id string) ([]byte, error) {
		raw, err := a.readIslands()
		if err != nil {
			return nil, err
		}
		return copyWorldFile(raw, id, strings.TrimSpace(name))
	})
}

// PlanWorldRemove löscht die Welt id (Binding); „standard“ bleibt. Danach ist die Standard-Welt gewählt.
func (a *App) PlanWorldRemove(id string) WorldsView {
	if id == standardWorld {
		return a.worldsView(errors.New("die Standard-Welt bleibt"))
	}
	if !worldID.MatchString(id) {
		return a.worldsView(fmt.Errorf("ungültige Welt-ID %q", id))
	}
	if err := os.Remove(a.worldFile(id)); err != nil {
		return a.worldsView(err)
	}
	return a.PlanWorldSelect(standardWorld)
}
