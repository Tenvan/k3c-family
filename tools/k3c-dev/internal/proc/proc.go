// Package proc startet Kindprozesse so, dass sie sich samt Kindern beenden lassen (check_run, die Dienste aus
// B-067). Unter Windows hängt jeder Prozess nach dem Start in einem Job Object: Beenden trifft dann auch Enkel,
// deren Elternprozess schon weg ist, und stirbt k3c-dev, räumt Windows den Job ab. Die Plattform-Teile liegen in
// proc_windows.go und proc_other.go.
package proc

import (
	"context"
	"os/exec"
	"runtime"
	"sync"
	"time"
)

// waitDelay begrenzt das Warten auf Ausgabe-Pipes, die ein Enkelprozess nach dem Ende noch offen hält.
const waitDelay = 5 * time.Second

// Executable liefert den Programmnamen für die Plattform: unter Windows heißt npm "npm.cmd".
func Executable(name string) string {
	if runtime.GOOS == "windows" && name == "npm" {
		return "npm.cmd"
	}
	return name
}

// Cmd ist ein exec.Cmd, dessen ganzer Prozessbaum sich beenden lässt. Start, Run und Wait ersetzen die Methoden des
// eingebetteten exec.Cmd, alles andere (Dir, Env, Stdout, ProcessState …) gilt wie dort.
type Cmd struct {
	*exec.Cmd
	mu  sync.Mutex
	job job // leer, solange nicht gestartet oder ohne Job
}

// Command baut einen Befehl ohne Konsolenfenster bzw. in eigener Prozessgruppe. Endet ctx, beendet Kill den ganzen
// Baum, nicht nur den direkten Prozess.
func Command(ctx context.Context, args []string) *Cmd {
	c := &Cmd{Cmd: exec.CommandContext(ctx, Executable(args[0]), args[1:]...)}
	prepare(c.Cmd)
	c.Cancel = c.Kill
	c.WaitDelay = waitDelay
	return c
}

// Start startet den Prozess und hängt ihn in einen Job.
//
// ponytail: zwischen Start und Zuordnung liegt ein kurzes Fenster, in dem ein Enkel dem Job entkommen kann;
// schließen ließe es sich nur mit CREATE_SUSPENDED und eigenem Resume, das exec.Cmd nicht anbietet.
func (c *Cmd) Start() error {
	if err := c.Cmd.Start(); err != nil {
		return err
	}
	c.mu.Lock()
	c.job = attach(c.Process.Pid)
	c.mu.Unlock()
	return nil
}

// Wait wartet auf das Ende und räumt danach den Job ab; übrig gebliebene Enkel enden dabei mit.
func (c *Cmd) Wait() error {
	err := c.Cmd.Wait()
	c.mu.Lock()
	c.job.close()
	c.job = job{}
	c.mu.Unlock()
	return err
}

// Run ist Start und Wait.
func (c *Cmd) Run() error {
	if err := c.Start(); err != nil {
		return err
	}
	return c.Wait()
}

// Kill beendet den ganzen Baum: über den Job, sonst (keine Zuordnung, andere Plattform) über KillTree.
func (c *Cmd) Kill() error {
	c.mu.Lock()
	defer c.mu.Unlock()
	if c.job.terminate() {
		return nil
	}
	if c.Process == nil {
		return nil
	}
	return KillTree(c.Process.Pid)
}
