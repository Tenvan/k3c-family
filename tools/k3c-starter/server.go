package main

import (
	"errors"
	"fmt"
	"net"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"time"
)

const (
	defaultPort = "8080"
	logLines    = 40
	// healthWait ist die Frist, in der der gestartete Server auf /api/health antworten muss.
	healthWait = 15 * time.Second
	// gracefulWait ist die Frist, in der der Server nach Strg+C Räume speichert und herunterfährt (er wartet selbst bis zu 7 s).
	gracefulWait = 10 * time.Second
	killWait     = 5 * time.Second
)

// layout sind die Pfade, die der Starter braucht: Arbeitsordner (saves/, reports/, dist/), Server-EXE und dist/.
type layout struct{ root, serverExe, dist string }

// Status ist der Zustand für das Fenster (JSON an das Frontend).
type Status struct {
	Running  bool     `json:"running"`
	Starting bool     `json:"starting"`
	Port     string   `json:"port"`
	URLs     []string `json:"urls"`
	Message  string   `json:"message"`
	Log      []string `json:"log"`
}

// supervisor startet bin/k3c-server als Kindprozess und hält seinen Zustand: Der Starter enthält keine Server-Logik,
// er ruft denselben Kern auf, den auch `task start` und das Docker-Image benutzen.
type supervisor struct {
	lay  layout
	port string

	mu       sync.Mutex
	cmd      *exec.Cmd
	running  bool
	stopping bool
	message  string
	log      []string
}

func newSupervisor(lay layout) *supervisor {
	port := os.Getenv("K3C_HTTP_PORT")
	if port == "" {
		port = defaultPort
	}
	return &supervisor{lay: lay, port: port, message: "Server ist gestoppt."}
}

// findLayout sucht dist/index.html im Arbeitsordner, im Ordner der EXE und darüber (bin/k3c-starter.exe im Repo);
// die Server-EXE liegt neben dem Starter oder in <Wurzel>/bin. K3C_ROOT (Entwicklungsmodus: der Kindprozess läuft in
// tools/k3c-starter und baut unter build/bin) nennt die Wurzel ausdrücklich und gilt zuerst.
func findLayout(wd, exeDir string) (layout, error) {
	roots := []string{wd, exeDir, filepath.Dir(exeDir)}
	if r := os.Getenv("K3C_ROOT"); r != "" {
		roots = append([]string{r}, roots...)
	}
	for _, root := range roots {
		if root == "" || !fileExists(filepath.Join(root, "dist", "index.html")) {
			continue
		}
		name := "k3c-server"
		if runtime.GOOS == "windows" {
			name += ".exe"
		}
		for _, exe := range []string{filepath.Join(exeDir, name), filepath.Join(root, "bin", name)} {
			if fileExists(exe) {
				return layout{root: root, serverExe: exe, dist: filepath.Join(root, "dist")}, nil
			}
		}
		return layout{}, fmt.Errorf("%s fehlt neben dem Starter und in %s (bauen: task k3c-starter:build)", name, filepath.Join(root, "bin"))
	}
	return layout{}, errors.New("dist/index.html nicht gefunden (Spiel bauen: task build)")
}

func fileExists(path string) bool {
	info, err := os.Stat(path)
	return err == nil && !info.IsDir()
}

// portFree prüft auf Loopback, ob der Port frei ist (der Server bindet alle Adressen; belegt ist belegt).
func portFree(port string) bool {
	l, err := net.Listen("tcp", "127.0.0.1:"+port)
	if err != nil {
		return false
	}
	_ = l.Close()
	return true
}

// Start startet den Server; eine Fehlermeldung steht danach im Zustand, nie als Absturz.
func (s *supervisor) Start() {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.cmd != nil {
		return
	}
	if !portFree(s.port) {
		s.message = fmt.Sprintf("Port %s ist belegt (läuft der Server schon?). Anderen Port: Umgebungsvariable K3C_HTTP_PORT", s.port)
		return
	}
	cmd := exec.Command(s.lay.serverExe)
	cmd.Dir = s.lay.root
	cmd.Env = serverEnv(s.lay, s.port)
	w := &lineWriter{sink: s.addLog}
	cmd.Stdout, cmd.Stderr = w, w
	hideWindow(cmd)
	if err := cmd.Start(); err != nil {
		s.message = "Server startet nicht: " + err.Error()
		return
	}
	s.cmd, s.running, s.stopping = cmd, false, false
	s.message = "Server startet …"
	if err := afterStart(cmd); err != nil {
		s.addLogLocked("Hinweis: Server nicht an den Starter gebunden (bleibt nach hartem Ende des Starters liegen): " + err.Error())
	}
	go s.wait(cmd)
	go s.awaitHealth(cmd)
}

