package mcpsrv

import (
	"context"

	"k3c/tools/k3c-dev/internal/enginetools"
)

type levelIn struct {
	Seed  string `json:"seed" jsonschema:"Seed als Text, z. B. 0 oder familie-2026"`
	Biome string `json:"biome,omitempty" jsonschema:"forest (Standard), cave oder mine"`
}

type simIn struct {
	Seed   string                `json:"seed" jsonschema:"Seed als Text"`
	Ticks  int                   `json:"ticks" jsonschema:"Anzahl Ticks, 1 bis 100000 (30 pro Sekunde)"`
	Biome  string                `json:"biome,omitempty" jsonschema:"forest (Standard), cave oder mine"`
	Inputs []enginetools.Segment `json:"inputs,omitempty" jsonschema:"gedrückte Tasten je Monarch und Tick-Bereich; ohne Angabe ein Monarch ohne Eingaben"`
}

// levelGenerate ist das Tool level_generate (in-process, kein Server nötig).
func (s *Server) levelGenerate(_ context.Context, in levelIn) (string, error) {
	return enginetools.Level(in.Biome, in.Seed)
}

// simRun ist das Tool sim_run (in-process, kein Server nötig).
func (s *Server) simRun(_ context.Context, in simIn) (string, error) {
	return enginetools.Run(in.Biome, in.Seed, in.Ticks, in.Inputs)
}
