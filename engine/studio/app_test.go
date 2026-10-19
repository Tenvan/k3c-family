package studio

import (
	"context"
	"errors"
	"image"
	"image/png"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"

	assets "k3c/tools/assetkit"
)

// studioRepo legt ein Mini-Repo an: Slot-Register mit einem Grafik-Slot, zwei Bilder unter public/, daneben eine
// Datei außerhalb von public/.
func studioRepo(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	write(t, filepath.Join(root, assets.RegisterPath),
		`{"_comment": "", "slots": [{"id": "farm", "art": "grafik", "kategorie": "gebaeude", "beschreibung": "Farm", "wahl": []}]}`)
	write(t, filepath.Join(root, "geheim.txt"), "nicht ausliefern")
	for _, name := range []string{"well.png", "wagon.png"} {
		p := filepath.Join(root, "public", "grafik", "town", name)
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		f, err := os.Create(p)
		if err != nil {
			t.Fatal(err)
		}
		if err := png.Encode(f, image.NewNRGBA(image.Rect(0, 0, 64, 48))); err != nil {
			t.Fatal(err)
		}
		_ = f.Close()
	}
	return root
}

func write(t *testing.T, path, text string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
}

// B-459/AC-07: Speichern mit dem Stand aus AssetsList gelingt; hat ein anderer Prozess dazwischen geschrieben, wird
// abgelehnt und nichts geschrieben.
func TestAssetsSetSchreibregel(t *testing.T) {
	app := &App{root: studioRepo(t)}
	v := app.AssetsList()
	if v.Error != "" || v.Stand == "" || v.Preview != "/" || len(v.Slots) != 1 || v.Slots[0].Status != assets.StatusGap {
		t.Fatalf("Liste: %+v", v)
	}
	saved, err := app.AssetsSet("farm", []assets.Choice{{File: "grafik/town/well.png"}}, v.Stand)
	if err != nil || saved.Slot.Status != assets.StatusAssigned || saved.Stand == v.Stand {
		t.Fatalf("Speichern auf aktuellem Stand: %+v %v", saved, err)
	}
	if _, err := assets.SetChoices(app.root, "farm", []assets.Choice{{File: "grafik/town/wagon.png"}}); err != nil {
		t.Fatal(err) // anderer Schreiber (MCP)
	}
	before, _ := os.ReadFile(filepath.Join(app.root, assets.RegisterPath))
	_, err = app.AssetsSet("farm", []assets.Choice{{File: "grafik/town/well.png"}}, saved.Stand)
	if !errors.Is(err, assets.ErrStale) || err.Error() != "Register geändert, bitte erneut speichern" {
		t.Fatalf("veralteter Stand: %v", err)
	}
	after, _ := os.ReadFile(filepath.Join(app.root, assets.RegisterPath))
	if string(before) != string(after) {
		t.Error("abgelehntes Speichern hat geschrieben")
	}
}

// B-459/AC-07: Ändert ein anderer Prozess data/assets.json, sendet das Backend assets:changed.
func TestAssetsChangedEreignis(t *testing.T) {
	app := &App{root: studioRepo(t), poll: 10 * time.Millisecond, bus: newBus()}
	events, unsub := app.bus.subscribe()
	defer unsub()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	app.watch(ctx)
	if _, err := assets.SetChoices(app.root, "farm", []assets.Choice{{File: "grafik/town/well.png"}}); err != nil {
		t.Fatal(err)
	}
	select {
	case name := <-events:
		if name != evAssetsChanged {
			t.Errorf("Ereignis %q", name)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("kein assets:changed nach Änderung")
	}
}

// Der Handler liefert nur Dateien unter public/ und nur lesend.
func TestPublicHandler(t *testing.T) {
	h := publicHandler(studioRepo(t))
	get := func(method, p string) *httptest.ResponseRecorder {
		req := httptest.NewRequest(method, "/", nil)
		req.URL.Path = p // ungeprüft durchreichen, wie ein Aufruf aus der Oberfläche
		rec := httptest.NewRecorder()
		h.ServeHTTP(rec, req)
		return rec
	}
	if rec := get(http.MethodGet, "/grafik/town/well.png"); rec.Code != http.StatusOK || rec.Header().Get("Content-Type") != "image/png" {
		t.Errorf("Bild: %d %q", rec.Code, rec.Header().Get("Content-Type"))
	}
	for _, p := range []string{"/../geheim.txt", "/grafik/../../geheim.txt", `/..\geheim.txt`, "/../data/assets.json", "/grafik", "/", "/fehlt.png"} {
		if rec := get(http.MethodGet, p); rec.Code == http.StatusOK {
			t.Errorf("%s ausgeliefert: %q", p, rec.Body.String())
		}
	}
	if rec := get(http.MethodPost, "/grafik/town/well.png"); rec.Code != http.StatusMethodNotAllowed {
		t.Errorf("POST: %d", rec.Code)
	}
}
