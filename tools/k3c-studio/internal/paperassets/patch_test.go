package paperassets

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"
)

func TestMergePatch(t *testing.T) {
	cases := []struct{ name, ziel, patch, want string }{
		{"Feld ersetzen", `{"a":1,"b":2}`, `{"a":3}`, `{"a":3,"b":2}`},
		{"null löscht", `{"a":1,"b":2}`, `{"a":null}`, `{"b":2}`},
		{"verschachtelt", `{"u":{"m":"rot","l":5},"x":1}`, `{"u":{"m":"purpur"}}`, `{"u":{"m":"purpur","l":5},"x":1}`},
		{"Array ersetzt komplett", `{"p":[1,2,3]}`, `{"p":[9]}`, `{"p":[9]}`},
		{"neues Feld hinten", `{"a":1}`, `{"z":{"k":null,"v":2}}`, `{"a":1,"z":{"v":2}}`},
		{"Patch kein Objekt", `{"a":1}`, `[1,2]`, `[1,2]`},
		{"Ziel kein Objekt", `[1]`, `{"a":1}`, `{"a":1}`},
		{"Zahlen bleiben roh", `{"r":[-60, -145, 85.25, 151]}`, `{}`, `{"r":[-60,-145,85.25,151]}`},
	}
	for _, c := range cases {
		got, err := MergePatch([]byte(c.ziel), []byte(c.patch))
		if err != nil || string(got) != c.want {
			t.Errorf("%s: %s %v, erwartet %s", c.name, got, err, c.want)
		}
	}
	if _, err := MergePatch([]byte(`{"a":1}`), []byte(`{"a":`)); err == nil {
		t.Error("kaputter Patch ohne Fehler")
	}
}

func TestFormat(t *testing.T) {
	got, err := Format([]byte(`{"id":"x","pivot":[0,-84],"teile":[{"id":"a","lagen":[]}]}`))
	want := "{\n  \"id\": \"x\",\n  \"pivot\": [0, -84],\n  \"teile\": [\n    {\n      \"id\": \"a\",\n      \"lagen\": []\n    }\n  ]\n}\n"
	if err != nil || string(got) != want {
		t.Errorf("Format:\n%s", got)
	}
	// Eine unveränderte Fixture-Spec kommt Byte für Byte gleich heraus (kein Diff-Rauschen nach assets_set).
	for _, name := range []string{"monarch", "pferd", "mauer"} {
		orig, err := os.ReadFile(filepath.Join(repoWurzel(t), "tools/paper/fixtures/specs", name+".paper.json"))
		if err != nil {
			t.Fatal(err)
		}
		orig = bytes.ReplaceAll(orig, []byte("\r\n"), []byte("\n"))
		neu, err := MergePatch(orig, []byte(`{}`))
		if err != nil {
			t.Fatal(err)
		}
		if got, _ := Format(neu); !bytes.Equal(got, orig) {
			t.Errorf("%s: Format weicht vom Original ab", name)
		}
	}
}

func TestSchluesselUndFelder(t *testing.T) {
	raw := []byte(`{"$schema":"x","id":"m","umhang":{"material":"rot"},"ausgabe":{"hoehe_px":1},"animationen":{"idle":{},"lauf":{}}}`)
	if got := Schluessel(raw, "animationen"); len(got) != 2 || got[0] != "idle" || got[1] != "lauf" {
		t.Errorf("Schluessel: %v", got)
	}
	if got := Schluessel(raw); len(got) != 5 || got[0] != "$schema" {
		t.Errorf("Schluessel oben: %v", got)
	}
	got, err := Felder(raw, []string{"ausgabe", "umhang"})
	if err != nil || string(got) != `{"umhang":{"material":"rot"},"ausgabe":{"hoehe_px":1}}` {
		t.Errorf("Felder: %s %v", got, err)
	}
	if got, err := Felder(raw, nil); err != nil || bytes.Contains(got, []byte("$schema")) || !bytes.Contains(got, []byte(`"animationen"`)) {
		t.Errorf("Felder alle: %s %v", got, err)
	}
	if _, err := Felder(raw, []string{"farbe"}); err == nil || err.Error() != "Feld „farbe“ fehlt; vorhanden: id, umhang, ausgabe, animationen" {
		t.Errorf("unbekannt: %v", err)
	}
}
