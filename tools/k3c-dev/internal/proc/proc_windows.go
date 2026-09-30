//go:build windows

package proc

import (
	"os/exec"
	"strconv"
	"syscall"
)

// createNoWindow verhindert ein Konsolenfenster je Kindprozess.
const createNoWindow = 0x08000000

func prepare(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{CreationFlags: createNoWindow}
}

// KillTree beendet einen Prozess samt allen Kindern.
func KillTree(pid int) error {
	cmd := exec.Command("taskkill", "/T", "/F", "/PID", strconv.Itoa(pid))
	prepare(cmd)
	return cmd.Run()
}
