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

// Grundgerüst für Welt-Aufträge im Chat (B-488, WZ19.3; als feste Instruktion B-516, WZ21.2): Der Text entsteht aus
// denselben Daten wie die Prüfung des Ladens (Biome, Inselgrößen, Standard-Welt, Bosse über eine Probe mit
// sim.ParseWorld) und geht als System-Instruktion an die CLI; 🧑 schreibt im Chat nur den Wunsch. Die Antwort prüft
// PlanWorldAnswer.

// promptBiome ist ein Biom für den Prompt: die Tiefe, ihr Name und die Arten, die ein geplanter Chunk dort haben darf.
type promptBiome struct {
	depth        int
	name         string
	chunkKinds   []string
	objectKinds  []string
	endBossHere  bool
}

// worldInstruction liefert das Grundgerüst; es schreibt nichts.
func worldInstruction() string {
	var b strings.Builder
	b.WriteString("Du entwirfst eine Welt für das Spiel „Family Three Crowns“ als Welt-Datei im JSON-Format. ")
	b.WriteString("Antworte mit der vollständigen Datei als JSON in einem Code-Block, ohne Kommentare im JSON.\n\n")
	b.WriteString("## Änderungsmodus (Schritt für Schritt)\n\n")
	b.WriteString("Du arbeitest an der aktuellen Welt (Abschnitt „Aktuelle Welt“ unten, mit Prüfergebnis). Sie ist die Wahrheit, nicht ")
	b.WriteString("der Chat-Verlauf und nicht die Beispiele: Gib immer die vollständige Datei zurück, übernimm alles Bestehende unverändert ")
	b.WriteString("(Inseln, Namen, Tiefen, Skalierung, `plannedChunks`, `chunkCounts`) und ändere nur, was verlangt ist. ")
	b.WriteString("Meldet die Prüfung Fehler, behebe sie mit, ohne anderes zu ändern. Ist ein Wunsch nach den Regeln unmöglich, ")
	b.WriteString("setze die nächste gültige Variante um und nenne die Abweichung in einem Satz vor dem Code-Block.\n\n")
	b.WriteString("## Format\n\n")
	b.WriteString("Eine Welt hat `id`, `name`, optional `notes` und `islands` (Inseln in Spielreihenfolge, mindestens eine). ")
	b.WriteString("Eine Insel hat `id`, `name`, `depths` (Tiefen der Stufen), `depthScaling` (`hp`, `damage`, `speed`, alle > 0: ")
	b.WriteString("Basis der Potenz je Tiefe) und optional `plannedChunks` und `chunkCounts`. ")
	b.WriteString("IDs: nur Kleinbuchstaben, Ziffern und Bindestrich, höchstens 32 Zeichen, die Insel-ID ist in der Welt eindeutig. ")
	b.WriteString("Andere Felder sind verboten, fehlende Pflichtfelder ebenso.\n\n")
	b.WriteString("Beispiel 1 (die Standard-Welt; nur das Format, keine Vorlage für Zahl der Inseln oder Stufen):\n\n```json\n")
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
	b.WriteString(islandRulesText(biomes))
	b.WriteString("## Geplante Chunks und Inselgrößen (Hafen Pflicht, sonst optional)\n\n")
	b.WriteString(plannedText(biomes))
	b.WriteString("\n`chunkCounts` ordnet einer Tiefe (Schlüssel als Text, z. B. `\"0\"`) eine Inselgröße zu, erlaubt sind nur diese Chunk-Anzahlen:\n\n")
	for _, s := range sim.IslandSizes() {
		fmt.Fprintf(&b, "- %s: %d\n", s.Name, s.ChunkCount)
	}
	b.WriteString("\n## Prüfregeln\n\n")
	b.WriteString("Die Antwort wird streng wie beim Laden einer Welt geprüft: gültiges JSON, keine unbekannten Felder, alle Pflichtfelder, ")
	b.WriteString("bekannte Tiefen, Boss-Regel oben, Hafen-Regel, Faktoren > 0, Chunk-Anzahlen nur aus der Tabelle, geplante Chunks nur mit Arten ihres Bioms. ")
	b.WriteString("Bei einem Fehler nennt das Studio dir alle Gründe mit Insel und Tiefe und lässt dich korrigieren; ")
	b.WriteString("nur einen fehlenden Hafen ergänzt es selbst. Gespeichert wird nichts ohne Ralf.\n")
	return b.String()
}

