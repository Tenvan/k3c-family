//go:build windows

package main

import (
	"syscall"
	"unsafe"
)

// Go liest die Uhr unter Windows aus dem Takt des Systems (0,5 bis 1 ms grob). Für Frames unter einer Millisekunde ist
// das zu ungenau, darum misst der Prototyp mit dem Hochleistungszähler (QueryPerformanceCounter, ohne cgo).
var (
	kernel32 = syscall.NewLazyDLL("kernel32.dll")
	qpc      = kernel32.NewProc("QueryPerformanceCounter")
	qpf      = kernel32.NewProc("QueryPerformanceFrequency")
	qpcFreq  = func() float64 {
		var f int64
		qpf.Call(uintptr(unsafe.Pointer(&f)))
		return float64(f)
	}()
)

// clock liefert die echte Zeit in Sekunden seit einem festen, beliebigen Nullpunkt.
func clock() float64 {
	var c int64
	qpc.Call(uintptr(unsafe.Pointer(&c)))
	return float64(c) / qpcFreq
}
