package main

import (
	"encoding/json"
	"strings"
	"testing"

	"k3c/engine/balance"
	"k3c/engine/level"
	"k3c/engine/sim"
)

// RS2.2: bot entscheidet auf dem gesendeten Zustand wie das Profil im Prozess (je Anfrage eine Antwort, Reihenfolge
// der Anfrage, fehlende Monarchen ohne Kommando).
func TestBotGleichProfil(t *testing.T) {
	b, err := level.LoadBiome("forest")
	if err != nil {
		t.Fatal(err)
	}
	w, err := sim.CreateWorld(b, "k3c-dev", sim.Options{})
	if err != nil {
		t.Fatal(err)
	}
	sim.AddPlayer(w)
	sim.AddPlayer(w)
	for range 90 {
		sim.Step(w, nil, 1.0/30)
	}
	state, err := json.Marshal(w)
	if err != nil {
		t.Fatal(err)
	}
	req, _ := json.Marshal(botRequest{State: state, Players: []int{1, 0, 7}})
	code, out, errOut := call(t, string(req)+"\n"+string(req)+"\n", "bot", "--bot", "coop2")
	if code != exitOK {
		t.Fatalf("exit %d: %s", code, errOut)
	}
	lines := strings.Split(strings.TrimSpace(out), "\n")
	if len(lines) != 2 {
		t.Fatalf("zwei Antworten erwartet: %q", out)
	}
	var got botReply
	if err := json.Unmarshal([]byte(lines[0]), &got); err != nil || got.Event != "commands" || len(got.Commands) != 2 {
		t.Fatalf("antwort: %v %+v", err, got)
	}
	var dec sim.World
	_ = json.Unmarshal(state, &dec)
	bot, _ := balance.BotByName("coop2")
	for i, idx := range []int{1, 0} {
		c := bot(&dec, dec.Players[idx])
		want := botCommand{Player: idx, MoveX: c.MoveX, Sprint: c.Sprint, Pay: c.Pay, Attack: c.Attack, Skill: c.Skill}
		if got.Commands[i] != want {
			t.Errorf("monarch %d: %+v, erwartet %+v", idx, got.Commands[i], want)
		}
	}
}

func TestBotFehler(t *testing.T) {
	if code, _, _ := call(t, "", "bot", "--bot", "raten"); code != exitUsage {
		t.Errorf("unbekannter Bot: exit %d", code)
	}
	if code, out, _ := call(t, "kein json\n", "bot", "--bot", "saver"); code != exitFailed || !strings.Contains(out, `"error"`) {
		t.Errorf("kaputte Anfrage: exit %d %q", code, out)
	}
	if code, out, _ := call(t, "", "bot", "--bot", "saver"); code != exitOK || out != "" {
		t.Errorf("leere Eingabe: exit %d %q", code, out)
	}
}
