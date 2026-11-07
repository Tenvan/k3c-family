package core

import (
	"fmt"
	"runtime"
	"unsafe"
)

// RngHash ruft k3c_rng_hash: FNV-1a des Seeds wie rng.HashSeed.
func (l *Lib) RngHash(seed string) (uint32, error) {
	p, err := l.Proc("k3c_rng_hash")
	if err != nil {
		return 0, err
	}
	b := []byte(seed)
	r := p(bufPtr(b), uintptr(len(b)))
	runtime.KeepAlive(b)
	return uint32(r), nil
}

// RngNext liefert die ersten n Zahlen von rng.New(seed).Next() aus dem Rust-Kern.
func (l *Lib) RngNext(seed string, n int) ([]float64, error) {
	p, err := l.Proc("k3c_rng_next_n")
	if err != nil {
		return nil, err
	}
	if n < 0 {
		return nil, fmt.Errorf("core: n = %d", n)
	}
	out := make([]float64, n)
	if n == 0 {
		return out, nil
	}
	b := []byte(seed)
	escape(unsafe.Pointer(&out[0]))
	p(bufPtr(b), uintptr(len(b)), uintptr(unsafe.Pointer(&out[0])), uintptr(n))
	runtime.KeepAlive(b)
	return out, nil
}

// bufPtr ist die Adresse des ersten Bytes (0 bei leerem Puffer).
func bufPtr(b []byte) uintptr {
	if len(b) == 0 {
		return 0
	}
	escape(unsafe.Pointer(&b[0]))
	return uintptr(unsafe.Pointer(&b[0]))
}
