package playtest

import (
	"testing"
	"time"
)

func TestMakeStats(t *testing.T) {
	tr := Transcript{Segments: []Segment{{Text: "eins zwei drei"}, {Text: "vier"}}}
	st := MakeStats(60*time.Second, 6*time.Second, 120000, tr, 3)
	if st.TotalMs != 66000 || st.Words != 4 || st.Segments != 2 || st.Frames != 3 {
		t.Fatalf("Stats: %+v", st)
	}
	if st.SecPerMin != 33 || st.Factor != 0.55 {
		t.Fatalf("Verhältnis: %+v", st)
	}
	if z := MakeStats(time.Second, 0, 0, tr, 0); z.Factor != 0 || z.SecPerMin != 0 {
		t.Fatalf("ohne Aufnahmelänge: %+v", z)
	}
}
