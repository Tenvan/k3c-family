package sim

import (
	"math"
	"testing"
)

// SecondsToNight zählt bis zum Beginn der Nacht: am Tag über die Dämmerung, im Morgengrauen über den nächsten Tag,
// in der Nacht 0; der Zyklus-Faktor teilt (B-889).
func TestSecondsToNight(t *testing.T) {
	phases, length := cyclePhases(globalDayNight)
	night := phases[2].start
	cases := []struct {
		time, speed float64
		want        float64
	}{
		{0, 1, night},
		{10, 1, night - 10},
		{phases[3].start + 1, 1, length - phases[3].start - 1 + night},
		{10, 2, (night - 20) / 2},
	}
	for _, c := range cases {
		w := &World{Time: c.time, CycleSpeed: c.speed}
		w.Cycle = cycleAt(globalDayNight, c.time*c.speed)
		if got := SecondsToNight(w); math.Abs(got-c.want) > 1e-9 {
			t.Errorf("Zeit %v × %v: %v, erwartet %v", c.time, c.speed, got, c.want)
		}
	}
	w := &World{Time: night + 1, CycleSpeed: 1}
	w.Cycle = cycleAt(globalDayNight, night+1)
	if SecondsToNight(w) != 0 {
		t.Error("in der Nacht nicht 0")
	}
}
