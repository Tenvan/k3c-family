package sim

import (
	"slices"
	"testing"
)

// Verhalten aller Gegner aus data/enemies.json (K1.3, B-013, gegner.md § 2). Die Tests laufen über alle Gegner mit
// dem Trait, neue Arten in den Daten sind damit automatisch geprüft.

// traitTests: getestetes Verhalten je bekanntem Trait.
var traitTests = map[string]string{
	"stealsGold":       "TestGegnerStealsGold",
	"fleesAtHalfHp":    "TestGegnerFleesAtHalfHp",
	"ignoresWalls":     "TestGegnerIgnoresWallsLaeuftAmTorVorbei",
	"flying":           "TestGegnerTraitsGetestet (nur mit ignoresWalls)",
	"ranged":           "TestGegnerRanged",
	"prefersTroops":    "TestGegnerPrefersTroops",
	"prefersTowers":    "TestGegnerPrefersTowers",
	"prefersMonarch":   "TestGegnerPrefersMonarch",
	"prefersBuildings": "TestGegnerGebaeudeAlsZiel",
	"aoe":              "TestGegnerFlaechenschlag",
	"swarm":            "TestGegnerSchwarmPlatz",
	"phases":           "TestGegnerPhasen",
	"kiting":           "TestGegnerKiting",
}

func kindsWith(t *testing.T, trait string) []string {
	t.Helper()
	var out []string
	for kind, d := range enemyData {
		if slices.Contains(d.Traits, trait) {
			out = append(out, kind)
		}
	}
	if len(out) == 0 {
		t.Fatalf("kein Gegner mit %s", trait)
	}
	slices.Sort(out)
	return out
}

// Datentest: Jedes bekannte Trait hat ein getestetes Verhalten, `flying` steht nur mit `ignoresWalls`.
func TestGegnerTraitsGetestet(t *testing.T) {
	for _, tr := range knownTraits {
		if traitTests[tr] == "" {
			t.Fatalf("Trait %s ohne Test", tr)
		}
	}
	for _, kind := range kindsWith(t, "flying") {
		if !slices.Contains(enemyData[kind].Traits, "ignoresWalls") {
			t.Fatalf("%s fliegt ohne ignoresWalls", kind)
		}
	}
}

// Gegner ohne Traits laufen auf die Burg zu und schlagen eine Truppe in Reichweite mit dem Schaden aus den Daten.
func TestGegnerOhneTraitsNahkampf(t *testing.T) {
	for kind, d := range enemyData {
		if len(d.Traits) > 0 {
			continue
		}
		w := quietWorld(t)
		x := w.HubX - 30
		e := spawnEnemy(w, kind, x)
		stepFor(w, 1)
		tr := warriorAt(w, e.X+1)
		e.Cooldown = 0
		stepEnemies(w, dt)
		if e.X <= x || tr.HP != tr.MaxHP-d.Damage {
			t.Fatalf("%s bei %v (Start %v), Krieger %v/%v", kind, e.X, x, tr.HP, tr.MaxHP)
		}
	}
}

// stealsGold: klaut dem nächsten von zwei Spielern Gold, flieht damit und verschwindet im Portal.
func TestGegnerStealsGold(t *testing.T) {
	for _, kind := range kindsWith(t, "stealsGold") {
		w := quietWorld(t)
		p1, p2 := AddPlayer(w), AddPlayer(w)
		e := spawnEnemy(w, kind, w.HubX-40)
		e.X = w.HubX - 30
		p1.X, p2.X = e.X+0.5, e.X+1
		g1, g2 := p1.Gold, p2.Gold
		stepEnemies(w, dt)
		if p1.Gold != g1-waves.StealGold || e.CarriedGold != waves.StealGold || !e.Fleeing {
			t.Fatalf("%s: Gold %d → %d, trägt %d, flieht %v", kind, g1, p1.Gold, e.CarriedGold, e.Fleeing)
		}
		stepFor(w, 5)
		if len(w.Enemies) != 0 || p2.Gold != g2 || p1.HP != p1.MaxHP {
			t.Fatalf("%s: %d Gegner übrig, Spieler 2 Gold %d (%d)", kind, len(w.Enemies), p2.Gold, g2)
		}
	}
}

