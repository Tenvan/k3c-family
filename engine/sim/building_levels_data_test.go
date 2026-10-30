package sim

import (
	"reflect"
	"testing"
)

// B-569 AC-01: Hub-Gebäude ohne Produkt-Zahlziel haben eigene `levels` (Stufe n ab Hub-Stufe n); Stufe 1 trägt die Grundwerte,
// die HP wachsen, und die Zahl der Stufen entspricht der Hub-Stufenzahl.
func TestGebaeudeLevelsInDenDaten(t *testing.T) {
	_, w := fixtureIsland(t)
	for _, kind := range []string{"storage", "nursery", "barracks", "tavern", "healer"} {
		b := buildings[kind]
		if len(b.Levels) != len(hubOf(w).Levels) {
			t.Errorf("%s: %d levels, erwartet %d (Hub-Stufen)", kind, len(b.Levels), len(hubOf(w).Levels))
			continue
		}
		if first := b.Levels[0]; first.HP != b.HP || first.BuildSeconds != b.BuildSeconds || !reflect.DeepEqual(first.Cost, b.Cost) {
			t.Errorf("%s: Stufe 1 %+v weicht von den Grundwerten ab", kind, first)
		}
		for n := 1; n < len(b.Levels); n++ {
			if b.Levels[n].HP <= b.Levels[n-1].HP {
				t.Errorf("%s: HP der Stufe %d nicht größer als Stufe %d", kind, n+1, n)
			}
		}
	}
}
