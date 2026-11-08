package main

import (
	"bufio"
	"encoding/json"
	"flag"
	"fmt"
	"io"

	"k3c/engine/balance"
	"k3c/engine/sim"
)

// Unterbefehl bot (RS2.2): Entscheider der Bot-Geräte der Workbench (sim_test online, botdev). Die Workbench hält
// Verbindung und Zustand der Stufe selbst und fragt je Zustand an; k3c-sim dekodiert ihn in eine sim.World und lässt
// das Profil entscheiden. Eine Anfrage je Zeile, eine Antwort je Zeile, bis stdin endet (Exit 0).
//
//	stdin:  {"state":{…Zustand der Stufe wie snap/delta…},"players":[0,2]}
//	stdout: {"event":"commands","commands":[{"player":0,"moveX":1,"sprint":false,"pay":true,"attack":false,"skill":0}]}
//
// Monarchen aus "players", die im Zustand fehlen, bekommen kein Kommando.

// botRequest ist eine Anfrage.
type botRequest struct {
	State   json.RawMessage `json:"state"`
	Players []int           `json:"players"`
}

// botCommand ist das Kommando eines Monarchen; Felder wie sim.PlayerCommand, ohne omitempty.
type botCommand struct {
	Player int     `json:"player"`
	MoveX  float64 `json:"moveX"`
	Sprint bool    `json:"sprint"`
	Pay    bool    `json:"pay"`
	Attack bool    `json:"attack"`
	Skill  int     `json:"skill"`
}

type botReply struct {
	Event    string       `json:"event"`
	Commands []botCommand `json:"commands"`
}

func cmdBot(args []string, v env) error {
	fs := flag.NewFlagSet("k3c-sim bot", flag.ContinueOnError)
	fs.SetOutput(io.Discard)
	name := fs.String("bot", "", "Bot-Profil oder Mischung je Spieler (saver+passive)")
	if err := fs.Parse(args); err != nil {
		return usageError{err}
	}
	if fs.NArg() > 0 {
		return usagef("unerwartete Argumente: %v", fs.Args())
	}
	bot, ok := balance.BotByName(*name)
	if !ok {
		return usagef("unbekannter Bot %q (bekannt: %v, je Spieler mit %s)", *name, balance.BotNames(), balance.MixSep)
	}
	sc := bufio.NewScanner(v.stdin)
	sc.Buffer(make([]byte, 0, 1<<20), 64<<20) // ein Zustand mit Level ist groß
	for sc.Scan() {
		reply, err := decide(bot, sc.Bytes())
		if err != nil {
			return err
		}
		v.out.emit(reply)
	}
	return sc.Err()
}

// decide beantwortet eine Anfrage.
func decide(bot balance.Bot, line []byte) (botReply, error) {
	var req botRequest
	if err := json.Unmarshal(line, &req); err != nil {
		return botReply{}, fmt.Errorf("anfrage nicht lesbar: %w", err)
	}
	var w sim.World
	if err := json.Unmarshal(req.State, &w); err != nil {
		return botReply{}, fmt.Errorf("zustand nicht lesbar: %w", err)
	}
	reply := botReply{Event: "commands", Commands: []botCommand{}}
	for _, idx := range req.Players {
		for _, p := range w.Players {
			if p.Index != idx {
				continue
			}
			c := bot(&w, p)
			reply.Commands = append(reply.Commands, botCommand{Player: idx, MoveX: c.MoveX, Sprint: c.Sprint, Pay: c.Pay,
				Attack: c.Attack, Skill: c.Skill})
			break
		}
	}
	return reply, nil
}
