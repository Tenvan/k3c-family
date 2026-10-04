package main

import (
	"bytes"
	"context"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
)

const testToken = "geheim-1234"

// testServer startet den echten Handler mit Räumen in-process (httptest, nur Loopback). skew verschiebt die Uhr des
// Managers, damit ein Test die Leer-Frist der Räume überspringen kann.
func testServer(t *testing.T) (*httptest.Server, *room.Manager, *store.Saves, *atomic.Int64) {
	t.Helper()
	dir := t.TempDir()
	saves := &store.Saves{Dir: filepath.Join(dir, "saves")}
	m := room.NewManager(saves)
	skew := &atomic.Int64{}
	m.Now = func() time.Time { return time.Now().Add(time.Duration(skew.Load())) }
	ctx, cancel := context.WithCancel(context.Background())
	go m.Run(ctx)
	srv := httptest.NewServer(k3cnet.NewHandler(k3cnet.Config{Rooms: m, StatusToken: testToken, StartedAt: time.Now(),
		Saves: saves, Reports: &store.Reports{Dir: filepath.Join(dir, "reports")}}))
	t.Cleanup(func() { srv.Close(); cancel(); m.Close() })
	return srv, m, saves, skew
}

// recorder hält jede gesendete Nachricht je Gerät fest (an der Verbindung, vor dem Schreiben).
type recorder struct {
	mu   sync.Mutex
	msgs map[string][][]byte
}

func (r *recorder) sent(device string, msg []byte) {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.msgs == nil {
		r.msgs = map[string][][]byte{}
	}
	r.msgs[device] = append(r.msgs[device], msg)
}

// inputs liefert die input-Nachrichten je Bot, der Schlüssel ohne Lauf-Kennung („Raum-Bot“).
func (r *recorder) inputs() map[string][]string {
	out := map[string][]string{}
	for dev, msgs := range r.msgs {
		parts := strings.Split(dev, "-")
		key := parts[len(parts)-2] + "-" + parts[len(parts)-1]
		for _, m := range msgs {
			if bytes.Contains(m, []byte(`"t":"input"`)) {
				out[key] = append(out[key], string(m))
			}
		}
	}
	return out
}

func testConfig(srv *httptest.Server, seed string, d time.Duration, rec *recorder) config {
	return config{url: srv.URL, token: testToken, seed: seed, rooms: 2, players: 2, duration: d,
		tag: strings.ReplaceAll(strings.ToLower(seed), "_", ""), sent: rec.sent}
}

// AC-01 und AC-05: 2 Räume × 2 Bots, 5 s → je Raum eine Tick-Reihe; danach kein Bot mehr verbunden, nach der Leer-Frist
// sind Räume und Test-Spielstände weg; die Ausgabe nennt das Token nicht.
func TestLaufSammeltTicksUndRaeumtAuf(t *testing.T) {
	srv, m, saves, skew := testServer(t)
	var out, errOut bytes.Buffer
	args := []string{"-url", srv.URL, "-token", testToken, "-rooms", "2", "-players", "2", "-duration", "5s"}
	if code := run(context.Background(), args, func(string) string { return "" }, &out, &errOut); code != 0 {
		t.Fatalf("Exit %d, stderr: %s", code, errOut.String())
	}
	if strings.Contains(out.String()+errOut.String(), testToken) {
		t.Fatalf("Token in der Ausgabe:\n%s%s", out.String(), errOut.String())
	}
	if n := strings.Count(out.String(), "Zustände, Tick"); n != 2 {
		t.Fatalf("erwartet 2 Tick-Reihen, Ausgabe:\n%s", out.String())
	}
	status, _ := m.Status()
	for _, r := range status {
		if !strings.HasPrefix(r.Name, room.TestPrefix) || r.Devices != 0 || r.Running {
			t.Fatalf("Raum nach dem Lauf: %+v", r)
		}
	}
	skew.Store(int64(room.EmptyFor + time.Second))
	m.Sweep()
	if status, _ := m.Status(); len(status) != 0 || saves.Count() != 0 {
		t.Fatalf("nach der Leer-Frist offen: %d Räume, %d Spielstände", len(status), saves.Count())
	}
}

