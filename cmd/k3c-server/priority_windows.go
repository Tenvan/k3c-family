//go:build windows

package main

import (
	"unsafe"

	"golang.org/x/sys/windows"
)

// tuneProcess hebt den Server aus dem Hintergrund-Modus von Windows (RS1.3, Entscheidung 006): Priorität „über
// normal“ und Power-Throttling (EcoQoS) aus, damit der Scheduler den Takt unter Last nicht auf E-Kerne oder
// Effizienz-Frequenzen schiebt. Fehler sind harmlos (ältere Windows-Version): der Server läuft dann wie bisher.
func tuneProcess() (prio, throttling error) {
	h := windows.CurrentProcess()
	prio = windows.SetPriorityClass(h, windows.ABOVE_NORMAL_PRIORITY_CLASS)
	state := struct{ version, controlMask, stateMask uint32 }{
		version: 1, controlMask: 1, stateMask: 0, // PROCESS_POWER_THROTTLING_EXECUTION_SPEED: Steuerung übernehmen, Zustand aus
	}
	proc := windows.NewLazySystemDLL("kernel32.dll").NewProc("SetProcessInformation")
	const processPowerThrottling = 4
	r, _, e := proc.Call(uintptr(h), processPowerThrottling, uintptr(unsafe.Pointer(&state)), unsafe.Sizeof(state))
	if r == 0 {
		throttling = e
	}
	return prio, throttling
}
