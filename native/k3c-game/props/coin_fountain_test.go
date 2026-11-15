package props

import (
	"math"
	"testing"

	"k3c/engine/sim"
)

func TestCoinPoseBogen(t *testing.T) {
	c := &sim.Coin{X: 110, FromX: 100, LaunchAt: 1, LandsAt: 2}
	if _, _, shown := coinPose(c, 0.5); shown {
		t.Error("vor dem Start liegt die Münze noch in der Truhe")
	}
	if x, lift, shown := coinPose(c, 1.5); !shown || math.Abs(x-105) > 1e-9 || math.Abs(lift-fountainHeight) > 1e-9 {
		t.Errorf("Scheitel: x %v, Höhe %v, sichtbar %v", x, lift, shown)
	}
	if x, lift, shown := coinPose(c, 2); !shown || x != 110 || lift != 0 {
		t.Errorf("gelandet: x %v, Höhe %v", x, lift)
	}
	if x, lift, shown := coinPose(&sim.Coin{X: 7}, 99); !shown || x != 7 || lift != 0 {
		t.Errorf("Münze ohne Fontäne: x %v, Höhe %v", x, lift)
	}
}
