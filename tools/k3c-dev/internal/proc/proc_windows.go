//go:build windows

package proc

import (
	"os/exec"
	"strconv"
	"syscall"
	"unsafe"

	"golang.org/x/sys/windows"
)

// createNoWindow verhindert ein Konsolenfenster je Kindprozess.
const createNoWindow = 0x08000000

func prepare(cmd *exec.Cmd) {
	cmd.SysProcAttr = &syscall.SysProcAttr{CreationFlags: createNoWindow}
}

// job ist ein Job Object mit KILL_ON_JOB_CLOSE; der Nullwert heißt „kein Job“.
type job struct{ h windows.Handle }

// attach legt einen Job an und ordnet den Prozess zu; scheitert etwas, gibt es keinen Job und Kill nimmt KillTree.
func attach(pid int) job {
	h, err := windows.CreateJobObject(nil, nil)
	if err != nil {
		return job{}
	}
	info := windows.JOBOBJECT_EXTENDED_LIMIT_INFORMATION{
		BasicLimitInformation: windows.JOBOBJECT_BASIC_LIMIT_INFORMATION{LimitFlags: windows.JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE},
	}
	if _, err := windows.SetInformationJobObject(h, windows.JobObjectExtendedLimitInformation,
		uintptr(unsafe.Pointer(&info)), uint32(unsafe.Sizeof(info))); err != nil {
		_ = windows.CloseHandle(h)
		return job{}
	}
	ph, err := windows.OpenProcess(windows.PROCESS_SET_QUOTA|windows.PROCESS_TERMINATE, false, uint32(pid))
	if err != nil {
		_ = windows.CloseHandle(h)
		return job{}
	}
	defer func() { _ = windows.CloseHandle(ph) }()
	if err := windows.AssignProcessToJobObject(h, ph); err != nil {
		_ = windows.CloseHandle(h)
		return job{}
	}
	return job{h: h}
}

// terminate beendet alle Prozesse im Job; false heißt: kein Job, der Aufrufer nimmt KillTree.
func (j job) terminate() bool {
	return j.h != 0 && windows.TerminateJobObject(j.h, 1) == nil
}

// close schließt das Handle; wegen KILL_ON_JOB_CLOSE enden dabei alle übrigen Prozesse im Job.
func (j job) close() {
	if j.h != 0 {
		_ = windows.CloseHandle(j.h)
	}
}

// KillTree beendet einen Prozess samt allen Kindern, die über die Eltern-Kette erreichbar sind (für Prozesse ohne
// Job, z. B. übernommene Dienste).
func KillTree(pid int) error {
	cmd := exec.Command("taskkill", "/T", "/F", "/PID", strconv.Itoa(pid))
	prepare(cmd)
	return cmd.Run()
}
