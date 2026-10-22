//go:build windows

package main

import (
	_ "embed"
	"syscall"
	"time"
	"unsafe"
)

// Fenster-Icon zur Laufzeit (Windows): `wails build` bettet icon/icon.ico als Ressource in die EXE ein, ein Dev-Lauf
// (`go build`, `wails dev`) hat sie nicht, das Fenster bliebe beim Standard-Icon. Deshalb setzt der Start das Icon des
// eigenen Hauptfensters selbst (WM_SETICON) aus dem eingebetteten PNG; mit Ressource ändert das nichts.

//go:embed icon/appicon.png
var windowIconPNG []byte

var (
	user32                    = syscall.NewLazyDLL("user32.dll")
	procEnumWindows           = user32.NewProc("EnumWindows")
	procGetWindowThreadProcID = user32.NewProc("GetWindowThreadProcessId")
	procIsWindowVisible       = user32.NewProc("IsWindowVisible")
	procCreateIconFromResEx   = user32.NewProc("CreateIconFromResourceEx")
	procSendMessage           = user32.NewProc("SendMessageW")
)

const (
	wmSetIcon  = 0x0080
	iconSmall  = 0
	iconBig    = 1
	resVersion = 0x00030000
)

// setWindowIcon setzt das Icon aller sichtbaren Fenster dieses Prozesses; ein Fenster entsteht kurz nach dem Start, deshalb
// mit wenigen Wiederholungen. Läuft in eigener Goroutine, ein Fehler lässt das Standard-Icon stehen.
func setWindowIcon() {
	big, small := iconOf(32), iconOf(16)
	if big == 0 {
		return
	}
	pid := uint32(syscall.Getpid())
	for range 20 {
		set := false
		cb := syscall.NewCallback(func(hwnd, _ uintptr) uintptr {
			var owner uint32
			_, _, _ = procGetWindowThreadProcID.Call(hwnd, uintptr(unsafe.Pointer(&owner)))
			if visible, _, _ := procIsWindowVisible.Call(hwnd); owner == pid && visible != 0 {
				_, _, _ = procSendMessage.Call(hwnd, wmSetIcon, iconBig, big)
				_, _, _ = procSendMessage.Call(hwnd, wmSetIcon, iconSmall, small)
				set = true
			}
			return 1
		})
		_, _, _ = procEnumWindows.Call(cb, 0)
		if set {
			return
		}
		time.Sleep(250 * time.Millisecond)
	}
}

// iconOf macht aus dem PNG ein Icon-Handle der Kantenlänge px (0 bei Fehler).
func iconOf(px int) uintptr {
	h, _, _ := procCreateIconFromResEx.Call(uintptr(unsafe.Pointer(&windowIconPNG[0])), uintptr(len(windowIconPNG)), 1, resVersion, uintptr(px), uintptr(px), 0)
	return h
}
