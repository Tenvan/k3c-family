package core

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

// libForTest baut die Bibliothek bei Bedarf (cargo vorhanden) oder überspringt den Test; in der CI baut
// `task rust:build` vorher.
func libForTest(t *testing.T) *Lib {
	t.Helper()
	if err := open(&Lib{}, ""); errors.Is(err, errUnsupported) {
		t.Skip("Plattform ohne Kern-Brücke (Linux nur mit -tags k3ccore)")
	}
	if l, err := Find(); err == nil {
		return l
	}
	if _, err := exec.LookPath("cargo"); err != nil {
		t.Skip("keine Rust-Bibliothek und kein cargo: task rust:build")
	}
	root, _ := filepath.Abs("../..")
	cmd := exec.Command("cargo", "build", "--release", "--locked", "-p", "k3c-core")
	cmd.Dir = filepath.Join(root, "rust")
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("cargo build: %v\n%s", err, out)
	}
	l, err := Find()
	if err != nil {
		t.Fatal(err)
	}
	return l
}

func TestProbeAufrufe(t *testing.T) {
	l := libForTest(t)
	if l.ABI() != ABIVersion {
		t.Fatalf("ABI %d", l.ABI())
	}
	if got := l.Add(40, 2); got != 42 {
		t.Fatalf("Add = %d", got)
	}
	if got := l.Add(-5, 3); got != -2 {
		t.Fatalf("Add negativ = %d", got)
	}
	if got := l.SumBytes([]byte{1, 2, 3, 250}); got != 256 {
		t.Fatalf("SumBytes = %d", got)
	}
	if got := l.SumBytes(nil); got != 0 {
		t.Fatalf("SumBytes leer = %d", got)
	}
}

func TestFehlendeBibliothekMeldetFehler(t *testing.T) {
	missing := filepath.Join(t.TempDir(), "gibt-es-nicht")
	t.Setenv(EnvLib, missing)
	if _, err := Load(missing); err == nil {
		t.Fatal("Fehler erwartet")
	}
	if c := Candidates(); len(c) == 0 || c[0] != missing {
		t.Fatal("EnvLib hat keinen Vorrang")
	}
	if _, err := os.Stat(missing); err == nil {
		t.Fatal("unerwartet vorhanden")
	}
}

// BenchmarkAufrufKosten misst einen Aufruf Go → Rust (k3c_add): untere Schranke der Brückenkosten je Aufruf.
func BenchmarkAufrufKosten(b *testing.B) {
	l, err := Find()
	if err != nil {
		b.Skip(err)
	}
	var x int64
	for b.Loop() {
		x = l.Add(x, 1)
	}
	_ = x
}

// BenchmarkAufrufMitPuffer4k: Aufruf mit 4-KiB-Puffer (Zeiger und Länge), dazu die Rechenzeit der Summe.
func BenchmarkAufrufMitPuffer4k(b *testing.B) {
	l, err := Find()
	if err != nil {
		b.Skip(err)
	}
	buf := make([]byte, 4096)
	for b.Loop() {
		l.SumBytes(buf)
	}
}
