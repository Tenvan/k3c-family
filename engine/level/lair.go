package level

import (
	"fmt"
	"math"
)

// CheckLairFit prüft, ob das Biom b als tiefste Stufe einer Insel den Bau des Endbosses samt Camp-Abstand trägt
// (docs/rules/bosse.md § 1, B-981): Mindestens so viele Chunks wie Camps (EventChunks.recruitCamp.min) liegen außerhalb
// der Hub-Chunks und der Randchunks nicht näher als lairCampGapUnits am Bau. chunkCount ist die Inselgröße der Stufe
// (0 = kürzeste Länge des Bioms). Unabhängig vom Seed; nichts wird repariert, die Meldung nennt den Grund.
func CheckLairFit(b Biome, chunkCount int) error {
	cw := b.ChunkWidthUnits
	count := chunkCount
	if count == 0 {
		count = int(math.Round(float64(b.LengthUnits.Min) / cw))
		count += count % 2 // gerade Anzahl wie frame()
	}
	if count < 4 {
		return fmt.Errorf("biom %s: %d Chunks sind zu wenig für den Bau des Endbosses", b.ID, count)
	}
	hubChunks := hubChunksOf(b)
	hubStart := count/2 - hubChunks/2
	lair := cw
	if b.ExitSide == "right" {
		lair = float64(count-1) * cw
	}
	safe := 0
	for i := 1; i < count-1; i++ {
		if i >= hubStart && i < hubStart+hubChunks {
			continue
		}
		if math.Abs(float64(i)*cw+cw/2-lair) >= lairCampGapUnits {
			safe++
		}
	}
	if want := b.EventChunks["recruitCamp"].Min; safe < want {
		return fmt.Errorf("biom %s: nur %d Chunks mit %d Units Abstand zum Bau, %d Camp(s) brauchen Platz (Inselgröße %d Chunks)", b.ID, safe, lairCampGapUnits, want, count)
	}
	return nil
}
