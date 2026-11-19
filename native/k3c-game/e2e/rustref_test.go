package e2e

// Referenz für den Rust-Port (RU32.1, B-1014): Der Ablauf (flow) als Datei testdata/rust/e2e/flow.json: Eingabe-Folge,
// Takt des Skill-Punkts, die 14 Stationen mit Takt und kanonischem Welt-Hash, dazu ein kleiner Probe-Baum mit seinem
// Hash. Rust (rust/crates/k3c-game/tests/e2e_flow.rs) fährt dieselbe Eingabe-Folge und vergleicht.
//
// Der Hash über json.Marshal (run_test.go › frameHash) hängt an den Bytes von encoding/json; Rust erzeugt dieselben
// Bäume, aber nicht dieselben Bytes. Darum rechnen Go und Rust diesen kanonischen Hash (tests/common/canon.rs):
//
//   - Eingabe: der JSON-Baum der Welt je Stufe (json.Marshal, dann json.Unmarshal in any).
//   - Objekte: Schlüssel aufsteigend nach Bytes, `{"k":v,...}`; Listen `[v,...]`; null und leere Liste beide `[]`.
//   - Zahlen: 16 Hex-Stellen der f64-Bits (minus null zählt als null); Wahrheitswerte true/false.
//   - Texte: JSON-Text ohne HTML-Escaping: nur `"` `\` und die Steuerzeichen unter U+0020 werden ersetzt (\n \r \t,
//     sonst \u00xx), alles andere bleibt UTF-8.
//   - Stufen nach Index aufsteigend, je Stufe `<index>:<Form>\n`; ohne Stufe `-`.
//   - Hash: FNV-1a 64 über die UTF-8-Bytes, 16 Hex-Stellen.
//
// Schreiben (im Ordner native/k3c-game; die Datei ist eingecheckt):
//
//	go test ./e2e -run TestRustRef -write
//
// Ohne -write prüft der Test, dass die Datei zum aktuellen Lauf passt.

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"hash/fnv"
	"maps"
	"math"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"strings"
	"testing"

	"k3c/engine/sim"
)

var writeRef = flag.Bool("write", false, "testdata/rust/e2e/flow.json neu schreiben")

const refFile = "../../../testdata/rust/e2e/flow.json"

// refStation ist eine Station der Referenz.
type refStation struct {
	Name string `json:"name"`
	Tick int    `json:"tick"`
	Hash string `json:"hash"`
}

// refProbe prüft die kanonische Form selbst: ein Baum mit Zahl, null, leerer Liste, Umlaut und Verschachtelung.
type refProbe struct {
	Tree json.RawMessage `json:"tree"`
	Hash string          `json:"hash"`
}

// refFlow ist der Inhalt von flow.json.
type refFlow struct {
	Script   string       `json:"script"`
	Found    int          `json:"found"`
	Stations []refStation `json:"stations"`
	Probe    refProbe     `json:"probe"`
}

const probeTree = `{"zahl":0.1,"ganz":42,"neg":-1.5e-7,"leer":[],"nichts":null,"text":"Grüße \"Ä\" <b>&\n",` +
	`"tief":{"b":[1,{"y":true,"x":false}],"a":{"k":"v"}},"liste":[3,2,1]}`

// canon schreibt die kanonische Form eines Baums aus encoding/json (any).
func canon(b *strings.Builder, v any) {
	switch x := v.(type) {
	case nil:
		b.WriteString("[]")
	case bool:
		b.WriteString(strconv.FormatBool(x))
	case float64:
		if x == 0 {
			x = 0 // minus null
		}
		fmt.Fprintf(b, "%016x", math.Float64bits(x))
	case string:
		canonString(b, x)
	case []any:
		b.WriteByte('[')
		for i, e := range x {
			if i > 0 {
				b.WriteByte(',')
			}
			canon(b, e)
		}
		b.WriteByte(']')
	case map[string]any:
		b.WriteByte('{')
		for i, k := range slices.Sorted(maps.Keys(x)) {
			if i > 0 {
				b.WriteByte(',')
			}
			canonString(b, k)
			b.WriteByte(':')
			canon(b, x[k])
		}
		b.WriteByte('}')
	default:
		panic(fmt.Sprintf("kanonische Form: Typ %T", v))
	}
}

