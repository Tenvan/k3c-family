package devrun

import (
	"context"
	"os"
	"os/exec"
	"os/signal"
	"path/filepath"
	"sync"
	"syscall"
	"time"
)

// child ist ein laufendes Kind.
type child struct {
	cmd   *exec.Cmd
	done  chan struct{} // schließt, sobald das Kind beendet ist; danach ist code gesetzt
	code  int           // Exit-Code (durch ein Signal oder hart beendet: -1)
	began time.Time
	mu    sync.Mutex // schützt job
	job   uintptr    // Job-Objekt (Windows); 0 = keins oder schon geschlossen
}

// build übersetzt das Paket nach out; Ausgabe und Fehler gehen auf die Konsole.
func (r *runner) build(ctx context.Context, out string) error {
	if err := os.MkdirAll(filepath.Dir(out), 0o755); err != nil {
		return err
	}
	cmd := exec.CommandContext(ctx, "go", "build", "-o", out, r.c.Package)
	cmd.Dir = r.dir
	cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	return cmd.Run()
}

// start startet die Kind-EXE in einer eigenen Prozessgruppe (Strg+C im Terminal trifft nur den Dev-Lauf, der das Kind
// dann sauber beendet) und bindet es an ein Job-Objekt. Ein Fehler wird geloggt; Run wartet dann auf die nächste Änderung.
func (r *runner) start() {
	cmd := exec.Command(r.child, r.c.Args...)
	cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	cmd.Env = append(append(os.Environ(), r.c.Env...), childEnv+"=1")
	prepare(cmd)
	if err := cmd.Start(); err != nil {
		r.log.Error("💥 Kind nicht gestartet", "exe", r.child, "err", err)
		return
	}
	job, err := bindChild(cmd)
	if err != nil {
		r.log.Warn("🚫 Kind nicht an den Dev-Lauf gebunden, bleibt nach einem harten Ende liegen", "err", err)
	}
	ch := &child{cmd: cmd, done: make(chan struct{}), began: time.Now(), job: job}
	go ch.wait()
	r.cur = ch
	r.log.Info("🚀 Kind läuft", "pid", cmd.Process.Pid)
}

// wait wartet auf das Ende des Kindes und schließt dann seinen Job (beendet übrig gebliebene Nachfahren).
func (ch *child) wait() {
	_ = ch.cmd.Wait()
	ch.code = -1
	if ps := ch.cmd.ProcessState; ps != nil {
		ch.code = ps.ExitCode()
	}
	ch.mu.Lock()
	closeJob(ch.job)
	ch.job = 0
	ch.mu.Unlock()
	close(ch.done)
}

// kill beendet das Kind samt Nachfahren hart (Job-Objekt, sonst Prozess-Handle bzw. Prozessgruppe).
func (ch *child) kill() {
	ch.mu.Lock()
	defer ch.mu.Unlock()
	if killTree(ch.cmd, ch.job) != nil {
		_ = ch.cmd.Process.Kill()
	}
}

// shutdown beendet das Kind am Ende von Run; ein zweites Strg+C während der Frist beendet es sofort hart.
func (r *runner) shutdown() {
	if r.cur == nil {
		return
	}
	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, os.Interrupt, syscall.SIGTERM)
	defer signal.Stop(sigs)
	r.stop(sigs)
}

// stop bittet das Kind um ein sauberes Ende (der Server speichert dabei die Räume), beendet es nach Grace oder bei
// abort hart samt Nachfahren und wartet, bis es weg ist. abort nil: nur die Frist zählt.
func (r *runner) stop(abort <-chan os.Signal) {
	ch := r.cur
	if ch == nil {
		return
	}
	r.cur = nil
	select {
	case <-ch.done: // schon von selbst beendet
		return
	default:
	}
	if err := requestStop(ch.cmd); err != nil {
		r.log.Warn("🚫 sauberes Beenden nicht möglich, Kind wird hart beendet", "err", err)
	} else if r.await(ch, abort) {
		r.log.Info("🛑 Kind beendet", "code", ch.code)
		return
	}
	ch.kill()
	<-ch.done
	r.log.Info("🛑 Kind hart beendet")
}

// await wartet bis Grace auf das Ende des Kindes; false: Frist abgelaufen oder abgebrochen.
func (r *runner) await(ch *child, abort <-chan os.Signal) bool {
	select {
	case <-ch.done:
		return true
	case <-abort:
		r.log.Warn("🛑 zweites Signal, Kind wird sofort hart beendet")
	case <-time.After(r.c.Grace):
		r.log.Warn("⏳ Kind reagiert nicht, wird hart beendet", "fristS", r.c.Grace.Seconds())
	}
	return false
}
