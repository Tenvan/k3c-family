package mcpsrv

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strings"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/assetsui"
)

func jsonRoh(s string) json.RawMessage { return json.RawMessage(s) }

// MC1.5: ohne Testquelle antwortet assets_review über assetsui.Dienst (RV1) aus dem Checkout.
func TestAssetsReviewNutztDienst(t *testing.T) {
	t.Parallel()
	if _, ok := newAssetReview("x").(assetsui.Dienst); !ok {
		t.Errorf("newAssetReview: %T", newAssetReview("x"))
	}
}

// MC1/AC-01: assets_review gegen die Stubs (Fixture fixtures/ui/AssetsReview.json, Basis HEAD~1).
func TestAssetsReview(t *testing.T) {
	mitFixtureReview(t)
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	text, isErr := callText(t, cs, "assets_review", map[string]any{"basis": "HEAD~1"})
	for _, want := range []string{
		"Review gegen HEAD~1 (32eb7a7, Basis-Stand aktuell): 5 Assets\n",
		"\ngeaendert · monarch · umhang.material koenigsrot → purpur · Pixel 1,53 % · Overlay .cache/paper/mcp/monarch-idle-0-diff.png",
		"\nneu · pferd · public/grafik/reittiere/pferd.paper.json",
		"\ngeloescht · fass · public/grafik/requisiten/fass.paper.json",
		"\nungueltig · banner · teile[0].lagen[0].material · material-unbekannt · Material „scharlach“ fehlt in der Stil-Datei",
		"\ngeaendert · mauer · stil: true · Pixel 17,07 %",
	} {
		if isErr || !strings.Contains(text, want) {
			t.Errorf("ohne %q:\n%s", want, text)
		}
	}
	if _, err := os.Stat(filepath.Join(root, ".cache", "paper", "mcp", "monarch-idle-0-diff.png")); err != nil {
		t.Errorf("Overlay nicht geschrieben: %v", err)
	}
	if text, isErr := callText(t, cs, "assets_review", map[string]any{"basis": "main"}); !isErr || !strings.Contains(text, "Basis „main“ ungültig") {
		t.Errorf("ungültige Basis: %q", text)
	}
}

// Kopfzeile bei Basis-Stand lokal wie fixtures/mcp/assets_review-lokal.txt.
func TestAssetsReviewKopfLokal(t *testing.T) {
	t.Parallel()
	got := reviewKopf("origin/main", "32eb7a7", "lokal", 5)
	want := "Review gegen origin/main (32eb7a7) · Basis-Stand: lokal (ohne vorheriges Fetch; origin/main kann veraltet sein): 5 Assets"
	if got != want {
		t.Errorf("%q", got)
	}
}

// vorlagenAusDatei liest data/asset-prompts.json und die Platzhalter je Vorlage.
func vorlagenAusDatei(t *testing.T, root string) map[string][]string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(root, "data", "asset-prompts.json"))
	if err != nil {
		t.Fatal(err)
	}
	var vorlagen []struct{ ID, Text string }
	if err := json.Unmarshal(b, &vorlagen); err != nil {
		t.Fatal(err)
	}
	re := regexp.MustCompile(`\{(asset|rig|teil|pfad|zustand|basis|eingabe|material|ton|slot|datei)\}`)
	out := map[string][]string{}
	for _, v := range vorlagen {
		var namen []string
		for _, m := range re.FindAllStringSubmatch(v.Text, -1) {
			if !slices.Contains(namen, m[1]) {
				namen = append(namen, m[1])
			}
		}
		out[v.ID] = namen
	}
	return out
}

