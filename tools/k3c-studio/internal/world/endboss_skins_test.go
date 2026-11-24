package world

import (
	"strings"
	"testing"
)

// WE6.4 (B-981): Der Endboss je Insel lässt sich setzen und entfernen; ein unbekannter Endboss lehnt ab.
func TestIslandEndBossSetzenUndEntfernen(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	if got := e.SetIslandEndBoss(ctx, "hanse", "island1", "forestChieftain"); got.Error != "" {
		t.Fatalf("setzen: %s", got.Error)
	}
	if d := e.Load("hanse"); d.Islands[0].EndBoss != "forestChieftain" {
		t.Fatalf("Endboss nicht gelesen: %+v", d.Islands[0])
	}
	if got := e.SetIslandEndBoss(ctx, "hanse", "island1", "gibtEsNicht"); got.Error == "" {
		t.Error("unbekannter Endboss muss ablehnen")
	}
	if got := e.SetIslandEndBoss(ctx, "hanse", "island1", ""); got.Error != "" {
		t.Fatalf("entfernen: %s", got.Error)
	}
	if d := e.Load("hanse"); d.Islands[0].EndBoss != "" {
		t.Errorf("Endboss bleibt nach dem Entfernen: %q", d.Islands[0].EndBoss)
	}
	if d := e.Load("hanse"); len(d.EndBosses) == 0 || d.EndBosses[0].Name == "" {
		t.Errorf("Endboss-Liste fehlt: %+v", d.EndBosses)
	}
}

// WE6.4 (B-984): Skins setzen, lesen und leeren; ein ungültiges Format lehnt ab und die Datei bleibt unverändert.
func TestSkinsSetzenLesenUndAblehnen(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	if got := e.SetSkins(ctx, "hanse", map[string]string{"gegner/wolf": "gegner/ninja"}); got.Error != "" {
		t.Fatalf("setzen: %s", got.Error)
	}
	if d := e.Load("hanse"); d.Skins["gegner/wolf"] != "gegner/ninja" {
		t.Fatalf("Skins nicht gelesen: %+v", d.Skins)
	}
	before := readWorld(t, e, "hanse")
	if got := e.SetSkins(ctx, "hanse", map[string]string{"wolf": "gegner/ninja"}); got.Error == "" || !strings.Contains(got.Error, "wolf") {
		t.Errorf("ungültiges Format muss ablehnen: %q", got.Error)
	}
	if readWorld(t, e, "hanse") != before {
		t.Error("abgelehnte Skins dürfen die Datei nicht ändern")
	}
	if got := e.SetSkins(ctx, "hanse", nil); got.Error != "" {
		t.Fatalf("leeren: %s", got.Error)
	}
	if d := e.Load("hanse"); len(d.Skins) != 0 {
		t.Errorf("Skins bleiben nach dem Leeren: %+v", d.Skins)
	}
}

// Eine unbekannte Insel oder ein unbekannter Endboss lehnt ab; die Datei bleibt unverändert, es entsteht keine neue Insel.
func TestIslandEndBossLehntUnbekannteInselUndBossAb(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	before := readWorld(t, e, "hanse")
	if got := e.SetIslandEndBoss(ctx, "hanse", "gibtEsNicht", "forestChieftain"); got.Error == "" {
		t.Error("unbekannte Insel muss ablehnen")
	}
	if got := e.SetIslandEndBoss(ctx, "hanse", "island1", "gibtEsNicht"); got.Error == "" {
		t.Error("unbekannter Endboss muss ablehnen")
	}
	if readWorld(t, e, "hanse") != before {
		t.Error("abgelehnte Endboss-Änderung darf die Datei nicht ändern")
	}
}
