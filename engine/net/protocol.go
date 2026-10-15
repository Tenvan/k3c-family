package net

import (
	"encoding/json"
	"errors"

	"k3c/engine/room"
	"k3c/engine/sim"
)

// Protokoll v6 (docs/protocol.md, Beispiele in testdata/protocol/). Eine Änderung hier ändert auch das Dokument, die
// Beispiele und den Client (docs/arbeitsweise.md › Grenzfall Protokoll).

// ProtocolVersion steht nur im Handschlag (hello, welcome).
const ProtocolVersion = 6

// Codes ohne Gegenstück in engine/room.
const (
	codeRoomClosed = "room_closed"
	codeReplaced   = "replaced"
	codeVersion    = "version"
	codeBadRequest = "bad_request"
)

// messages sind die deutschen Texte zur Anzeige je Code.
var messages = map[string]string{
	"room_full":      "Raum ist voll",
	"too_many_slots": "Zu viele Spieler an diesem Gerät",
	"too_many_rooms": "Zu viele Räume",
	"room_not_found": "Raum nicht gefunden",
	"save_exists":    "Spielstand gibt es schon",
	"save_not_found": "Spielstand nicht gefunden",
	codeRoomClosed:   "Raum ist geschlossen",
	codeReplaced:     "An anderer Stelle geöffnet",
	codeVersion:      "Veraltete Version, Seite neu laden",
	codeBadRequest:   "Ungültige Nachricht",
	"forbidden":      "Nur im Dev-Mode erlaubt",
}

// codeOf übersetzt einen Fehler aus engine/room in seinen Code; alles andere ist bad_request.
func codeOf(err error) string {
	for _, e := range []error{room.ErrRoomFull, room.ErrTooManySlots, room.ErrTooManyRooms, room.ErrRoomNotFound,
		room.ErrSaveExists, room.ErrSaveNotFound, room.ErrClosed, room.ErrForbidden} {
		if errors.Is(err, e) {
			return e.Error()
		}
	}
	return codeBadRequest
}

// inMsg ist jede Nachricht Gerät → Server; Zeiger heißen „Pflichtfeld, fehlt = bad_request“.
type inMsg struct {
	T        string    `json:"t"`
	V        int       `json:"v"`
	Device   string    `json:"device"`
	Save     string    `json:"save"`
	Fresh    *bool     `json:"fresh"`
	Depth    int       `json:"depth"`
	Grade    string    `json:"grade"`
	Goal     string    `json:"goal"`
	Defeat   string    `json:"defeat"`
	Slots    []int     `json:"slots"`
	Room     string    `json:"room"`
	Slot     *int      `json:"slot"`
	Seq      int64     `json:"seq"`
	P        []inInput `json:"p"`
	SkillID  string    `json:"skill"`    // learn: Skill-ID (input.p[].skill ist dagegen der Slot)
	Action   string    `json:"action"`   // dev
	Amount   int       `json:"amount"`   // dev: gold, material
	Resource string    `json:"resource"` // dev: material
	Factor   int       `json:"factor"`   // dev: timescale
	Paused   *bool     `json:"paused"`   // dev: pause
	On       *bool     `json:"on"`       // dev: cheapBuild, timescaleAuto
	Level    int       `json:"level"`    // dev: buildAll (1 bis 5)
}

type inInput struct {
	Slot   *int    `json:"slot"`
	MoveX  float64 `json:"moveX"`
	Sprint bool    `json:"sprint"`
	Pay    bool    `json:"pay"`
	Attack bool    `json:"attack"` // gehalten: Schlag, sobald bereit
	Skill  int     `json:"skill"`  // gehalten: Skill-Slot 1 bis 4 feuert, sobald bereit; 0 = keiner
}

type errorMsg struct {
	T       string `json:"t"`
	Code    string `json:"code"`
	Message string `json:"message"`
}

func errMsg(code string) errorMsg { return errorMsg{"error", code, messages[code]} }

