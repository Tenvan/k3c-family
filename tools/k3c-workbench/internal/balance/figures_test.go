package balance

import (
	"testing"
)

// Ältere Berichte ohne Varianten und Kennzahlen bleiben lesbar.
func TestAlterBerichtLesbar(t *testing.T) {
	s, err := ReadSummary([]byte(`{"version":1,"seeds":3,"pass":true,"targets":[]}`))
	if err != nil || s.Seeds != 3 || s.Varianten != nil || s.Kennzahlen != nil {
		t.Fatalf("%v %+v", err, s)
	}
}
