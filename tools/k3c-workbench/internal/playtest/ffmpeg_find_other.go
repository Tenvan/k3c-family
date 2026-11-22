//go:build !windows

package playtest

// registryPathDirs gibt es nur unter Windows.
func registryPathDirs() []string { return nil }
