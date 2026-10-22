//go:build !windows

package backend

import (
	"os/exec"
	"syscall"
)

// Prozessbaum der Claude Code CLI (B-514), Unix: eigene Prozessgruppe, der Abbruch trifft die ganze Gruppe.

// chatTree ist auf Unix leer; die Gruppe ergibt sich aus der PID.
type chatTree uintptr

func prepareTree(cmd *exec.Cmd) { cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true} }

func bindTree(*exec.Cmd) (chatTree, error) { return 0, nil }

func killTree(cmd *exec.Cmd, _ chatTree) error {
	if err := syscall.Kill(-cmd.Process.Pid, syscall.SIGKILL); err != nil {
		return cmd.Process.Kill()
	}
	return nil
}

func closeTree(chatTree) {}
