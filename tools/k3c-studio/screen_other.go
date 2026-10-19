//go:build !windows

package main

// onScreen prüft nur unter Windows; anderswo gilt die gemerkte Position.
func onScreen(windowState) bool { return true }

// showError meldet Startfehler nur über stderr (main).
func showError(string, string) {}
