//go:build !windows

// Package winicon: Das Fenster-Icon setzt nur die Windows-Fassung (winicon_windows.go).
package winicon

// Set tut außerhalb von Windows nichts.
func Set([]byte) {}

// Handle gibt es nur unter Windows.
func Handle([]byte, int) uintptr { return 0 }
