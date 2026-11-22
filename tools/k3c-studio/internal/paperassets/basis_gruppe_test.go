package paperassets

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// doppelWolf legt dieselbe ID „wolf“ in zwei Gruppen an (B-819).
func doppelWolf(t *testing.T) (string, CLI) {
	t.Helper()
	root, cli := testRepo(t)
	b, err := os.ReadFile(filepath.Join(root, "public/grafik/figuren/monarch.paper.json"))
	if err != nil {
		t.Fatal(err)
	}
	wolf := strings.Replace(string(b), `"id": "monarch"`, `"id": "wolf"`, 1)
	schreibe(t, root, "public/grafik/gegner/wolf.paper.json", wolf)
	schreibe(t, root, "public/grafik/reittiere/wolf.paper.json", wolf)
	return root, cli
}

// B-819/AC-01: „gegner/wolf“ ist auffindbar und änderbar, das Reittier bleibt unberührt.
func TestGruppeIDLesbarUndAenderbar(t *testing.T) {
	t.Parallel()
	root, cli := doppelWolf(t)
	a, err := Finde(root, "gegner/wolf")
	if err != nil || a.Gruppe != "gegner" || a.Pfad != "public/grafik/gegner/wolf.paper.json" {
		t.Fatalf("Finde: %+v, %v", a, err)
	}
	reittier := filepath.Join(root, "public/grafik/reittiere/wolf.paper.json")
	vorher, _ := os.ReadFile(reittier)
	r, err := Set(context.Background(), cli, "gegner/wolf", []byte(`{"umhang":{"material":"purpur"}}`))
	if err != nil {
		t.Fatal(err)
	}
	if r.Asset.Gruppe != "gegner" {
		t.Errorf("Asset: %+v", r.Asset)
	}
	b, _ := os.ReadFile(filepath.Join(root, a.Pfad))
	if !strings.Contains(string(b), `"material": "purpur"`) {
		t.Errorf("gegner/wolf nicht geändert:\n%.300s", b)
	}
	nachher, _ := os.ReadFile(reittier)
	if string(vorher) != string(nachher) {
		t.Error("reittiere/wolf wurde verändert")
	}
}

// B-819/AC-02: bloße ID in zwei Gruppen wird mit Trefferliste abgelehnt, nichts wird geschrieben.
func TestMehrdeutigeIDAbgelehnt(t *testing.T) {
	t.Parallel()
	root, cli := doppelWolf(t)
	_, err := Finde(root, "wolf")
	var m *MehrdeutigFehler
	if !errors.As(err, &m) || strings.Join(m.Treffer, ",") != "gegner/wolf,reittiere/wolf" || !strings.Contains(err.Error(), "mehrdeutig") {
		t.Fatalf("Finde wolf: %v", err)
	}
	vorher := hashBaum(t, root, "public/grafik")
	if _, err := Set(context.Background(), cli, "wolf", []byte(`{"umhang":{"material":"purpur"}}`)); !errors.As(err, &m) {
		t.Errorf("Set wolf: %v", err)
	}
	if hashBaum(t, root, "public/grafik") != vorher {
		t.Error("abgelehntes Set hat Dateien verändert")
	}
	if _, err := Anlegen(context.Background(), cli, "wolf", "figur", "x", "natur"); err == nil || !strings.Contains(err.Error(), "gibt es schon") {
		t.Errorf("Anlegen wolf: %v", err)
	}
}

// B-819: eindeutige bloße IDs gehen weiter, unbekannte Gruppe nennt die vorhandenen Gruppen.
func TestGruppeIDFehlerfaelle(t *testing.T) {
	t.Parallel()
	root, _ := doppelWolf(t)
	if a, err := Finde(root, "monarch"); err != nil || a.Gruppe != "figuren" {
		t.Errorf("monarch: %+v, %v", a, err)
	}
	_, err := Finde(root, "tiere/wolf")
	if err == nil || !strings.Contains(err.Error(), "tiere") || !strings.Contains(err.Error(), "figuren, gebaeude, gegner, reittiere") {
		t.Errorf("unbekannte Gruppe: %v", err)
	}
	if _, err := Finde(root, "gegner/monarch"); err == nil || !strings.Contains(err.Error(), "fehlt") {
		t.Errorf("gegner/monarch: %v", err)
	}
}
