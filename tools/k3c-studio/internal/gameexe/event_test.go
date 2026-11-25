package gameexe

import "testing"

func TestParseEvent(t *testing.T) {
	ev, ok := ParseEvent(`{"event":"progress","done":3,"total":6}`)
	if !ok || ev.Event != "progress" || ev.Done != 3 || ev.Total != 6 {
		t.Errorf("progress: %+v %v", ev, ok)
	}
	ev, ok = ParseEvent(`{"event":"result","report":{"a":1}}`)
	if !ok || string(ev.Report) != `{"a":1}` {
		t.Errorf("report: %+v %v", ev, ok)
	}
	ev, ok = ParseEvent(`{"event":"report","files":["a.json","b.md"]}`)
	if !ok || len(ev.Files) != 2 {
		t.Errorf("files: %+v %v", ev, ok)
	}
	for _, bad := range []string{"", "text", `{"x":1}`, `{"event":""}`} {
		if _, ok := ParseEvent(bad); ok {
			t.Errorf("%q gilt nicht als Event", bad)
		}
	}
}
