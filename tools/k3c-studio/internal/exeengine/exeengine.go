// Package exeengine ist die Engine-Seite der Studio-Tests: `k3c-game sim world|hub` über die Spiel-EXE der Repo-Wurzel
// mit den Daten eines Fixture-Checkouts (gameexe.Options.Data). Es ersetzt seit RU40.4 die Go-Engine im Prozess
// (engine/simcli entfernt; Entscheidung 012: Orakel ist das Verhalten). Fehlt die EXE oder ist sie veraltet (gameexe.Find),
// überspringen die Tests mit dem Bau-Befehl im Grund (CI baut sie im Studio-Job nicht). Nur Tests importieren das Paket.
package exeengine

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"sync"
	"testing"

	"k3c/tools/k3c-studio/internal/gameexe"
)

var (
	once   sync.Once
	root   string // Repo-Wurzel mit frischer EXE; leer = nicht verfügbar
	reason string // Grund, wenn nicht verfügbar
)

// probe sucht die Repo-Wurzel (Ordner mit Taskfile.yml und rust/Cargo.toml) ab dem Arbeitsordner aufwärts und prüft dort
// die EXE.
func probe() {
	dir, err := os.Getwd()
	for err == nil {
		if isRepoRoot(dir) {
			break
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			err = errors.New("Repo-Wurzel nicht gefunden")
			break
		}
		dir = parent
	}
	if err != nil {
		reason = err.Error()
		return
	}
	if _, _, err := gameexe.Find(dir); err != nil {
		reason = err.Error()
		return
	}
	root = dir
}

func isRepoRoot(dir string) bool {
	for _, f := range []string{"Taskfile.yml", filepath.Join("rust", "Cargo.toml")} {
		if _, err := os.Stat(filepath.Join(dir, f)); err != nil {
			return false
		}
	}
	return true
}

// Available liefert die Repo-Wurzel mit frischer EXE oder den Grund, warum sie fehlt.
func Available() (string, error) {
	once.Do(probe)
	if root == "" {
		return "", errors.New(reason)
	}
	return root, nil
}

// Require überspringt den Test ohne frische EXE.
func Require(t testing.TB) {
	t.Helper()
	if _, err := Available(); err != nil {
		t.Skip("k3c-game.exe nicht verfügbar: " + err.Error())
	}
}

// For ist Engine für einen Test und überspringt t sofort ohne EXE. Bewusst nicht erst beim ersten Engine-Aufruf: ein
// t.Skip mitten im Editor (Goexit) ließe dessen Sperren liegen und der nächste Test hinge.
func For(t testing.TB, sub, fixture string) func(ctx context.Context, input []byte) (json.RawMessage, error) {
	t.Helper()
	Require(t)
	return Engine(sub, fixture)
}

// Engine liefert die Engine-Seite für den Unterbefehl sub (world oder hub) mit den Daten unter fixture/data; sie passt auf
// world.Engine und hubplan.Engine. Ohne EXE liefert jeder Aufruf den Grund als Fehler (Tests rufen vorher Require).
func Engine(sub, fixture string) func(ctx context.Context, input []byte) (json.RawMessage, error) {
	return func(ctx context.Context, input []byte) (json.RawMessage, error) {
		r, err := Available()
		if err != nil {
			return nil, err
		}
		data, err := filepath.Abs(filepath.Join(fixture, "data")) // die EXE läuft in ihrem eigenen Arbeitsordner
		if err != nil {
			return nil, err
		}
		ev, err := gameexe.Call(ctx, gameexe.Options{Root: r, Data: data, Sub: sub, Input: input})
		return ev.Report, err
	}
}
