// Package menu baut die Menü-Bildschirme des nativen Spiels (Sprint NG7, B-800; NG13): Hauptmenü mit Version, Lobby
// für 1–2 Spieler, Spielstände, Optionen, Lizenzen, Pause und Tastenbelegung auf dem Papier-Hintergrund; Spielende
// führt ins Hauptmenü. Vorbild sind die Menüs des Browsers
// (src/scenes/pauseHub.ts, lobbyOptions.ts, saveMenu.ts, optionsLogic.ts); Online-Räume, Raum-Code, Werkzeug-Seiten und
// Dev-Optionen gibt es nativ nicht.
//
// Aufbau: Jeder Bildschirm ist ein view.Screen; Untermenüs legt er über den Menü-Stapel von game auf (StepPush),
// zurück geht es mit StepPop. Die Stapeltiefe ist höchstens zwei (Hauptmenü bzw. Pause und ein Untermenü), Rückfragen
// liegen im Bildschirm selbst; Home (View + Menu, Pos1) ist darum im Untermenü dasselbe wie B. Update ändert den
// Zustand (reine Logik, Tabellen-Tests), Draw baut daraus nur die Texte und zeichnet.
//
// Spielende: game schließt das Spiel view.EndDelay Takte nach Frame.Outcome und legt das Hauptmenü auf; das zeigt
// den Ausgang aus dem Frame (Frame.Outcome bleibt bis zum nächsten Spiel). Plätze und Einstellungen liest menu aus
// der Session (Seats, Settings).
package menu

import (
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// menus hält, was die Bildschirme teilen: Zeichner, Tastenbelegung, Plätze des laufenden Spiels, Meldung fürs
// Hauptmenü.
type menus struct {
	paint  painter
	keys   view.KeyTable              // Tastenbelegung (game baut sie aus input und hud); nil = Seite ohne Zeilen
	seats  [view.MaxPlayers]view.Seat // Plätze aus Session.Seats beim letzten Update des Pause-Menüs
	nSeats int
	notice note // zeigt das nächste Hauptmenü (Fehler beim Verlassen)
}

var (
	_ view.Menus     = (*menus)(nil)
	_ view.MenuPages = (*menus)(nil)
)

// New liefert die Einstiege des Menüs; lib (Grafik) darf nil sein, dann zeichnet die Debug-Schrift. keys liefert die
// Zeilen der Seite Tasten im Pause-Menü (B-807).
func New(lib *art.Library, keys view.KeyTable) view.Menus {
	font := &art.Font{Fallback: true}
	if lib != nil && lib.Font != nil {
		font = lib.Font
	}
	return &menus{paint: painter{font: font, lib: lib}, keys: keys}
}

// Main liefert das Hauptmenü; eine vorgemerkte Meldung (Fehler beim Verlassen) erscheint darin.
func (m *menus) Main() view.Screen {
	sc := newMain(m)
	sc.note, m.notice = m.notice, note{}
	return sc
}

// Pause liefert das Pause-Menü.
func (m *menus) Pause() view.Screen { return newPause(m) }

// Page liefert ein Untermenü für den Aufnahme-Modus (view.MenuPages): "saves", "options" (inGame: über der Pause),
// "lobby" (geführt von by), "keys" (Tastenbelegung) oder "licenses"; nil für andere Namen.
func (m *menus) Page(name string, inGame bool, by view.DeviceID) view.Screen {
	switch name {
	case "saves":
		return newSaves(m)
	case "options":
		return newOptions(m, inGame)
	case "lobby":
		return newLobby(m, by)
	case "keys":
		return newKeys(m)
	case "licenses":
		return newLicenses(m)
	}
	return nil
}

// sync übernimmt die Plätze des laufenden Spiels aus der Session (ohne Slices zu halten).
func (m *menus) sync(s view.Session) {
	m.nSeats = 0
	for _, st := range s.Seats() {
		if m.nSeats < view.MaxPlayers {
			m.seats[m.nSeats] = st
			m.nSeats++
		}
	}
}

// seatOf liefert den Spieler des Geräts d aus den zuletzt übernommenen Plätzen (-1 ohne Platz).
func (m *menus) seatOf(d view.DeviceID) int {
	for _, st := range m.seats[:m.nSeats] {
		if st.Device == d {
			return st.Player
		}
	}
	return -1
}
