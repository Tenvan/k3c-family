package game

// Menü der Szene im Aufnahme-Modus (--menu, Scene.Menu): nach dem Vorlauf das Hauptmenü, ein Untermenü
// (view.MenuPages) oder die Pause über dem stehenden Spiel; danach rechnet ein Takt den Frame zum Menü.

import (
	"fmt"
	"log/slog"

	"k3c/native/k3c-game/view"
)

// openMenu legt das Menü der Szene auf. main, saves und lobby zeigen kein Spiel: Ein Spiel der Szene endet vorher
// (Session.Close speichert, die Spielstand-Liste zeigt es). pause und options mit Szene liegen über dem pausierten
// Spiel.
func (g *Game) openMenu() error {
	sc := g.args.Scene
	if sc.Menu == "" {
		return nil
	}
	if sc.InGame() {
		if !g.sim.Active() {
			return fmt.Errorf("💥 --menu %s: kein laufendes Spiel", sc.Menu)
		}
		g.stack = Stack{}
		g.sim.Pause(true)
		g.stack.Push(g.parts.Menus.Pause())
	} else if g.sim.Active() {
		if err := g.sim.Close(); err != nil {
			return err
		}
		g.stack = Stack{}
		g.stack.Push(g.parts.Menus.Main())
	}
	g.mode = ModeMenu
	if err := g.openPage(sc); err != nil {
		return err
	}
	slog.Info("🚦 Aufnahme: Menü offen", "menu", sc.Menu, "spiel", sc.InGame())
	if g.advance(); g.err != nil {
		return g.err
	}
	return nil
}

// openPage legt für saves, options und lobby das Untermenü auf (main und pause haben keins).
func (g *Game) openPage(sc Scene) error {
	if sc.Menu == "main" || sc.Menu == "pause" {
		return nil
	}
	pages, ok := g.parts.Menus.(view.MenuPages)
	var page view.Screen
	if ok {
		page = pages.Page(sc.Menu, sc.InGame(), sc.Devices()[0])
	}
	if page == nil {
		return fmt.Errorf("💥 --menu %s: das Menü kennt diese Seite nicht", sc.Menu)
	}
	g.stack.Push(page)
	return nil
}
