package backend

import (
	"cmp"
	"fmt"
	"slices"
	"strings"

	"k3c/data"
	"k3c/engine/level"
	"k3c/engine/sim"
)

// Prompt-Grundgerüst für den Chat (B-488, WZ19.3): Der Text entsteht aus denselben Daten wie die Prüfung des Ladens
// (Biome, Inselgrößen, Standard-Welt, Bosse über eine Probe mit sim.ParseWorld) und endet mit „MEINE WÜNSCHE:“, den
// 🧑 im Chat ergänzt. Die Antwort prüft PlanWorldAnswer.

const promptWishes = "MEINE WÜNSCHE:"

// promptBiome ist ein Biom für den Prompt: die Tiefe, ihr Name und die Arten, die ein geplanter Chunk dort haben darf.
type promptBiome struct {
	depth        int
	name         string
	chunkKinds   []string
	objectKinds  []string
	endBossHere  bool
}

// PlanWorldPrompt liefert das Prompt-Grundgerüst (Binding); es schreibt nichts.
func (a *App) PlanWorldPrompt() string {
	var b strings.Builder
	b.WriteString("Du entwirfst eine Welt für das Spiel „Family Three Crowns“ als Welt-Datei im JSON-Format. ")
	b.WriteString("Antworte nur mit der Datei als JSON (gern in einem Code-Block), ohne weiteren Text und ohne Kommentare.\n\n")
	b.WriteString("## Format\n\n")
	b.WriteString("Eine Welt hat `id`, `name`, optional `notes` und `islands` (Inseln in Spielreihenfolge, mindestens eine). ")
	b.WriteString("Eine Insel hat `id`, `name`, `depths` (Tiefen der Stufen), `depthScaling` (`hp`, `damage`, `speed`, alle > 0: ")
	b.WriteString("Basis der Potenz je Tiefe) und optional `plannedChunks` und `chunkCounts`. ")
	b.WriteString("IDs: nur Kleinbuchstaben, Ziffern und Bindestrich, höchstens 32 Zeichen, die Insel-ID ist in der Welt eindeutig. ")
	b.WriteString("Andere Felder sind verboten, fehlende Pflichtfelder ebenso.\n\nBeispiel (die Standard-Welt):\n\n```json\n")
	b.WriteString(exampleWorld())
	b.WriteString("\n```\n\n## Tiefen und Biome\n\n")
	biomes := promptBiomes()
	for _, p := range biomes {
		fmt.Fprintf(&b, "- Tiefe %d: %s", p.depth, p.name)
		if p.endBossHere {
			b.WriteString(" (hier darf eine Insel enden: Endboss)")
		}
		b.WriteString("\n")
	}
	b.WriteString("\nJede Tiefe einer Insel kommt höchstens einmal vor, Lücken sind erlaubt (z. B. `[0, 4]`). ")
	b.WriteString("Jede Tiefe hat einen Miniboss; die tiefste Tiefe einer Insel braucht einen Endboss, sonst wird die Insel abgelehnt ")
	b.WriteString("(Beispiel der Meldung: „Insel insel3: kein Endboss in der tiefsten Stufe 2“).\n\n")
	b.WriteString("## Geplante Chunks und Inselgrößen (optional, im Zweifel weglassen)\n\n")
	b.WriteString(plannedText(biomes))
	b.WriteString("\n`chunkCounts` ordnet einer Tiefe (Schlüssel als Text, z. B. `\"0\"`) eine Inselgröße zu, erlaubt sind nur diese Chunk-Anzahlen:\n\n")
	for _, s := range sim.IslandSizes() {
		fmt.Fprintf(&b, "- %s: %d\n", s.Name, s.ChunkCount)
	}
	b.WriteString("\n## Prüfregeln\n\n")
	b.WriteString("Die Antwort wird streng wie beim Laden einer Welt geprüft: gültiges JSON, keine unbekannten Felder, alle Pflichtfelder, ")
	b.WriteString("bekannte Tiefen, Boss-Regel oben, Faktoren > 0, Chunk-Anzahlen nur aus der Tabelle, geplante Chunks nur mit Arten ihres Bioms. ")
	b.WriteString("Bei einem Fehler wird der Grund mit Insel und Tiefe genannt; nichts wird repariert oder gespeichert.\n\n")
	b.WriteString(promptWishes + "\n")
	return b.String()
}