// islandRulesText nennt die Regeln für Stufen je Insel aus den Daten: Endboss-Tiefen und die Hafen-Regel
// (geplante-chunks.md § 10), mit einem zweiten Beispiel mit Inseln unterschiedlicher Stufenzahl.
func islandRulesText(biomes []promptBiome) string {
	var ends []string
	deepest := -1
	for _, p := range biomes {
		if p.endBossHere {
			ends = append(ends, fmt.Sprint(p.depth))
			deepest = max(deepest, p.depth)
		}
	}
	slot, _ := harborSlot(nil, 0)
	var b strings.Builder
	b.WriteString("## Stufen je Insel, Hafen\n\n")
	fmt.Fprintf(&b, "- Die tiefste Tiefe jeder Insel muss eine Endboss-Tiefe sein: %s. Jede Insel enthält also Tiefe %d.\n", strings.Join(ends, ", "), deepest)
	fmt.Fprintf(&b, "- Hafen (Pflicht): Jede Insel außer der letzten hat Tiefe %d und plant dort genau einen Chunk der Art `%s`, ", harborDepth, level.HarborKind)
	fmt.Fprintf(&b, "Standard ist Platz %d: `\"plannedChunks\": { \"%d\": [ { \"slot\": %d, \"kind\": \"%s\" } ] }`. ", slot, harborDepth, slot, level.HarborKind)
	b.WriteString("Ohne Hafen gibt es keinen Inselwechsel; andere Tiefen haben keinen Hafen.\n")
	fmt.Fprintf(&b, "- Darum: Eine Insel mit nur einer Stufe (`[%d]`) geht nur als letzte Insel; jede andere hat mindestens die Stufen `[%d, %d]`. ", deepest, harborDepth, deepest)
	b.WriteString("„Eine Stufe mehr je Insel“ heißt also z. B. 2, 3, 4 … Stufen, Lücken sind erlaubt.\n\n")
	b.WriteString("Zweites Beispiel (Ausschnitt, Inseln mit unterschiedlicher Stufenzahl):\n\n```json\n")
	b.WriteString(stepsExample(deepest, slot))
	b.WriteString("\n```\n\n")
	return b.String()
}

// stepsExample sind drei Inseln mit 2, 3 und 4 Stufen, jede mit Hafen (Pflicht für alle außer der letzten).
func stepsExample(deepest, slot int) string {
	var lines []string
	for n := 1; n <= 3; n++ {
		depths := []int{}
		for d := range n {
			depths = append(depths, d)
		}
		depths = append(depths, deepest)
		lines = append(lines, fmt.Sprintf(`    { "id": "insel%d", "name": "Insel %d", "depths": %s, "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 }, "plannedChunks": { "%d": [ { "slot": %d, "kind": %q } ] } }`,
			n, n, depthsText(depths), harborDepth, slot, level.HarborKind))
	}
	return "\"islands\": [\n" + strings.Join(lines, ",\n") + "\n  ]"
}

// exampleWorld ist die Standard-Welt als Beispiel-Datei (ID „meine-welt“), aus den eingebetteten Daten.
func exampleWorld() string {
	w := sim.StandardWorld()
	var islands []string
	for _, i := range w.Islands {
		planned := ""
		if len(i.PlannedChunks) > 0 {
			planned = `, "plannedChunks": ` + string(jsonText(i.PlannedChunks))
		}
		islands = append(islands, fmt.Sprintf(`    { "id": %q, "name": %q, "depths": %s, "depthScaling": %s%s }`,
			i.ID, i.Name, depthsText(i.Depths), scalingText(DraftScaling{HP: i.DepthScaling.HP, Damage: i.DepthScaling.Damage, Speed: i.DepthScaling.Speed}), planned))
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