// serverEnv ist die Umgebung des Servers: feste Pfade, Port; K3C_DEV bleibt aus, außer jemand setzt es bewusst.
func serverEnv(lay layout, port string) []string {
	env := append(os.Environ(), "K3C_HTTP_PORT="+port, "K3C_DIST="+lay.dist)
	if _, set := os.LookupEnv("K3C_DEV"); !set {
		env = append(env, "K3C_DEV=0")
	}
	return env
}

// wait wartet auf das Ende des Prozesses und merkt es sich.
func (s *supervisor) wait(cmd *exec.Cmd) {
	err := cmd.Wait()
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.cmd != cmd {
		return
	}
	s.cmd, s.running = nil, false
	if s.stopping {
		s.message = "Server ist gestoppt."
		return
	}
	s.message = "Server beendet sich unerwartet"
	if err != nil {
		s.message += ": " + err.Error()
	}
	s.message += ". Meldungen unten."
}

// awaitHealth setzt „läuft“, sobald /api/health antwortet.
func (s *supervisor) awaitHealth(cmd *exec.Cmd) {
	client := http.Client{Timeout: time.Second}
	deadline := time.Now().Add(healthWait)
	for time.Now().Before(deadline) {
		s.mu.Lock()
		gone := s.cmd != cmd
		s.mu.Unlock()
		if gone {
			return
		}
		if res, err := client.Get("http://127.0.0.1:" + s.port + "/api/health"); err == nil {
			_ = res.Body.Close()
			if res.StatusCode == http.StatusOK {
				s.mu.Lock()
				if s.cmd == cmd && !s.stopping {
					s.running, s.message = true, "Server läuft."
				}
				s.mu.Unlock()
				return
			}
		}
		time.Sleep(200 * time.Millisecond)
	}
	s.mu.Lock()
	if s.cmd == cmd && !s.running && !s.stopping {
		s.message = "Server antwortet nicht auf /api/health. Meldungen unten."
	}
	s.mu.Unlock()
}

// Stop bittet den Server um ein sauberes Ende (Strg+C: er speichert die Räume) und beendet ihn erst nach gracefulWait hart.
func (s *supervisor) Stop() {
	s.mu.Lock()
	cmd := s.cmd
	if cmd == nil {
		s.mu.Unlock()
		return
	}
	s.stopping = true
	s.message = "Server wird beendet …"
	s.mu.Unlock()
	if err := requestStop(cmd); err != nil {
		s.addLog("Hinweis: sauberes Beenden nicht möglich, Server wird beendet: " + err.Error())
		_ = cmd.Process.Kill()
	} else if !s.waitGone(gracefulWait) {
		s.addLog("Hinweis: Server reagiert nicht, wird beendet")
		_ = cmd.Process.Kill()
	}
	s.waitGone(killWait)
}

// waitGone wartet, bis der Prozess weg ist; false nach Ablauf der Frist.
func (s *supervisor) waitGone(limit time.Duration) bool {
	deadline := time.Now().Add(limit)
	for time.Now().Before(deadline) {
		s.mu.Lock()
		gone := s.cmd == nil
		s.mu.Unlock()
		if gone {
			return true
		}
		time.Sleep(50 * time.Millisecond)
	}
	return false
}

func (s *supervisor) addLog(line string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.addLogLocked(line)
}

// addLogLocked hängt eine Zeile an; der Aufrufer hält s.mu.
func (s *supervisor) addLogLocked(line string) {
	s.log = append(s.log, line)
	if len(s.log) > logLines {
		s.log = s.log[len(s.log)-logLines:]
	}
}

// State ist der Zustand für das Fenster; die Adressen nennt er nur, solange der Server läuft.
func (s *supervisor) State() Status {
	s.mu.Lock()
	defer s.mu.Unlock()
	st := Status{Running: s.running, Starting: s.cmd != nil && !s.running, Port: s.port, Message: s.message, Log: append([]string{}, s.log...), URLs: []string{}}
	if s.running {
		st.URLs = lanURLs(s.port)
	}
	return st
}

// lanURLs nennt localhost und die IPv4-Adressen im Heimnetz.
func lanURLs(port string) []string {
	urls := []string{"http://localhost:" + port + "/"}
	addrs, _ := net.InterfaceAddrs()
	for _, a := range addrs {
		if ip, ok := a.(*net.IPNet); ok && ip.IP.To4() != nil && !ip.IP.IsLoopback() && !ip.IP.IsLinkLocalUnicast() {
			urls = append(urls, "http://"+ip.IP.String()+":"+port+"/")
		}
	}
	return urls
}

// lineWriter zerlegt die Ausgabe des Servers in Zeilen.
type lineWriter struct {
	mu   sync.Mutex
	buf  strings.Builder
	sink func(string)
}

func (w *lineWriter) Write(p []byte) (int, error) {
	w.mu.Lock()
	defer w.mu.Unlock()
	for _, b := range p {
		if b != '\n' {
			w.buf.WriteByte(b)
			continue
		}
		if line := strings.TrimRight(w.buf.String(), "\r"); line != "" {
			w.sink(line)
		}
		w.buf.Reset()
	}
	return len(p), nil
}
