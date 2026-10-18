//go:build windows

package main

import (
	"os/exec"
	"sync"
	"syscall"
	"time"
	"unsafe"
)

const (
	createNewConsole      = 0x00000010 // eigene, verborgene Konsole: nur so lässt sich dem Server ein Signal schicken
	createNewProcessGroup = 0x00000200 // eigene Prozessgruppe: Strg+Pause (CTRL_BREAK) lässt sich gezielt an sie schicken
	ctrlBreakEvent        = 1
	jobKillOnClose        = 0x00002000 // JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
	jobExtendedLimitClass = 9          // JobObjectExtendedLimitInformation
	processSetQuota       = 0x0100
	processTerminate      = 0x0001
	ctrlLinger            = 500 * time.Millisecond
)

var (
	kernel32        = syscall.NewLazyDLL("kernel32.dll")
	procAttach      = kernel32.NewProc("AttachConsole")
	procFree        = kernel32.NewProc("FreeConsole")
	procGenerate    = kernel32.NewProc("GenerateConsoleCtrlEvent")
	procSetHandler  = kernel32.NewProc("SetConsoleCtrlHandler")
	procCreateJob   = kernel32.NewProc("CreateJobObjectW")
	procSetJobInfo  = kernel32.NewProc("SetInformationJobObject")
	procAssignToJob = kernel32.NewProc("AssignProcessToJobObject")
	stopMu          sync.Mutex
	jobOnce         sync.Once
	job             uintptr
	errJob          error
)

// Strukturen von JOBOBJECT_EXTENDED_LIMIT_INFORMATION (64-Bit-Layout, wie in winnt.h).
type jobBasicLimits struct {
	PerProcessUserTimeLimit, PerJobUserTimeLimit int64
	LimitFlags                                   uint32
	MinimumWorkingSetSize, MaximumWorkingSetSize uintptr
	ActiveProcessLimit                           uint32
	Affinity                                     uintptr
	PriorityClass, SchedulingClass               uint32
}

type jobExtendedLimits struct {
	Basic                                                                   jobBasicLimits
	IO                                                                      [6]uint64
	ProcessMemoryLimit, JobMemoryLimit, PeakProcessMemory, PeakJobMemoryUse uintptr
}

// hideWindow startet den Server in einer verborgenen eigenen Konsole.
func hideWindow(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{HideWindow: true, CreationFlags: createNewConsole | createNewProcessGroup}
}

// afterStart hängt den Server an ein Job-Objekt, das ihn beendet, sobald der Starter endet (auch bei Absturz oder
// Abmeldung): So bleibt kein verwaister Server auf dem Port.
func afterStart(cmd *exec.Cmd) error {
	jobOnce.Do(func() {
		h, _, err := procCreateJob.Call(0, 0)
		if h == 0 {
			errJob = err
			return
		}
		info := jobExtendedLimits{}
		info.Basic.LimitFlags = jobKillOnClose
		r, _, err := procSetJobInfo.Call(h, jobExtendedLimitClass, uintptr(unsafe.Pointer(&info)), unsafe.Sizeof(info))
		if r == 0 {
			errJob = err
			return
		}
		job = h
	})
	if job == 0 {
		return errJob
	}
	ph, err := syscall.OpenProcess(processSetQuota|processTerminate, false, uint32(cmd.Process.Pid))
	if err != nil {
		return err
	}
	defer func() { _ = syscall.CloseHandle(ph) }()
	if r, _, err := procAssignToJob.Call(job, uintptr(ph)); r == 0 {
		return err
	}
	return nil
}

// requestStop schickt dem Server Strg+Pause (Go meldet es wie Strg+C als os.Interrupt; er speichert die Räume und fährt herunter). Ein GUI-Prozess hat keine Konsole:
// Er hängt sich kurz an die verborgene Konsole des Servers, ignoriert Strg+C selbst und löst es aus.
func requestStop(cmd *exec.Cmd) error {
	stopMu.Lock()
	defer stopMu.Unlock()
	_, _, _ = procFree.Call()
	if r, _, err := procAttach.Call(uintptr(cmd.Process.Pid)); r == 0 {
		return err
	}
	defer func() {
		// Die Zustellung läuft asynchron in einem eigenen Thread: Erst nach einer Weile wieder auf Standard stellen,
		// sonst trifft das Strg+C den Starter selbst.
		time.Sleep(ctrlLinger)
		_, _, _ = procSetHandler.Call(0, 0)
		_, _, _ = procFree.Call()
	}()
	_, _, _ = procSetHandler.Call(0, 1)
	if r, _, err := procGenerate.Call(ctrlBreakEvent, uintptr(cmd.Process.Pid)); r == 0 { // Strg+Pause an die Prozessgruppe des Servers (Go liest es als os.Interrupt)
		return err
	}
	return nil
}
