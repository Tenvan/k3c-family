package view

import "testing"

// controls baut Settings.Controls aus den ersten Werten, der Rest ist Gamepad.
func controls(ps ...string) [MaxPlayers]string {
	var c [MaxPlayers]string
	for i := range c {
		c[i] = ProfileGamepad
		if i < len(ps) {
			c[i] = ps[i]
		}
	}
	return c
}

// Standard: Spieler 1 WASD, alle anderen Gamepad; unbekannte Werte fallen auf den Standard des Platzes.
func TestDefaultControls(t *testing.T) {
	if got := DefaultSettings().Controls; got != controls(ProfileWASD) {
		t.Errorf("Standard %v", got)
	}
	var unknown [MaxPlayers]string
	unknown[0] = "joystick"
	if got := normalizeControls(unknown); got != controls(ProfileWASD) {
		t.Errorf("unbekannt %v", got)
	}
}

// B-893/AC-02: Zwei Spieler haben nie beide ein Tastatur-Profil: Normalized macht das spätere zum Gamepad,
// NextProfile überspringt Tastatur-Profile, solange ein anderer Spieler eins hat.
func TestOneKeyboardPlayer(t *testing.T) {
	both := controls(ProfileWASD, ProfileArrows)
	if got := normalizeControls(both); got != controls(ProfileWASD, ProfileGamepad) {
		t.Errorf("zwei Tastatur-Profile normalisiert zu %v", got)
	}
	s := Settings{Controls: controls(ProfileWASD)}
	for _, dir := range []int{1, -1} {
		if got := s.NextProfile(1, dir); got != ProfileGamepad {
			t.Errorf("Spieler 2 blättert (%d) zu %q, Tastatur ist vergeben", dir, got)
		}
	}
	if got := s.NextProfile(0, 1); got != ProfileArrows {
		t.Errorf("Spieler 1 WASD → %q, erwartet pfeile", got)
	}
	if got := s.NextProfile(0, -1); got != ProfileGamepad {
		t.Errorf("Spieler 1 WASD rückwärts → %q, erwartet gamepad", got)
	}
	free := Settings{Controls: controls(ProfileGamepad)}
	if got := free.NextProfile(1, 1); got != ProfileWASD {
		t.Errorf("Spieler 2 bei freier Tastatur → %q, erwartet wasd", got)
	}
	if got := free.NextProfile(MaxPlayers, 1); got != ProfileGamepad {
		t.Errorf("Spieler außerhalb → %q", got)
	}
}

// Die Tastatur folgt dem Tastatur-Profil; ohne eins bleibt sie WASD. Pads haben immer das Profil gamepad.
func TestKeyboardProfileAndProfileOf(t *testing.T) {
	cases := []struct {
		c    [MaxPlayers]string
		want string
	}{
		{controls(ProfileWASD), ProfileWASD},
		{controls(ProfileGamepad, ProfileArrows), ProfileArrows},
		{controls(ProfileGamepad), ProfileWASD},
	}
	for _, c := range cases {
		s := Settings{Controls: c.c}
		if got := s.KeyboardProfile(); got != c.want {
			t.Errorf("%v: Tastatur %q, erwartet %q", c.c, got, c.want)
		}
		if got := s.ProfileOf(DeviceID{Kind: DeviceKeyboard}); got != c.want {
			t.Errorf("%v: ProfileOf(keys1) %q", c.c, got)
		}
		if got := s.ProfileOf(DeviceID{Kind: DeviceGamepad, Index: 3}); got != ProfileGamepad {
			t.Errorf("%v: ProfileOf(pad) %q", c.c, got)
		}
	}
}
