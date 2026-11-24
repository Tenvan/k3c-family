package mcpsrv

import (
	"context"
	"errors"
)

// sim_run rechnet mit der Spiel-EXE des Checkouts (`k3c-game sim run`, RU34.2), neben level_generate (tools_engine.go);
// replay_run rechnet bis RU40.3 im Prozess über engine/simcli (B-924).

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
	Head   bool      `json:"head,omitempty" jsonschema:"Spiel mit Fenster öffnen und den Lauf zeigen; ohne Angabe headless"`
	Speed  int       `json:"speed,omitempty" jsonschema:"Zeitraffer 1 bis 64 (Standard 1 = Echtzeit), nur mit head"`
}

// runInput ist die Eingabe-JSON der EXE (`sim run`): ohne head und speed, die kennt sie als Schalter, nicht als Feld.
type runInput struct {
	Seed   string    `json:"seed"`
	Ticks  int       `json:"ticks"`
	Biome  string    `json:"biome,omitempty"`
	Inputs []segment `json:"inputs,omitempty"`
}

// simRun ist das Tool sim_run (`k3c-game sim run [--window]`, kein Server nötig); während der Sim-Sperre lehnt es ab (B-788).
func (s *Server) simRun(ctx context.Context, in simIn) (string, error) {
	if err := simLockErr("sim_run"); err != nil {
		return "", err
	}
	if in.Speed != 0 && !in.Head {
		return "", errors.New("sim_run: speed gilt nur mit head")
	}
	if in.Speed < 0 || in.Speed > 64 {
		return "", errors.New("sim_run: speed muss 1 bis 64 sein")
	}
	limit := engineTimeout
	if in.Head {
		limit = headTimeout
	}
	return s.engineCall(ctx, "run", runInput{Seed: in.Seed, Ticks: in.Ticks, Biome: in.Biome, Inputs: in.Inputs}, in.Head, in.Speed, limit)
}
