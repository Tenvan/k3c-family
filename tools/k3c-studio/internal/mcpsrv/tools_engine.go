package mcpsrv

import (
	"context"
	"encoding/json"
	"time"

	"k3c/tools/k3c-studio/internal/gameexe"
)

// level_generate und sim_run rechnen mit der Spiel-EXE des Checkouts (`k3c-game sim`, RU34, docs/standards/k3c-game-laeufe.md)
// über internal/gameexe; ist sie nicht gebaut oder veraltet, nennt der Fehler den Bau-Befehl. sim_run steht in tools_sim.go.

// engineTimeout begrenzt einen Aufruf (auch sim_run, 100000 Ticks dauern Sekunden).
const engineTimeout = 2 * time.Minute

// headTimeout begrenzt einen Lauf mit Fenster (sim_run head): man sieht zu und kann das Fenster schließen.
const headTimeout = 60 * time.Minute

type levelIn struct {
	Seed  string `json:"seed" jsonschema:"Seed als Text, z. B. 0 oder familie-2026"`
	Biome string `json:"biome,omitempty" jsonschema:"forest (Standard), cave oder mine"`
}

// levelGenerate ist das Tool level_generate (`k3c-game sim level`, kein Server nötig); es rechnet mit der EXE des Checkouts.
func (s *Server) levelGenerate(ctx context.Context, in levelIn) (string, error) {
	return s.engineCall(ctx, "level", in, false, 0, engineTimeout)
}

// gameCall ist der Aufruf der Spiel-EXE; Tests ersetzen ihn.
var gameCall = gameexe.Call

// engineCall startet `k3c-game sim <sub>` aus dem Checkout des Aufrufs (RU34.2) und liefert den Text der Zeile result.
// head öffnet das Spiel mit Fenster (nur run), speed ist der Zeitraffer dazu.
func (s *Server) engineCall(ctx context.Context, sub string, in any, head bool, speed int, limit time.Duration) (string, error) {
	raw, err := json.Marshal(in)
	if err != nil {
		return "", err
	}
	ctx, cancel := context.WithTimeout(ctx, limit)
	defer cancel()
	ev, err := gameCall(ctx, gameexe.Options{Root: s.ws(ctx).Root, Sub: sub, Input: raw, Window: head, Speed: speed})
	if err != nil {
		return "", err
	}
	return ev.Text, nil
}
