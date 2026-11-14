package game

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

var (
	keys1 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 0}
	keys2 = view.DeviceID{Kind: view.DeviceKeyboard, Index: 1}
	pad3  = view.DeviceID{Kind: view.DeviceGamepad, Index: 3}
	pad5  = view.DeviceID{Kind: view.DeviceGamepad, Index: 5}
	pad6  = view.DeviceID{Kind: view.DeviceGamepad, Index: 6}
)

// devicesIn baut eine Eingabe; Geräte in confirm drücken Bestätigen.
func devicesIn(ds []view.DeviceID, confirm ...view.DeviceID) view.Input {
	var in view.Input
	for _, d := range ds {
		di := view.DeviceInput{Device: d}
		if slices.Contains(confirm, d) {
			di.Actions = []view.Action{view.ActionConfirm}
		}
		in.Devices = append(in.Devices, di)
	}
	return in
}

// Zuordnungs-Regel (B-798/AC-02): Pad geht, neues Pad kommt bzw. ein freies Gerät bestätigt, zwei verwaiste Plätze,
// kein freies Gerät; nie über den Index.
func TestRebinds(t *testing.T) {
	seats := []view.Seat{{Player: 0, Device: pad3}, {Player: 1, Device: keys2}}
	cases := []struct {
		name  string
		seats []view.Seat
		in    view.Input
		fresh []view.DeviceID
		want  []Rebind
	}{
		{"Pad da", seats, devicesIn([]view.DeviceID{keys1, keys2, pad3}), nil, nil},
		{"Pad weg, nichts Neues", seats, devicesIn([]view.DeviceID{keys1, keys2}), nil, nil},
		{"Pad weg, neues Pad", seats, devicesIn([]view.DeviceID{keys1, keys2, pad5}), []view.DeviceID{pad5}, []Rebind{{0, pad5}}},
		{"Pad weg, altes freies Pad bestätigt", seats, devicesIn([]view.DeviceID{keys1, keys2, pad5}, pad5), nil, []Rebind{{0, pad5}}},
		{"Pad weg, Tastatur bestätigt", seats, devicesIn([]view.DeviceID{keys1, keys2}, keys1), nil, []Rebind{{0, keys1}}},
		{"besetztes Gerät bestätigt", seats, devicesIn([]view.DeviceID{keys1, keys2}, keys2), nil, nil},
		{"zwei verwaist", []view.Seat{{Player: 0, Device: pad3}, {Player: 1, Device: pad6}},
			devicesIn([]view.DeviceID{keys1, keys2, pad5}, keys2), []view.DeviceID{pad5}, []Rebind{{0, keys2}, {1, pad5}}},
		{"nur Spieler 1 verwaist", []view.Seat{{Player: 1, Device: pad3}}, devicesIn([]view.DeviceID{keys1}, keys1), nil, []Rebind{{1, keys1}}},
	}
	for _, c := range cases {
		if got := Rebinds(nil, c.seats, c.in, c.fresh); !slices.Equal(got, c.want) {
			t.Errorf("%s: %+v, erwartet %+v", c.name, got, c.want)
		}
	}
}

// Geräte-Cues: das erste Update meldet nichts, danach verbunden und getrennt mit Platz; Unseated und Lost im Frame.
func TestDevicesObserveDecorate(t *testing.T) {
	var d devices
	seats := []view.Seat{{Player: 0, Device: pad3}}
	if cues := d.observe(devicesIn([]view.DeviceID{keys1, pad3}), seats, nil); len(cues) != 0 {
		t.Fatalf("erstes Update: %+v", cues)
	}
	cues := d.observe(devicesIn([]view.DeviceID{keys1, pad5}), seats, nil)
	want := []view.Cue{{Kind: view.CueDeviceConnected, Player: -1, Device: pad5}, {Kind: view.CueDeviceLost, Player: 0, Device: pad3}}
	if !slices.Equal(cues, want) || !slices.Equal(d.fresh, []view.DeviceID{pad5}) {
		t.Errorf("Cues %+v, neu %v", cues, d.fresh)
	}
	if cues = d.observe(devicesIn([]view.DeviceID{keys1, pad5}), seats, nil); len(cues) != 0 || len(d.fresh) != 0 {
		t.Errorf("ohne Änderung: %+v", cues)
	}
	f := &view.Frame{Seats: []view.Seat{{Player: 0, Device: pad3}, {Player: 1, Device: keys1}}}
	d.decorate(f, devicesIn([]view.DeviceID{keys1, keys2, pad5}))
	if !slices.Equal(f.Unseated, []view.DeviceID{keys2, pad5}) || !f.Seats[0].Lost || f.Seats[1].Lost {
		t.Errorf("Frame: frei %v, Plätze %+v", f.Unseated, f.Seats)
	}
	in := devicesIn([]view.DeviceID{keys1, keys2, pad5})
	if n := testing.AllocsPerRun(10, func() { d.observe(in, f.Seats, nil); d.decorate(f, in) }); n != 0 {
		t.Errorf("Geräte legen %v Objekte an", n)
	}
}
