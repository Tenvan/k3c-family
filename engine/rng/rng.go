// Package rng ist der deterministische Zufallsgenerator (mulberry32), Port von src/core/rng.ts.
// Gleicher Seed => gleiche Zahlenfolge wie in TypeScript; geprüft gegen testdata/golden/rng.json.
package rng

import (
	"math"
	"unicode/utf16"
)

// Rng liefert eine feste Zahlenfolge je Seed.
type Rng struct {
	state uint32
}

// Weight ist ein Eintrag für Weighted. Eine Liste statt einer map, weil TS in Einfügereihenfolge zieht.
type Weight struct {
	Key string
	W   float64
}

// HashSeed macht aus einem Text eine 32-Bit-Zahl (FNV-1a) über UTF-16-Codeeinheiten wie `charCodeAt`.
func HashSeed(seed string) uint32 {
	h := uint32(0x811c9dc5)
	for _, c := range utf16.Encode([]rune(seed)) {
		h ^= uint32(c)
		h *= 0x01000193
	}
	return h
}

// New erzeugt einen Generator für einen Text-Seed.
func New(seed string) *Rng {
	return &Rng{state: HashSeed(seed)}
}

// Next liefert eine Zahl in [0, 1).
func (r *Rng) Next() float64 {
	r.state += 0x6d2b79f5
	t := r.state
	t = (t ^ t>>15) * (t | 1)
	t ^= t + (t^t>>7)*(t|61)
	return float64(t^t>>14) / 4294967296
}

// Int liefert eine ganze Zahl in [lo, hi], beide inklusive.
func (r *Rng) Int(lo, hi int) int {
	return lo + int(math.Floor(r.Next()*float64(hi-lo+1)))
}

// Weighted zieht einen Schlüssel nach Gewicht, in der Reihenfolge der Liste.
func (r *Rng) Weighted(weights []Weight) string {
	total := 0.0
	for _, w := range weights {
		total += w.W
	}
	if total <= 0 {
		panic("Weighted braucht mindestens ein positives Gewicht")
	}
	// float64(…) rundet das Produkt, sonst darf Go es mit dem Abziehen zu FMA fusionieren (B-071).
	roll := float64(r.Next() * total)
	for _, w := range weights {
		roll -= w.W
		if roll < 0 {
			return w.Key
		}
	}
	return weights[len(weights)-1].Key
}

// Pick wählt ein Element; wie in TS ein Fehler bei leerer Liste.
func Pick[T any](r *Rng, items []T) T {
	if len(items) == 0 {
		panic("Pick auf leerer Liste")
	}
	return items[r.Int(0, len(items)-1)]
}

// Shuffle mischt in place (Fisher-Yates von hinten wie in TS) und gibt die Liste zurück.
func Shuffle[T any](r *Rng, items []T) []T {
	for i := len(items) - 1; i > 0; i-- {
		j := r.Int(0, i)
		items[i], items[j] = items[j], items[i]
	}
	return items
}
