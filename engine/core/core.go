// Package core lädt den Rust-Kern k3c-core (rust/k3c-core) als Bibliothek, ohne cgo (Entscheidung 006, B-725).
// Windows: `k3c_core.dll` über syscall.LoadDLL; Linux (Pi, Docker): `libk3c_core.so` über purego (dlopen ohne cgo, nur mit `-tags k3ccore`, siehe lib_linux.go).
// Der Kern ist optional: Fehlt die Bibliothek, meldet Find einen Fehler mit den geprüften Pfaden und der Server läuft
// mit der Go-Simulation weiter. Die C-ABI steht in rust/k3c-core/src/lib.rs; ABIVersion muss mit k3c_abi_version übereinstimmen.
package core

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"unsafe"
)

// ABIVersion ist die erwartete ABI-Version der Bibliothek (rust/k3c-core/src/lib.rs, ABI_VERSION).
const ABIVersion = 1

var errUnsupported = errors.New("diese Plattform wird nicht unterstützt")

// EnvLib nennt den Pfad der Bibliothek (Vorrang vor der Suche).
const EnvLib = "K3C_CORE_LIB"

// Lib ist die geladene Bibliothek. Aufrufe sind nebenläufig sicher (der Rust-Teil hält keinen Zustand).
type Lib struct {
	path string
	proc func(name string) (Proc, error) // plattformabhängig (lib_*.go): Symbol als Aufruf mit Zahlen-Argumenten
	abi  func() uint32
	add  func(a, b int64) int64
	sum  func(p *byte, n int) uint64
}

// Proc ruft eine C-Funktion der Bibliothek mit bis zu sechs Zahlen- oder Zeiger-Argumenten (als uintptr) und liefert
// den Rückgabewert. Zeiger müssen bis zum Rückkehr des Aufrufs gültig sein (runtime.KeepAlive beim Aufrufer).
type Proc func(args ...uintptr) uintptr

// Proc liefert das Symbol name der Bibliothek als Aufruf; Fehler, wenn es fehlt.
func (l *Lib) Proc(name string) (Proc, error) { return l.proc(name) }

// escapeSink zwingt Puffer auf den Heap: Zeiger, die als uintptr über Proc gehen, sieht die Escape-Analyse nicht; ein
// Puffer auf dem Go-Stack würde bei Stack-Wachstum verschoben, während Rust noch darauf zeigt.
var escapeSink unsafe.Pointer

// escape markiert p als entkommend (die Allokation des Aufrufers landet auf dem Heap, der GC verschiebt nie).
//
//go:noinline
func escape(p unsafe.Pointer) {
	escapeSink = p
	_ = escapeSink
	escapeSink = nil
}

// Path ist der Ort, von dem die Bibliothek geladen wurde.
func (l *Lib) Path() string { return l.path }

// ABI liefert die ABI-Version der geladenen Bibliothek.
func (l *Lib) ABI() uint32 { return l.abi() }

// Add ist die Probe-Funktion k3c_add (misst die Aufrufkosten Go → Rust).
func (l *Lib) Add(a, b int64) int64 { return l.add(a, b) }

// SumBytes ist die Probe-Funktion k3c_sum_bytes (Zeiger und Länge über die Grenze).
func (l *Lib) SumBytes(b []byte) uint64 {
	if len(b) == 0 {
		return 0
	}
	return l.sum(unsafe.SliceData(b), len(b))
}

// libName ist der Dateiname der Bibliothek auf dieser Plattform.
func libName() string {
	switch runtime.GOOS {
	case "windows":
		return "k3c_core.dll"
	case "darwin":
		return "libk3c_core.dylib"
	}
	return "libk3c_core.so"
}

// Candidates sind die Orte, an denen Find sucht: EnvLib, neben der EXE, `lib/` neben der EXE, `rust/target/release`
// (Entwicklung) vom Arbeitsordner und seinen Eltern aus.
func Candidates() []string {
	var out []string
	if p := os.Getenv(EnvLib); p != "" {
		out = append(out, p)
	}
	name := libName()
	if exe, err := os.Executable(); err == nil {
		dir := filepath.Dir(exe)
		out = append(out, filepath.Join(dir, name), filepath.Join(dir, "lib", name))
	}
	if wd, err := os.Getwd(); err == nil {
		for dir := wd; ; dir = filepath.Dir(dir) {
			out = append(out, filepath.Join(dir, "rust", "target", "release", name))
			if filepath.Dir(dir) == dir {
				break
			}
		}
	}
	return out
}

// Find lädt die Bibliothek vom ersten Kandidaten, der existiert. Fehlt sie überall: Fehler mit allen Pfaden.
func Find() (*Lib, error) {
	cands := Candidates()
	for _, p := range cands {
		if _, err := os.Stat(p); err == nil {
			return Load(p)
		}
	}
	return nil, fmt.Errorf("core: %s nicht gefunden (gesucht: %s); bauen mit `task rust:build` oder %s setzen",
		libName(), strings.Join(cands, ", "), EnvLib)
}

// Load lädt die Bibliothek von path und prüft die ABI-Version.
func Load(path string) (*Lib, error) {
	l := &Lib{path: path}
	if err := open(l, path); err != nil {
		return nil, fmt.Errorf("core: %s laden: %w", path, err)
	}
	if got := l.abi(); got != ABIVersion {
		return nil, fmt.Errorf("core: %s hat ABI %d, erwartet %d (neu bauen: task rust:build)", path, got, ABIVersion)
	}
	return l, nil
}
