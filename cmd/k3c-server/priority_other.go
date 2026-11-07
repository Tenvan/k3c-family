//go:build !windows

package main

// tuneProcess: Auf Linux (Pi, Docker) gibt es nichts anzupassen.
func tuneProcess() (prio, throttling error) { return nil, nil }