type welcomeMsg struct {
	T      string `json:"t"`
	V      int    `json:"v"`
	TickHz int    `json:"tickHz"`
	Limits struct {
		MonarchsPerRoom int `json:"monarchsPerRoom"`
		SlotsPerDevice  int `json:"slotsPerDevice"`
		Rooms           int `json:"rooms"`
	} `json:"limits"`
}

func welcome() welcomeMsg {
	w := welcomeMsg{T: "welcome", V: ProtocolVersion, TickHz: room.TickHz}
	w.Limits.MonarchsPerRoom, w.Limits.SlotsPerDevice, w.Limits.Rooms = room.MaxMonarchs, room.MaxSlots, room.MaxRooms
	return w
}

type roomsMsg struct {
	T     string      `json:"t"`
	Rooms []room.Info `json:"rooms"`
}

type joinedMsg struct {
	T    string      `json:"t"`
	Room string      `json:"room"`
	Name string      `json:"name"`
	You  []room.Seat `json:"you"`
}

type seatsMsg struct {
	T        string      `json:"t"`
	You      []room.Seat `json:"you"`
	Monarchs []string    `json:"monarchs"`
}

type levelMsg struct {
	T      string `json:"t"`
	Stage  int    `json:"stage"` // Index der Stufe in Island.Stages, Schlüssel des Stroms (B-176)
	Depth  int    `json:"depth"`
	Layout any    `json:"layout"`
}

type stateMsg struct {
	T     string         `json:"t"` // snap oder delta
	Stage int            `json:"stage"`
	Tick  int            `json:"tick"`
	Ack   int64          `json:"ack"`
	S     map[string]any `json:"s"`
}

// stateOf ist der Zustand für `snap`: die Welt ohne seed, biome, level, rng, widthUnits, mit events und depth, oben
// mit dem Wirtschaftsstand aus sim.EconomyOf (stockMax, hubLevel, hubUpgrade, danger; W5.1), je
// Spieler mit points und actions (actions.go). Das Feld
// `free` der Spieler (B-059) fällt weg, der Zustand der Monarchen steht in `seats`. timescale > 0 (nur Dev-Mode) steht
// als `devTimescale` darin, daneben die Dev-Pause als `devPaused` (B-231). Bauplätze laut filterSites (W11.2).
func stateOf(w *sim.World, timescale int, paused bool) map[string]any {
	raw, _ := json.Marshal(w)
	var s map[string]any
	_ = json.Unmarshal(raw, &s)
	raw, _ = json.Marshal(sim.EconomyOf(w))
	_ = json.Unmarshal(raw, &s) // ergänzt s um die Felder der Wirtschaft, Werte unverändert aus der Sim
	delete(s, "seed")
	delete(s, "widthUnits")
	for _, p := range s["players"].([]any) {
		delete(p.(map[string]any), "free")
	}
	addActions(w, s["players"].([]any))
	if sites, ok := s["sites"].([]any); ok {
		s["sites"] = filterSites(w, sites)
	}
	s["depth"] = w.Biome.Depth
	if timescale > 0 {
		s["devTimescale"] = timescale
		s["devPaused"] = paused // immer mitsenden: das Delta kennt kein Entfernen von Feldern
	}
	return s
}

// filterSites setzt je Bauplatz die Platz-Stufe `hubLevel` (sim.SiteHubLevel) und lässt unbezahlte Plätze über Hub-Stufe
// + 1 weg (B-408, W11.2); bezahlte, wartende und gebaute bleiben. sites ist w.Sites als JSON, gleiche Reihenfolge.
func filterSites(w *sim.World, sites []any) []any {
	out := make([]any, 0, len(sites))
	for i, e := range sites {
		site, lv := w.Sites[i], sim.SiteHubLevel(w, w.Sites[i])
		if site.State == "unpaid" && site.PaidGold == 0 && lv > w.HubLevel+1 {
			continue
		}
		e.(map[string]any)["hubLevel"] = lv
		out = append(out, e)
	}
	return out
}
