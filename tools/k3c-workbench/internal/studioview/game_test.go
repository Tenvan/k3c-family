package studioview

import (
	"context"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

// WB1.4 (005 › 5, Sprint-README WB1 › D-04): „Im Spiel ansehen“ legt den Testspielstand voll-<seed> in der eingebundenen
// Engine an und liefert die Adresse des eigenen Loopback-Listeners; Spielstände liegen außerhalb des Repos.

// repoRoot ist die Wurzel des Checkouts (tools/k3c-workbench/internal/studioview → ../../../..).
func repoRoot(t *testing.T) string {
	t.Helper()
	root, err := filepath.Abs(filepath.Join("..", "..", "..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	return root
}

// gameRoot ist ein Checkout mit der Standard-Welt und einem gebauten Client (dist/index.html).
func gameRoot(t *testing.T, client bool) string {
	t.Helper()
	root := t.TempDir()
	raw, err := os.ReadFile(filepath.Join(repoRoot(t), "data", "worlds", "standard.json"))
	if err != nil {
		t.Fatal(err)
	}
	write(t, root, "data/worlds/standard.json", string(raw))
	write(t, root, "public/audio/tag.ogg", "OggS")
	if client {
		write(t, root, "dist/index.html", "<!doctype html><title>K3C</title>")
	}
	return root
}

func newGame(t *testing.T, root string) (*Game, string) {
	t.Helper()
	saves := t.TempDir()
	ctx, cancel := context.WithCancel(context.Background())
	g := NewGame(ctx, root, saves, nil)
	t.Cleanup(func() {
		cancel()
		g.Close(time.Second)
	})
	return g, saves
}

func TestImSpielAnsehenLegtVollausbauAnUndLiefertLoopbackAdresse(t *testing.T) {
	root := gameRoot(t, true)
	g, saves := newGame(t, root)
	link, err := g.Open("abc")
	if err != nil {
		t.Fatal(err)
	}
	u, err := url.Parse(link)
	if err != nil || u.Hostname() != "127.0.0.1" || u.Path != "/index.html" || u.Port() == "" {
		t.Fatalf("Adresse %q: %v", link, err)
	}
	if want := "game.html?autostart=1&save=voll-abc"; u.Fragment != want {
		t.Errorf("Fragment %q, erwartet %q", u.Fragment, want)
	}
	if _, err := os.Stat(filepath.Join(saves, "voll-abc.json")); err != nil {
		t.Errorf("Testspielstand fehlt: %v", err)
	}
	if _, err := os.Stat(filepath.Join(root, "saves")); !os.IsNotExist(err) {
		t.Errorf("saves/ im Checkout angelegt: %v", err)
	}
	// Derselbe Listener liefert Seite, Spiel-API und public/.
	for path, want := range map[string]string{"/index.html": "K3C", "/audio/tag.ogg": "OggS", "/api/health": ""} {
		body := get(t, "http://"+u.Host+path, "")
		if !strings.Contains(body, want) {
			t.Errorf("%s: %q ohne %q", path, body, want)
		}
	}
	again, err := g.Open("abc")
	if err != nil || !strings.HasPrefix(again, "http://"+u.Host+"/") {
		t.Errorf("zweiter Aufruf %q, %v: neuer Listener statt desselben", again, err)
	}
}

func TestImSpielAnsehenAblehnungen(t *testing.T) {
	g, _ := newGame(t, gameRoot(t, true))
	for _, seed := range []string{"", "Abc", "a b", "../x", strings.Repeat("a", 28)} {
		if _, err := g.Open(seed); err == nil {
			t.Errorf("Seed %q angenommen", seed)
		}
	}
	g2, _ := newGame(t, gameRoot(t, false))
	if _, err := g2.Open("abc"); err == nil || !strings.Contains(err.Error(), "task build") {
		t.Errorf("ohne Client: %v", err)
	}
}

func TestListenerNurLoopbackUndEigenerHost(t *testing.T) {
	g, _ := newGame(t, gameRoot(t, true))
	link, err := g.Open("abc")
	if err != nil {
		t.Fatal(err)
	}
	u, _ := url.Parse(link)
	if !strings.HasPrefix(u.Host, "127.0.0.1:") {
		t.Fatalf("Host %q", u.Host)
	}
	if code := status(t, "http://"+u.Host+"/index.html", "boese.example"); code != http.StatusForbidden {
		t.Errorf("fremder Host-Kopf: %d", code)
	}
	if code := status(t, "http://"+u.Host+"/", ""); code != http.StatusNotFound {
		t.Errorf("Verzeichnisliste der Wurzel: %d", code)
	}
}

func get(t *testing.T, link, host string) string {
	t.Helper()
	res := do(t, link, host)
	defer func() { _ = res.Body.Close() }()
	b, _ := io.ReadAll(res.Body)
	return string(b)
}

func status(t *testing.T, link, host string) int {
	t.Helper()
	res := do(t, link, host)
	_ = res.Body.Close()
	return res.StatusCode
}

func do(t *testing.T, link, host string) *http.Response {
	t.Helper()
	req, err := http.NewRequest(http.MethodGet, link, nil)
	if err != nil {
		t.Fatal(err)
	}
	if host != "" {
		req.Host = host
	}
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	return res
}

// Review WB1.4: Nach Close startet Open nichts mehr (kein geschlossener Server, kein beendeter Raum-Takt); doppeltes Close
// ist harmlos, und Close beendet die Räume selbst, ohne auf das Ende des äußeren ctx zu warten.
func TestOpenNachCloseAbgelehnt(t *testing.T) {
	g, _ := newGame(t, gameRoot(t, true))
	if _, err := g.Open("abc"); err != nil {
		t.Fatal(err)
	}
	start := time.Now()
	g.Close(3 * time.Second)
	if d := time.Since(start); d > 2*time.Second {
		t.Errorf("Close wartete %v: Räume nicht selbst beendet", d)
	}
	g.Close(time.Second)
	if _, err := g.Open("abc"); err == nil || !strings.Contains(err.Error(), "beendet") {
		t.Errorf("Open nach Close: %v", err)
	}
}

// Review WB1.4: Schreibende Anfragen an die Spiel-API nur von derselben Seite (Origin, Sec-Fetch-Site).
func TestSpielAPISchreibendNurGleicheHerkunft(t *testing.T) {
	g, _ := newGame(t, gameRoot(t, true))
	link, err := g.Open("abc")
	if err != nil {
		t.Fatal(err)
	}
	u, _ := url.Parse(link)
	for name, hdr := range map[string][2]string{"fremder Origin": {"Origin", "http://boese.example"}, "cross-site": {"Sec-Fetch-Site", "cross-site"}} {
		req, _ := http.NewRequest(http.MethodPost, "http://"+u.Host+"/api/rooms", strings.NewReader("{}"))
		req.Header.Set(hdr[0], hdr[1])
		res, err := http.DefaultClient.Do(req)
		if err != nil {
			t.Fatal(err)
		}
		_ = res.Body.Close()
		if res.StatusCode != http.StatusForbidden {
			t.Errorf("%s: %d", name, res.StatusCode)
		}
	}
}
