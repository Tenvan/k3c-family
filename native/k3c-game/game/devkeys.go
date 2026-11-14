package game

// Entwickler-Tasten (B-816): F3 schaltet den Fluss zwischen Kacheln und Shader, F2 die Leistungsanzeige. Beide Werte
// stehen in den Einstellungen (Session.SetSettings, bleiben erhalten); es gibt keinen Eintrag im Optionen-Menü. Wie das
// Vollbild gelten die Tasten nicht im Aufnahme-Modus; dort setzen --river und --stats die Werte.

import (
	"log/slog"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/world"
)

// devKeys wertet F3 und F2 aus und speichert die geänderten Einstellungen (nicht vor dem ersten Frame, dessen
// Settings sind noch leer).
func (g *Game) devKeys() {
	if g.args.Shots != "" || g.frame.Settings.Language == "" || g.pressed == nil {
		return
	}
	s := g.frame.Settings
	changed := false
	if g.pressed(ebiten.KeyF3) {
		s.River, changed = s.NextRiver(world.RiverStart()), true
		slog.Info("🚦 Fluss-Fassung umgeschaltet", "fassung", s.River)
	}
	if g.pressed(ebiten.KeyF2) {
		s.Stats, changed = !s.Stats, true
		slog.Info("🚦 Leistungsanzeige", "an", s.Stats)
	}
	if !changed {
		return
	}
	if err := g.sim.SetSettings(s); err != nil {
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
	}
}

// applyDevArgs übernimmt --river und --stats in die Einstellungen (Aufnahme der beiden Fassungen und der Anzeige).
func (g *Game) applyDevArgs() {
	if g.args.River == "" && !g.args.Stats {
		return
	}
	s := g.sim.Settings()
	if g.args.River != "" {
		s.River = g.args.River
	}
	s.Stats = s.Stats || g.args.Stats
	if err := g.sim.SetSettings(s); err != nil {
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
	}
}
