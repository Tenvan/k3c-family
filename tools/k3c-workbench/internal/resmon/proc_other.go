//go:build !windows

package resmon

// readProcess kennt außerhalb von Windows nur, was die Go-Laufzeit selbst meldet; RSS, Handles und Kinder bleiben 0.
func readProcess() procSnapshot { return procSnapshot{} }
