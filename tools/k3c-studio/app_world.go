package main

// Welten-Editor (B-879, Port aus dem früheren Asset-Studio): Welten unter data/worlds/ ansehen, das Level einer
// Insel-Stufe als Chunk-Vorschau holen und Werte einer Welt übersteuern (overrides, B-680) über internal/world. Level,
// Basis-Werte und Prüfung rechnet simcli world (B-726); es laufen keine Simulationen (Sperre 2026-11-11).

import (
	"context"
	"encoding/json"
	"strings"
	"time"

	"k3c/tools/k3c-studio/internal/world"
)

// worldEngine ist die Engine-Seite des Editors (simcli world); Tests setzen einen Ersatz.
var worldEngine = world.Direct

// worlds ist der Welten-Editor des Checkouts.
func (a *App) worlds() world.Editor { return world.Editor{Root: a.root, Engine: worldEngine(a.root)} }

// worldCtx endet mit dem Studio; vor startup gilt der Hintergrund.
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

// WorldLevel holt das Level der Insel island in der Stufe stage zum Seed der Engine (Binding).
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

// WorldCreate legt eine neue Welt an (Binding, B-954): eine Insel, Tiefen 0-4, optional von extends erbend.
func (a *App) WorldCreate(id, name, extends string) world.CreateView {
	a.wait()
	view := a.worlds().CreateView(a.worldCtx(), id, name, extends)
	if a.log != nil {
		if view.Error != "" {
			a.log.Warn("🚫 Welt nicht angelegt", "ns", "dev", "world", id, "error", view.Error)
		} else {
			a.log.Info("💾 Welt angelegt", "ns", "dev", "world", id)
		}
	}
	return view
}

// WorldDelete löscht die Welt id (Binding, B-1172): feste Welten und Welten mit Erben werden abgelehnt (error), Spielstände
// der Welt bleiben und stehen als Warnung in der Antwort. Die Oberfläche fragt vorher nach dem Namen der Welt.
func (a *App) WorldDelete(id string) world.CreateView {
	a.wait()
	view := a.worlds().DeleteView(id)
	if a.log != nil {
		if view.Error != "" {
			a.log.Warn("🚫 Welt nicht gelöscht", "ns", "dev", "world", id, "error", view.Error)
		} else {
			a.log.Info("🧹 Welt gelöscht", "ns", "dev", "world", id)
		}
	}
	return view
}

// WorldNatureSet setzt einen Natur-Wert der Insel-Stufe fest; value null setzt ihn auf „aus Seed“ zurück (Binding,
// B-891). Geschrieben wird nur eine gültige Welt; die Antwort ist das neue Level zum Seed.
func (a *App) WorldNatureSet(id, island string, stage int, key string, value *int, seed string) world.LevelView {
	a.wait()
	view := a.worlds().SetNature(a.worldCtx(), id, island, stage, key, value, seed)
	if a.log != nil {
		if view.Rejected != "" {
			a.log.Warn("🚫 Natur-Wert abgelehnt", "ns", "dev", "world", id, "island", island, "key", key, "error", view.Rejected)
		} else {
			a.log.Info("💾 Natur-Wert gespeichert", "ns", "dev", "world", id, "island", island, "key", key)
		}
	}
	return view
}

// WorldSetKind setzt die Art der Welt: "spinoff" (Ableger oder Episode) oder "new" (komplett neu); leer entfernt sie
// (Binding, B-1217). Geschrieben wird nur eine gültige Welt.
func (a *App) WorldSetKind(id, kind string) world.CreateView {
	a.wait()
	view := a.worlds().SetKindView(a.worldCtx(), id, kind)
	if a.log != nil {
		if view.Error != "" {
			a.log.Warn("🚫 Art der Welt abgelehnt", "ns", "dev", "world", id, "kind", kind, "error", view.Error)
		} else {
			a.log.Info("💾 Art der Welt gespeichert", "ns", "dev", "world", id, "kind", kind)
		}
	}
	return view
}

// WorldSetExtends setzt die Eltern-Welt einer Welt; leer entfernt sie (Binding, B-1217).
func (a *App) WorldSetExtends(id, parent string) world.CreateView {
	a.wait()
	view := a.worlds().SetExtendsView(a.worldCtx(), id, parent)
	if a.log != nil {
		if view.Error != "" {
			a.log.Warn("🚫 Eltern-Welt abgelehnt", "ns", "dev", "world", id, "parent", parent, "error", view.Error)
		} else {
			a.log.Info("💾 Eltern-Welt gespeichert", "ns", "dev", "world", id, "parent", parent)
		}
	}
	return view
}

// WorldIslandEtappe setzt die Etappe einer Insel; null entfernt sie (Binding, B-963). Geschrieben wird nur eine gültige Welt.
func (a *App) WorldIslandEtappe(id, island string, etappe *int) world.CreateView {
	a.wait()
	view := a.worlds().SetEtappeView(a.worldCtx(), id, island, etappe)
	if a.log != nil {
		if view.Error != "" {
			a.log.Warn("🚫 Etappe abgelehnt", "ns", "dev", "world", id, "island", island, "error", view.Error)
		} else {
			a.log.Info("💾 Etappe gespeichert", "ns", "dev", "world", id, "island", island)
		}
	}
	return view
}

// worldPoll ist der Takt des Welten-Wächters (B-1216): Änderungen an data/worlds/ und den Basis-Dateien erreichen die
// Oberfläche als Ereignis `world:changed`, egal ob Editor, MCP oder Hand sie schrieben.
const worldPoll = time.Second

// WorldPicks liefert Wetter, Reittier und Einsiedler der Welt id samt gültigen IDs (Binding, SI7.1).
func (a *App) WorldPicks(id string) world.PicksView {
	a.wait()
	return a.worlds().PicksView(id)
}

// WorldSetPick setzt Wetter (weather), Reittier (mount) oder Einsiedler (hermit) der Welt id; "" entfernt den Eintrag
// (Binding, SI7.1). Geschrieben wird nur der Block worlds der Katalog-Datei.
func (a *App) WorldSetPick(id, key, value string) world.CreateView {
	a.wait()
	view := a.worlds().SetPickView(id, key, value)
	if a.log != nil {
		if view.Error != "" {
			a.log.Warn("🚫 Auswahl abgelehnt", "ns", "dev", "world", id, "art", key, "error", view.Error)
		} else {
			a.log.Info("💾 Auswahl gespeichert", "ns", "dev", "world", id, "art", key, "wert", value)
		}
	}
	return view
}
