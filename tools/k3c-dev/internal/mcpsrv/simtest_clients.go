package mcpsrv

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"time"

	"k3c/engine/sim"
	"k3c/tools/k3c-dev/internal/balance"
	"k3c/tools/k3c-dev/internal/botdev"
	"k3c/tools/k3c-dev/internal/botfeed"
	"k3c/tools/k3c-dev/internal/browser"
	"k3c/tools/k3c-dev/internal/serverapi"
)

// Modus online mit Clients (B-348/AC-04, TR1.3): je Client ein Raum. Die Workbench legt ihn mit einem
// Beobachter-Gerät an (botdev), öffnet den Client headless mit ?room=<Code>&players=<n>&botfeed=<Feed> und schickt
// im Takt die Kommandos ihres Bots über den Bot-Feed (/bot/<lauf>/<client>, nur Loopback). Den Zustand liest der Bot
// über das Beobachter-Gerät: Das Protokoll kennt keinen Zuschauer, also belegt es einen Platz und spielt selbst als
// Bot mit; der Client bräuchte für den anderen Weg (Zustand über den Feed) eine Erweiterung von B-349.
// Mit attach hängt sich der Lauf an einen laufenden Raum, die Clients öffnet der Mensch mit der Feed-Adresse.

var clientTick = 200 * time.Millisecond // Takt der Bot-Kommandos; Test-Naht

// clientSeat ist ein Client des Laufs: sein Raum, das Beobachter-Gerät, der Feed und der Browser.
type clientSeat struct {
	code     string
	feedName string
	feed     *botfeed.Feed
	observer *botdev.Device
	stop     func() // Browser beenden; nil bei attach
}

// launchBrowser ist der Standard für Server.launch: Browser headless starten; stop beendet ihn und löscht das Profil.
func launchBrowser(ctx context.Context, exe, profile, url string) (func(), error) {
	cmd, err := browser.Start(ctx, exe, profile, url)
	if err != nil {
		return nil, err
	}
	return func() {
		_ = cmd.Kill()
		_ = cmd.Wait()
		_ = os.RemoveAll(profile)
	}, nil
}

// clientsRunner prüft, was der Modus braucht (Browser, Vite, Feed), und liefert den Runner.
func (s *Server) clientsRunner(ctx context.Context, spec simSpec, c *serverapi.Client) (func(context.Context, *simRun), error) {
	feedBase := s.feedBase()
	if feedBase == "" {
		return nil, fmt.Errorf("sim_test start abgelehnt: der Bot-Feed braucht den laufenden HTTP-Server von k3c-dev")
	}
	var exe, vite string
	if spec.attach == "" {
		var err error
		if exe, err = s.findBrowser(); err != nil {
			return nil, fmt.Errorf("sim_test start abgelehnt: %v; laufende Clients mit attach: <Raumcode> nutzen", err)
		}
		if vite, err = s.viteBase(ctx); err != nil {
			return nil, fmt.Errorf("sim_test start abgelehnt: %v, erst svc_start Vite", err)
		}
	}
	return func(ctx context.Context, run *simRun) { s.runClients(ctx, run, spec, c, exe, vite, feedBase) }, nil
}

// feedBase ist ws://<Adresse von k3c-dev>, leer solange der HTTP-Server nicht läuft.
func (s *Server) feedBase() string {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.http == nil {
		return ""
	}
	return "ws://" + s.addr
}

// viteBase ist die Adresse des Vite-Diensts des Checkouts; antwortet er nicht, ist das ein Fehler.
func (s *Server) viteBase(ctx context.Context) (string, error) {
	port := os.Getenv("K3C_VITE_PORT")
	if ws := s.ws(ctx); ws.name != "" {
		w, err := s.worktreeServices(ws)
		if err != nil {
			return "", err
		}
		port = w.ports["K3C_VITE_PORT"]
	}
	if port == "" {
		port = "5173"
	}
	base := "http://127.0.0.1:" + port
	ctx, cancel := context.WithTimeout(ctx, 2*time.Second)
	defer cancel()
	req, _ := http.NewRequestWithContext(ctx, http.MethodGet, base+"/", nil)
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", fmt.Errorf("der Vite-Dienst des Checkouts antwortet nicht unter %s", base)
	}
	_ = resp.Body.Close()
	return base, nil
}

