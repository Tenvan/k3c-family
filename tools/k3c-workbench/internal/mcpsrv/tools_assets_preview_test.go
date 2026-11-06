package mcpsrv

import (
	"bytes"
	"context"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// paperRepoMitUI ist paperRepo plus die Beispielantworten der Oberfläche (PV1.8-Stubs lesen sie aus dem Checkout).
func paperRepoMitUI(t *testing.T) string {
	t.Helper()
	root := paperRepo(t)
	dateien, err := filepath.Glob(filepath.Join(repoWurzel(t), "tools", "paper", "fixtures", "ui", "*.json"))
	if err != nil || len(dateien) == 0 {
		t.Fatalf("fixtures/ui: %v", err)
	}
	for _, d := range dateien {
		kopiereAusRepo(t, root, "tools/paper/fixtures/ui/"+filepath.Base(d), "tools/paper/fixtures/ui/"+filepath.Base(d))
	}
	return root
}

func callResult(t *testing.T, cs *mcp.ClientSession, name string, args any) *mcp.CallToolResult {
	t.Helper()
	res, err := cs.CallTool(context.Background(), &mcp.CallToolParams{Name: name, Arguments: args})
	if err != nil {
		t.Fatal(err)
	}
	return res
}

// MC1/AC-03: assets_preview liefert ein Bild image/png plus Text mit Teil-Boxen.
func TestAssetsPreviewBild(t *testing.T) {
	cs := connect(t, New(Config{Version: "test", Root: paperRepo(t)}))
	for anim, fixture := range map[string]string{"": "monarch-idle-0.png", "reiten": "monarch-galopp-0.png"} {
		args := map[string]any{"id": "monarch"}
		if anim != "" {
			args["animation"] = anim
		}
		res := callResult(t, cs, "assets_preview", args)
		if res.IsError || len(res.Content) < 2 {
			t.Fatalf("%s: %+v", anim, res.Content)
		}
		bild, ok := res.Content[0].(*mcp.ImageContent)
		want, _ := os.ReadFile(filepath.Join(repoWurzel(t), "tools/paper/fixtures/render/monarch", fixture))
		if !ok || bild.MIMEType != "image/png" || !bytes.Equal(bild.Data, want) {
			t.Errorf("%s: kein Bild %s", anim, fixture)
		}
		text := res.Content[1].(*mcp.TextContent).Text
		zeige := map[string]string{"": "idle", "reiten": "reiten"}[anim]
		if !regexp.MustCompile(`^monarch · ` + zeige + ` · Frame 0 · Basis – · \d+×\d+ px\nTeile \(x,y,b,h px\): \S+ \d+,\d+,\d+,\d+`).MatchString(text) ||
			!strings.Contains(text, "rumpf 53,58,47,78") {
			t.Errorf("%s: Text\n%s", anim, text)
		}
	}
}

func TestAssetsPreviewFehler(t *testing.T) {
	cs := connect(t, New(Config{Version: "test", Root: paperRepo(t)}))
	for args, want := range map[string]string{
		`{"id":"monarch","animation":"flug"}`: "idle, lauf, reiten",
		`{"id":"x"}`:                          "Asset „x“ fehlt",
		`{"id":"monarch","basis":"main"}`:     "Basis „main“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop",
	} {
		res, err := cs.CallTool(context.Background(), &mcp.CallToolParams{Name: "assets_preview", Arguments: jsonRoh(args)})
		if err != nil {
			t.Fatal(err)
		}
		if text := resultText(res); !res.IsError || !strings.Contains(text, want) {
			t.Errorf("%s: %q", args, text)
		}
	}
}

func TestAssetsPreviewBasis(t *testing.T) {
	cs := connect(t, New(Config{Version: "test", Root: paperRepoMitUI(t)}))
	res := callResult(t, cs, "assets_preview", map[string]any{"id": "monarch", "basis": "HEAD~1"})
	if res.IsError || len(res.Content) < 2 {
		t.Fatalf("%v", resultText(res))
	}
	if bild, ok := res.Content[0].(*mcp.ImageContent); !ok || bild.MIMEType != "image/png" || len(bild.Data) == 0 {
		t.Error("kein Bild")
	}
	if text := res.Content[1].(*mcp.TextContent).Text; !strings.Contains(text, "monarch · idle · Frame 0 · Basis HEAD~1 · ") {
		t.Errorf("Text:\n%s", text)
	}
}
