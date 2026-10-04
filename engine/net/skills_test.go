package net

import (
	"bytes"
	"context"
	"slices"
	"strconv"
	"testing"
	"time"

	"k3c/engine/sim"
)

// S2.1 (B-123): Schlag, Skills und Aktionen über Protokoll v4.

// (g) Ein Client mit v 3 bekommt version, die Verbindung schließt.
func TestSkillsHandschlagV3(t *testing.T) {
	srv, _ := wsServer(t)
	c := dial(t, srv)
	c.send(map[string]any{"t": "hello", "v": 3, "device": "alt"})
	if m := c.next(); m["code"] != "version" {
		t.Fatalf("v3: %v", m)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	if _, _, err := c.ws.Read(ctx); err == nil {
		t.Fatal("Verbindung nicht geschlossen")
	}
}

// (b) skill außerhalb 0–4 ist bad_request, die ganze Nachricht fällt weg (ack bleibt); attack und skill gültig zählen.
func TestSkillsEingabeUngueltig(t *testing.T) {
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	create(x, "eingabe", 0)
	x.entered()
	input := func(seq int, p map[string]any) {
		x.send(map[string]any{"t": "input", "seq": seq, "p": []map[string]any{p}})
	}
	input(1,map[string]any{"slot": 0, "attack": true, "skill": 1})
	for i, skill := range []int{5, -1} {
		input(2+i,map[string]any{"slot": 0, "skill": skill})
		x.expectError("bad_request")
	}
	m.Room("KRNZ").Tick()
	if d := x.expect("delta", "seats"); d["ack"] != float64(1) {
		t.Fatalf("ack nach abgelehnter Eingabe: %v", d["ack"])
	}
}

// poolSave ist ein frischer Spielstand mit `pool` Punkten im Fund-Pool und ohne gelernte Skills.
func poolSave(t *testing.T, pool int) []byte {
	t.Helper()
	srv, m := wsServer(t)
	x := hello(t, srv, "xbox")
	create(x, "pool", 0)
	x.entered()
	x.send(map[string]any{"t": "leave"})
	x.expect("rooms", "seats")
	waitFor(t, func() bool { _, ok := m.Store.(memSaves).peek("pool"); return ok })
	raw, _ := m.Store.(memSaves).peek("pool")
	out := bytes.Replace(raw, []byte(`"skillPool":0`), []byte(`"skillPool":`+strconv.Itoa(pool)), 1)
	if bytes.Equal(out, raw) {
		t.Fatalf("skillPool nicht im Spielstand: %s", raw)
	}
	return out
}

// player0 ist Spieler 0 aus einem snap oder dem set eines delta.
func player0(s map[string]any) map[string]any {
	list, ok := s["players"].([]any)
	if !ok {
		list, _ = s["players"].(map[string]any)["set"].([]any)
	}
	for _, p := range list {
		if p.(map[string]any)["index"] == float64(0) {
			return p.(map[string]any)
		}
	}
	return nil // nicht im Zustand (delta ohne Änderung)
}

// (c) learn mit freiem Punkt: Skill und points im nächsten Zustand; Fehler (Feld fehlt, unbekannt, fremder Slot,
// keine Punkte) sind bad_request.
func TestSkillsLernenUeberProtokoll(t *testing.T) {
	save := poolSave(t, 1)
	srv, m := wsServer(t)
	m.Store.(memSaves)["pool"] = save
	x := hello(t, srv, "xbox")
	x.send(map[string]any{"t": "create", "save": "pool", "fresh": false, "depth": 0, "slots": []int{0}})
	x.expect("joined")
	x.expect("level")
	p := player0(x.expect("snap")["s"].(map[string]any))
	if p["points"] != float64(1) || !hasAction(p, "learn") {
		t.Fatalf("vor dem Lernen: %v", p)
	}
	for _, bad := range []map[string]any{{"t": "learn", "slot": 0}, {"t": "learn", "skill": "taunt"}, {"t": "respec"},
		{"t": "learn", "slot": 0, "skill": "gibtsnicht"}, {"t": "learn", "slot": 1, "skill": "taunt"}} {
		x.send(bad)
		x.expectError("bad_request")
	}
	x.send(map[string]any{"t": "learn", "slot": 0, "skill": "taunt"})
	learned := func() bool { // die Nachricht läuft in der Lese-Goroutine des Servers: bis zum Zustand mit dem Skill ticken
		m.Room("KRNZ").Tick()
		d := x.expect("delta", "seats")["s"].(map[string]any)
		if d["players"] == nil {
			return false
		}
		p = player0(d)
		return p != nil && p["skills"] != nil
	}
	waitFor(t, learned)
	if skills, _ := p["skills"].([]any); !slices.Contains(skills, any("taunt")) || p["points"] != float64(0) || hasAction(p, "learn") {
		t.Fatalf("nach dem Lernen: %v", p)
	}
	x.send(map[string]any{"t": "learn", "slot": 0, "skill": "shieldBash"})
	x.expectError("bad_request")
}

func hasAction(p map[string]any, action string) bool {
	for _, a := range p["actions"].([]any) {
		if a.(map[string]any)["action"] == action {
			return true
		}
	}
	return false
}

// (f) actions: attack nur bereit und lebend, skill nur für belegte bereite Slots (Nummer 1–4), learn nur mit Punkten.
func TestSkillsAktionsliste(t *testing.T) {
	p := &sim.Player{Skills: []string{"taunt", "shieldBash"}, Slots: []string{"taunt", "shieldBash", ""}, Cooldowns: []float64{0, 3}}
	w := &sim.World{SkillPoints: 2, Players: []*sim.Player{p}}
	got := actionsOf(w, p)
	if len(got) != 2 || got[0]["action"] != "attack" || got[1]["slot"] != 1 || got[1]["skill"] != "taunt" {
		t.Fatalf("ohne Punkte: %v", got)
	}
	w.SkillPoints, p.AttackCooldown = 3, 0.5
	if got := actionsOf(w, p); len(got) != 2 || got[0]["action"] != "skill" || got[1]["action"] != "learn" {
		t.Fatalf("Schlag abklingend, Punkt frei: %v", got)
	}
	p.RespawnIn = 4
	if got := actionsOf(w, p); len(got) != 1 || got[0]["action"] != "learn" {
		t.Fatalf("tot: %v", got)
	}
}
