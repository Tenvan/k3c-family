package world

import (
	"encoding/json"
	"strings"
	"testing"
)

// WE6.3 (B-987, Studio-Anteil): Das Insel-Feld endBoss und das Welt-Feld skins lassen sich über world_set setzen und
// bleiben in der Datei erhalten; die Welt bleibt gültig.
func TestSetEndBossUndSkinsBleibenErhalten(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	patch := `{"skins":{"gegner/wolf":"gegner/ninja"},"islands":[{"id":"island1","endBoss":"forestChieftain"}]}`
	if _, err := e.Set(ctx, "hanse", json.RawMessage(patch)); err != nil {
		t.Fatal(err)
	}
	text := readWorld(t, e, "hanse")
	for _, want := range []string{`"endBoss": "forestChieftain"`, `"skins"`, `"gegner/ninja"`} {
		if !strings.Contains(text, want) {
			t.Fatalf("fehlt %q:\n%s", want, text)
		}
	}
}

// Ein Endboss, der es nicht gibt, lehnt die Prüfung ab und die Datei bleibt, wie sie ist.
func TestSetEndBossUnbekanntLehntAb(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	before := readWorld(t, e, "hanse")
	if _, err := e.Set(ctx, "hanse", json.RawMessage(`{"islands":[{"id":"island1","endBoss":"gibtEsNicht"}]}`)); err == nil {
		t.Fatal("unbekannter Endboss muss ablehnen")
	}
	if readWorld(t, e, "hanse") != before {
		t.Error("Datei darf sich bei Ablehnung nicht ändern")
	}
}
