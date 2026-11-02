package backend

import (
	"cmp"
	"encoding/json"
	"errors"
	"fmt"
	"slices"
	"strings"

	"k3c/engine/level"
	"k3c/tools/assetkit/ticket"
)

// Aktionen des Welten Editors, des Gebäude-Planers und das Ticket (B-679): dieselben Aufrufe wie die Seiten; Speichern
// geht nur auf dem Stand der Datei zur Zeit der Prüfung (sonst Ablehnung, nichts überschrieben).

func worldActions() []actionSpec {
	return []actionSpec{
		{"welt_waehlen", "id", "Welt wählen (gilt für Editor und Vorschauen)", prepareWorldSelect},
		{"welt_anlegen", "name", "Neue Welt anlegen (eine Insel wie die Standard-Welt)", prepareWorldAdd},
		{"welt_kopieren", "name", "Gewählte Welt unter neuem Namen kopieren", prepareWorldCopy},
		{"welt_loeschen", "id", "Welt löschen (nicht die Standard-Welt)", prepareWorldRemove},
		{"welt_speichern", "welt: {ganze Welt-Datei wie im Welt-Auftrag, ohne id}", "Gewählte Welt ersetzen (Name, Notizen, Inseln, Planung)", prepareWorldSave},
		{"chunks_planen", `insel, tiefe, chunks: [{"slot": -7, "kind": "harbor"}], anzahl (0 = Biom-Länge)`, "Chunk-Planung einer Tiefe der gewählten Welt setzen", prepareChunks},
		{"testspielstand_anlegen", "seed (a-z, 0-9, -)", "Voll ausgebauten Testspielstand anlegen", prepareTestSave},
		{"testspielstand_loeschen", "name", "Testspielstand löschen", prepareTestSaveDelete},
	}
}

func hubActions() []actionSpec {
	return []actionSpec{
		{"gebaeude_aendern", "aenderung: {JSON-Merge-Patch auf data/hub.json; Listen ganz angeben}", "Gebäude-Planer ändern (data/hub.json und Regeltabelle)", prepareHub},
		{"ticket", "titel, beschreibung, typ (Idee|Problem|Frage), prio (hoch|mittel|niedrig), domaene (REG|SIM|SRV|CLI|PLAT|INF|DEV), projekt? (Kürzel)", "Ticket im Backlog anlegen", prepareTicket},
	}
}

type nameField struct {
	Name string `json:"name"`
}

type idField struct {
	ID string `json:"id"`
}

// worldsResult macht aus der Antwort der Welt-Aufrufe ein Ergebnis.
func worldsResult(v WorldsView) (string, error) {
	if v.Error != "" {
		return "", errors.New(v.Error)
	}
	return "gewählt: " + v.Active, nil
}

func knownWorld(a *App, id string) error {
	if err := need("id", id); err != nil {
		return err
	}
	if !slices.ContainsFunc(a.listWorlds(), func(w WorldInfo) bool { return w.ID == id }) {
		return fmt.Errorf("unbekannte Welt %q", id)
	}
	return nil
}

func prepareWorldSelect(a *App, raw json.RawMessage) (actionPlan, error) {
	var f idField
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Welt wählen: " + f.ID}
	p.run = func() (string, error) { return worldsResult(a.PlanWorldSelect(f.ID)) }
	return p, knownWorld(a, f.ID)
}

func prepareWorldAdd(a *App, raw json.RawMessage) (actionPlan, error) {
	return prepareWorldName(a, raw, "Welt anlegen: ", a.PlanWorldAdd, a.newWorldContent)
}

func prepareWorldCopy(a *App, raw json.RawMessage) (actionPlan, error) {
	return prepareWorldName(a, raw, "Welt kopieren als ", a.PlanWorldCopy, a.copyWorldContent)
}

// prepareWorldName prüft die neue Welt vorab wie beim Anlegen (Prüfung des Ladens, B-686): eine Kopie einer ungültigen
// Welt wird keine ausführbare Karte.
func prepareWorldName(a *App, raw json.RawMessage, title string, do func(string) WorldsView, content func(string) func(string) ([]byte, error)) (actionPlan, error) {
	var f nameField
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: title + f.Name, details: []string{"Vorlage: " + a.activeWorld()}}
	if err := need("name", f.Name); err != nil {
		return p, err
	}
	id, err := a.newWorldID(strings.TrimSpace(f.Name))
	if err != nil {
		return p, err
	}
	file, err := content(f.Name)(id)
	if err != nil {
		return p, err
	}
	if list := worldProblems(file, id); len(list) > 0 {
		return p, errors.New(strings.Join(list, "; "))
	}
	p.run = func() (string, error) { return worldsResult(do(f.Name)) }
	return p, nil
}

