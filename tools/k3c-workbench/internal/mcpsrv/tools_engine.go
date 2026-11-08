package mcpsrv

import (
	"context"
	"encoding/json"
	"time"

	"k3c/tools/k3c-workbench/internal/simexe"
)

// level_generate und sim_run rechnen im Kindprozess bin/k3c-sim (RS2.2, B-726); der Text ist byte-gleich zum
// früheren Aufruf im Prozess (cmd/k3c-sim/main_test.go).

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
	return s.engineCall(ctx, "level", in)
}

// simRun ist das Tool sim_run (`k3c-sim run`, kein Server nötig).
func (s *Server) simRun(ctx context.Context, in simIn) (string, error) {
	return s.engineCall(ctx, "run", in)
}

func (s *Server) engineCall(ctx context.Context, sub string, in any) (string, error) {
	raw, err := json.Marshal(in)
	if err != nil {
		return "", err
	}
	ev, err := simexe.Call(ctx, s.ws(ctx).root, []string{sub}, raw, engineTimeout)
	if err != nil {
		return "", err
	}
	return ev.Text, nil
}