func (s *Server) runClients(ctx context.Context, run *simRun, spec simSpec, c *serverapi.Client, exe, vite, feedBase string) {
	o := &onlineRun{}
	if st, err := c.Status(ctx); err == nil {
		o.failures = len(st.Failures)
	}
	started := time.Now()
	s.sims.setPhase(run, "verbinden")
	seats, err := s.openSeats(ctx, run, spec, o, c.Base, exe, vite, feedBase)
	defer s.closeSeats(seats)
	if err != nil {
		s.finishRun(run, "fehler", "", []string{"verbinden: " + err.Error()}, simReportDir(run))
		return
	}
	s.sims.setPhase(run, "messen")
	bot, _ := balance.BotByName(spec.bots)
	logPath := filepath.Join(run.root, "logs", "k3c-client.jsonl")
	deadline := time.After(spec.duration)
	drive, sample := time.NewTicker(clientTick), time.NewTicker(onlineSample)
	defer drive.Stop()
	defer sample.Stop()
	for done := false; !done; {
		select {
		case <-ctx.Done():
			done = true
		case <-deadline:
			done = true
		case <-drive.C:
			for _, seat := range seats {
				seat.drive(ctx, bot)
			}
		case <-sample.C:
			o.sample(ctx, c)
			s.sims.setLines(run, clientLines(spec, o, seats, 0, readClientMetrics(logPath, started, o.codes)))
		}
	}
	o.sample(context.Background(), c)
	newFailures := 0
	if st, err := c.Status(context.Background()); err == nil {
		newFailures = len(st.Failures) - o.failures
	}
	m := readClientMetrics(logPath, started, o.codes)
	s.finishRun(run, "fertig", clientVerdict(spec, o, seats, m, newFailures), clientLines(spec, o, seats, newFailures, m), simReportDir(run))
}

// openSeats legt je Client Raum, Beobachter und Feed an und startet den Browser (ohne attach).
func (s *Server) openSeats(ctx context.Context, run *simRun, spec simSpec, o *onlineRun, base, exe, vite, feedBase string) ([]*clientSeat, error) {
	bot, _ := balance.BotByName(spec.bots)
	var seats []*clientSeat
	for i := range spec.clients {
		d, err := botdev.Dial(ctx, base, fmt.Sprintf("simtest-%s-beobachter-%d", run.id, i), bot)
		if err != nil {
			return seats, err
		}
		o.devices = append(o.devices, d)
		seat := &clientSeat{observer: d, feedName: run.id + "/" + strconv.Itoa(i)}
		seats = append(seats, seat)
		if spec.attach != "" {
			seat.code, err = spec.attach, d.Join(ctx, spec.attach)
		} else {
			seat.code, err = d.Create(ctx, fmt.Sprintf("test-sim-%s-%d", run.id, i))
		}
		if err != nil {
			return seats, err
		}
		o.codes = append(o.codes, seat.code)
		seat.feed = s.feeds.Open(seat.feedName)
		if spec.attach != "" {
			continue
		}
		url := fmt.Sprintf("%s/game.html?room=%s&players=%d&botfeed=%s%s%s", vite, seat.code, spec.players,
			feedBase, botfeed.Prefix, seat.feedName)
		// eigener Ordner je Start: Lauf-IDs beginnen nach einem Neustart von k3c-dev wieder bei 1
		profile := filepath.Join(run.root, ".work", "simtest-browser",
			fmt.Sprintf("%s-%d-%d", run.id, run.started.UnixNano(), i))
		if seat.stop, err = s.launch(ctx, exe, profile, url); err != nil {
			return seats, fmt.Errorf("start des Browsers gescheitert: %v", err)
		}
		s.log.Info("🌐 client gestartet", "ns", "simtest", "id", run.id, "room", seat.code, "url", url)
	}
	return seats, nil
}

func (s *Server) closeSeats(seats []*clientSeat) {
	for _, seat := range seats {
		if seat.stop != nil {
			seat.stop()
		}
		if seat.feed != nil {
			s.feeds.Close(seat.feedName)
		}
		seat.observer.Stop() // verlässt den Raum; der Testraum wird danach aufgeräumt
	}
}

// drive schickt je Monarch des Clients ein Kommando. Die Monarchen des Clients sind alle außer dem des Beobachters,
// nach Index geordnet: Slot k ist der k-te (der Server vergibt freie Plätze aufsteigend). Monarchen auf einer anderen
// Stufe als der Beobachter sieht er nicht; sie bekommen kein Kommando.
func (seat *clientSeat) drive(ctx context.Context, bot balance.Bot) {
	w, own := seat.observer.World()
	if w == nil {
		return
	}
	var cmds []botfeed.Cmd
	for _, p := range clientMonarchs(w, own) {
		cmds = append(cmds, botfeed.CmdOf(len(cmds), bot(w, p)))
	}
	seat.feed.Send(ctx, cmds)
}

func clientMonarchs(w *sim.World, own int) []*sim.Player {
	var out []*sim.Player
	for _, p := range w.Players {
		if p.Index != own {
			out = append(out, p)
		}
	}
	slices.SortFunc(out, func(a, b *sim.Player) int { return a.Index - b.Index })
	return out
}

// feedTotals zählt verbundene Clients, nie verbundene und zugestellte Kommandos.
func feedTotals(seats []*clientSeat) (now, never, sent int) {
	for _, seat := range seats {
		if seat.feed == nil {
			continue
		}
		c, ever := seat.feed.Connected()
		if c {
			now++
		}
		if !ever {
			never++
		}
		sent += seat.feed.Sent()
	}
	return now, never, sent
}
