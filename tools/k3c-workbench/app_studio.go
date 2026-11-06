package main

// Lesende Teile des früheren Asset-Studios im Bereich Assets (Entscheidung 005 › 5, Sprint WB1): Sound, Quellen und
// „Im Spiel ansehen“ über internal/studioview. Ändern bleibt Sache der MCP-Tools assets_*.

import (
	"context"
	"log/slog"
	"time"

	"k3c/tools/k3c-workbench/internal/studioview"
)

// studioCloseWait ist die Frist, in der „Im Spiel ansehen“ beim Beenden seine Räume speichert.
const studioCloseWait = 5 * time.Second

// StudioSounds listet die Sound-Slots mit Status und Dateien (Binding).
func (a *App) StudioSounds() ([]studioview.SoundEintrag, error) {
	a.wait()
	return studioview.Sounds(a.root)
}

// StudioSoundData liefert eine Audiodatei unter public/ als Data-URL zum Anhören (Binding).
func (a *App) StudioSoundData(pfad string) (string, error) {
	a.wait()
	return studioview.SoundData(a.root, pfad)
}

// StudioSources liefert Katalog und Recherche-Treffer (Binding).
func (a *App) StudioSources() (studioview.Quellen, error) {
	a.wait()
	return studioview.Sources(a.root)
}

// StudioGameOpen legt den Testspielstand voll-<seed> an und liefert die Adresse zum Ansehen im Spiel (Binding).
func (a *App) StudioGameOpen(seed string) (string, error) {
	a.wait()
	return a.studioGame().Open(seed)
}

// studioGame legt „Im Spiel ansehen“ beim ersten Aufruf an; es lebt so lange wie die Befehle der Oberfläche.
func (a *App) studioGame() *studioview.Game {
	a.mu.Lock()
	defer a.mu.Unlock()
	if a.game == nil {
		ctx := a.svcCtx
		if ctx == nil {
			ctx = context.Background()
		}
		var log *slog.Logger
		if a.log != nil {
			log = a.log.Logger
		}
		a.game = studioview.NewGame(ctx, a.root, "", log)
	}
	return a.game
}

// closeStudio beendet den Listener von „Im Spiel ansehen“ und wartet auf das Speichern der Räume.
func (a *App) closeStudio() {
	a.mu.Lock()
	g := a.game
	a.mu.Unlock()
	if g != nil {
		g.Close(studioCloseWait)
	}
}
