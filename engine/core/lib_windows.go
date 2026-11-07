//go:build windows

package core

import (
	"syscall"
	"unsafe"
)

// open bindet die Symbole über syscall.LoadDLL und SyscallN (kein cgo, keine Allokation je Aufruf).
func open(l *Lib, path string) error {
	dll, err := syscall.LoadDLL(path)
	if err != nil {
		return err
	}
	addr := func(name string) (uintptr, error) {
		p, err := dll.FindProc(name)
		if err != nil {
			return 0, err
		}
		return p.Addr(), nil
	}
	abi, err := addr("k3c_abi_version")
	if err != nil {
		return err
	}
	add, err := addr("k3c_add")
	if err != nil {
		return err
	}
	sum, err := addr("k3c_sum_bytes")
	if err != nil {
		return err
	}
	l.abi = func() uint32 { r, _, _ := syscall.SyscallN(abi); return uint32(r) }
	l.add = func(a, b int64) int64 { r, _, _ := syscall.SyscallN(add, uintptr(a), uintptr(b)); return int64(r) }
	l.sum = func(p *byte, n int) uint64 {
		r, _, _ := syscall.SyscallN(sum, uintptr(unsafe.Pointer(p)), uintptr(n))
		return uint64(r)
	}
	return nil
}