// AC-01: Die Tick-Reihe je Raum steigt und deckt den Lauf ab.
func TestTickReiheJeRaum(t *testing.T) {
	srv, _, _, _ := testServer(t)
	runs, err := load(context.Background(), testConfig(srv, "reihe", 2*time.Second, &recorder{}))
	if err != nil || len(runs) != 2 {
		t.Fatalf("runs %d, err %v", len(runs), err)
	}
	for _, r := range runs {
		if len(r.Ticks) < 20 || r.Code == "" || r.Errors != 0 {
			t.Fatalf("%s: %d Ticks, Code %q, %d Fehler", r.Name, len(r.Ticks), r.Code, r.Errors)
		}
		for i := 1; i < len(r.Ticks); i++ {
			if r.Ticks[i].Tick <= r.Ticks[i-1].Tick || r.Ticks[i].At < r.Ticks[i-1].At {
				t.Fatalf("%s: Reihe fällt bei %d: %v", r.Name, i, r.Ticks[i-1:i+1])
			}
		}
	}
}

// AC-02: Zwei Läufe mit gleichem Seed senden je Bot dieselbe Eingabefolge (gemeinsamer Anfang, weil die Zahl der
// Zustände mit der Wanduhr schwankt); ein anderer Seed sendet eine andere.
func TestGleicherSeedGleicheEingaben(t *testing.T) {
	lauf := func(seed, tag string) map[string][]string {
		srv, _, _, _ := testServer(t)
		rec := &recorder{}
		c := testConfig(srv, seed, 2*time.Second, rec)
		c.tag = tag
		if _, err := load(context.Background(), c); err != nil {
			t.Fatal(err)
		}
		return rec.inputs()
	}
	a, b, other := lauf("seed-a", "eins"), lauf("seed-a", "zwei"), lauf("seed-b", "drei")
	if len(a) != 4 || len(b) != 4 {
		t.Fatalf("erwartet 4 Bots, bekam %d und %d", len(a), len(b))
	}
	differs := false
	for bot, seqA := range a {
		seqB := b[bot]
		n := min(len(seqA), len(seqB), len(other[bot]))
		if n < 20 {
			t.Fatalf("%s: zu wenige Eingaben (%d, %d)", bot, len(seqA), len(seqB))
		}
		for i := range n {
			if seqA[i] != seqB[i] {
				t.Fatalf("%s: Eingabe %d weicht ab: %s / %s", bot, i, seqA[i], seqB[i])
			}
			differs = differs || seqA[i] != other[bot][i]
		}
	}
	if !differs {
		t.Fatal("anderer Seed sendet dieselben Eingaben")
	}
}

// Server nicht erreichbar oder Token falsch → Exit 2, keine Räume, Token nicht in der Ausgabe.
func TestFehlerOhneRaeume(t *testing.T) {
	srv, m, _, _ := testServer(t)
	for name, args := range map[string][]string{
		"Token falsch":     {"-url", srv.URL, "-token", "falsch-9876"},
		"nicht erreichbar": {"-url", "http://127.0.0.1:1", "-token", "falsch-9876"},
		"kein Token":       {"-url", srv.URL},
	} {
		var out, errOut bytes.Buffer
		code := run(context.Background(), append(args, "-duration", "1s"), func(string) string { return "" }, &out, &errOut)
		if code != 2 || strings.Contains(out.String()+errOut.String(), "falsch-9876") {
			t.Fatalf("%s: Exit %d, Ausgabe %s%s", name, code, out.String(), errOut.String())
		}
	}
	if status, _ := m.Status(); len(status) != 0 {
		t.Fatalf("Räume angelegt: %+v", status)
	}
}

// Das Token aus der Umgebung gilt, wenn -token fehlt.
func TestTokenAusUmgebung(t *testing.T) {
	c, err := parse(nil, func(k string) string { return map[string]string{envToken: "env"}[k] }, &bytes.Buffer{})
	if err != nil || c.token != "env" {
		t.Fatalf("token %q, err %v", c.token, err)
	}
}

func TestWsURL(t *testing.T) {
	if got := wsURL("https://pi:8080"); got != "wss://pi:8080/ws" {
		t.Fatal(got)
	}
	if got := wsURL("http://127.0.0.1:8080"); got != "ws://127.0.0.1:8080/ws" {
		t.Fatal(got)
	}
}
