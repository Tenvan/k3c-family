package simrun

import (
	"slices"
	"strings"
	"testing"

	"k3c/tools/k3c-workbench/internal/balance"
)

// B-640: eine benannte Baseline ersetzt die Baseline des Repos im Aufruf der Balance-CLI.
func TestSpecBaselineArgsUndPruefung(t *testing.T) {
	arg := func(s Spec) string {
		a := s.args("bin/k3c-balance", "reports/simtest-run-1")
		return a[slices.Index(a, "--baseline")+1]
	}
	if got := arg(Spec{}); got != balance.BaselinePath {
		t.Errorf("ohne Name: %s", got)
	}
	if got := arg(Spec{Baseline: "M1"}); got != "reports/baseline/M1/balance.json" {
		t.Errorf("mit Name: %s", got)
	}
	if err := (Spec{Baseline: "M1"}).Check(); err != nil {
		t.Error(err)
	}
	for _, bad := range []string{"../M1", "M 1", "a/b", strings.Repeat("x", 41)} {
		if err := (Spec{Baseline: bad}).Check(); err == nil {
			t.Errorf("Name %q angenommen", bad)
		}
	}
	if title := (Spec{Baseline: "M1"}).Title(); !strings.Contains(title, "gegen M1") {
		t.Errorf("Titel: %s", title)
	}
}
