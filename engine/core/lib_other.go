//go:build !windows && !(linux && k3ccore)

package core

func open(*Lib, string) error { return errUnsupported }
