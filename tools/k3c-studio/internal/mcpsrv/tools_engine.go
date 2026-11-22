package mcpsrv

import (
	"context"
	"encoding/json"
	"time"

	"k3c/engine/simcli"
)

// level_generate rechnet im Prozess über engine/simcli (B-924, wie `k3c-sim level`); eine Panik der Engine wird zum
// Fehler des Tools (simcli.Exec). sim_run und replay_run bleiben in k3c-workbench.

// engineTimeout begrenzt einen Aufruf.
const engineTimeout = 2 * time.Minute

type levelIn struct {
	Seed  string `json:"seed" jsonschema:"Seed als Text, z. B. 0 oder familie-2026"`
	Biome string `json:"biome,omitempty" jsonschema:"forest (Standard), cave oder mine"`
}

// levelGenerate ist das Tool level_generate (`k3c-sim level`, kein Server nötig).
func (s *Server) levelGenerate(ctx context.Context, in levelIn) (string, error) {
	if err := s.ownCheckoutErr(ctx, "level_generate"); err != nil {
		return "", err
	}
	raw, err := json.Marshal(in)
	if err != nil {
		return "", err
	}
	ctx, cancel := context.WithTimeout(ctx, engineTimeout)
	defer cancel()
	ev, err := simcli.Call(ctx, []string{"level"}, raw, nil)
	if err != nil {
		return "", err
	}
	return ev.Text, nil
}
