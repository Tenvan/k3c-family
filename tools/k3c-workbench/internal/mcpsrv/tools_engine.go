package mcpsrv

import (
	"context"
	"encoding/json"
	"time"

	"k3c/engine/simcli"
)

// level_generate und sim_run rechnen im Prozess über engine/simcli (B-924, wie `k3c-sim level` und `k3c-sim run`); eine
// Panik der Engine wird zum Fehler des Tools (simcli.Exec).

// engineTimeout begrenzt einen Aufruf (sim_run mit 100000 Ticks dauert Sekunden).
const engineTimeout = 2 * time.Minute

type levelIn struct {
	Seed  string `json:"seed" jsonschema:"Seed als Text, z. B. 0 oder familie-2026"`
	Biome string `json:"biome,omitempty" jsonschema:"forest (Standard), cave oder mine"`
}

// segment sind gedrückte Tasten eines Monarchen über einen Tick-Bereich (engine/enginetools › Segment).
type segment struct {
	Player   int     `json:"player,omitempty" jsonschema:"Monarch ab 0, höchstens 3"`
	FromTick int     `json:"fromTick" jsonschema:"erster Tick (einschließlich)"`
	ToTick   int     `json:"toTick" jsonschema:"letzter Tick (ausschließlich)"`
	MoveX    float64 `json:"moveX,omitempty" jsonschema:"-1 (links) bis 1 (rechts)"`
	Sprint   bool    `json:"sprint,omitempty"`
	Pay      bool    `json:"pay,omitempty" jsonschema:"Bezahl-Taste gehalten"`
}

type simIn struct {
	Seed   string    `json:"seed" jsonschema:"Seed als Text"`
	Ticks  int       `json:"ticks" jsonschema:"Anzahl Ticks, 1 bis 100000 (30 pro Sekunde)"`
	Biome  string    `json:"biome,omitempty" jsonschema:"forest (Standard), cave oder mine"`
	Inputs []segment `json:"inputs,omitempty" jsonschema:"gedrückte Tasten je Monarch und Tick-Bereich; ohne Angabe ein Monarch ohne Eingaben"`
}

// levelGenerate ist das Tool level_generate (`k3c-sim level`, kein Server nötig).
func (s *Server) levelGenerate(ctx context.Context, in levelIn) (string, error) {
	if err := s.ownCheckoutErr(ctx, "level_generate"); err != nil {
		return "", err
	}
	return s.engineCall(ctx, "level", in)
}

// simRun ist das Tool sim_run (`k3c-sim run`, kein Server nötig); während der Sim-Sperre lehnt es ab (B-788).
func (s *Server) simRun(ctx context.Context, in simIn) (string, error) {
	if err := simLockErr("sim_run"); err != nil {
		return "", err
	}
	if err := s.ownCheckoutErr(ctx, "sim_run"); err != nil {
		return "", err
	}
	return s.engineCall(ctx, "run", in)
}

func (s *Server) engineCall(ctx context.Context, sub string, in any) (string, error) {
	raw, err := json.Marshal(in)
	if err != nil {
		return "", err
	}
	ctx, cancel := context.WithTimeout(ctx, engineTimeout)
	defer cancel()
	ev, err := simcli.Call(ctx, []string{sub}, raw, nil)
	if err != nil {
		return "", err
	}
	return ev.Text, nil
}
