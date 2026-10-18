// Emoji je Chunk-Art und Objekt-Art (B-485/AC-01, WZ17.4) und die Reihenfolge der durchsuchbaren Liste. Reine Zuordnung,
// kein DOM; Emojis aus einem Codepunkt (kein U+FE0F), damit sie überall gleich breit erscheinen.

/** Emoji für Arten ohne eigenen Eintrag */
export const FALLBACK_ICON = '🧩';

const ICONS: Readonly<Record<string, string>> = {
  // Chunk-Arten
  forest: '🌲', clearing: '🌼', rocks: '🪨', stream: '💧', tunnel: '🚇', cavern: '🦇', crystals: '💎', chasm: '🧗',
  shaft: '🪜', rails: '🚃', oreVein: '🟫', gallery: '🔦', forge: '🔨', lava: '🌋', grotto: '🐚', geode: '🔮',
  portal: '🌀', chest: '💰', recruitCamp: '⛺', hub: '🏰', edge: '🚧',
  // Objekt-Arten (portal, chest, recruitCamp wie oben)
  tree: '🌳', bush: '🌿', rock: '🪨', copperOre: '🟠', stoneVein: '🧱', copperVein: '🟧', ironVein: '🔩', crystalVein: '🔷',
  exit: '🚪',
};

export const iconOf = (kind: string): string => (Object.hasOwn(ICONS, kind) ? ICONS[kind] : FALLBACK_ICON);

/** Eintrag der Liste: `match` = passt zum Suchtext (sonst abgeblendet) */
export interface RankedOption {
  value: string;
  match: boolean;
}

/** Alle Einträge, Treffer oben: erst die, die mit dem Suchtext beginnen, dann die, die ihn enthalten, dann der Rest
 *  (abgeblendet). Innerhalb jeder Gruppe bleibt die feste Reihenfolge; Groß- und Kleinschreibung zählt nicht. */
export function rankOptions(values: readonly string[], query: string): RankedOption[] {
  const q = query.trim().toLowerCase();
  const rank = (v: string) => (q === '' ? 0 : v.toLowerCase().startsWith(q) ? 0 : v.toLowerCase().includes(q) ? 1 : 2);
  return values
    .map((value, i) => ({ value, i, r: rank(value) }))
    .sort((a, b) => a.r - b.r || a.i - b.i)
    .map(({ value, r }) => ({ value, match: r < 2 }));
}
