//go:build windows

package windowstate

import (
	"syscall"
	"unsafe"
)

var procMonitorFromRect = syscall.NewLazyDLL("user32.dll").NewProc("MonitorFromRect")

// OnScreen meldet, ob ein Punkt der Titelleiste (100 px vom linken Rand) auf einem angeschlossenen Bildschirm liegt,
// das Fenster also greifbar ist. Fehlt der Bildschirm (abgesteckt, Anordnung geändert), bleibt die Position Wails
// überlassen. Width taugt nicht als Maß: X ist physisch, Width in DIP gespeichert.
func OnScreen(w State) bool {
	px, py := int32(w.X+100), int32(w.Y+15)
	rect := [4]int32{px, py, px + 1, py + 1} // left, top, right, bottom
	const monitorDefaultToNull = 0
	mon, _, _ := procMonitorFromRect.Call(uintptr(unsafe.Pointer(&rect)), monitorDefaultToNull)
	return mon != 0
}
