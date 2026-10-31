package net

import (
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// B-675/AC-01: Der Zustand trägt Kontrolleffekte der Gegner (stun, slow, taunt), Schild und Last Stand des Monarchen,
// seine wirkenden Passive (auras) und die Stürme mit Restdauer, mit den Schlüsseln aus s2c-snapshot-status.json.
func TestStatusEffekteImZustand(t *testing.T) {
	w := statusWorld(t)
	s := stateOf(w, 0, false)
	ex := want(t, "snapshot-status")["s"].(map[string]any)
	gotP := s["players"].([]any)[0].(map[string]any)
	wantP := ex["players"].(map[string]any)["set"].([]any)[0].(map[string]any)
	for _, k := range []string{"shield", "shieldFor", "lastStandFor", "auras"} {
		if _, ok := wantP[k]; !ok {
			t.Fatalf("Beispiel ohne players[].%s", k)
		}
		if _, ok := gotP[k]; !ok {
			t.Fatalf("Zustand ohne players[].%s: %v", k, gotP)
		}
	}
	if d := sameKeys("auras", wantP["auras"].([]any)[1], gotP["auras"].([]any)[1], false); d != "" {
		t.Error(d)
	}
	auras := gotP["auras"].([]any)
	if len(auras) != 3 || auras[0].(map[string]any)["skill"] != "armorAura" || auras[0].(map[string]any)["radius"] != 8.0 ||
		auras[1].(map[string]any)["castle"] != true || auras[2].(map[string]any)["radius"] != 0.0 {
		t.Fatalf("auras %v (thickSkin ohne Wirkort fehlt, frostArmor mit Radius 0)", auras)
	}
	if _, ok := s["players"].([]any)[1].(map[string]any)["auras"]; ok {
		t.Fatal("Spieler 2 ohne Passive trägt auras")
	}
	gotE := s["enemies"].([]any)[0].(map[string]any)
	for _, k := range []string{"stun", "slow", "slowFor", "tauntId", "tauntFor"} {
		if gotE[k] == nil {
			t.Fatalf("Zustand ohne enemies[].%s: %v", k, gotE)
		}
	}
	storm := s["storms"].([]any)[0]
	if d := sameKeys("storms", ex["storms"].([]any)[0], storm, false); d != "" || storm.(map[string]any)["left"] != 4.5 {
		t.Fatalf("%s %v", d, storm)
	}
}

// statusWorld: Spieler 1 mit Schild, Last Stand und Passiven, ein Gegner mit allen Kontrolleffekten, ein Sturm;
// Spieler 2 ohne Effekte.
func statusWorld(t *testing.T) *sim.World {
	t.Helper()
	b, err := level.LoadBiome("forest")
	if err != nil {
		t.Fatal(err)
	}
	w, err := sim.CreateWorld(b, "status", sim.Options{CycleSpeed: 1})
	if err != nil {
		t.Fatal(err)
	}
	p := sim.AddPlayer(w)
	sim.AddPlayer(w).Skills = nil
	p.Skills = []string{"thickSkin", "armorAura", "holyGround", "frostArmor"}
	p.Shield, p.ShieldFor, p.LastStandFor = 480, 9.5, 8
	w.Enemies = []*sim.Enemy{{ID: 42, Kind: "goblin", X: p.X + 6, HP: 10, MaxHP: 10, Traits: []string{}, Stun: 1.5, Slow: 0.5, SlowFor: 4.2, TauntID: p.ID, TauntFor: 3}}
	w.Storms = []*sim.Storm{{X: p.X + 6, Radius: 5, PerSecond: 10, Left: 4.5, Owner: p.ID}}
	return w
}

// Ein toter Monarch zeigt keine Auren (Passive wirken nur lebend).
func TestStatusAurenNurLebend(t *testing.T) {
	w := statusWorld(t)
	p := &sim.Player{HP: 0, RespawnIn: 3, Skills: []string{"armorAura"}}
	if a := sim.AurasOf(w, p); a != nil {
		t.Fatalf("tot: %v", a)
	}
	p.HP, p.RespawnIn = 10, 0
	if a := sim.AurasOf(w, p); len(a) != 1 {
		t.Fatalf("lebend: %v", a)
	}
}
