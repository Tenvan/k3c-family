package mcpsrv

import (
	"encoding/json"

	"k3c/tools/workkit/mcpkit"
)

// targetOf liest das Ziel eines check_run-Aufrufs aus den rohen Argumenten (vor dem Kürzen); andere Tools: leer (B-876).
// Die Zähler (mcpkit.Stats) tragen es als Call.Target ins Aufruf-Log.
func targetOf(tool, args string) string {
	if tool != "check_run" {
		return ""
	}
	var in struct {
		Target string `json:"target"`
	}
	if json.Unmarshal([]byte(args), &in) != nil {
		return ""
	}
	return mcpkit.Clip(in.Target)
}
