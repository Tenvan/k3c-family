package main

// Welten-Editor (B-879, Port aus dem früheren Asset-Studio): Welten unter data/worlds/ ansehen, das Level einer
// Insel-Stufe als Chunk-Vorschau holen und Werte einer Welt übersteuern (overrides, B-680) über internal/world. Level,
// Basis-Werte und Prüfung rechnet bin/k3c-sim world (B-726); es laufen keine Simulationen (Sperre 2026-11-11).

import (
	"context"
	"encoding/json"
	"strings"

	"k3c/tools/k3c-workbench/internal/world"
)

// worldEngine ist die Engine-Seite des Editors (bin/k3c-sim world); Tests setzen einen Ersatz.
var worldEngine = world.SimExe

// worlds ist der Welten-Editor des Checkouts.
func (a *App) worlds() world.Editor { return world.Editor{Root: a.root, Engine: worldEngine(a.root)} }

// worldCtx endet mit der Workbench; vor startup gilt der Hintergrund.
func (a *App) worldCtx() context.Context {
	if a.svcCtx != nil {
		return a.svcCtx
	}
	return context.Background()
}

// WorldList liefert die Welten, die Standard-Welt zuerst (Binding).
func (a *App) WorldList() []world.Info {
	a.wait()
	return a.worlds().List()
}

// WorldDetail liefert Kopf und Inseln der Welt id (Binding).
func (a *App) WorldDetail(id string) world.Detail {
	a.wait()
	return a.worlds().Load(id)
}

// WorldLevel holt das Level der Insel island in der Stufe stage zum Seed von bin/k3c-sim (Binding).
func (a *App) WorldLevel(id, island string, stage int, seed string) world.LevelView {
	a.wait()
	return a.worlds().Level(a.worldCtx(), id, island, stage, seed)
}

// WorldValues liefert die Werte der Welt mit Basiswert und Override je Feld (Binding).
func (a *App) WorldValues(id string) world.ValuesView {
	a.wait()
	return a.worlds().Values(a.worldCtx(), id)
}

// WorldOverrideSet übersteuert ein Feld der Welt; null setzt es zurück (Binding). Geschrieben wird nur eine gültige Welt.
func (a *App) WorldOverrideSet(id, file string, path []string, value json.RawMessage) world.ValuesView {
	a.wait()
	view := a.worlds().SetOverride(a.worldCtx(), id, file, path, value)
	if a.log != nil {
		field := file + "." + strings.Join(path, ".")
		if view.Error != "" {
			a.log.Warn("🚫 Override abgelehnt", "ns", "dev", "world", id, "field", field, "error", view.Error)
		} else {
			a.log.Info("💾 Override gespeichert", "ns", "dev", "world", id, "field", field)
		}
	}
	return view
}
