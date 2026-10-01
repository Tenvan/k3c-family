package enginetools

import (
	"fmt"
	"maps"
	"slices"
	"strings"

	"k3c/engine/sim"
)

// MaxTicks ist die Obergrenze für sim_run (Entscheidung 🧑 2026-10-01): 30 Ticks/s, gut drei Tag/Nacht-Zyklen.
const MaxTicks = 100_000

// TickHz und der Zeitschritt sind die des Servers (engine/room: sim.Step(w, commands, 1.0/TickHz)).
const TickHz = 30

const maxPlayers = 4

// Segment hält die Eingabe eines Monarchen von FromTick (einschließlich) bis ToTick (ausschließlich) gedrückt.
type Segment struct {
	Player   int     `json:"player,omitempty" jsonschema:"Monarch ab 0, höchstens 3"`
	FromTick int     `json:"fromTick" jsonschema:"erster Tick (einschließlich)"`
	ToTick   int     `json:"toTick" jsonschema:"letzter Tick (ausschließlich)"`
	MoveX    float64 `json:"moveX,omitempty" jsonschema:"-1 (links) bis 1 (rechts)"`
	Sprint   bool    `json:"sprint,omitempty"`
	Pay      bool    `json:"pay,omitempty" jsonschema:"Bezahl-Taste gehalten"`
}

// tally zählt Ereignisse einer Art über den ganzen Lauf.
type tally map[string]int

// Run rechnet `ticks` Ticks mit einem oder mehreren Monarchen (so viele, wie `inputs` nennt, mindestens einer)
// und fasst den Zustand am Ende zusammen. Eine gefallene Burg ist eine Niederlage mit Neustart am Hub (engine/sim
// castleFallen), kein Spielende: der Lauf geht weiter, die Zusammenfassung nennt Zahl und ersten Tick.
func Run(biomeID, seed string, ticks int, inputs []Segment) (string, error) {
	if ticks < 1 || ticks > MaxTicks {
		return "", fmt.Errorf("ticks muss zwischen 1 und %d liegen (30 Ticks pro Sekunde), war %d", MaxTicks, ticks)
	}
	players, err := playerCount(inputs)
	if err != nil {
		return "", err
	}
	b, err := loadBiome(biomeID)
	if err != nil {
		return "", err
	}
	w, err := sim.CreateWorld(b, seed, sim.Options{})
	if err != nil {
		return "", fmt.Errorf("welt nicht erzeugbar: %w", err)
	}
	for range players {
		sim.AddPlayer(w)
	}
	events, firstFall := tally{}, 0
	for tick := 0; tick < ticks; tick++ {
		sim.Step(w, commandsAt(inputs, players, tick), 1.0/TickHz)
		for _, e := range w.Events {
			if kind, _ := e["type"].(string); kind != "" {
				events[kind]++
				if kind == "castleFallen" && firstFall == 0 {
					firstFall = tick + 1
				}
			}
		}
	}
	return summary(w, b.ID, seed, ticks, firstFall, events), nil
}

// playerCount ist die Zahl der Monarchen und prüft die Segmente.
func playerCount(inputs []Segment) (int, error) {
	players := 1
	for i, s := range inputs {
		if s.Player < 0 || s.Player >= maxPlayers {
			return 0, fmt.Errorf("inputs[%d]: player muss zwischen 0 und %d liegen", i, maxPlayers-1)
		}
		if s.ToTick < s.FromTick || s.FromTick < 0 {
			return 0, fmt.Errorf("inputs[%d]: fromTick..toTick ungültig (%d..%d)", i, s.FromTick, s.ToTick)
		}
		players = max(players, s.Player+1)
	}
	return players, nil
}

// commandsAt sammelt die Eingaben aller Segmente, die im Tick gelten; spätere Segmente überschreiben frühere.
func commandsAt(inputs []Segment, players, tick int) []sim.PlayerCommand {
	cmds := make([]sim.PlayerCommand, players)
	for _, s := range inputs {
		if tick >= s.FromTick && tick < s.ToTick {
			cmds[s.Player] = sim.PlayerCommand{MoveX: s.MoveX, Sprint: s.Sprint, Pay: s.Pay}
		}
	}
	return cmds
}

func summary(w *sim.World, biome, seed string, ticks, firstFall int, events tally) string {
	gold := make([]string, len(w.Players))
	for i, p := range w.Players {
		gold[i] = fmt.Sprint(p.Gold)
	}
	troops := tally{}
	for _, t := range w.Troops {
		troops[t.Kind]++
	}
	end := "Ticks erreicht, Burg nie gefallen"
	if firstFall > 0 {
		end = fmt.Sprintf("Ticks erreicht, Burg %d× gefallen (erstmals bei Tick %d)", events["castleFallen"], firstFall)
	}
	return strings.Join([]string{
		fmt.Sprintf("sim_run %s · Seed %q · %d Ticks (%.1f s Spielzeit)", biome, seed, ticks, float64(ticks)/TickHz),
		fmt.Sprintf("Tag %d (%s) · Welle %d · Gegner %d · Burg %.0f HP", w.Cycle.Day, w.Cycle.Phase, w.Wave, len(w.Enemies), w.Castle.HP),
		"Gold je Monarch: " + strings.Join(gold, ", "),
		"Truppen: " + counts(troops),
		fmt.Sprintf("Verluste: %d Gebäude zerstört, %d Monarchen am Boden · Gold gestohlen %d×", events["destroyed"], events["playerDown"], events["goldStolen"]),
		"Ende: " + end,
	}, "\n")
}

// counts schreibt eine Zählung sortiert ("archer 1, peasant 3"), damit der Text deterministisch bleibt.
func counts(m tally) string {
	if len(m) == 0 {
		return "keine"
	}
	parts := make([]string, 0, len(m))
	for _, k := range slices.Sorted(maps.Keys(m)) {
		parts = append(parts, fmt.Sprintf("%s %d", k, m[k]))
	}
	return strings.Join(parts, ", ")
}
