package local

import (
	"fmt"

	"k3c/native/k3c-game/view"
)

// Rebind gibt dem Spieler player das Gerät d (verlorenes Pad, B-798); Raum und Monarch bleiben. Ein Gerät steuert
// höchstens einen Spieler.
func (s *Session) Rebind(player int, d view.DeviceID) error {
	if err := s.seated(player); err != nil {
		return err
	}
	for p, dv := range s.devs {
		if dv == d && p != player {
			return userErr(fmt.Sprintf("%s steuert schon Spieler %d", d, p+1), nil)
		}
	}
	if s.devs[player] == d {
		return nil
	}
	s.devs[player] = d
	s.last = s.devices()
	s.cues = append(s.cues, view.Cue{Kind: view.CueRebound, Player: player, Device: d})
	s.log.Info("🔌 Gerät neu zugeordnet", "player", player, "device", d.String())
	return nil
}

// Learn lernt für den Spieler den Skill; die Simulation prüft Punkte und Voraussetzungen.
func (s *Session) Learn(player int, skill string) error {
	if err := s.seated(player); err != nil {
		return err
	}
	if err := s.r.Learn(deviceID, s.p, player, skill); err != nil {
		return userErr(fmt.Sprintf("Skill „%s“ lässt sich jetzt nicht lernen", skill), err)
	}
	s.log.Info("✅ Skill gelernt", "player", player, "skill", skill)
	return nil
}

// Respec setzt die Skills des Spielers zurück (nur am Tag an der Burg, prüft die Simulation).
func (s *Session) Respec(player int) error {
	if err := s.seated(player); err != nil {
		return err
	}
	if err := s.r.Respec(deviceID, s.p, player); err != nil {
		return userErr("Skills lassen sich nur am Tag an der Burg zurücksetzen", err)
	}
	s.log.Info("✅ Skills zurückgesetzt", "player", player)
	return nil
}

// seated prüft, ob ein Spiel läuft und der Spieler darin spielt.
func (s *Session) seated(player int) error {
	if s.r == nil {
		return errNoGame
	}
	if _, ok := s.devs[player]; !ok {
		return userErr(fmt.Sprintf("Spieler %d spielt nicht", player+1), nil)
	}
	return nil
}
