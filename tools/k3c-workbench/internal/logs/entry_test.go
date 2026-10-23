package logs

import "testing"

func TestParseTrenntCheckoutVonDenDaten(t *testing.T) {
	e, ok := parse([]byte(`{"time":"2026-09-30T09:00:00Z","level":"info","msg":"x","ns":"mcp","checkout":"bed-projekt","tool":"check_run"}`))
	if !ok || e.Checkout != "bed-projekt" {
		t.Fatalf("Checkout %q (ok %v)", e.Checkout, ok)
	}
	if _, in := e.Data["checkout"]; in || e.Data["tool"] != "check_run" {
		t.Fatalf("Daten %v: checkout gehört in die Spalte, tool bleibt", e.Data)
	}
}
