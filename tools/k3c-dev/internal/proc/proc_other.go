//go:build !windows

package proc

import (
	"os/exec"
	"syscall"
)

func prepare(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true}
}

// KillTree beendet die Prozessgruppe, die mit dem Prozess begonnen hat.
func KillTree(pid int) error {
	return syscall.Kill(-pid, syscall.SIGKILL)
}
