package hud

import (
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// B-894/AC-05: Ein Wechsel des Eingabe-Profils in den Einstellungen ändert die Hinweise sofort: Nach dem nächsten
// Listen zeigt das offene Skill-Menü des Tastatur-Spielers den Bedienhinweis des neuen Profils, der Pad-Spieler
// bleibt beim Xbox-Hinweis; der Beitritts-Hinweis nennt die Bestätigen-Taste des Profils.
func TestProfileSwitchChangesHints(t *testing.T) {
	w, f := twoPlayers(t, 1)
	f.Settings = view.DefaultSettings()
	h := New(nil).(*hud)
	s := &session{w: w}
	h.Update(input(acts(view.ActionSkillMenu), acts(view.ActionSkillMenu)), f, s)
	h.Listen(f)
	if got := h.seats[0].menuV.footer; !strings.HasPrefix(got, "A / D") {
		t.Fatalf("WASD: %q", got)
	}
	f.Settings.Controls[0] = view.ProfileArrows
	f.Unseated = []view.DeviceID{{Kind: view.DeviceGamepad, Index: 9}, keys1}
	h.Listen(f)
	if got := h.seats[0].menuV.footer; !strings.HasPrefix(got, "Pfeil links / Pfeil rechts") {
		t.Errorf("nach Wechsel auf Pfeile: %q", got)
	}
	if got := h.seats[1].menuV.footer; got != "◄ ► wählen · A bestätigen · B schließen" {
		t.Errorf("Pad bleibt: %q", got)
	}
	if strings.Contains(h.joinS, "Leertaste") || !strings.Contains(h.joinS, "Enter") {
		t.Errorf("Beitritt im Profil pfeile: %q", h.joinS)
	}
}
