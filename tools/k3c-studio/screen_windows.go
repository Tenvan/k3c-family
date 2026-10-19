//go:build windows

package main

import (
	"syscall"
	"unsafe"
)

var procMessageBox = syscall.NewLazyDLL("user32.dll").NewProc("MessageBoxW")

// showError zeigt einen Startfehler in einem Fenster, weil die EXE keine Konsole hat.
func showError(title, text string) {
	t, _ := syscall.UTF16PtrFromString(title)
	m, _ := syscall.UTF16PtrFromString(text)
	const mbIconError = 0x10
	_, _, _ = procMessageBox.Call(0, uintptr(unsafe.Pointer(m)), uintptr(unsafe.Pointer(t)), mbIconError)
}

var procMonitorFromRect = syscall.NewLazyDLL("user32.dll").NewProc("MonitorFromRect")

// onScreen meldet, ob ein Punkt der Titelleiste (100 px vom linken Rand) auf einem angeschlossenen Bildschirm liegt,
// das Fenster also greifbar ist. Fehlt der Bildschirm (abgesteckt, Anordnung geändert), bleibt die Position Wails
// überlassen. Width taugt nicht als Maß: X ist physisch, Width in DIP gespeichert.
func onScreen(w windowState) bool {
	px, py := int32(w.X+100), int32(w.Y+15)
	rect := [4]int32{px, py, px + 1, py + 1} // left, top, right, bottom
	const monitorDefaultToNull = 0
	mon, _, _ := procMonitorFromRect.Call(uintptr(unsafe.Pointer(&rect)), monitorDefaultToNull)
	return mon != 0
}
