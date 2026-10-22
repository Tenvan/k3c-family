//go:build windows

package backend

import (
	"os/exec"
	"syscall"
	"unsafe"
)

// Prozessbaum der Claude Code CLI (B-514): Die CLI startet eigene Kindprozesse (Node, Git Bash). Ein Abbruch über
// exec.CommandContext träfe nur den direkten Prozess. Das Kind bekommt deshalb ein Job-Objekt (KILL_ON_JOB_CLOSE); der
// Abbruch beendet das ganze Job-Objekt. Wie in engine/devrun läuft das Kind zwischen Start und Zuordnung Mikrosekunden
// frei; ein dennoch entwischter Enkel wäre nur ein möglicher Waise.

const (
	jobKillOnClose        = 0x00002000 // JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
	jobExtendedLimitClass = 9          // JobObjectExtendedLimitInformation
	processSetQuota       = 0x0100
	processTerminate      = 0x0001
)

var (
	kernel32         = syscall.NewLazyDLL("kernel32.dll")
	procCreateJob    = kernel32.NewProc("CreateJobObjectW")
	procSetJobInfo   = kernel32.NewProc("SetInformationJobObject")
	procAssignToJob  = kernel32.NewProc("AssignProcessToJobObject")
	procTerminateJob = kernel32.NewProc("TerminateJobObject")
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

// chatTree ist das Job-Objekt des Prozessbaums; 0 heißt: kein Job.
type chatTree uintptr

// prepareTree richtet den Aufruf vor dem Start ein (Windows: nichts, der Job entsteht nach dem Start).
func prepareTree(*exec.Cmd) {}

// bindTree legt das Job-Objekt an und hängt den gestarteten Prozess daran.
func bindTree(cmd *exec.Cmd) (chatTree, error) {
	job, _, err := procCreateJob.Call(0, 0)
	if job == 0 {
		return 0, err
	}
	info := jobExtendedLimits{}
	info.Basic.LimitFlags = jobKillOnClose
	if r, _, err := procSetJobInfo.Call(job, jobExtendedLimitClass, uintptr(unsafe.Pointer(&info)), unsafe.Sizeof(info)); r == 0 {
		closeTree(chatTree(job))
		return 0, err
	}
	ph, err := syscall.OpenProcess(processSetQuota|processTerminate, false, uint32(cmd.Process.Pid))
	if err != nil {
		closeTree(chatTree(job))
		return 0, err
	}
	defer func() { _ = syscall.CloseHandle(ph) }()
	if r, _, err := procAssignToJob.Call(job, uintptr(ph)); r == 0 {
		closeTree(chatTree(job))
		return 0, err
	}
	return chatTree(job), nil
}

// killTree beendet den Prozessbaum; ohne Job nur den direkten Prozess.
func killTree(cmd *exec.Cmd, job chatTree) error {
	if job == 0 {
		return cmd.Process.Kill()
	}
	if r, _, err := procTerminateJob.Call(uintptr(job), 1); r == 0 {
		return err
	}
	return nil
}

// closeTree schließt den Job-Handle; noch laufende Mitglieder beendet Windows dabei.
func closeTree(job chatTree) {
	if job != 0 {
		_ = syscall.CloseHandle(syscall.Handle(job))
	}
}
