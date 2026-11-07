//go:build windows

package main

import "testing"

func TestTuneProcess(t *testing.T) {
	prio, throttling := tuneProcess()
	if prio != nil {
		t.Fatalf("Priorität: %v", prio)
	}
	if throttling != nil {
		t.Logf("Power-Throttling nicht abschaltbar: %v", throttling)
	}
}
