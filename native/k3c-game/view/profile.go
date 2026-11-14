package view

// Eingabe-Profile (B-893): Jeder Spieler wählt in Optionen › Steuerung sein Gerät, Tastatur mit WASD, Tastatur mit
// Pfeiltasten oder Gamepad. Die Tastatur ist ein Gerät (DeviceID{DeviceKeyboard, 0}); ihre Belegung folgt dem
// Tastatur-Profil (KeyboardProfile). Höchstens ein Spieler hat ein Tastatur-Profil. Die Spielernummer folgt weiter dem
// Beitritt in der Lobby; das Profil bestimmt die Belegung und die Tasten-Hinweise des Geräts (ProfileOf).

// Werte von Settings.Controls.
const (
	ProfileWASD    = "wasd"    // Tastatur, Bewegung mit A/D
	ProfileArrows  = "pfeile"  // Tastatur, Bewegung mit ←/→
	ProfileGamepad = "gamepad" // Gamepad (Xbox-Belegung)
)

// Profiles sind die Profile in der Reihenfolge der Auswahl.
var Profiles = [...]string{ProfileWASD, ProfileArrows, ProfileGamepad}

// IsKeyboardProfile meldet, ob p ein Tastatur-Profil ist.
func IsKeyboardProfile(p string) bool { return p == ProfileWASD || p == ProfileArrows }

// defaultControls: Spieler 1 Tastatur (WASD), Spieler 2 … 4 Gamepad.
func defaultControls() [MaxPlayers]string {
	var c [MaxPlayers]string
	for i := range c {
		c[i] = ProfileGamepad
	}
	c[0] = ProfileWASD
	return c
}

// normalizeControls setzt unbekannte Werte auf den Standard des Platzes und jedes weitere Tastatur-Profil auf Gamepad.
func normalizeControls(c [MaxPlayers]string) [MaxPlayers]string {
	def := defaultControls()
	keyboard := false
	for i, p := range c {
		switch {
		case p != ProfileWASD && p != ProfileArrows && p != ProfileGamepad:
			p = def[i]
			if IsKeyboardProfile(p) && keyboard {
				p = ProfileGamepad
			}
		case IsKeyboardProfile(p) && keyboard:
			p = ProfileGamepad
		}
		keyboard = keyboard || IsKeyboardProfile(p)
		c[i] = p
	}
	return c
}

// KeyboardProfile ist die Belegung der Tastatur: das Tastatur-Profil eines Spielers, ohne eins ProfileWASD (die
// Tastatur bleibt für Menüs und Beitritt bedienbar).
func (s Settings) KeyboardProfile() string {
	for _, p := range s.Controls {
		if IsKeyboardProfile(p) {
			return p
		}
	}
	return ProfileWASD
}

// ProfileOf ist das Profil des Geräts d: Gamepad für Pads, sonst das Tastatur-Profil (KeyboardProfile).
func (s Settings) ProfileOf(d DeviceID) string {
	if d.Kind == DeviceGamepad {
		return ProfileGamepad
	}
	return s.KeyboardProfile()
}

// NextProfile liefert das Profil, das Spieler player beim Blättern um dir (±1, im Kreis) bekommt. Tastatur-Profile
// überspringt es, solange ein anderer Spieler eins hat (höchstens ein Tastatur-Spieler, B-893).
func (s Settings) NextProfile(player, dir int) string {
	if player < 0 || player >= MaxPlayers {
		return ProfileGamepad
	}
	taken := false
	for i, p := range s.Controls {
		taken = taken || (i != player && IsKeyboardProfile(p))
	}
	n := len(Profiles)
	cur := 0
	for i, p := range Profiles {
		if p == s.Controls[player] {
			cur = i
		}
	}
	for step := 1; step <= n; step++ {
		next := Profiles[((cur+step*dir)%n+n)%n]
		if !taken || !IsKeyboardProfile(next) {
			return next
		}
	}
	return ProfileGamepad
}
