// Emoji und deutscher Name je Chunk-Art und Objekt-Art (B-485/AC-01, WZ17.4) und die Reihenfolge der durchsuchbaren Liste.
// Reine Zuordnung, kein DOM. Der Wert (`kind` in den Daten) bleibt technisch, angezeigt wird der Name. Namen nach
// docs/glossar.md, wo es den Begriff gibt. Emojis aus einem Codepunkt (kein U+FE0F), damit sie überall gleich breit erscheinen.

/** Emoji für Arten ohne eigenen Eintrag */
export const FALLBACK_ICON = '🧩';

const KINDS: Readonly<Record<string, readonly [icon: string, name: string]>> = {
  // Chunk-Arten
  forest: ['🌲', 'Wald'], clearing: ['🌼', 'Lichtung'], rocks: ['🪨', 'Felsen'], stream: ['💧', 'Bach'],
  tunnel: ['🚇', 'Tunnel'], cavern: ['🦇', 'Gewölbe'], crystals: ['💎', 'Kristalle'], chasm: ['🧗', 'Schlucht'],
  shaft: ['🪜', 'Schacht'], rails: ['🚃', 'Gleise'], oreVein: ['🟫', 'Erzader'], gallery: ['🔦', 'Stollengang'],
  forge: ['🔨', 'Esse'], lava: ['🌋', 'Lava'], grotto: ['🐚', 'Grotte'], geode: ['🔮', 'Geode'],
  portal: ['🌀', 'Portal'], chest: ['💰', 'Truhe'], recruitCamp: ['⛺', 'Camp'], hub: ['🏰', 'Hub'], edge: ['🚧', 'Rand'],
  // Objekt-Arten (portal, chest, recruitCamp wie oben)
  tree: ['🌳', 'Baum'], bush: ['🌿', 'Busch'], rock: ['🪨', 'Fels'], copperOre: ['🟠', 'Kupfererz'],
  stoneVein: ['🧱', 'Stein-Ader'], copperVein: ['🟧', 'Kupfer-Ader'], ironVein: ['🔩', 'Eisen-Ader'],
  crystalVein: ['🔷', 'Kristall-Ader'], exit: ['🚪', 'Ausgang'],
  // Gebäude-Arten der Ruinen (data/ruins.json › kinds, WZ14.7)
  workshop: ['🏹', 'Werkstatt'], storage: ['📦', 'Lager'], farm: ['🌾', 'Farm'], barracks: ['🛡', 'Kaserne'],
  tavern: ['🍺', 'Taverne'], healer: ['💚', 'Heilplatz'], smithy: ['⚒', 'Schmiede'], armory: ['🪖', 'Rüstkammer'],
};

const entry = (kind: string) => (Object.hasOwn(KINDS, kind) ? KINDS[kind] : undefined);

export const iconOf = (kind: string): string => entry(kind)?.[0] ?? FALLBACK_ICON;
/** Deutscher Name; unbekannte Arten zeigen ihren Wert */
export const nameOf = (kind: string): string => entry(kind)?.[1] ?? kind;

/** Wert zu einer Eingabe: der deutsche Name einer der `options` (ohne Groß- und Kleinschreibung) wird zu dieser Art,
 *  alles andere bleibt, wie es ist (Objekt-Namen werden in der Chunk-Liste also nicht zur Objekt-Art). */
export function resolveKind(text: string, options: readonly string[]): string {
  const t = text.trim().toLowerCase();
  return options.find((k) => nameOf(k).toLowerCase() === t) ?? text;
}

/** Eintrag der Liste: `match` = passt zum Suchtext (sonst abgeblendet) */
export interface RankedOption {
  value: string;
  match: boolean;
}

/** Alle Einträge, Treffer oben: erst die, deren Name oder Wert mit dem Suchtext beginnt, dann die, die ihn enthalten,
 *  dann der Rest (abgeblendet). Innerhalb jeder Gruppe bleibt die feste Reihenfolge; Groß- und Kleinschreibung zählt nicht. */
export function rankOptions(values: readonly string[], query: string): RankedOption[] {
  const q = query.trim().toLowerCase();
  const rank = (v: string) => {
    if (q === '') return 0;
    const words = [v.toLowerCase(), nameOf(v).toLowerCase()];
    return words.some((w) => w.startsWith(q)) ? 0 : words.some((w) => w.includes(q)) ? 1 : 2;
  };
  return values
    .map((value, i) => ({ value, i, r: rank(value) }))
    .sort((a, b) => a.r - b.r || a.i - b.i)
    .map(({ value, r }) => ({ value, match: r < 2 }));
}
