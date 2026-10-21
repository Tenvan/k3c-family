//go:build !windows

package devrun

import (
	"errors"
	"os"
	"os/exec"
	"syscall"
)

// prepare startet das Kind in einer eigenen Prozessgruppe, damit Strg+C im Terminal nur den Dev-Lauf trifft.
func prepare(cmd *exec.Cmd) { cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true} }

// bindChild: Außerhalb von Windows gibt es kein Job-Objekt; ein verwaistes Kind bleibt möglich.
func bindChild(*exec.Cmd) (uintptr, error) { return 0, nil }

// closeJob: kein Job-Objekt.
func closeJob(uintptr) {}

// killTree beendet die ganze Prozessgruppe des Kindes. Das Kind ist noch nicht eingesammelt (stop prüft vorher done),
// seine PID ist also nicht wiederverwendet.
func killTree(cmd *exec.Cmd, _ uintptr) error { return syscall.Kill(-cmd.Process.Pid, syscall.SIGKILL) }

// requestStop bittet das Kind per Signal um ein sauberes Ende.
func requestStop(cmd *exec.Cmd) error { return cmd.Process.Signal(os.Interrupt) }

// addrInUse sagt, ob Listen an einer belegten Adresse scheiterte.
func addrInUse(err error) bool { return errors.Is(err, syscall.EADDRINUSE) }