func prepareWorldRemove(a *App, raw json.RawMessage) (actionPlan, error) {
	var f idField
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Welt löschen: " + f.ID}
	if f.ID == standardWorld {
		return p, errors.New("die Standard-Welt bleibt")
	}
	p.run = func() (string, error) { return worldsResult(a.PlanWorldRemove(f.ID)) }
	return p, knownWorld(a, f.ID)
}

// onWorld führt save nur aus, wenn noch die geprüfte Welt id gewählt ist (eine andere Welt mit gleichem Inhalt hätte
// denselben Stand).
func onWorld(a *App, id string, save func() PlanSaved) (string, error) {
	if now := a.activeWorld(); now != id {
		return "", fmt.Errorf("geprüft für Welt %s, gewählt ist %s: bitte neu vorschlagen lassen", id, now)
	}
	return planResult(save())
}

// planResult macht aus PlanSaved ein Ergebnis.
func planResult(s PlanSaved) (string, error) {
	if s.Error != "" {
		return "", errors.New(s.Error)
	}
	return strings.TrimSpace("gespeichert. " + s.Hint), nil
}

func prepareWorldSave(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		World json.RawMessage `json:"welt"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	world := a.activeWorld()
	p := actionPlan{title: "Welt speichern: " + world}
	if len(f.World) == 0 {
		return p, fmt.Errorf("feld %q fehlt", "welt")
	}
	view := a.PlanLoad() // Stand vor dem Prüfen: ändert sich die Datei danach, lehnt Speichern ab
	ans := a.PlanWorldAnswer(string(f.World))
	if ans.Draft == nil || !ans.OK {
		return p, errors.New(cmp.Or(ans.Error, "Welt nicht lesbar"))
	}
	d := *ans.Draft
	for _, isl := range d.Islands {
		p.details = append(p.details, fmt.Sprintf("Insel %s (%s): Tiefen %v", isl.ID, orNone(isl.Name), isl.Depths))
	}
	if ans.Hint != "" {
		p.details = append(p.details, ans.Hint)
	}
	if chk := a.PlanWorldCheck(d); !chk.OK {
		return p, errors.New(strings.Join(a.draftProblems(d, chk.Error), "; "))
	}
	if view.Error != "" {
		return p, errors.New(view.Error)
	}
	p.run = func() (string, error) {
		return onWorld(a, world, func() PlanSaved { return a.PlanWorldSave(d, view.Stand) })
	}
	return p, nil
}

func prepareChunks(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		Island string               `json:"insel"`
		Depth  int                  `json:"tiefe"`
		Chunks []level.PlannedChunk `json:"chunks"`
		Count  int                  `json:"anzahl"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	world := a.activeWorld()
	p := actionPlan{title: fmt.Sprintf("Chunks %s, Tiefe %d (Welt %s)", f.Island, f.Depth, world),
		details: []string{fmt.Sprintf("%d geplante Chunks, Anzahl %d", len(f.Chunks), f.Count)}}
	view := a.PlanLoad()
	if view.Error != "" {
		return p, errors.New(view.Error)
	}
	if note := a.chunksHarbor(view, f.Island, f.Depth, &f.Chunks, f.Count); note != "" {
		p.details = append(p.details, note)
	}
	if chk := a.PlanCheck(f.Island, f.Depth, f.Chunks, f.Count); !chk.OK {
		return p, errors.New(chk.Error)
	}
	if err := a.planWorldError(f.Island, f.Depth, f.Chunks, f.Count); err != nil {
		return p, err
	}
	p.run = func() (string, error) {
		return onWorld(a, world, func() PlanSaved { return a.PlanSave(f.Island, f.Depth, f.Chunks, f.Count, view.Stand) })
	}
	return p, nil
}

func testSaveResult(v TestSavesView) (string, error) {
	if v.Error != "" {
		return "", errors.New(v.Error)
	}
	return fmt.Sprintf("%d Testspielstände", len(v.Saves)), nil
}

