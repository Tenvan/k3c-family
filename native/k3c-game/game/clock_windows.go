//go:build windows

package game

import (
	"syscall"
	"unsafe"
)

// Go liest die Uhr unter Windows aus dem Takt des Systems (0,5 bis 1 ms grob). Für Bilder unter einer Millisekunde ist
// das zu ungenau, darum misst der Messmodus mit dem Hochleistungszähler (QueryPerformanceCounter, ohne cgo; wie
// native/k3c-ebiten/clock_windows.go). Nur für Messwerte, nie für Spiel-Logik.
var (
	kernel32 = syscall.NewLazyDLL("kernel32.dll")
	qpc      = kernel32.NewProc("QueryPerformanceCounter")
	qpf      = kernel32.NewProc("QueryPerformanceFrequency")
	qpcFreq  = func() float64 {
		var f int64
		_, _, _ = qpf.Call(uintptr(unsafe.Pointer(&f)))
		return float64(f)
	}()
)

// clock liefert die echte Zeit in Sekunden seit einem festen, beliebigen Nullpunkt.
func clock() float64 {
	var c int64
	_, _, _ = qpc.Call(uintptr(unsafe.Pointer(&c)))
	return float64(c) / qpcFreq
}
