package core

import (
	"bytes"
	"encoding/json"
	"fmt"
	"runtime"
	"slices"
	"unsafe"

	"k3c/data"
)

// Fehlercodes von k3c_level_generate (rust/k3c-core/src/ffi_level.rs); ≤ -3 heißt: Puffer zu klein, Betrag = Größe.
const (
	LevelErrInput    = -1 // ungültige Eingabe (JSON, unbekanntes Feld, Seed kein UTF-8)
	LevelErrGenerate = -2 // der Generator lehnt ab (wie ein Fehler von level.GenerateWith)
)

// LevelGen ruft den Level-Generator des Rust-Kerns. Das Symbol wird einmal gesucht; Aufrufe sind nebenläufig sicher,
// solange jeder Aufrufer seinen eigenen Ausgabe-Puffer nutzt.
type LevelGen struct{ p Proc }

// LevelGen bindet k3c_level_generate.
func (l *Lib) LevelGen() (*LevelGen, error) {
	p, err := l.Proc("k3c_level_generate")
	if err != nil {
		return nil, err
	}
	return &LevelGen{p: p}, nil
}

// Generate erzeugt das Layout-JSON (Form von json.Marshal(level.Layout)) für input (LevelInput), seed und opts (JSON
// wie level.Options mit `planned`, `chunkCount`, optional `withSim`; leer = Nullwert). Ergebnis in out (wächst bei
// Bedarf, der Aufrufer kann den Puffer wiederverwenden). Fehler des Generators tragen den Text aus Rust.
func (g *LevelGen) Generate(input []byte, seed string, opts, out []byte) ([]byte, error) {
	out = out[:cap(out)]
	for range 2 {
		n := g.call(input, seed, opts, out)
		switch {
		case n >= 0:
			return out[:n], nil
		case n == LevelErrInput || n == LevelErrGenerate:
			msg := out
			if i := bytes.IndexByte(msg, 0); i >= 0 {
				msg = msg[:i]
			}
			if len(msg) == 0 {
				msg = []byte("(ohne Text, Puffer zu klein)")
			}
			kind := "Eingabe"
			if n == LevelErrGenerate {
				kind = "Generator"
			}
			return nil, fmt.Errorf("core: Level (%s): %s", kind, msg)
		default:
			out = slices.Grow(out[:0], int(-n))[:int(-n)]
		}
	}
	return nil, fmt.Errorf("core: Level: Puffer reicht nach Vergrößerung nicht")
}

func (g *LevelGen) call(input []byte, seed string, opts, out []byte) int64 {
	var sp uintptr
	if len(seed) > 0 {
		escape(unsafe.Pointer(unsafe.StringData(seed)))
		sp = uintptr(unsafe.Pointer(unsafe.StringData(seed)))
	}
	r := g.p(bufPtr(input), uintptr(len(input)), sp, uintptr(len(seed)), bufPtr(opts), uintptr(len(opts)),
		bufPtr(out), uintptr(len(out)))
	runtime.KeepAlive(input)
	runtime.KeepAlive(seed)
	runtime.KeepAlive(opts)
	runtime.KeepAlive(out)
	return int64(r)
}

// levelRules sind die Regelwerte aus data/, die der Generator neben dem Biom braucht (rust/k3c-core/src/level/biome.rs, Rules).
type levelRules struct {
	PayRangeUnits float64            `json:"payRangeUnits"`
	SitePlacement json.RawMessage    `json:"sitePlacement"`
	HubSites      []json.RawMessage  `json:"hubSites"`
	Guards        json.RawMessage    `json:"guards"`
	RuinKinds     []string           `json:"ruinKinds"`
	RuinEdges     map[string]float64 `json:"ruinEdges"`
}

// LevelInput baut die Eingabe des Generators: biome ist json.Marshal(level.Biome) (oder die Datei aus data/biomes/),
// dazu die Regeln aus data/ (hub.json, economy.json, guards.json, ruins.json, buildings.json) wie engine/level sie
// lädt. Einmal je Biom vorbereiten; engine/core importiert engine/level nicht (Importzyklus).
func LevelInput(biome []byte) ([]byte, error) {
	var hub struct {
		SitePlacement json.RawMessage   `json:"sitePlacement"`
		Sites         []json.RawMessage `json:"sites"`
		IslandSites   []json.RawMessage `json:"islandSites"`
	}
	var economy struct {
		PayRangeUnits float64 `json:"payRangeUnits"`
	}
	var ruins struct {
		Kinds []string `json:"kinds"`
	}
	var buildings map[string]struct {
		Plantation *struct {
			StripUnits float64 `json:"stripUnits"`
		} `json:"plantation"`
	}
	var guards json.RawMessage
	for name, v := range map[string]any{"hub.json": &hub, "economy.json": &economy, "ruins.json": &ruins,
		"buildings.json": &buildings, "guards.json": &guards} {
		raw, err := data.Files.ReadFile(name)
		if err != nil {
			return nil, err
		}
		if err := json.Unmarshal(raw, v); err != nil {
			return nil, fmt.Errorf("core: %s: %w", name, err)
		}
	}
	r := levelRules{PayRangeUnits: economy.PayRangeUnits, SitePlacement: hub.SitePlacement,
		HubSites: slices.Concat(hub.Sites, hub.IslandSites), Guards: guards, RuinKinds: ruins.Kinds,
		RuinEdges: map[string]float64{}}
	for kind, b := range buildings {
		if b.Plantation != nil {
			r.RuinEdges[kind] = b.Plantation.StripUnits
		}
	}
	return json.Marshal(struct {
		Biome json.RawMessage `json:"biome"`
		Rules levelRules      `json:"rules"`
	}{biome, r})
}
