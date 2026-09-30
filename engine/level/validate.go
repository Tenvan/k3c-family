package level

import (
	"fmt"
	"math"
	"strconv"
)

// Validate prüft die Spielbarkeits-Regeln wie `validateLevel` in TS. Leere Liste = Level ist gültig.
func Validate(l Layout, b Biome) []string {
	var errs []string
	add := func(format string, args ...any) { errs = append(errs, fmt.Sprintf(format, args...)) }
	count := func(kind string) int { return countKind(l, kind) }

	// Toleranz: Runden auf ganze Chunks (±0.5) plus Auffüllen auf gerade Chunk-Anzahl (+1).
	tolerance := float64(l.ChunkWidthUnits * 1.5) // float64(…) verhindert FMA (B-071)
	if l.WidthUnits < float64(b.LengthUnits.Min)-tolerance {
		add("Level zu kurz")
	}
	if l.WidthUnits > float64(b.LengthUnits.Max)+tolerance {
		add("Level zu lang")
	}
	if count("exit") != 1 {
		add("Genau ein Tiefen-Eingang erwartet")
	}
	if n := count("portal"); n != b.Portals.Count {
		add("%d Portale erwartet, %d gefunden", b.Portals.Count, n)
	}
	return append(append(errs, validateChunks(l, b)...), validateEntities(l, b)...)
}

// validateChunks prüft Lage von Ausgang und Portalen sowie die Zahl der Ereignis-Chunks.
func validateChunks(l Layout, b Biome) []string {
	var errs []string
	add := func(format string, args ...any) { errs = append(errs, fmt.Sprintf(format, args...)) }
	expectedExit := 0
	if b.ExitSide == "right" {
		expectedExit = len(l.Chunks) - 1
	}
	for _, c := range l.Chunks {
		if c.Kind == "exit" {
			if c.Index != expectedExit {
				add("Tiefen-Eingang nicht am Rand")
			}
			break
		}
	}
	for _, c := range l.Chunks {
		dist := math.Abs(c.StartUnits + l.ChunkWidthUnits/2 - l.HubCenterUnits)
		if c.Kind == "portal" && dist < b.Portals.MinDistanceFromHubUnits {
			add("Portal zu nah am Hub (%s Units)", strconv.FormatFloat(dist, 'f', -1, 64))
		}
	}
	for _, kind := range eventKinds {
		want, n := b.EventChunks[kind], countKind(l, kind)
		if n < want.Min || n > want.Max {
			add("%s: %d (erwartet %d-%d)", kind, n, want.Min, want.Max)
		}
	}
	return errs
}

func countKind(l Layout, kind string) int {
	n := 0
	for _, c := range l.Chunks {
		if c.Kind == kind {
			n++
		}
	}
	return n
}

func validateEntities(l Layout, b Biome) []string {
	var errs []string
	skillPoints, castles, outside := 0, 0, false
	for _, e := range l.Entities {
		switch e.Kind {
		case "skillPoint":
			skillPoints++
		case "castle":
			castles++
		}
		outside = outside || e.X < 0 || e.X > l.WidthUnits
	}
	if skillPoints < b.SkillPoints.Min || skillPoints > b.SkillPoints.Max {
		errs = append(errs, fmt.Sprintf("skillPoints: %d", skillPoints))
	}
	if castles != 1 {
		errs = append(errs, "Genau eine Burg erwartet")
	}
	if outside {
		errs = append(errs, "Entity außerhalb des Levels")
	}
	return errs
}
