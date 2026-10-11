package mcpsrv

import (
	"context"
	"fmt"
	"path/filepath"
	"slices"
	"sync"
	"time"
)

// B-372: check_run wartet höchstens checkWait auf den Lauf. Dauert er länger, läuft er im Hintergrund weiter und die
// Antwort nennt eine Lauf-ID; check_run {run: <ID>} liefert Exit-Code und Fehlerzeilen. Der Lauf hängt nicht am
// Aufruf des Clients: bricht dieser ab (Timeout), läuft der Lauf zu Ende.
const (
	defaultCheckWait = 45 * time.Second
	maxJobs          = 20 // so viele Läufe bleiben über ihre ID abfragbar
)

type checkJob struct {
	id     string
	t      checkTarget
	key    string // Quelle samt Checkout, wie in checkRuns.running
	start  time.Time
	done   chan struct{}
	mu     sync.Mutex
	output []string // Fehlerzeilen des Ziels (N5: nur sie und das Ende, die volle Ausgabe steht im Konsolenpuffer)
	tail   []string // letzte fallbackLines Zeilen ohne Fehlermuster, für die Antwort ohne Fehlerzeilen
	res    runResult
}

func (j *checkJob) add(text string) {
	j.mu.Lock()
	defer j.mu.Unlock()
	if matchesAny(text, j.t.kinds) {
		j.output = append(j.output, text)
		return
	}
	j.tail = append(j.tail, text)
	if len(j.tail) > fallbackLines {
		j.tail = j.tail[1:]
	}
}

func (j *checkJob) setResult(res runResult) {
	j.mu.Lock()
	j.res = res
	j.mu.Unlock()
}

// answer ist die verdichtete Antwort eines beendeten Laufs.
func (j *checkJob) answer() (string, error) {
	j.mu.Lock()
	defer j.mu.Unlock()
	if j.res.err != nil {
		return "", fmt.Errorf("%s ließ sich nicht starten: %w", j.t.name, j.res.err)
	}
	return report(j.t, j.res, append(slices.Clone(j.output), j.tail...)), nil
}

func (c *checkRuns) newJob(t checkTarget, key string) *checkJob {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.seq++
	j := &checkJob{id: fmt.Sprintf("L%d", c.seq), t: t, key: key, start: time.Now(), done: make(chan struct{})}
	c.jobs[j.id] = j
	c.order = append(c.order, j.id)
	for len(c.order) > maxJobs {
		delete(c.jobs, c.order[0])
		c.order = c.order[1:]
	}
	return j
}

// runningID ist die Lauf-ID des noch laufenden Laufs mit diesem Schlüssel, sonst leer.
func (c *checkRuns) runningID(key string) string {
	c.mu.Lock()
	defer c.mu.Unlock()
	for i := len(c.order) - 1; i >= 0; i-- {
		j := c.jobs[c.order[i]]
		if j.key != key {
			continue
		}
		select {
		case <-j.done:
		default:
			return j.id
		}
	}
	return ""
}

func (c *checkRuns) job(id string) *checkJob {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.jobs[id]
}

// checkRun ist das Tool check_run: prüfen, sperren, laufen lassen, verdichten. Die volle Ausgabe steht im
// Konsolenpuffer check:<ziel>.
func (s *Server) checkRun(ctx context.Context, in checkIn) (string, error) {
	if in.Run != "" {
		return s.checkJobResult(in.Run)
	}
	t, err := findTarget(in.Target)
	if err != nil {
		return "", err
	}
	args, err := t.args(in.Pattern)
	if err != nil {
		return "", err
	}
	// Sperre, Ergebnis und Konsole je Checkout: Läufe verschiedener Worktrees stören sich nicht.
	ws := s.ws(ctx)
	key := ws.source(t.name)
	if !s.checks.begin(key) {
		return "", fmt.Errorf("%s läuft bereits (Lauf-ID %s); auf das Ende warten, Ergebnis mit check_run {run}, Ausgabe über console_tail check:%s",
			t.name, s.checks.runningID(key), t.name)
	}
	s.notifyCheck(CheckState{Name: key, Running: true, At: time.Now()})
	job := s.checks.newJob(t, key)
	go s.runCheck(context.WithoutCancel(ctx), job, filepath.Join(ws.root, t.dir), args)
	select {
	case <-job.done:
		return job.answer()
	case <-time.After(s.checkWait):
		return fmt.Sprintf("%s läuft im Hintergrund seit %s · Lauf-ID %s · Ergebnis mit check_run {run: %q}, Ausgabe mit console_tail check:%s",
			t.name, formatMs(float64(time.Since(job.start).Milliseconds())), job.id, job.id, t.name), nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

// runCheck führt den Lauf aus und meldet Ende, Ergebnis und Freigabe der Sperre, auch nach einer Panik.
func (s *Server) runCheck(ctx context.Context, j *checkJob, dir string, args []string) {
	var res runResult
	defer close(j.done)
	defer func() { s.checks.end(j.key, res) }() // sonst bliebe das Ziel gesperrt
	// Ende melden, solange das Ziel noch gesperrt ist: sonst könnte „läuft“ des nächsten Laufs davor ankommen.
	defer func() { s.notifyCheck(stateOf(j.key, res)) }()
	defer func() {
		if p := recover(); p != nil {
			res = runResult{err: fmt.Errorf("abgestürzt: %v", p)}
		}
		j.setResult(res)
		s.logRun(j.key, res)
	}()
	source := "check:" + j.key
	s.console.Reset(source)
	res = s.run(ctx, runSpec{dir: dir, args: args, timeout: j.t.timeout,
		out: func(stream, text string) {
			s.console.Add(source, stream, text)
			j.add(text)
		}})
}

// checkJobResult ist die Antwort auf check_run {run}: Ergebnis des Laufs oder „läuft noch“.
func (s *Server) checkJobResult(id string) (string, error) {
	j := s.checks.job(id)
	if j == nil {
		return "", fmt.Errorf("unbekannte Lauf-ID %q (gemerkt werden die letzten %d Läufe seit dem Start von k3c-dev)", id, maxJobs)
	}
	select {
	case <-j.done:
		return j.answer()
	default:
		return fmt.Sprintf("%s läuft noch seit %s · Lauf-ID %s", j.t.name, formatMs(float64(time.Since(j.start).Milliseconds())), j.id), nil
	}
}
