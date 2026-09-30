// Package proc startet Kindprozesse so, dass sie sich samt Kindern beenden lassen (check_run, ab B-067 die
// Dienste). Die Plattform-Teile liegen in proc_windows.go und proc_other.go.
package proc

import (
	"context"
	"os/exec"
	"runtime"
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

// Command baut einen Befehl ohne Konsolenfenster bzw. in eigener Prozessgruppe. Endet ctx, beendet KillTree den
// ganzen Baum, nicht nur den direkten Prozess.
func Command(ctx context.Context, args []string) *exec.Cmd {
	cmd := exec.CommandContext(ctx, Executable(args[0]), args[1:]...)
	prepare(cmd)
	cmd.Cancel = func() error { return KillTree(cmd.Process.Pid) }
	cmd.WaitDelay = waitDelay
	return cmd
}
