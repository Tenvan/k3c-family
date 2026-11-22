package playtest

import (
	"bytes"
	"context"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
	"sync"
	"time"

	"k3c/tools/workkit/proc"
)

// Echte Prozesse über tools/workkit/proc (Job-Objekt: stirbt die Workbench, enden ffmpeg und Spiel mit; ohne
// Konsolenfenster). ffmpeg kommt aus dem PATH.

// tailWriter hält die letzte nichtleere Zeile der Ausgabe.
type tailWriter struct {
	mu   sync.Mutex
	last string
	part string
}

func (t *tailWriter) Write(p []byte) (int, error) {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.part += string(p)
	for {
		i := strings.IndexByte(t.part, '\n')
		if i < 0 {
			break
		}
		if l := strings.TrimSpace(t.part[:i]); l != "" {
			t.last = l
		}
		t.part = t.part[i+1:]
	}
	if l := strings.TrimSpace(t.part); l != "" && len(t.part) < 4096 {
		t.last = l
	}
	return len(p), nil
}

func (t *tailWriter) Tail() string {
	t.mu.Lock()
	defer t.mu.Unlock()
	return t.last
}

type realProc struct {
	cmd   *proc.Cmd
	stdin io.WriteCloser
	tail  *tailWriter
}

func (p *realProc) Wait() error  { return p.cmd.Wait() }
func (p *realProc) Kill() error  { return p.cmd.Kill() }
func (p *realProc) Tail() string { return p.tail.Tail() }

// Quit schreibt `q` auf stdin; ohne stdin (Spiel) wird hart beendet.
func (p *realProc) Quit() error {
	if p.stdin == nil {
		return p.cmd.Kill()
	}
	_, err := io.WriteString(p.stdin, "q\n")
	_ = p.stdin.Close()
	return err
}

// ExecSpawn startet args[0] in dir. Nur ffmpeg bekommt stdin (für `q`); stderr landet im Tail.
func ExecSpawn(args []string, dir string) (Proc, error) {
	c := proc.Command(context.Background(), args)
	c.Dir = dir
	t := &tailWriter{}
	c.Stderr = t
	p := &realProc{cmd: c, tail: t}
	if strings.EqualFold(strings.TrimSuffix(filepath.Base(args[0]), ".exe"), "ffmpeg") {
		in, err := c.StdinPipe()
		if err != nil {
			return nil, err
		}
		p.stdin = in
	}
	if err := c.Start(); err != nil {
		return nil, err
	}
	return p, nil
}

// Game findet die Spiel-EXE wie „Im Spiel ansehen“: erst das Release-Ergebnis, dann das Paket; Arbeitsordner wie dort.
func Game(root string) func() (string, string, error) {
	return func() (string, string, error) {
		for _, c := range [][2]string{{"rust/target/release/k3c-game.exe", "."}, {"dist/k3c-game/k3c-game.exe", "dist/k3c-game"}} {
			exe := filepath.Join(root, filepath.FromSlash(c[0]))
			if st, err := os.Stat(exe); err == nil && !st.IsDir() {
				return exe, filepath.Join(root, filepath.FromSlash(c[1])), nil
			}
		}
		return "", "", ErrNoGame
	}
}

// NewRecorder baut den Recorder für die Ablage store unter root mit echten Prozessen.
func NewRecorder(root string, store *Store, log func(level, msg string, kv ...any)) *Recorder {
	return &Recorder{Store: store, Spawn: ExecSpawn, LookPath: exec.LookPath, Game: Game(root), Now: time.Now, Log: log,
		Settle: 2 * time.Second, QuitWait: 10 * time.Second}
}

// Devices sind die wählbaren Geräte der Seite.
type Devices struct {
	Mics    []string `json:"mics"`
	FFmpeg  bool     `json:"ffmpeg"` // ffmpeg im PATH
	Message string   `json:"message"`
}

var quoted = regexp.MustCompile(`"([^"]+)"`)

// ParseMics liest die Audio-Geräte aus der Ausgabe von `ffmpeg -list_devices true -f dshow -i dummy` (neues Format
// mit `(audio)` je Zeile und altes mit Abschnittsüberschrift); „Alternative name“-Zeilen zählen nicht.
func ParseMics(out string) []string {
	var mics []string
	seen := map[string]bool{}
	audio := false
	for _, l := range strings.Split(out, "\n") {
		switch {
		case strings.Contains(l, "DirectShow audio devices"):
			audio = true
		case strings.Contains(l, "DirectShow video devices"):
			audio = false
		}
		if strings.Contains(l, "Alternative name") {
			continue
		}
		m := quoted.FindStringSubmatch(l)
		if m == nil || seen[m[1]] || !(strings.Contains(l, "(audio)") || (audio && !strings.Contains(l, "(video)"))) {
			continue
		}
		seen[m[1]] = true
		mics = append(mics, m[1])
	}
	return mics
}

// ListDevices fragt ffmpeg nach den Mikrofonen (ffmpeg beendet den Aufruf mit Fehlercode, die Liste steht in stderr).
func ListDevices(lookPath func(string) (string, error)) Devices {
	ff, err := lookPath("ffmpeg")
	if err != nil {
		return Devices{Mics: []string{}, Message: ErrNoFFmpeg.Error()}
	}
	ctx, cancel := context.WithTimeout(context.Background(), 15*time.Second)
	defer cancel()
	var buf bytes.Buffer
	c := proc.Command(ctx, []string{ff, "-hide_banner", "-list_devices", "true", "-f", "dshow", "-i", "dummy"})
	c.Stdout, c.Stderr = &buf, &buf
	_ = c.Run()
	mics := ParseMics(buf.String())
	if mics == nil {
		mics = []string{}
	}
	return Devices{Mics: mics, FFmpeg: true}
}
