package main

import "testing"

// Die Namensabbildung: Test-Präfix weg, CamelCase zu snake_case, Großbuchstabenfolgen und Ziffern bleiben zusammen.
func TestSnake(t *testing.T) {
	cases := map[string]string{
		"TestFooBar":      "foo_bar",
		"TestRU15Foo":     "ru15_foo",
		"TestHUDLayout":   "hud_layout",
		"TestFoo_bar":     "foo_bar",
		"TestSitePayable": "site_payable",
		"TestAB":          "ab",
		"TestTick2Wave":   "tick2_wave",
		"TestX":           "x",
	}
	for in, want := range cases {
		if got := snake(in); got != want {
			t.Errorf("snake(%q) = %q, erwartet %q", in, got, want)
		}
	}
}

func TestTheme(t *testing.T) {
	if got := theme("engine/sim/story_points_test.go"); got != "Geschichte (GS1)" {
		t.Errorf("story: %q", got)
	}
	if got := theme("engine/sim/foo_test.go"); got != "allgemein" {
		t.Errorf("allgemein: %q", got)
	}
}

func TestCompare(t *testing.T) {
	a := area{name: "x"}
	tests := []goTest{{"TestFooBar", "a_test.go"}, {"TestNope", "b_test.go"}}
	r := compare(a, tests, map[string]bool{"foo_bar": true})
	if r.total != 2 || r.matched != 1 || len(r.missing) != 1 || r.missing[0].name != "TestNope" {
		t.Errorf("Ergebnis %+v", r)
	}
}
