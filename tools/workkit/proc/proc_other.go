//go:build !windows

package proc

import (
	"os/exec"
	"syscall"
)

func prepare(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true}
}

// job ist hier die Prozessgruppe (Setpgid): terminate und close treffen über KillTree auch verwaiste Enkel.
type job struct{ pid int }

func attach(pid int) job { return job{pid: pid} }

func (j job) terminate() bool { return j.pid != 0 && KillTree(j.pid) == nil }

func (j job) close() {
	if j.pid != 0 {
		_ = KillTree(j.pid) // übrige Prozesse der Gruppe; die Gruppe kann schon leer sein
	}
}

// KillTree beendet die Prozessgruppe, die mit dem Prozess begonnen hat.
func KillTree(pid int) error {
	return syscall.Kill(-pid, syscall.SIGKILL)
}
