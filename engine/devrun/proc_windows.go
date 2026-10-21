//go:build windows

package devrun

import (
	"errors"
	"os/exec"
	"syscall"
	"unsafe"
)

// Jedes Kind bekommt ein eigenes Job-Objekt (KILL_ON_JOB_CLOSE): Endet der Dev-Lauf, auch hart oder durch Absturz,
// schließt Windows den Handle und beendet Kind und Nachfahren, kein verwaister Prozess hält den Port. Mit BREAKAWAY_OK
// überleben Programme, die das Kind ausdrücklich mit CREATE_BREAKAWAY_FROM_JOB startet (z. B. ein Spielserver aus dem
// Studio), das Ende von `task <app>:dev`.
//
// Zwischen CreateProcess und AssignProcessToJobObject läuft das Kind bereits. Ein Start mit CREATE_SUSPENDED ginge nur
// mit dem Handle des Hauptthreads, den os/exec sofort schließt. Vertretbar: Binden dauert Mikrosekunden, ein Go-Programm
// braucht für Laufzeit-Init und eigenen Start um Größenordnungen länger, bevor es Kinder startet; ein dennoch
// entwischter Enkel wäre nur ein möglicher Waise, kein Fehler im Ablauf.

const (
	createNewProcessGroup = 0x00000200 // eigene Prozessgruppe: Strg+Pause lässt sich gezielt an sie schicken, Strg+C trifft nur den Dev-Lauf
	ctrlBreakEvent        = 1
	wsaeAddrInUse         = 10048      // WSAEADDRINUSE
	jobBreakawayOK        = 0x00000800 // JOB_OBJECT_LIMIT_BREAKAWAY_OK
	jobKillOnClose        = 0x00002000 // JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
	jobExtendedLimitClass = 9          // JobObjectExtendedLimitInformation
	processSetQuota       = 0x0100
	processTerminate      = 0x0001
)

var (
	kernel32         = syscall.NewLazyDLL("kernel32.dll")
	procGenerate     = kernel32.NewProc("GenerateConsoleCtrlEvent")
	procCreateJob    = kernel32.NewProc("CreateJobObjectW")
	procSetJobInfo   = kernel32.NewProc("SetInformationJobObject")
	procAssignToJob  = kernel32.NewProc("AssignProcessToJobObject")
	procTerminateJob = kernel32.NewProc("TerminateJobObject")
)

// Strukturen von JOBOBJECT_EXTENDED_LIMIT_INFORMATION (64-Bit-Layout, wie in winnt.h; wie tools/k3c-starter).
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

// prepare startet das Kind in einer eigenen Prozessgruppe.
func prepare(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{CreationFlags: createNewProcessGroup}
}

// bindChild legt das Job-Objekt des Kindes an und hängt das Kind daran; 0 heißt: kein Job.
func bindChild(cmd *exec.Cmd) (uintptr, error) {
	job, _, err := procCreateJob.Call(0, 0)
	if job == 0 {
		return 0, err
	}
	info := jobExtendedLimits{}
	info.Basic.LimitFlags = jobKillOnClose | jobBreakawayOK
	if r, _, err := procSetJobInfo.Call(job, jobExtendedLimitClass, uintptr(unsafe.Pointer(&info)), unsafe.Sizeof(info)); r == 0 {
		closeJob(job)
		return 0, err
	}
	ph, err := syscall.OpenProcess(processSetQuota|processTerminate, false, uint32(cmd.Process.Pid))
	if err != nil {
		closeJob(job)
		return 0, err
	}
	defer func() { _ = syscall.CloseHandle(ph) }()
	if r, _, err := procAssignToJob.Call(job, uintptr(ph)); r == 0 {
		closeJob(job)
		return 0, err
	}
	return job, nil
}

// closeJob schließt den Job-Handle; noch laufende Mitglieder beendet Windows dabei (KILL_ON_JOB_CLOSE).
func closeJob(job uintptr) {
	if job != 0 {
		_ = syscall.CloseHandle(syscall.Handle(job))
	}
}

// killTree beendet das Kind samt Nachfahren über sein Job-Objekt, ohne Job über das Prozess-Handle. Keine PID: eine
// wiederverwendete PID könnte einen fremden Prozess treffen.
func killTree(cmd *exec.Cmd, job uintptr) error {
	if job == 0 {
		return cmd.Process.Kill()
	}
	if r, _, err := procTerminateJob.Call(job, 1); r == 0 {
		return err
	}
	return nil
}

// requestStop schickt dem Kind Strg+Pause (Go meldet es wie Strg+C als os.Interrupt, der Server speichert die Räume und
// fährt herunter). Der Dev-Lauf teilt die Konsole mit dem Kind; ohne Konsole (GUI-Prozess) schlägt es fehl, dann
// beendet stop das Kind hart.
func requestStop(cmd *exec.Cmd) error {
	if r, _, err := procGenerate.Call(ctrlBreakEvent, uintptr(cmd.Process.Pid)); r == 0 {
		return err
	}
	return nil
}

// addrInUse sagt, ob Listen an einer belegten Adresse scheiterte.
func addrInUse(err error) bool {
	var errno syscall.Errno
	return errors.As(err, &errno) && errno == wsaeAddrInUse
}
