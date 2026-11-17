package detvec

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"flag"
	"fmt"
	"math"
	"os"
	"testing"
)

var write = flag.Bool("write", false, "Vektor-Dateien unter testdata/rust neu schreiben")

const vectorsPath = "../../../testdata/rust/det-vectors.json.gz"

// Go wählt math.Exp auf amd64 zur Laufzeit: mit AVX und FMA den FMA-Pfad (exp_amd64.s, useFMA), sonst den
// SSE-Pfad, der in der letzten Stelle abweicht (bei etwa 9 % der Eingaben). Die Vektoren und alle Golden
// stammen vom FMA-Pfad; probeX zeigt, welchen Pfad dieser Rechner nimmt.
var (
	probeX   = math.Float64frombits(0xc048ea1cac083127) // -49.829
	probeFMA = uint64(0x3b714a7e823b04b1)
)

func hasFMA() bool { return math.Float64bits(math.Exp(probeX)) == probeFMA }

// row ist eine Zeile der Vektor-Datei; Zahlen als Bits in 16 Hex-Stellen.
type row struct {
	F string `json:"f"`
	A string `json:"a"`
	B string `json:"b,omitempty"`
	R string `json:"r"`
}

func bits(x float64) string { return fmt.Sprintf("%016x", math.Float64bits(x)) }

// compute rechnet eine Eingabe mit der Go-Standardbibliothek.
func compute(in input) float64 {
	switch in.f {
	case "pow":
		return math.Pow(in.a, in.b)
	case "exp":
		return math.Exp(in.a)
	case "log":
		return math.Log(in.a)
	case "fmax":
		return math.Max(in.a, in.b)
	case "fmin":
		return math.Min(in.a, in.b)
	}
	panic("unbekannte Funktion " + in.f)
}

func build() []row {
	var rows []row
	for _, in := range inputs() {
		r := row{F: in.f, A: bits(in.a), R: bits(compute(in))}
		if in.f == "pow" || in.f == "fmax" || in.f == "fmin" {
			r.B = bits(in.b)
		}
		rows = append(rows, r)
	}
	return rows
}

// TestWriteVectors schreibt mit -write die Vektor-Datei; ohne -write vergleicht er Go mit der Datei.
func TestWriteVectors(t *testing.T) {
	fma := hasFMA()
	got := build()
	if *write {
		if !fma {
			t.Fatal("🚫 CPU ohne AVX/FMA: Go rechnet Exp auf dem SSE-Pfad; Vektoren nur auf einem Rechner mit FMA schreiben")
		}
		writeVectors(t, got)
		return
	}
	want := readVectors(t)
	if len(want) != len(got) {
		t.Fatalf("%d Zeilen, erwartet %d (neu schreiben mit -write)", len(got), len(want))
	}
	if !fma {
		t.Log("⏳ CPU ohne AVX/FMA: exp und pow mit gebrochenem Exponenten weichen ab und werden übersprungen")
	}
	for i, w := range want {
		g := got[i]
		if !fma && (w.F == "exp" || w.F == "pow") {
			continue
		}
		if g != w {
			t.Fatalf("Zeile %d: %s(%s, %s) = %s, erwartet %s", i, g.F, g.A, g.B, g.R, w.R)
		}
	}
}

func writeVectors(t *testing.T, rows []row) {
	t.Helper()
	var text bytes.Buffer
	text.WriteString("[\n")
	for i, r := range rows {
		line, err := json.Marshal(r)
		if err != nil {
			t.Fatal(err)
		}
		text.Write(line)
		if i < len(rows)-1 {
			text.WriteString(",")
		}
		text.WriteString("\n")
	}
	text.WriteString("]\n")
	var buf bytes.Buffer
	zw := gzip.NewWriter(&buf)
	if _, err := zw.Write(text.Bytes()); err != nil {
		t.Fatal(err)
	}
	if err := zw.Close(); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll("../../../testdata/rust", 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(vectorsPath, buf.Bytes(), 0o644); err != nil {
		t.Fatal(err)
	}
	t.Logf("💾 %s: %d Zeilen", vectorsPath, len(rows))
}

func readVectors(t *testing.T) []row {
	t.Helper()
	raw, err := os.ReadFile(vectorsPath)
	if err != nil {
		t.Fatal(err)
	}
	zr, err := gzip.NewReader(bytes.NewReader(raw))
	if err != nil {
		t.Fatal(err)
	}
	var rows []row
	if err := json.NewDecoder(zr).Decode(&rows); err != nil {
		t.Fatal(err)
	}
	return rows
}
