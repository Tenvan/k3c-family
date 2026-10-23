//go:build !windows

package main

// showError meldet Startfehler nur über stderr (main).
func showError(string, string) {}
