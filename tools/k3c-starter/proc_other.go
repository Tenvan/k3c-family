//go:build !windows

package main

import (
	"os"
	"os/exec"
)

// hideWindow: außerhalb von Windows gibt es kein Konsolenfenster zu verbergen.
func hideWindow(*exec.Cmd) {}

// afterStart: kein Job-Objekt nötig.
func afterStart(*exec.Cmd) error { return nil }

// requestStop bittet den Server per Signal um ein sauberes Ende.
func requestStop(cmd *exec.Cmd) error { return cmd.Process.Signal(os.Interrupt) }
