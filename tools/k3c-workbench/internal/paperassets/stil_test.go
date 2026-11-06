package paperassets

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// StilHash rechnet wie tools/paper/schema/hash.ts (kanonische Form ohne $schema).
func TestStilHashWieHashTS(t *testing.T) {
	t.Parallel()
	stubSkript(t)
	wurzel := repoWurzel(t)
	skript := filepath.ToSlash(filepath.Join(wurzel, "tools/paper/schema/hash.ts"))
	stil := filepath.ToSlash(filepath.Join(wurzel, StilPfad))
	out, err := exec.Command("bun", "-e", `import { stilHash } from '`+skript+`'; import { readFileSync } from 'node:fs';`+
		`console.log(stilHash(readFileSync('`+stil+`', 'utf8')))`).Output()
	if err != nil {
		t.Fatal(err)
	}
	b, _ := os.ReadFile(filepath.Join(wurzel, StilPfad))
	if got, err := StilHash(b); err != nil || got != strings.TrimSpace(string(out)) {
		t.Errorf("Go %s %v, hash.ts %s", got, err, out)
	}
}

func TestFeldDiff(t *testing.T) {
	t.Parallel()
	felder, err := FeldDiff([]byte(`{"m":{"stein":{"basis":"#1","licht":"#2"}},"a":1,"weg":true}`),
		[]byte(`{"m":{"stein":{"basis":"#3","licht":"#2"}},"a":1,"neu":[1]}`))
	if err != nil {
		t.Fatal(err)
	}
	var zeilen []string
	for _, f := range felder {
		zeilen = append(zeilen, Wert(f.Alt)+" "+f.Pfad+" "+Wert(f.Neu))
	}
	if got := strings.Join(zeilen, "|"); got != "#1 m.stein.basis #3|– neu [1]|true weg –" {
		t.Errorf("FeldDiff: %s", got)
	}
}

func TestMaterialienEinerSpec(t *testing.T) {
	t.Parallel()
	b, _ := os.ReadFile(filepath.Join(repoWurzel(t), "tools/paper/fixtures/specs/monarch.paper.json"))
	m := Materialien(b)
	for _, name := range []string{"koenigsblau", "koenigsrot", "smaragd", "hermelin"} { // Lagen, Umhang, tausch
		if !m[name] {
			t.Errorf("%s fehlt in %v", name, m)
		}
	}
}

func TestStilSet(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	r, err := StilSet(context.Background(), cli, []byte(`{"materialien":{"stein":{"basis":"#a8a294"}}}`))
	if err != nil {
		t.Fatal(err)
	}
	if len(r.Felder) != 1 || r.Felder[0].Pfad != "materialien.stein.basis" || Wert(r.Felder[0].Neu) != "#a8a294" {
		t.Errorf("Felder: %+v", r.Felder)
	}
	if len(r.Betroffen) != 1 || r.Betroffen[0].ID != "mauer" {
		t.Errorf("Betroffen: %+v", r.Betroffen)
	}
	b, _ := os.ReadFile(filepath.Join(root, filepath.FromSlash(StilPfad)))
	if !strings.Contains(string(b), "#a8a294") {
		t.Error("Stil nicht geschrieben")
	}
	if r, err := StilSet(context.Background(), cli, []byte(`{"papier":{"lage":{"dx":1}}}`)); err != nil || len(r.Betroffen) != 3 {
		t.Errorf("papier betrifft alle: %+v %v", r, err)
	}
}

func TestStilSetAbgelehnt(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	vorher := hashBaum(t, root, StilPfad)
	for patch, regel := range map[string]string{
		`{"farbe":"#fff"}`:                          "schema",
		`{"materialien":{"stein":{"basis":"rot"}}}`: "schema",
		`{"einzeltoene":{"stein":"#ffffff"}}`:       "material-doppelt",
		`{"materialien":{"koenigsrot":null}}`:       "material-unbekannt",
	} {
		_, err := StilSet(context.Background(), cli, []byte(patch))
		ab, ok := AlsAbgelehnt(err)
		if !ok || ab.Fehler[0].Regel != regel {
			t.Errorf("%s: %v", patch, err)
		}
	}
	if hashBaum(t, root, StilPfad) != vorher {
		t.Error("Stil-Datei geändert")
	}
}
