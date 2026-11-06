package mcpsrv

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/assetsui"
)

// fixtureReview beantwortet die Review-Methoden mit den Beispielantworten tools/paper/fixtures/ui/ (Basis HEAD~1);
// den echten Vergleich prüft RV1 (internal/review, internal/assetsui), MC1 nur die Formatierung.
type fixtureReview struct{ wurzel string }

func ladeFixture[T any](f fixtureReview, name string) (T, error) {
	var v T
	b, err := os.ReadFile(filepath.Join(f.wurzel, "tools", "paper", "fixtures", "ui", name+".json"))
	if err != nil {
		return v, err
	}
	return v, json.Unmarshal(b, &v)
}

func (f fixtureReview) AssetsReview(string) (assetsui.Review, error) {
	return ladeFixture[assetsui.Review](f, "AssetsReview")
}

func (f fixtureReview) AssetsFrame(id, _ string, _ int, _, _ string) (assetsui.Frame, error) {
	if id != "monarch" {
		return assetsui.Frame{}, fmt.Errorf("Asset „%s“ fehlt", id)
	}
	return ladeFixture[assetsui.Frame](f, "AssetsFrame")
}

func (f fixtureReview) AssetsPixelDiff(id, _ string, _ int, _ string) (assetsui.PixelDiff, error) {
	if id != "monarch" {
		return assetsui.PixelDiff{}, fmt.Errorf("Asset „%s“ fehlt", id)
	}
	return ladeFixture[assetsui.PixelDiff](f, "AssetsPixelDiff")
}

// mitFixtureReview setzt die Fixture-Quelle für die Dauer des Tests ein.
func mitFixtureReview(t *testing.T) {
	t.Helper()
	alt := newAssetReview
	newAssetReview = func(string) assetReview { return fixtureReview{wurzel: repoWurzel(t)} }
	t.Cleanup(func() { newAssetReview = alt })
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
	mitFixtureReview(t)
	cs := connect(t, New(Config{Version: "test", Root: paperRepo(t)}))
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
