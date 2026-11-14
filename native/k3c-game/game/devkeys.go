package game

// Entwickler-Tasten (B-816, B-888): game liest die Tasten der Tabelle view.DevKeys, die keine Geräte-Aktion sind (F9 und
// F11 laufen über input als ActionCheats bzw. ActionFullscreen). F2 und F3 ändern die Einstellungen (Session.SetSettings,
// bleiben erhalten), F1 und F10 den Frame-Zustand view.DevView, F5–F8 die Session (view.DevSession), F12 nimmt das
// nächste Bild auf. Keine Taste gilt im Aufnahme-Modus; dort setzen --river und --stats die Werte.

import (
	"log/slog"

	"k3c/native/k3c-game/view"
	"k3c/native/k3c-game/world"
)

// devKeys wertet die Entwickler-Tasten aus (nicht vor dem ersten Frame, dessen Settings sind noch leer).
func (g *Game) devKeys() {
	if g.args.Shots != "" || g.frame.Settings.Language == "" || g.pressed == nil {
		return
	}
	set := g.frame.Settings
	for _, k := range view.DevKeys {
		if k.Action < 0 && g.pressed(k.Key) {
			g.devCommand(k.Cmd, &set)
		}
	}
	if set == g.frame.Settings {
		return
	}
	if err := g.sim.SetSettings(set); err != nil { // F2 und F3 zugleich: einmal speichern
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
	}
}

// devCommand führt den Befehl einer Entwickler-Taste aus; F2 und F3 ändern set (devKeys speichert). false: game wertet
// ihn nicht selbst aus (Geräte-Aktion).
func (g *Game) devCommand(c view.DevCmd, set *view.Settings) bool {
	switch c {
	case view.DevHelp:
		g.dev.Help = !g.dev.Help
		slog.Info("🐛 Tastenübersicht", "an", g.dev.Help)
	case view.DevDebug:
		g.dev.Debug = view.NextDebug(g.dev.Debug)
		slog.Info("🐛 Debug-Overlay", "seite", g.dev.Debug)
	case view.DevShot:
		g.shotNext = true
	case view.DevStats, view.DevRiver:
		devSettings(c, set)
	case view.DevSave, view.DevLoad, view.DevPause, view.DevSpeed:
		g.devSession(c)
	default:
		return false
	}
	return true
}

// devSettings schaltet Leistungsanzeige bzw. Fluss-Fassung in s.
func devSettings(c view.DevCmd, s *view.Settings) {
	if c == view.DevRiver {
		s.River = s.NextRiver(world.RiverStart())
		slog.Info("🚦 Fluss-Fassung umgeschaltet", "fassung", s.River)
	} else {
		s.Stats = !s.Stats
		slog.Info("🚦 Leistungsanzeige", "an", s.Stats)
	}
}

// devSession führt F5–F8 über die Session aus; ohne laufendes Spiel oder ohne Entwickler-Rolle geschieht nichts.
func (g *Game) devSession(c view.DevCmd) {
	ds, ok := g.sim.(view.DevSession)
	if !ok || !g.sim.Active() {
		slog.Info("🚫 Entwickler-Taste ohne laufendes Spiel", "befehl", int(c))
		return
	}
	var err error
	switch c {
	case view.DevSave:
		if err = g.sim.Save(); err == nil {
			slog.Info("💾 Schnellspeichern")
		}
	case view.DevLoad:
		err = ds.Reload()
	case view.DevPause:
		if ds.Halted() {
			ds.StepOnce()
		} else {
			ds.Halt(true)
		}
	case view.DevSpeed:
		err = g.devSpeed(ds)
	}
	if err != nil {
		slog.Warn("🚫 Entwickler-Taste abgelehnt", "befehl", int(c), "err", err)
		g.cues = append(g.cues, view.Cue{Kind: view.CueDenied, Player: -1, Text: err.Error()})
	}
}

// devSpeed beendet den Halt bzw. schaltet den Zeitraffer eine Stufe weiter (über den ersten Spieler, raumweit).
func (g *Game) devSpeed(ds view.DevSession) error {
	if ds.Halted() {
		ds.Halt(false)
		return nil
	}
	if len(g.frame.Seats) == 0 {
		return nil
	}
	f := view.NextTimescale(g.frame.Timescale)
	return ds.Dev(g.frame.Seats[0].Player, view.DevAction{Action: "timescale", Factor: f})
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
