package mcpsrv

import (
	"regexp"
	"strings"
	"testing"
)

// MC1/AC-05: {alle} antwortet sofort mit dem Job, {job, wait} liefert das Ergebnis.
func TestAssetsRender(t *testing.T) {
	if testing.Short() {
		t.Skip("ruft die Render-CLI (bun) gut zehnmal auf, je 0,3 s; läuft ohne -short (B-619, B-830)")
	}
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	text, isErr := callText(t, cs, "assets_render", map[string]any{"id": "monarch"})
	if isErr || !regexp.MustCompile(`^monarch gerendert: \d+ PNGs?, manifest\.json · renderKey \S+\n`).MatchString(text) {
		t.Errorf("eins:\n%s", text)
	}
	text, isErr = callText(t, cs, "assets_render", map[string]any{"alle": true})
	m := regexp.MustCompile(`^Job (j-\d+) gestartet: 3 Assets \(alle\)\nWeiter: assets_render \{job: "j-\d+"\} oder \{job: "j-\d+", wait: true\}`).FindStringSubmatch(text)
	if isErr || m == nil {
		t.Fatalf("alle:\n%s", text)
	}
	job := m[1]
	text, _ = callText(t, cs, "assets_render", map[string]any{"job": job})
	if !regexp.MustCompile(`^Job ` + job + ` (läuft \d/3 \(\w+\)|fertig: )`).MatchString(text) {
		t.Errorf("Status:\n%s", text)
	}
	text, isErr = callText(t, cs, "assets_render", map[string]any{"job": job, "wait": true})
	if isErr || !regexp.MustCompile(`^Job `+job+` fertig: 3/3 Assets, \d+ PNGs, 0 Fehler, \d+,\d s\n`).MatchString(text) ||
		!strings.Contains(text, "\nmonarch · ") || !strings.Contains(text, "\npferd · ") || !strings.Contains(text, "\nmauer · ") {
		t.Errorf("wait:\n%s", text)
	}
}

func TestAssetsRenderFehler(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	cs := connect(t, New(Config{Version: "test", Root: paperRepo(t)}))
	for _, args := range []map[string]any{{}, {"id": "monarch", "alle": true}} {
		if text, isErr := callText(t, cs, "assets_render", args); !isErr || !strings.Contains(text, "{id}, {alle: true} oder {job") {
			t.Errorf("%v: %q", args, text)
		}
	}
	if text, isErr := callText(t, cs, "assets_render", map[string]any{"job": "j-999"}); !isErr || !strings.HasPrefix(text, "Job „j-999“ unbekannt") {
		t.Errorf("unbekannter Job: %q", text)
	}
}
