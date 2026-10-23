//go:build windows

package resmon

import (
	"unsafe"

	"golang.org/x/sys/windows"
)

var (
	kernel32        = windows.NewLazySystemDLL("kernel32.dll")
	procMemoryInfo  = kernel32.NewProc("K32GetProcessMemoryInfo")
	procHandleCount = kernel32.NewProc("GetProcessHandleCount")
)

// maxParentChain begrenzt das Folgen der Eltern-Kette (wiederverwendete PIDs können Zyklen bilden).
const maxParentChain = 64

// memoryCounters ist PROCESS_MEMORY_COUNTERS aus psapi.h.
type memoryCounters struct {
	cb                         uint32
	pageFaultCount             uint32
	peakWorkingSetSize         uintptr
	workingSetSize             uintptr
	quotaPeakPagedPoolUsage    uintptr
	quotaPagedPoolUsage        uintptr
	quotaPeakNonPagedPoolUsage uintptr
	quotaNonPagedPoolUsage     uintptr
	pagefileUsage              uintptr
	peakPagefileUsage          uintptr
}

// memory liefert Arbeitssatz und privaten Speicher (Commit) eines Prozesses; bei Fehler 0.
func memory(h windows.Handle) (rss, private uint64) {
	var c memoryCounters
	c.cb = uint32(unsafe.Sizeof(c))
	if r, _, _ := procMemoryInfo.Call(uintptr(h), uintptr(unsafe.Pointer(&c)), uintptr(c.cb)); r == 0 {
		return 0, 0
	}
	return uint64(c.workingSetSize), uint64(c.pagefileUsage)
}

// readProcess misst den eigenen Prozess, seine Handles und alle Nachkommen (go test, vitest, Vite, Spielserver).
func readProcess() procSnapshot {
	var s procSnapshot
	self := windows.CurrentProcess()
	s.rss, s.private = memory(self)
	var handles uint32
	if r, _, _ := procHandleCount.Call(uintptr(self), uintptr(unsafe.Pointer(&handles))); r != 0 {
		s.handles = int(handles)
	}
	s.children = descendants(windows.GetCurrentProcessId())
	return s
}

// descendants sammelt alle Nachkommen von root aus einem Prozess-Schnappschuss.
func descendants(root uint32) []procInfo {
	snap, err := windows.CreateToolhelp32Snapshot(windows.TH32CS_SNAPPROCESS, 0)
	if err != nil {
		return nil
	}
	defer func() { _ = windows.CloseHandle(snap) }()
	parent := map[uint32]uint32{}
	name := map[uint32]string{}
	var e windows.ProcessEntry32
	e.Size = uint32(unsafe.Sizeof(e))
	for err = windows.Process32First(snap, &e); err == nil; err = windows.Process32Next(snap, &e) {
		parent[e.ProcessID], name[e.ProcessID] = e.ParentProcessID, windows.UTF16ToString(e.ExeFile[:])
	}
	var out []procInfo
	for pid := range parent {
		if isDescendant(pid, root, parent) {
			out = append(out, child(pid, name[pid]))
		}
	}
	return out
}

func isDescendant(pid, root uint32, parent map[uint32]uint32) bool {
	for range maxParentChain {
		p, ok := parent[pid]
		if !ok || p == 0 {
			return false
		}
		if p == root {
			return true
		}
		pid = p
	}
	return false
}

func child(pid uint32, name string) procInfo {
	pi := procInfo{pid: pid, name: name}
	h, err := windows.OpenProcess(windows.PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
	if err != nil {
		return pi
	}
	defer func() { _ = windows.CloseHandle(h) }()
	pi.rss, _ = memory(h)
	return pi
}