func prepareTestSave(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		Seed string `json:"seed"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Testspielstand anlegen: " + f.Seed}
	if !gameFullSeed.MatchString(f.Seed) {
		return p, errors.New("seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 27 Zeichen)")
	}
	if a.engine == nil {
		return p, errors.New("keine Engine im Studio")
	}
	p.run = func() (string, error) { return testSaveResult(a.WorldTestSaveCreate(f.Seed)) }
	return p, nil
}

func prepareTestSaveDelete(a *App, raw json.RawMessage) (actionPlan, error) {
	var f nameField
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Testspielstand löschen: " + f.Name}
	if !gameSaveName.MatchString(f.Name) {
		return p, fmt.Errorf("name %q ungültig", f.Name)
	}
	if a.engine == nil {
		return p, errors.New("keine Engine im Studio")
	}
	p.run = func() (string, error) { return testSaveResult(a.WorldTestSaveDelete(f.Name)) }
	return p, nil
}

func prepareHub(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		Patch json.RawMessage `json:"aenderung"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Gebäude-Planer ändern", details: []string{}}
	cur := a.HubPlan()
	if cur.Error != "" {
		return p, errors.New(cur.Error)
	}
	cand, changed, err := mergePatch([]byte(cur.Raw), f.Patch)
	if err != nil {
		return p, err
	}
	p.details = changed
	chk := a.HubCheck(string(cand))
	if chk.Error != "" {
		return p, errors.New(chk.Error)
	}
	for _, fd := range chk.Findings {
		p.details = append(p.details, fmt.Sprintf("Befund %s %s: %s", fd.Rule, fd.Site, fd.Detail))
	}
	if len(chk.Findings) > 0 {
		return p, fmt.Errorf("%d Befunde der Prüfung", len(chk.Findings))
	}
	p.run = func() (string, error) { return hubResult(a.HubSave(string(cand), cur.Stand)) }
	return p, nil
}

func hubResult(r HubSaveResult) (string, error) {
	switch {
	case r.Stale != "":
		return "", errors.New(r.Stale)
	case r.Error != "":
		return "", errors.New(r.Error)
	case r.RulesError != "":
		return "", errors.New("hub.json gespeichert, Regeltabelle nicht: " + r.RulesError)
	case r.Unchanged:
		return "unverändert", nil
	}
	return strings.TrimSpace("gespeichert. " + r.Notice), nil
}

// mergePatch wendet einen JSON-Merge-Patch (RFC 7386) auf doc an und nennt die geänderten obersten Schlüssel.
func mergePatch(doc, patch []byte) ([]byte, []string, error) {
	var d, pt map[string]any
	if err := json.Unmarshal(doc, &d); err != nil {
		return nil, nil, errors.New("data/hub.json nicht lesbar: " + err.Error())
	}
	if err := json.Unmarshal(patch, &pt); err != nil || len(pt) == 0 {
		return nil, nil, errors.New("aenderung muss ein nicht leeres JSON-Objekt sein")
	}
	var keys []string
	for k := range pt {
		keys = append(keys, "geändert: "+k)
	}
	slices.Sort(keys)
	out, err := json.Marshal(mergeValue(d, pt))
	return out, keys, err
}

func mergeValue(target any, patch any) any {
	pm, ok := patch.(map[string]any)
	if !ok {
		return patch
	}
	tm, ok := target.(map[string]any)
	if !ok {
		tm = map[string]any{}
	}
	for k, v := range pm {
		if v == nil {
			delete(tm, k)
			continue
		}
		tm[k] = mergeValue(tm[k], v)
	}
	return tm
}

func prepareTicket(a *App, raw json.RawMessage) (actionPlan, error) {
	var in ticket.Input
	if err := fields(raw, &in); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Ticket: " + in.Titel, details: []string{in.Typ + " · " + in.Prio + " · " + in.Domaene, in.Beschreibung}}
	if err := ticket.Validate(a.root, in); err != nil {
		return p, err
	}
	in.Kontext.App, in.Kontext.Seite = "k3c-studio", "chat"
	p.run = func() (string, error) {
		c, err := a.TicketCreate(in)
		return c.ID + " " + c.Pfad, err
	}
	return p, nil
}
