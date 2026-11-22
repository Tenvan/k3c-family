import type { WorldDetail } from '../api';

// Prompts zu einzelnen Teilen der Geschichte einer Welt (B-1216, docs/rules/welten.md § 2.2). Geschrieben wird nur über
// das MCP-Tool world_story_set; die Welt-Datei ändert niemand von Hand.

/** Ein Teil der Geschichte: Start, Ende, Flaschenpost, Wortschatz, Etappe oder ein Eintrag einer Insel. */
export type StoryPart =
  | { kind: 'start' }
  | { kind: 'end' }
  | { kind: 'bottles' }
  | { kind: 'lexicon' }
  | { kind: 'etappe'; n: string }
  | { kind: 'islandAll'; island: string }
  | { kind: 'island'; island: string; entry: 'arrival' | 'boss' | 'done' };

export const ENTRY_NAMES = { arrival: 'Ankunft', boss: 'Endboss', done: 'Sieg' } as const;

const ENTRIES = ['arrival', 'boss', 'done'] as const;

/** Zeilen einer Insel-Geschichte (ein Eintrag oder alle drei). */
function islandLines(detail: WorldDetail, island: string, entry?: (typeof ENTRIES)[number]): string[] {
  const story = detail.islands.find((i) => i.id === island)?.story;
  return (entry ? [entry] : [...ENTRIES]).flatMap((e) => story?.[e] ?? []);
}

/** Zeilen der Welt-Teile (ohne Insel): Schlüssel der Geschichte → Zeilen. */
function worldLines(s: WorldDetail['story'], part: StoryPart): string[] {
  if (part.kind === 'etappe') return s?.etappen?.[part.n] ?? [];
  if (part.kind === 'bottles') return (s?.bottles ?? []).flat();
  const key = part.kind === 'start' || part.kind === 'end' || part.kind === 'lexicon' ? part.kind : null;
  return (key && s?.[key]) || [];
}

/** Zeilen eines Teils (leer, wenn es fehlt). */
export function linesOf(detail: WorldDetail, part: StoryPart): string[] {
  if (part.kind === 'island') return islandLines(detail, part.island, part.entry);
  if (part.kind === 'islandAll') return islandLines(detail, part.island);
  return worldLines(detail.story, part);
}

function label(detail: WorldDetail, part: StoryPart): string {
  switch (part.kind) {
    case 'start': return 'Start der Kampagne (story.start)';
    case 'end': return 'Ende der Kampagne (story.end)';
    case 'bottles': return 'Flaschenpost (story.bottles)';
    case 'lexicon': return 'Wortschatz der Kunstsprache (story.lexicon)';
    case 'etappe': return `Abschluss der Etappe ${part.n} (story.etappen["${part.n}"])`;
    case 'islandAll': {
      const name = detail.islands.find((i) => i.id === part.island)?.name ?? part.island;
      return `die ganze Geschichte der Insel ${part.island} („${name}“: Ankunft, Endboss und Sieg, islands.${part.island}.story)`;
    }
    case 'island': {
      const name = detail.islands.find((i) => i.id === part.island)?.name ?? part.island;
      return `${ENTRY_NAMES[part.entry]} der Insel ${part.island} („${name}“, islands.${part.island}.story.${part.entry})`;
    }
  }
}

/** Auftrag an Claude für genau einen Teil: Ist-Zustand, Regeln und der Weg über world_story_set. */
export function partPrompt(detail: WorldDetail, part: StoryPart): string {
  const now = linesOf(detail, part);
  return [
    `Welten-Editor, Geschichte der Welt „${detail.name}“ (Welt-ID ${detail.id}): ändere ${label(detail, part)}.`,
    now.length > 0 ? `Jetzt: ${now.map((l) => `„${l}“`).join(' / ')}` : 'Jetzt: fehlt.',
    'Gewässer: Der Untertitel (zweiter Text) einer Welt ist immer das Gewässer, an dem sie sich orientiert: Fluss, See, Küste oder Meer; Geschichte, Kunstwörter und Bilder der Welt beziehen sich darauf.',
    'Regeln nach docs/rules/welten.md § 2.2: Kunstsprache der Welt, je Zeile 2 bis 3 echte Wörter in GROSSBUCHSTABEN, jedes Kunstwort steht im Wortschatz (lexicon), 1 bis 2 Zeilen je Eintrag.',
    `Lies zuerst world_get {id: "${detail.id}"}, schreibe mit world_story_set (nur dieses Feld, neue Kunstwörter in lexicon ergänzen) und prüfe mit world_check {id: "${detail.id}"}. Ändere die Welt-Datei nicht von Hand.`,
    'Was soll sich ändern: ',
  ].join('\n');
}
