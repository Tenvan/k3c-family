//go:build windows

package main

import (
	"os/exec"
	"strconv"
	"syscall"
)

const (
	createNewProcessGroup = 0x00000200 // eigene Prozessgruppe: Strg+Pause lässt sich gezielt an sie schicken, Strg+C trifft nur den Watcher
	ctrlBreakEvent        = 1
)

var procGenerate = syscall.NewLazyDLL("kernel32.dll").NewProc("GenerateConsoleCtrlEvent")

// prepare startet das Kind in einer eigenen Prozessgruppe.
func prepare(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{CreationFlags: createNewProcessGroup}
}

// killTree beendet den Prozess samt Kindern (`task` startet `go build` als Kind).
func killTree(cmd *exec.Cmd) error {
	return exec.Command("taskkill", "/T", "/F", "/PID", strconv.Itoa(cmd.Process.Pid)).Run()
}

// requestStop schickt dem Kind Strg+Pause (Go meldet es wie Strg+C als os.Interrupt, der Server speichert die Räume und
// fährt herunter). Der Watcher teilt die Konsole mit dem Kind, ein Anhängen wie im Starter ist nicht nötig.
func requestStop(cmd *exec.Cmd) error {
	if r, _, err := procGenerate.Call(ctrlBreakEvent, uintptr(cmd.Process.Pid)); r == 0 {
		return err
	}
	return nil
}
