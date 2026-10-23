//go:build !windows

package windowstate

// OnScreen prüft nur unter Windows; anderswo gilt die gemerkte Position.
func OnScreen(State) bool { return true }
