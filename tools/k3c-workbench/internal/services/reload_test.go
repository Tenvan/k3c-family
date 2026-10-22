package services

import (
	"context"
	"slices"
	"testing"
)

func TestReload(t *testing.T) {
	f := &fake{}
	f.healthy.Store(true)
	c := testController(f, nil, svc("A", false), svc("B", false), svc("C", false))
	ctx := context.Background()
	if _, err := c.Start(ctx, "B"); err != nil {
		t.Fatal(err)
	}
	a2, b2 := svc("A", false), svc("B", false)
	a2.Port, b2.Port = 7, 8
	pending := c.Reload(ctx, []Service{b2, a2, svc("D", false)}) // C entfällt, D neu, A geändert, B läuft

	if got := c.Names(); !slices.Equal(got, []string{"B", "A", "D"}) {
		t.Errorf("Reihenfolge: %v", got)
	}
	if !slices.Equal(pending, []string{"B"}) {
		t.Errorf("pending: %v", pending)
	}
	for name, port := range map[string]int{"A": 7, "B": 1} { // B behält die alte Konfiguration bis zum Neustart
		u, _ := c.unit(name)
		if u.status().Port != port || u.svc.Port != port {
			t.Errorf("%s: Port %d / %d, erwartet %d", name, u.status().Port, u.svc.Port, port)
		}
	}
	if st, _ := c.unit("B"); st.status().State != Running {
		t.Errorf("B: %s", st.status().State)
	}
}