// MC1/AC-04: jede Vorlage ist als Prompt asset-<id> mit ihren Platzhaltern abrufbar.
func TestAssetsPromptsAlle(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	root := paperRepo(t)
	cs := connect(t, New(Config{Version: "test", Root: root}))
	res, err := cs.ListPrompts(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	prompts := map[string]*mcp.Prompt{}
	for _, p := range res.Prompts {
		prompts[p.Name] = p
	}
	vorlagen := vorlagenAusDatei(t, root)
	if len(vorlagen) < 10 {
		t.Fatalf("nur %d Vorlagen", len(vorlagen))
	}
	for id, namen := range vorlagen {
		p := prompts["asset-"+id]
		if p == nil || p.Title == "" {
			t.Errorf("asset-%s fehlt", id)
			continue
		}
		var args []string
		for _, a := range p.Arguments {
			args = append(args, a.Name)
			if pflicht := a.Name != "zustand" && a.Name != "basis"; a.Required != pflicht {
				t.Errorf("asset-%s: %s Required=%v", id, a.Name, a.Required)
			}
		}
		if strings.Join(args, ",") != strings.Join(namen, ",") {
			t.Errorf("asset-%s: Argumente %v, Platzhalter %v", id, args, namen)
		}
	}
}

func TestAssetsPromptEinsetzen(t *testing.T) {
	t.Parallel() // ohne t.Setenv und globale Zustände (B-830)
	cs := connect(t, New(Config{Version: "test", Root: paperRepo(t)}))
	ctx := context.Background()
	res, err := cs.GetPrompt(ctx, &mcp.GetPromptParams{Name: "asset-material-aendern", Arguments: map[string]string{
		"asset": "monarch", "rig": "figur", "pfad": "public/grafik/figuren/monarch.paper.json", "zustand": "gültig, Version 1"}})
	if err != nil {
		t.Fatal(err)
	}
	text := res.Messages[0].Content.(*mcp.TextContent).Text
	if len(res.Messages) != 1 || res.Messages[0].Role != "user" || !strings.HasPrefix(text, "Rufe zuerst `assets_guide {rig: figur}` auf") ||
		regexp.MustCompile(`\{(asset|rig|teil|pfad|zustand|basis|eingabe|material|ton|slot|datei)\}`).MatchString(text) {
		t.Errorf("Prompt:\n%s", text)
	}
	if _, err := cs.GetPrompt(ctx, &mcp.GetPromptParams{Name: "asset-material-aendern", Arguments: map[string]string{
		"asset": "monarch", "rig": "figur", "pfad": "p"}}); err != nil {
		t.Errorf("zustand leer: %v", err)
	}
	for name, args := range map[string]map[string]string{
		"asset-material-aendern": {"rig": "figur", "pfad": "p"},
		"asset-teil-material":    {"asset": "a", "rig": "figur", "pfad": "p"},
		"asset-nachbessern":      {"asset": "a", "rig": "figur", "pfad": "p"},
	} {
		if _, err := cs.GetPrompt(ctx, &mcp.GetPromptParams{Name: name, Arguments: args}); err == nil {
			t.Errorf("%s ohne Pflicht-Argument angenommen", name)
		}
	}
}

// Jedes registrierte assets_*-Tool steht in instructions.md.
func TestInstructionsNennenAssetTools(t *testing.T) {
	t.Parallel()
	cs := connect(t, New(Config{Version: "test"}))
	for name := range toolSchemas(t, cs) {
		if strings.HasPrefix(name, "assets_") && !strings.Contains(Instructions(), "`"+name) {
			t.Errorf("%s fehlt in instructions.md", name)
		}
	}
}

// Review MC1.3: Cache-Dateinamen aus Spec- oder Review-Daten bleiben in .cache/paper/mcp/.
func TestCacheNameBleibtImOrdner(t *testing.T) {
	t.Parallel()
	for _, teile := range [][]string{{"monarch", "idle"}, {"..", "..", "x"}, {`a\b`, "c/d"}} {
		name := cacheName(teile...)
		if strings.ContainsAny(name, `/\`) || strings.Contains(name, "..") || name == "" {
			t.Errorf("%v → %q", teile, name)
		}
	}
	if got := cacheName("monarch", "idle", "0", "diff"); got != "monarch-idle-0-diff.png" {
		t.Errorf("normal: %q", got)
	}
}