func canonString(b *strings.Builder, s string) {
	b.WriteByte('"')
	for _, r := range s {
		switch {
		case r == '"' || r == '\\':
			b.WriteByte('\\')
			b.WriteRune(r)
		case r == '\n':
			b.WriteString(`\n`)
		case r == '\r':
			b.WriteString(`\r`)
		case r == '\t':
			b.WriteString(`\t`)
		case r < 0x20:
			fmt.Fprintf(b, `\u%04x`, r)
		default:
			b.WriteRune(r)
		}
	}
	b.WriteByte('"')
}

// fnvHex ist FNV-1a 64 als 16 Hex-Stellen.
func fnvHex(s string) string {
	h := fnv.New64a()
	h.Write([]byte(s))
	return fmt.Sprintf("%016x", h.Sum64())
}

// worldTree macht aus einer Welt den JSON-Baum (json.Marshal, json.Unmarshal in any).
func worldTree(t *testing.T, w *sim.World) any {
	t.Helper()
	raw, err := json.Marshal(w)
	if err != nil {
		t.Fatal(err)
	}
	var v any
	if err := json.Unmarshal(raw, &v); err != nil {
		t.Fatal(err)
	}
	return v
}

// canonHash ist der kanonische Welt-Hash des laufenden Spiels ("-" ohne Stufe).
func (r *run) canonHash() string {
	r.t.Helper()
	worlds := map[int]*sim.World{}
	for _, st := range r.frame().Stages {
		worlds[st.Index] = st.World
	}
	if len(worlds) == 0 {
		return "-"
	}
	var b strings.Builder
	for _, k := range slices.Sorted(maps.Keys(worlds)) {
		fmt.Fprintf(&b, "%d:", k)
		canon(&b, worldTree(r.t, worlds[k]))
		b.WriteByte('\n')
	}
	return fnvHex(b.String())
}

// buildRef fährt den Ablauf und danach das Spiel noch einmal mit derselben Eingabe-Folge bis zu den Takten der
// Stationen; dort wird der kanonische Hash gerechnet (der Ablauf selbst bleibt unverändert).
func buildRef(t *testing.T) refFlow {
	t.Helper()
	r := flow(t)
	ref := refFlow{Script: r.script, Found: r.found}
	again := newRun(t, "--flow", "--input", r.script)
	for _, s := range r.stations {
		again.to(s.Tick)
		if got := frameHash(t, again.frame()); got != s.Hash {
			t.Fatalf("Station %s: zweiter Lauf weicht ab (%s gegen %s)", s.Name, got, s.Hash)
		}
		ref.Stations = append(ref.Stations, refStation{Name: s.Name, Tick: s.Tick, Hash: again.canonHash()})
	}
	var probe any
	if err := json.Unmarshal([]byte(probeTree), &probe); err != nil {
		t.Fatal(err)
	}
	var b strings.Builder
	canon(&b, probe)
	ref.Probe = refProbe{Tree: json.RawMessage(probeTree), Hash: fnvHex(b.String())}
	return ref
}

func marshalRef(t *testing.T, ref refFlow) []byte {
	t.Helper()
	var buf bytes.Buffer
	enc := json.NewEncoder(&buf)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", "  ")
	if err := enc.Encode(ref); err != nil {
		t.Fatal(err)
	}
	return buf.Bytes()
}

// TestRustRef schreibt (mit -write) oder prüft die Referenz des Rust-Ports.
func TestRustRef(t *testing.T) {
	if testing.Short() && !*writeRef {
		t.Skip("Ende-zu-Ende (etwa 6 s): nur ohne -short")
	}
	got := marshalRef(t, buildRef(t))
	if *writeRef {
		if err := os.MkdirAll(filepath.Dir(refFile), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(refFile, got, 0o644); err != nil {
			t.Fatal(err)
		}
		t.Logf("📄 %s geschrieben (%d Bytes)", refFile, len(got))
		return
	}
	want, err := os.ReadFile(refFile)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(bytes.ReplaceAll(want, []byte("\r\n"), []byte("\n")), got) {
		t.Fatalf("❌ %s passt nicht zum Ablauf: go test ./e2e -run TestRustRef -write (im Ordner native/k3c-game), Grund im Commit nennen", refFile)
	}
	var ref refFlow
	if err := json.Unmarshal(want, &ref); err != nil || len(ref.Stations) != 14 {
		t.Fatalf("flow.json unlesbar oder ohne 14 Stationen: %v", err)
	}
}
