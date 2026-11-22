// Emoji und deutscher Name je Chunk-Art und Objekt-Art (B-879, Port aus dem Asset-Studio, world/kindIcons.ts). Reine
// Zuordnung, kein DOM. Der Wert (`kind` in den Daten) bleibt technisch, angezeigt wird der Name (Glossar, wo es den
// Begriff gibt). Emojis aus einem Codepunkt (kein U+FE0F), damit sie überall gleich breit erscheinen.

/** Emoji für Arten ohne eigenen Eintrag */
export const FALLBACK_ICON = '🧩';

const KINDS: Readonly<Record<string, readonly [icon: string, name: string]>> = {
  // Chunk-Arten
  forest: ['🌲', 'Wald'], clearing: ['🌼', 'Lichtung'], rocks: ['🪨', 'Felsen'], stream: ['💧', 'Bach'],
  tunnel: ['🚇', 'Tunnel'], cavern: ['🦇', 'Gewölbe'], crystals: ['💎', 'Kristalle'], chasm: ['🧗', 'Schlucht'],
  shaft: ['🪜', 'Schacht'], rails: ['🚃', 'Gleise'], oreVein: ['🟫', 'Erzader'], gallery: ['🔦', 'Stollengang'],
  forge: ['🔨', 'Esse'], lava: ['🌋', 'Lava'], grotto: ['🐚', 'Grotte'], geode: ['🔮', 'Geode'],
  portal: ['🌀', 'Portal'], chest: ['💰', 'Truhe'], recruitCamp: ['⛺', 'Camp'], hub: ['🏰', 'Hub'], edge: ['🚧', 'Rand'],
  harbor: ['⚓', 'Hafen'],
  // Objekt-Arten (portal, chest, recruitCamp wie oben)
  castle: ['🏯', 'Burg'], tree: ['🌳', 'Baum'], bush: ['🌿', 'Busch'], rock: ['🪨', 'Fels'], copperOre: ['🟠', 'Kupfererz'],
  stoneVein: ['🧱', 'Stein-Ader'], copperVein: ['🟧', 'Kupfer-Ader'], ironVein: ['🔩', 'Eisen-Ader'],
  crystalVein: ['🔷', 'Kristall-Ader'], exit: ['🚪', 'Ausgang'], skillPoint: ['🔹', 'Skill-Punkt'],
};

const entry = (kind: string) => (Object.hasOwn(KINDS, kind) ? KINDS[kind] : undefined);

export const iconOf = (kind: string): string => entry(kind)?.[0] ?? FALLBACK_ICON;
/** Deutscher Name; unbekannte Arten zeigen ihren Wert */
export const nameOf = (kind: string): string => entry(kind)?.[1] ?? kind;
