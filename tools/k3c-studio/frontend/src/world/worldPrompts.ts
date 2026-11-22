import type { WorldDetail } from '../api';

// Allgemeine Prompts des Welten-Editors (B-1216): Teile der Welt von Claude ändern lassen. Jeder Prompt nennt Welt, Insel,
// den Ist-Zustand, die Regeln und den Weg über die MCP-Tools (world_set, world_chunks_set, world_override_set, …); die
// Welt-Datei ändert niemand von Hand. Die Texte stehen hier, die Oberfläche (WorldPrompts) wählt und füllt sie.

export type PromptId = 'island-add' | 'island-size' | 'island-biomes' | 'island-scaling' | 'island-chunks' | 'island-nature' | 'island-etappe' | 'value' | 'world-copy' | 'world-notes';

export interface PromptSpec {
  id: PromptId;
  icon: string;
  name: string;
  /** braucht die Wahl einer Insel */
  island: boolean;
  /** Platzhalter der Eingabe „Was soll sich ändern?“ */
  hint: string;
}

export const PROMPTS: readonly PromptSpec[] = [
  { id: 'island-add', icon: '➕', name: 'Insel hinzufügen', island: false, hint: 'z. B. Insel 6 nach der Insel 5, sehr groß, mit Hafen' },
  { id: 'island-size', icon: '📏', name: 'Inselgröße ändern', island: true, hint: 'z. B. mittel (40 Chunks) in allen Tiefen' },
  { id: 'island-biomes', icon: '🌲', name: 'Tiefen und Biome ändern', island: true, hint: 'z. B. nur Wald und Höhle, drei Stufen' },
  { id: 'island-scaling', icon: '📈', name: 'Schwierigkeit der Insel (Skalierung)', island: true, hint: 'z. B. HP 1.6, Schaden 1.4, Tempo 1.1' },
  { id: 'island-chunks', icon: '🧩', name: 'Geplante Chunks ändern', island: true, hint: 'z. B. Truhe in Tiefe 2 auf Platz 4, Lichtung vor dem Portal' },
  { id: 'island-nature', icon: '🌿', name: 'Natur-Werte ändern', island: true, hint: 'z. B. mehr Bäume im Wald, weniger Felsen' },
  { id: 'island-etappe', icon: '🧭', name: 'Etappe und Reihenfolge ändern', island: true, hint: 'z. B. Insel 3 und 4 in einer Etappe, beliebig anfahrbar' },
  { id: 'value', icon: '🔧', name: 'Wert dieser Welt ändern (Override)', island: false, hint: 'z. B. Mauer kostet 5 Gold mehr, Start-Handwerker 2' },
  { id: 'world-copy', icon: '📄', name: 'Neue Welt aus dieser ableiten', island: false, hint: 'z. B. Welt „wueste“ mit Wüsten-Biomen und teurerer Mauer' },
  { id: 'world-notes', icon: '📝', name: 'Name und Notizen ändern', island: false, hint: 'z. B. Name „Familienwelt“, Notiz zur Reihenfolge' },
];

const TOOLS = {
  'island-add': 'world_set {id, patch: {islands: [{id: <vorherige Insel>, plannedChunks: {"0": [{slot: -7, kind: harbor}]}}, {id: <neue Insel>, name, depths oder biomes, chunkCounts}]}}; die Insel davor braucht in Tiefe 0 einen Hafen',
  'island-size': 'world_set {id, patch: {islands: [{id: <insel>, chunkCounts: {"<tiefe>": <16|24|40|64|100|160>}}]}} oder world_chunks_set; erlaubte Größen stehen in data/island-sizes.json, ein geplanter Chunk-Platz muss in die Größe passen',
  'island-biomes': 'world_set {id, patch: {islands: [{id: <insel>, biomes: [forest, cave, mine, ironhold, crystal]}]}} (oder depths)',
  'island-scaling': 'world_set {id, patch: {islands: [{id: <insel>, depthScaling: {hp, damage, speed}}]}}',
  'island-chunks': 'world_chunks_set {id, island, depth, chunks: [...]} nach docs/rules/geplante-chunks.md; jede Insel außer der letzten braucht in Tiefe 0 einen Hafen',
  'island-nature': 'world_set {id, patch: {islands: [{id: <insel>, nature oder natureByDepth}]}} nach docs/rules/welten.md § 2.1',
  'island-etappe': 'world_set {id, patch: {islands: [{id: <insel>, etappe: <n>}]}}; Etappen sind lückenlos ab 1 und nicht fallend (docs/rules/stufen.md)',
  value: 'world_override_set {id, file, path, value} (null entfernt den Override); Kind-Welten erben die Änderung',
  'world-copy': 'world_create {id, name, extends: "<diese Welt>"} und danach world_set für die Inseln und world_override_set für Werte',
  'world-notes': 'world_set {id, patch: {name, notes}}',
} as const satisfies Record<PromptId, string>;

/** Zusammenfassung der Welt für den Prompt: Inseln mit Größen, Biomen und Etappe. */
function summary(detail: WorldDetail): string {
  return detail.islands.map((i, n) => {
    const sizes = [...new Set(Object.values(i.chunkCounts ?? {}))];
    return `${i.id} („${i.name}“, Etappe ${i.etappe ?? n + 1}, Tiefen ${i.depths.join('/')}, Biome ${i.biomes.join('/')}${sizes.length > 0 ? `, Größe ${sizes.join('/')}` : ''})`;
  }).join('; ');
}

/** Auftrag an Claude: Ziel, Ist-Zustand der Welt (und der Insel), Werkzeug, Regeln und die Eingabe des Nutzers. */
export function worldPrompt(detail: WorldDetail, id: PromptId, island: string, wish: string): string {
  const spec = PROMPTS.find((p) => p.id === id)!;
  const isl = spec.island ? detail.islands.find((i) => i.id === island) : undefined;
  return [
    `Welten-Editor, Welt „${detail.name}“ (Welt-ID ${detail.id}, data/worlds/${detail.id}.json): ${spec.name}${isl ? `, Insel ${isl.id} („${isl.name}“)` : ''}.`,
    `Ist: ${detail.islands.length} Inseln: ${summary(detail) || 'keine'}.`,
    isl ? `Insel ${isl.id} jetzt: ${JSON.stringify({ depths: isl.depths, biomes: isl.biomes, chunkCounts: isl.chunkCounts, etappe: isl.etappe, plannedChunks: isl.plannedChunks })}` : '',
    `Lies zuerst world_get {id: "${detail.id}"}. Ändere nur über die MCP-Tools, nicht von Hand: ${TOOLS[id]}. Prüfe danach mit world_check {id: "${detail.id}"}.`,
    'Regeln: docs/rules/welten.md, stufen.md und geplante-chunks.md; die Geschichte der Welt bleibt gültig (Inseln hinzufügen heißt, ihre Zeilen später ergänzen).',
    `Was soll sich ändern: ${wish.trim()}`,
  ].filter(Boolean).join('\n');
}
