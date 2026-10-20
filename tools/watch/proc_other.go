//go:build !windows

package main

import (
	"os"
	"os/exec"
	"syscall"
)

// prepare startet das Kind in einer eigenen Prozessgruppe, damit Strg+C im Terminal nur den Watcher trifft.
func prepare(cmd *exec.Cmd) { cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true} }

// killTree beendet die ganze Prozessgruppe des Prozesses.
func killTree(cmd *exec.Cmd) error { return syscall.Kill(-cmd.Process.Pid, syscall.SIGKILL) }

// requestStop bittet das Kind per Signal um ein sauberes Ende.
func requestStop(cmd *exec.Cmd) error { return cmd.Process.Signal(os.Interrupt) }
