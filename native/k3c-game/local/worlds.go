package local

import (
	"io/fs"
	"os"
)

// WorldsDirEnv nennt den Ordner der Welt-Dateien (wie room.Manager.Worlds, K3C_WORLDS_DIR); ohne ihn gilt data/worlds,
// falls es neben dem Arbeitsordner liegt. Gelesen wird er nur im Dev-Mode (Settings.DevMode), sonst gilt nur die
// Standard-Welt (engine/room).
const WorldsDirEnv = "K3C_WORLDS_DIR"

// worldsFS liefert den Ordner der lokalen Welten oder nil, wenn es keinen gibt.
func worldsFS() fs.FS {
	dir := os.Getenv(WorldsDirEnv)
	if dir == "" {
		dir = "data/worlds"
	}
	if info, err := os.Stat(dir); err != nil || !info.IsDir() {
		return nil
	}
	return os.DirFS(dir)
}
