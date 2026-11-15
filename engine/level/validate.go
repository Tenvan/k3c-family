package level

import (
	"fmt"
	"math"
	"strconv"
)

// Validate prüft die Spielbarkeits-Regeln wie `validateLevel` in TS. Leere Liste = Level ist gültig.
func Validate(l Layout, b Biome) []string { return ValidateWith(l, b, Options{}) }

// ValidateWith prüft wie Validate; mit eingestellter Chunk-Anzahl (o.ChunkCount) gilt die Länge gegen die Anzahl statt
// gegen lengthUnits (geplante-chunks.md § 7).
func ValidateWith(l Layout, b Biome, o Options) []string {
	veinErrs := validateVeins(l, b, o)
	b.Lair = o.hasLair(b)
	b = b.withNature(l.Nature) // Ressourcen und Chunk-Gewichte nach den Natur-Werten der Stufe (B-891)
	b.Veins.Count = 0           // Adern prüft validateVeins
	errs := append(validateLength(l, b, o.ChunkCount), veinErrs...)
	add := func(format string, args ...any) { errs = append(errs, fmt.Sprintf(format, args...)) }
	// Portale, Truhen und Camps zählen als Objekte: feste Objekte geplanter Chunks stehen auch in anderen Chunk-Arten.
	if countKind(l, "exit") != 1 {
		add("Genau ein Tiefen-Eingang erwartet")
	}
	if n := countEntities(l, "portal"); n != b.Portals.Count {
		add("%d Portale erwartet, %d gefunden", b.Portals.Count, n)
	}
	return append(append(errs, validateChunks(l, b)...), validateEntities(l, b)...)
}

// validateLength prüft die Länge: mit eingestellter Anzahl genau so viele Chunks, sonst innerhalb von lengthUnits.
func validateLength(l Layout, b Biome, count int) []string {
	if count > 0 {
		if len(l.Chunks) != count || l.WidthUnits != float64(count)*l.ChunkWidthUnits {
			return []string{fmt.Sprintf("Level hat %d Chunks, erwartet %d", len(l.Chunks), count)}
		}
		return nil
	}
	var errs []string
	// Toleranz: Runden auf ganze Chunks (±0.5) plus Auffüllen auf gerade Chunk-Anzahl (+1).
	tolerance := float64(l.ChunkWidthUnits * 1.5) // float64(…) verhindert FMA (B-071)
	if l.WidthUnits < float64(b.LengthUnits.Min)-tolerance {
		errs = append(errs, "Level zu kurz")
	}
	if l.WidthUnits > float64(b.LengthUnits.Max)+tolerance {
		errs = append(errs, "Level zu lang")
	}
	return errs
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
	for _, e := range l.Entities {
		dist := math.Abs(e.X - l.HubCenterUnits)
		if e.Kind == "portal" && dist < b.Portals.MinDistanceFromHubUnits {
			add("Portal zu nah am Hub (%s Units)", strconv.FormatFloat(dist, 'f', -1, 64))
		}
	}
	errs = append(errs, validatePortalsOutsideLines(l)...)
	errs = append(errs, validateLair(l, b)...)
	for _, kind := range eventKinds {
		want, n := b.EventChunks[kind], countEntities(l, kind)
		if n < want.Min || n > want.Max {
			add("%s: %d (erwartet %d-%d)", kind, n, want.Min, want.Max)
		}
	}
	return errs
}

// validatePortalsOutsideLines meldet Portale, die nicht außerhalb der Verteidigungszone liegen (`sitePlacement.outsideUnits`, Q49, Q57).
func validatePortalsOutsideLines(l Layout) []string {
	var errs []string
	for _, e := range l.Entities {
		if dx := math.Abs(e.X - l.HubCenterUnits); e.Kind == "portal" && dx <= sitePlacement.OutsideUnits {
			errs = append(errs, fmt.Sprintf("Portal innerhalb der Verteidigungszone (%s Units)", strconv.FormatFloat(dx, 'f', -1, 64)))
		}
	}
	return errs
}

// validateLair: Biome mit `lair` haben genau einen Bau, an der inneren Kante des Ausgangs-Chunks und mit dem Hub-Abstand
// eines Portals; alle anderen keinen.
func validateLair(l Layout, b Biome) []string {
	want := 0
	if b.Lair {
		want = 1
	}
	if n := countEntities(l, "lair"); n != want {
		return []string{fmt.Sprintf("%d Bau erwartet, %d gefunden", want, n)}
	}
	edge := l.ChunkWidthUnits
	if b.ExitSide == "right" {
		edge = l.WidthUnits - l.ChunkWidthUnits
	}
	for _, e := range l.Entities {
		if e.Kind == "lair" && (e.X != edge || math.Abs(e.X-l.HubCenterUnits) < b.Portals.MinDistanceFromHubUnits) {
			return []string{fmt.Sprintf("Bau nicht vor dem Ausgang (%s Units)", strconv.FormatFloat(e.X, 'f', -1, 64))}
		}
	}
	return nil
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

func countEntities(l Layout, kind string) int {
	n := 0
	for _, e := range l.Entities {
		if e.Kind == kind {
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
	if b.Veins.Count > 0 && countEntities(l, b.Veins.Kind) != b.Veins.Count {
		errs = append(errs, fmt.Sprintf("falsche Anzahl Adern: %d", countEntities(l, b.Veins.Kind)))
	}
	return errs
}

// fixedVeins zählt die festen Adern der geplanten Chunks.
func fixedVeins(b Biome, plan []PlannedChunk, cw float64) int {
	n := 0
	for _, c := range plan {
		for _, o := range fixedObjects(c, cw) {
			if o.Kind == b.Veins.Kind {
				n++
			}
		}
	}
	return n
}

// validateVeins prüft die Zahl der Adern (B-114, B-891): höchstens das Ziel (Natur-Wert, mindestens die festen Adern
// geplanter Chunks) und mindestens das Ziel oder die Zahl des Bioms, je nachdem was kleiner ist. Mehr Adern als im
// Biom kann der Generator nur setzen, solange freie Chunks bleiben; die Zahl des Bioms hat immer Platz (checkRoom).
func validateVeins(l Layout, b Biome, o Options) []string {
	if b.Veins.Count == 0 {
		return nil
	}
	fixed := fixedVeins(b, o.Planned, l.ChunkWidthUnits)
	target := max(b.withNature(l.Nature).Veins.Count, fixed)
	floor := min(target, max(b.Veins.Count, fixed))
	if n := countEntities(l, b.Veins.Kind); n > target || n < floor {
		return []string{fmt.Sprintf("falsche Anzahl Adern: %d", n)}
	}
	return nil
}
