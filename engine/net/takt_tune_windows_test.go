//go:build windows

package net

import (
	"os"
	"unsafe"

	"golang.org/x/sys/windows"
)

// Mit K3C_TAKT_TUNE=1 läuft die Takt-Messung mit denselben Prozess-Einstellungen wie der Server
// (cmd/k3c-server/priority_windows.go): Priorität „über normal“, Power-Throttling aus.
func init() {
	if os.Getenv("K3C_TAKT_TUNE") != "1" {
		return
	}
	h := windows.CurrentProcess()
	_ = windows.SetPriorityClass(h, windows.ABOVE_NORMAL_PRIORITY_CLASS)
	state := struct{ version, controlMask, stateMask uint32 }{version: 1, controlMask: 1}
	proc := windows.NewLazySystemDLL("kernel32.dll").NewProc("SetProcessInformation")
	_, _, _ = proc.Call(uintptr(h), 4, uintptr(unsafe.Pointer(&state)), unsafe.Sizeof(state))
}
