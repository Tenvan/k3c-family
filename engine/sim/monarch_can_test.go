package sim

import (
	"slices"
	"testing"
)

// CanLearn und CanRespec: gleiche Regeln wie die Aktionen, aber ohne Wirkung (SK1.1, B-270).

// copyPlayer ist eine Kopie mit eigenen Skill- und Slot-Listen, damit die Aktion das Original nicht berührt.
func copyPlayer(p *Player) *Player {
	c := *p
	c.Skills, c.Slots = slices.Clone(p.Skills), slices.Clone(p.Slots)
	return &c
}

// sameErr: beide nil oder beide mit demselben Text.
func sameErr(a, b error) bool {
	if a == nil || b == nil {
		return a == nil && b == nil
	}
	return a.Error() == b.Error()
}

func TestCanLearnWieLearnSkillOhneSeiteneffekt(t *testing.T) {
	cases := []struct {
		name, id string
		points   int // zusätzliche Pool-Punkte vor dem Fall
		wantErr  bool
	}{
		{"unbekannt", "gibtEsNicht", 5, true},
		{"schon gelernt", "taunt", 5, true},
		{"keine Punkte", "shieldBash", 0, true},
		{"Tier fehlt", "ironWall", 5, true},
		{"erlaubt", "shieldBash", 5, false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			w2 := quietWorld(t)
			p2 := AddPlayer(w2)
			addPoolPoints(w2, 1)
			mustLearn(t, w2, p2, "taunt")
			addPoolPoints(w2, c.points)
			skills, slots := slices.Clone(p2.Skills), slices.Clone(p2.Slots)
			can := CanLearn(w2, p2, c.id)
			if !slices.Equal(p2.Skills, skills) || !slices.Equal(p2.Slots, slots) {
				t.Fatalf("CanLearn änderte den Spieler: %v %v", p2.Skills, p2.Slots)
			}
			if got := LearnSkill(w2, copyPlayer(p2), c.id); !sameErr(can, got) {
				t.Fatalf("CanLearn %v, LearnSkill %v", can, got)
			}
			if (can != nil) != c.wantErr {
				t.Fatalf("Fehler %v, erwartet Fehler: %v", can, c.wantErr)
			}
		})
	}
}

func TestCanRespecWieRespecOhneSeiteneffekt(t *testing.T) {
	cases := []struct {
		name    string
		phase   string
		dx      float64
		learn   bool
		wantErr bool
	}{
		{"Nacht", "night", 0, true, true},
		{"fern der Burg", "day", 10, true, true},
		{"Tag an der Burg", "day", 0, true, false},
		{"ohne gelernte Skills", "day", 0, false, false},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			w := quietWorld(t)
			p := AddPlayer(w)
			addPoolPoints(w, 2)
			if c.learn {
				mustLearn(t, w, p, "heal", "groupHeal")
			}
			w.Cycle.Phase = c.phase
			p.X = w.Castle.X + c.dx
			skills, slots, base := slices.Clone(p.Skills), slices.Clone(p.Slots), p.SkillBase
			can := CanRespec(w, p)
			if !slices.Equal(p.Skills, skills) || !slices.Equal(p.Slots, slots) || p.SkillBase != base {
				t.Fatalf("CanRespec änderte den Spieler: %v %v %d", p.Skills, p.Slots, p.SkillBase)
			}
			if got := Respec(w, copyPlayer(p)); !sameErr(can, got) {
				t.Fatalf("CanRespec %v, Respec %v", can, got)
			}
			if (can != nil) != c.wantErr {
				t.Fatalf("Fehler %v, erwartet Fehler: %v", can, c.wantErr)
			}
		})
	}
}
