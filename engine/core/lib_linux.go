//go:build linux && k3ccore

package core

import "github.com/ebitengine/purego"

// open bindet die Symbole über dlopen (purego, ohne cgo). Nur mit `-tags k3ccore`: Das Binary hängt dadurch dynamisch
// an libc, das heutige Docker-Image (distroless/static) kann das nicht laden; es braucht dann eine glibc-Basis
// (distroless/cc). Ohne den Tag bleibt der Server-Build statisch und `open` meldet „nicht unterstützt“ (lib_other.go).
func open(l *Lib, path string) error {
	h, err := purego.Dlopen(path, purego.RTLD_NOW|purego.RTLD_LOCAL)
	if err != nil {
		return err
	}
	l.proc = func(name string) (Proc, error) {
		sym, err := purego.Dlsym(h, name)
		if err != nil {
			return nil, err
		}
		return func(args ...uintptr) uintptr { r, _, _ := purego.SyscallN(sym, args...); return r }, nil
	}
	for name, fn := range map[string]any{
		"k3c_abi_version": &l.abi,
		"k3c_add":         &l.add,
		"k3c_sum_bytes":   &l.sum,
	} {
		sym, err := purego.Dlsym(h, name)
		if err != nil {
			return err
		}
		purego.RegisterFunc(fn, sym)
	}
	return nil
}
