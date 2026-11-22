package review

import (
	"encoding/json"
	"path/filepath"
	"strings"
	"testing"
)

// TestKanonischWieHashTS: Beispiel aus tools/paper/schema/hash.test.ts.
func TestKanonischWieHashTS(t *testing.T) {
	t.Parallel()
	var v any
	if err := json.Unmarshal([]byte(`{"b":[2,1],"a":{"d":1,"c":"x"}}`), &v); err != nil {
		t.Fatal(err)
	}
	if got, want := Kanonisch(v), `{"a":{"c":"x","d":1},"b":[2,1]}`; got != want {
		t.Fatalf("Kanonisch = %s, will %s", got, want)
	}
}

// TestKanonischZahlenUndTexte: Zahlen wie JS JSON.stringify, Texte ohne HTML-Escaping, CRLF als LF.
func TestKanonischZahlenUndTexte(t *testing.T) {
	t.Parallel()
	var v any
	raw := `{"z":[1.50,-0,1e21,1.5e-7,0.000001,100,-2.25],"t":"a<b>& \r\nc\"\\\u0001"}`
	if err := json.Unmarshal([]byte(raw), &v); err != nil {
		t.Fatal(err)
	}
	want := `{"t":"a<b>&` + " " + `\nc\"\\\u0001","z":[1.5,0,1e+21,1.5e-7,0.000001,100,-2.25]}`
	if got := Kanonisch(v); got != want {
		t.Fatalf("Kanonisch = %s\nwill       %s", got, want)
	}
}

// TestSpecHashWieBun: Go-Hash = hash.ts für Fixture-Spec und Stil-Datei.
func TestSpecHashWieBun(t *testing.T) {
	t.Parallel()
	spec := fixture("specs/monarch.paper.json")
	stil := filepath.Join(repoRoot, "public", "grafik", "stil.paper.json")
	out := bun(t, "-e", `const h=await import(process.cwd()+'/tools/paper/schema/hash.ts');const fs=require('fs');`+
		`for (const p of process.argv.slice(1)) console.log(h.specHash(fs.readFileSync(p,'utf8')))`, spec, stil)
	want := strings.Fields(out)
	for i, p := range []string{spec, stil} {
		got, err := SpecHash(lies(t, p))
		if err != nil {
			t.Fatal(err)
		}
		if got != want[i] {
			t.Errorf("%s: SpecHash = %s, hash.ts = %s", filepath.Base(p), got, want[i])
		}
	}
}

// TestSpecHashCRLF: Zeilenenden und Schlüsselreihenfolge ändern den Hash nicht, $schema zählt nicht (AC-02).
func TestSpecHashCRLF(t *testing.T) {
	t.Parallel()
	raw := lies(t, fixture("specs/monarch.paper.json"))
	a, err := SpecHash(raw)
	if err != nil {
		t.Fatal(err)
	}
	crlf := []byte(strings.ReplaceAll(strings.ReplaceAll(string(raw), "\r\n", "\n"), "\n", "\r\n"))
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatal(err)
	}
	delete(m, "$schema")
	umsortiert, _ := json.MarshalIndent(m, "", "\t")
	for name, v := range map[string][]byte{"crlf": crlf, "umsortiert": umsortiert} {
		b, err := SpecHash(v)
		if err != nil {
			t.Fatal(err)
		}
		if a != b {
			t.Errorf("%s: Hash %s ≠ %s", name, b, a)
		}
	}
	anders, _ := SpecHash([]byte(strings.Replace(string(raw), "koenigsrot", "purpur", 1)))
	if anders == a {
		t.Error("andere Spec, gleicher Hash")
	}
}

// TestRenderKey: Spec-, Stil-Hash und Renderer-Version ändern den Schlüssel.
func TestRenderKey(t *testing.T) {
	t.Parallel()
	k := RenderKey("a", "b")
	if k != renderKeyMit("a", "b", RendererVersion) || len(k) != 64 {
		t.Fatalf("RenderKey = %s", k)
	}
	seen := map[string]bool{k: true, RenderKey("a", "c"): true, RenderKey("c", "b"): true, renderKeyMit("a", "b", "paper-1"): true}
	if len(seen) != 4 {
		t.Fatal("RenderKey unterscheidet nicht alle Eingaben")
	}
}
