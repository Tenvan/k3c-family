//go:build windows

// Package winicon setzt das Fenster-Icon einer Wails-App zur Laufzeit (Windows): `wails build` bettet icon/icon.ico als
// Ressource in die EXE ein, ein Dev-Lauf (`go build`, `wails dev`) hat sie nicht, das Fenster bliebe beim
// Standard-Icon. Deshalb setzt der Start das Icon des eigenen Hauptfensters selbst (WM_SETICON) aus dem eingebetteten
// PNG; mit Ressource ändert das nichts. Gemeinsam für k3c-workbench und k3c-studio (WZ63.3).
package winicon

import (
	"syscall"
	"time"
	"unsafe"
)

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

// Set setzt das Icon aller sichtbaren Fenster dieses Prozesses aus png; ein Fenster entsteht kurz nach dem Start,
// deshalb mit wenigen Wiederholungen. Läuft in eigener Goroutine, ein Fehler lässt das Standard-Icon stehen.
func Set(png []byte) {
	big, small := Handle(png, 32), Handle(png, 16)
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

// Handle macht aus dem PNG ein Icon-Handle der Kantenlänge px (0 bei Fehler oder leerem PNG).
func Handle(png []byte, px int) uintptr {
	if len(png) == 0 {
		return 0
	}
	h, _, _ := procCreateIconFromResEx.Call(uintptr(unsafe.Pointer(&png[0])), uintptr(len(png)), 1, resVersion, uintptr(px), uintptr(px), 0)
	return h
}
