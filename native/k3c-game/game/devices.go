package game

// Geräte der Schleife (B-798): welche Geräte verbunden sind, welche keinen Platz haben, und die Zuordnungs-Regel für
// verlorene Pads. Ein Pad bekommt nach Wiederverbinden meist eine neue ebiten.GamepadID; der Platz hängt deshalb nie an
// einem festen Index, sondern geht an ein freies Gerät, das neu verbunden wird oder Bestätigen drückt.

import (
	"slices"

	"k3c/native/k3c-game/view"
)

// Rebind ist eine Neuzuordnung: Spieler Player übernimmt Gerät Device.
type Rebind struct {
	Player int
	Device view.DeviceID
}

// Rebinds ist die Zuordnungs-Regel: Verwaiste Plätze (Gerät nicht in in), nach Player aufsteigend, bekommen der Reihe
// nach je ein freies Gerät (in in, ohne Platz), das in diesem Update neu verbunden ist (fresh) oder Bestätigen
// gedrückt hat; Kandidaten in Input-Reihenfolge. Die Neuzuordnungen hängt Rebinds an dst.
func Rebinds(dst []Rebind, seats []view.Seat, in view.Input, fresh []view.DeviceID) []Rebind {
	used := 0 // Zahl der schon vergebenen Kandidaten
	for p := range view.MaxPlayers {
		if !orphaned(seats, p, in) {
			continue
		}
		k := 0
		for _, di := range in.Devices {
			if !candidate(di, seats, fresh) {
				continue
			}
			if k == used {
				dst = append(dst, Rebind{Player: p, Device: di.Device})
				used++
				break
			}
			k++
		}
	}
	return dst
}

// orphaned meldet, ob Spieler p einen Platz hat, dessen Gerät nicht verbunden ist.
func orphaned(seats []view.Seat, p int, in view.Input) bool {
	for _, s := range seats {
		if s.Player == p {
			_, ok := in.Of(s.Device)
			return !ok
		}
	}
	return false
}

// candidate meldet, ob das Gerät frei ist und neu verbunden wurde oder bestätigt hat.
func candidate(di view.DeviceInput, seats []view.Seat, fresh []view.DeviceID) bool {
	if seatOf(seats, di.Device) >= 0 {
		return false
	}
	return slices.Contains(fresh, di.Device) || slices.Contains(di.Actions, view.ActionConfirm)
}

// seatOf liefert den Spieler mit Gerät d (-1 = keiner).
func seatOf(seats []view.Seat, d view.DeviceID) int {
	for _, s := range seats {
		if s.Device == d {
			return s.Player
		}
	}
	return -1
}

// devices merkt sich die verbundenen Geräte über die Updates (Puffer, nach dem ersten Bild ohne Allokation).
type devices struct {
	started bool
	known   []view.DeviceID // verbunden im vorigen Update
	fresh   []view.DeviceID // in diesem Update neu verbunden
	unseat  []view.DeviceID // Puffer von Frame.Unseated
	rebinds []Rebind
}

// observe vergleicht die Geräte in in mit dem vorigen Update: fresh sind die neu verbundenen; für jedes neue und jedes
// getrennte Gerät hängt es einen Cue an cues (Player = sein Platz in seats). Das erste Update meldet nichts.
func (d *devices) observe(in view.Input, seats []view.Seat, cues []view.Cue) []view.Cue {
	d.fresh = d.fresh[:0]
	for _, di := range in.Devices {
		if d.started && !slices.Contains(d.known, di.Device) {
			d.fresh = append(d.fresh, di.Device)
			cues = append(cues, view.Cue{Kind: view.CueDeviceConnected, Player: seatOf(seats, di.Device), Device: di.Device})
		}
	}
	for _, k := range d.known {
		if _, ok := in.Of(k); !ok {
			cues = append(cues, view.Cue{Kind: view.CueDeviceLost, Player: seatOf(seats, k), Device: k})
		}
	}
	d.known = d.known[:0]
	for _, di := range in.Devices {
		d.known = append(d.known, di.Device)
	}
	d.started = true
	return cues
}

// decorate trägt die Geräte in den Frame ein: Unseated (verbunden ohne Platz) und Seat.Lost.
func (d *devices) decorate(f *view.Frame, in view.Input) {
	d.unseat = d.unseat[:0]
	for _, di := range in.Devices {
		if seatOf(f.Seats, di.Device) < 0 {
			d.unseat = append(d.unseat, di.Device)
		}
	}
	f.Unseated = d.unseat
	for i := range f.Seats {
		_, ok := in.Of(f.Seats[i].Device)
		f.Seats[i].Lost = !ok
	}
}
