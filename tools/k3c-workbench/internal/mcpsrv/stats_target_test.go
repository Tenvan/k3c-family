package mcpsrv

import (
	"strings"
	"testing"
	"time"

	"k3c/tools/workkit/mcpkit"
)

func TestFormatUptime(t *testing.T) {
	t.Parallel()
	cases := map[time.Duration]string{
		30 * time.Second: "< 1 min", 13 * time.Minute: "13 min",
		133 * time.Minute: "2 h 13 min", 76 * time.Hour: "3 T 4 h",
	}
	for d, want := range cases {
		if got := formatUptime(d); got != want {
			t.Errorf("formatUptime(%v) = %q, erwartet %q", d, got, want)
		}
	}
}

// check_run trägt sein Ziel aus den ungekürzten Argumenten ins Aufruf-Log, auch wenn args gekürzt wird (B-876).
func TestCheckRunZielImLog(t *testing.T) {
	t.Parallel()
	c := time.Date(2026, 9, 30, 12, 0, 0, 0, time.UTC)
	st := mcpkit.NewStats(func() time.Time { c = c.Add(time.Millisecond); return c }, nil, nil, targetOf)
	long := `{"wait":true,"pattern":"` + strings.Repeat("x", 300) + `","target":"task:check:diff"}`
	st.End(st.Begin("check_run", long), mcpkit.Outcome{OK: true})
	st.End(st.Begin("a", `{"target":"task:test"}`), mcpkit.Outcome{OK: true})
	st.End(st.Begin("check_run", `kaputt`), mcpkit.Outcome{Err: "x"})
	calls := st.Calls()
	if calls[2].Target != "task:check:diff" || strings.Contains(calls[2].Args, "task:check:diff") {
		t.Errorf("Ziel: %q, Args: %q", calls[2].Target, calls[2].Args)
	}
	if calls[1].Target != "" || calls[0].Target != "" {
		t.Errorf("Ziel nur bei gültigem check_run: %q, %q", calls[1].Target, calls[0].Target)
	}
}
