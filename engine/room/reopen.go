package room

import (
	"regexp"
	"slices"
	"strings"
)

// codeFormat: vier Buchstaben aus codeLetters.
var codeFormat = regexp.MustCompile(`^[` + codeLetters + `]{4}$`)

// Räume nach einem geplanten Neustart wieder öffnen (WZ25.1, B-508): Vor dem Herunterfahren für einen Neustart wegen
// geänderter Dateien merkt sich der Server die offenen Räume (Code und Spielstand); der neue Prozess öffnet sie mit
// demselben Code, damit die Geräte beim Wiederverbinden in ihren Raum zurückkehren statt in die Raumliste.

// RoomRef nennt einen offenen Raum: Code und Name seines Spielstands.
type RoomRef struct {
	Code string `json:"code"`
	Name string `json:"name"`
}

// OpenRefs sind die offenen Räume, nach Code sortiert; Test-Räume (TestPrefix) zählen nicht (sie räumen sich selbst auf).
func (m *Manager) OpenRefs() []RoomRef {
	m.mu.Lock()
	defer m.mu.Unlock()
	var out []RoomRef
	for code, r := range m.rooms {
		if !r.isTest() {
			out = append(out, RoomRef{Code: code, Name: r.Name})
		}
	}
	slices.SortFunc(out, func(a, b RoomRef) int { return strings.Compare(a.Code, b.Code) })
	return out
}

// Reopen öffnet die Räume refs ohne Gerät aus ihren Spielständen mit ihrem Code. Ein Raum ohne Spielstand, mit
// ungültigem Spielstand, mit belegtem Code oder jenseits der Raumgrenze wird übersprungen (Log 🚫); die Frist EmptyFor
// läuft ab dem Öffnen. Zurück: die Zahl der geöffneten Räume. Vor Run aufrufen (Run startet die Goroutinen).
func (m *Manager) Reopen(refs []RoomRef) int {
	opened := 0
	for _, ref := range refs {
		if m.reopen(ref) {
			opened++
		}
	}
	m.notify(opened > 0)
	return opened
}

func (m *Manager) reopen(ref RoomRef) bool {
	m.mu.Lock()
	defer m.mu.Unlock()
	skip := func(why string, err error) bool {
		m.log().Warn("🚫 Raum nach dem Neustart nicht geöffnet", "code", ref.Code, "name", ref.Name, "grund", why, "err", err)
		return false
	}
	switch {
	case m.shut:
		return skip("Server fährt herunter", nil)
	case !saveName.MatchString(ref.Name) || !codeFormat.MatchString(ref.Code):
		return skip("ungültiger Code oder Name", nil)
	case len(m.rooms) >= MaxRooms:
		return skip("zu viele Räume", nil)
	}
	for code, r := range m.rooms {
		if code == ref.Code || r.Name == ref.Name {
			return skip("Code oder Spielstand schon offen", nil)
		}
	}
	isl, err := m.open(ref.Name, false, 0, Options{})
	if err != nil {
		return skip("Spielstand nicht ladbar", err)
	}
	monarchs, ok := freeMonarchs(isl)
	if !ok {
		return skip("Monarchen unstimmig", nil)
	}
	r := &Room{Code: ref.Code, Name: ref.Name, m: m, isl: isl, start: startStage(isl, 0), monarchs: monarchs, devices: map[string]*device{}, met: newMetrics(m.now())}
	r.emptySince = m.now()
	m.rooms[r.Code] = r
	r.log().Info("🏰 Raum nach dem Neustart wieder geöffnet", "raeume", len(m.rooms))
	if m.ctx != nil {
		go r.run(m.ctx)
	}
	return true
}