// exampleWorld ist die Standard-Welt als Beispiel-Datei (ID „meine-welt“), aus den eingebetteten Daten.
func exampleWorld() string {
	w := sim.StandardWorld()
	var islands []string
	for _, i := range w.Islands {
		islands = append(islands, fmt.Sprintf(`    { "id": %q, "name": %q, "depths": %s, "depthScaling": %s }`,
			i.ID, i.Name, depthsText(i.Depths), scalingText(DraftScaling{HP: i.DepthScaling.HP, Damage: i.DepthScaling.Damage, Speed: i.DepthScaling.Speed})))
	}
	return fmt.Sprintf("{\n  \"id\": \"meine-welt\",\n  \"name\": \"Meine Welt\",\n  \"notes\": \"\",\n  \"islands\": [\n%s\n  ]\n}", strings.Join(islands, ",\n"))
}

// promptBiomes liest die Biome (aufsteigend nach Tiefe) und fragt die Prüfung des Ladens, wo ein Endboss sitzt.
func promptBiomes() []promptBiome {
	entries, _ := data.Files.ReadDir("biomes")
	var out []promptBiome
	for _, e := range entries {
		id, ok := strings.CutSuffix(e.Name(), ".json")
		if !ok {
			continue
		}
		b, err := level.LoadBiome(id)
		if err != nil {
			continue
		}
		p := promptBiome{depth: b.Depth, name: b.Name, chunkKinds: keysOf(b.ChunkWeights)}
		for _, kind := range b.ResourcesPerChunk {
			p.objectKinds = append(p.objectKinds, keysOf(kind.Value)...)
		}
		if b.Veins.Count > 0 {
			p.objectKinds = append(p.objectKinds, b.Veins.Kind)
		}
		p.objectKinds = slices.Compact(slices.Sorted(slices.Values(p.objectKinds)))
		p.endBossHere = endBossAt(b.Depth)
		out = append(out, p)
	}
	slices.SortFunc(out, func(x, y promptBiome) int { return cmp.Compare(x.depth, y.depth) })
	return out
}

func keysOf[V any](o level.Ordered[V]) []string {
	var out []string
	for _, e := range o {
		out = append(out, e.Key)
	}
	return out
}

// endBossAt: Eine Insel, die in Tiefe d endet, besteht die Prüfung des Ladens (Probe, die Daten entscheiden).
func endBossAt(depth int) bool {
	probe := fmt.Sprintf(`{"id":"probe","name":"probe","islands":[{"id":"probe","name":"probe","depths":[%d],"depthScaling":{"hp":1,"damage":1,"speed":1}}]}`, depth)
	_, err := sim.ParseWorld([]byte(probe), "probe")
	return err == nil || !strings.Contains(err.Error(), "kein Endboss")
}

// plannedText beschreibt geplante Chunks mit den Arten je Biom.
func plannedText(biomes []promptBiome) string {
	var b strings.Builder
	b.WriteString("`plannedChunks` legt je Tiefe (Schlüssel als Text) feste Chunks fest: `{ \"0\": [ { \"slot\": -3, \"kind\": \"clearing\", \"seed\": \"lichtung\" } ] }`. ")
	b.WriteString("`slot` zählt vom Hub: −n ist der n-te Chunk links, +n rechts, 0 gibt es nicht; `seed` (optional) benennt den Inhalt; ")
	b.WriteString("`objects` (optional) sind Einträge `{ \"kind\": …, \"at\": … }`. Arten je Biom:\n\n")
	for _, p := range biomes {
		fmt.Fprintf(&b, "- Tiefe %d (%s): Chunks %s; Objekte %s\n", p.depth, p.name, strings.Join(p.chunkKinds, ", "), strings.Join(p.objectKinds, ", "))
	}
	b.WriteString("\nÜberall zusätzlich erlaubt: `portal`, `chest`, `recruitCamp`.\n")
	return b.String()
}