// fleesAtHalfHp: Bei halber HP kämpft der Gegner noch, darunter flieht er ohne Angriff und verschwindet im Portal.
func TestGegnerFleesAtHalfHp(t *testing.T) {
	for _, kind := range kindsWith(t, "fleesAtHalfHp") {
		w := quietWorld(t)
		e := spawnEnemy(w, kind, w.HubX-40)
		e.X, e.HP = w.HubX-30, e.MaxHP/2
		tr := warriorAt(w, e.X+0.5)
		stepEnemies(w, dt)
		if e.Fleeing || tr.HP == tr.MaxHP {
			t.Fatalf("%s bei halber HP: flieht %v, Krieger %v", kind, e.Fleeing, tr.HP)
		}
		e.HP, e.Cooldown, tr.HP = e.MaxHP/2-1, 0, tr.MaxHP
		stepEnemies(w, dt)
		if !e.Fleeing || tr.HP != tr.MaxHP || e.X >= w.HubX-30 {
			t.Fatalf("%s unter halber HP: flieht %v, Krieger %v, bei %v", kind, e.Fleeing, tr.HP, e.X)
		}
		stepFor(w, 5)
		if len(w.Enemies) != 0 {
			t.Fatalf("%s nicht im Portal verschwunden", kind)
		}
	}
}

// ranged: Fernkämpfer schießen auf eine Truppe außerhalb der Nahkampf-Reichweite, das Geschoss trägt ihren Schaden.
func TestGegnerRanged(t *testing.T) {
	for _, kind := range kindsWith(t, "ranged") {
		w := quietWorld(t)
		e := spawnEnemy(w, kind, w.HubX-30)
		tr := warriorAt(w, e.X+5)
		stepEnemies(w, dt)
		if len(w.Projectiles) != 1 || w.Projectiles[0].TargetID != tr.ID || w.Projectiles[0].Damage != e.Damage {
			t.Fatalf("%s: Geschosse %+v", kind, w.Projectiles)
		}
		for range 60 {
			stepProjectiles(w, dt)
		}
		if tr.HP != tr.MaxHP-e.Damage {
			t.Fatalf("%s: Krieger %v/%v", kind, tr.HP, tr.MaxHP)
		}
	}
}

// prefersTroops: greift die Truppe an, obwohl zwei Spieler näher stehen.
func TestGegnerPrefersTroops(t *testing.T) {
	for _, kind := range kindsWith(t, "prefersTroops") {
		w := quietWorld(t)
		p1, p2 := AddPlayer(w), AddPlayer(w)
		e := spawnEnemy(w, kind, w.HubX-30)
		p1.X, p2.X = e.X+0.3, e.X+0.5
		tr := warriorAt(w, e.X+1.2)
		stepEnemies(w, dt)
		if tr.HP == tr.MaxHP || p1.HP != p1.MaxHP || p2.HP != p2.MaxHP {
			t.Fatalf("%s: Krieger %v, Spieler %v/%v", kind, tr.HP, p1.HP, p2.HP)
		}
	}
}

// prefersMonarch: greift den nächsten Spieler an, obwohl eine Truppe näher steht.
func TestGegnerPrefersMonarch(t *testing.T) {
	for _, kind := range kindsWith(t, "prefersMonarch") {
		w := quietWorld(t)
		p1, p2 := AddPlayer(w), AddPlayer(w)
		e := spawnEnemy(w, kind, w.HubX-30)
		tr := warriorAt(w, e.X+0.3)
		p1.X, p2.X = e.X+1, e.X+1.3
		stepEnemies(w, dt)
		if p1.HP == p1.MaxHP || p2.HP != p2.MaxHP || tr.HP != tr.MaxHP {
			t.Fatalf("%s: Spieler %v/%v, Krieger %v", kind, p1.HP, p2.HP, tr.HP)
		}
	}
}

// prefersTowers: schießt auf den gebauten Turm, obwohl zwei Spieler näher stehen.
func TestGegnerPrefersTowers(t *testing.T) {
	for _, kind := range kindsWith(t, "prefersTowers") {
		w := quietWorld(t)
		p1, p2 := AddPlayer(w), AddPlayer(w)
		tower := buildSite(t, w, "tower")
		dir := sign(w.HubX - tower.X)
		e := spawnEnemy(w, kind, tower.X-dir*6)
		p1.X, p2.X = e.X+dir*2, e.X+dir*3
		stepEnemies(w, dt)
		if len(w.Projectiles) != 1 || w.Projectiles[0].TargetID != tower.ID {
			t.Fatalf("%s: Geschosse %+v, Turm %d", kind, w.Projectiles, tower.ID)
		}
	}
}
