// Block „Hub-Stufe → Gebäude“ der Regeln (`docs/rules/materialien-gebaeude.md` § 3) aus `data/hub.json`. Reine
// Funktionen ohne Dateizugriff. Die Zahl der Hub-Stufen kommt aus `levels`, es gibt keine feste Obergrenze.
// Gleiche Regeln wie die Go-Fassung `table/` (Studio); fehlende Werte sind 0 bzw. leer, falsche Typen ein Fehler.

/** START und END markieren den erzeugten Block im Regeltext; je genau einmal. */
export const START = '<!-- rules:hub-levels:start -->';
export const END = '<!-- rules:hub-levels:end -->';

interface Site {
  kind: string;
  offsetUnits: number;
  fromDepth: number;
  needsDeeper: boolean;
  hubLevel: number;
  /** aus `islandSites`: gilt je Insel, nicht je Hub */
  island: boolean;
}

type Cost = Map<string, number>;

interface Hub {
  /** Kosten je Hub-Stufe samt aller Ausbauschritte (`steps`, B-592); das Material-Tor (`gate`) zählt nicht. */
  levels: Cost[];
  sites: Site[];
  islandSites: Site[];
  placement: { wall: number; tower: number; zones: number };
  merchant: { buy: number; sell: number; depth: number };
}

type Json = Record<string, unknown>;

const bad = (field: string): never => {
  throw new Error(`hub.json: ${field} hat den falschen Typ`);
};

function value<T>(v: unknown, field: string, zero: T, ok: (v: unknown) => boolean): T {
  if (v === undefined || v === null) return zero;
  return ok(v) ? (v as T) : bad(field);
}

const isObject = (v: unknown): boolean => typeof v === 'object' && !Array.isArray(v);
const obj = (v: unknown, field: string): Json => value<Json>(v, field, {}, isObject);
const list = (v: unknown, field: string): unknown[] => value<unknown[]>(v, field, [], Array.isArray);
const num = (v: unknown, field: string): number => value(v, field, 0, (x) => typeof x === 'number');
const int = (v: unknown, field: string): number => value(v, field, 0, Number.isInteger);
const str = (v: unknown, field: string): string => value(v, field, '', (x) => typeof x === 'string');
const bool = (v: unknown, field: string): boolean => value(v, field, false, (x) => typeof x === 'boolean');

function addCost(into: Cost, cost: unknown): void {
  for (const [key, v] of Object.entries(obj(cost, 'cost'))) into.set(key, (into.get(key) ?? 0) + int(v, `cost.${key}`));
}

function level(v: unknown): Cost {
  const raw = obj(v, 'levels');
  const out: Cost = new Map();
  addCost(out, raw.cost);
  for (const step of list(raw.steps, 'steps')) addCost(out, obj(step, 'steps').cost);
  return out;
}

function site(v: unknown, island: boolean): Site {
  const raw = obj(v, 'sites');
  return {
    kind: str(raw.kind, 'kind'),
    offsetUnits: num(raw.offsetUnits, 'offsetUnits'),
    fromDepth: int(raw.fromDepth, 'fromDepth'),
    needsDeeper: bool(raw.needsDeeper, 'needsDeeper'),
    hubLevel: int(raw.hubLevel, 'hubLevel'),
    island,
  };
}

function parseHub(json: string): Hub {
  let parsed: unknown;
  try {
    parsed = JSON.parse(json);
  } catch (e) {
    throw new Error(`hub.json: ${e instanceof Error ? e.message : String(e)}`);
  }
  const raw = obj(parsed, 'Wurzel');
  const placement = obj(raw.sitePlacement, 'sitePlacement');
  const merchant = obj(raw.merchant, 'merchant');
  return {
    levels: list(raw.levels, 'levels').map(level),
    sites: list(raw.sites, 'sites').map((s) => site(s, false)),
    islandSites: list(raw.islandSites, 'islandSites').map((s) => site(s, true)),
    placement: {
      wall: num(placement.zone1WallUnits, 'zone1WallUnits'),
      tower: num(placement.zone1TowerUnits, 'zone1TowerUnits'),
      zones: int(placement.zones, 'zones'),
    },
    merchant: {
      buy: num(merchant.buyOffsetUnits, 'buyOffsetUnits'),
      sell: num(merchant.sellOffsetUnits, 'sellOffsetUnits'),
      depth: int(merchant.onlyDepth, 'onlyDepth'),
    },
  };
}

const KIND_NAMES = new Map(Object.entries({
  workshop: 'Werkstatt', bowyer: 'Bogner', stairsUp: 'Treppe hoch', stairsDown: 'Treppe runter', nursery: 'Baumschule',
  healer: 'Heilplatz', tavern: 'Taverne', barracks: 'Kaserne', smithy: 'Schmiede', armory: 'Rüstkammer', storage: 'Lager',
}));

const MATERIAL_NAMES = new Map(Object.entries({
  wood: 'Holz', stone: 'Stein', copper: 'Kupfer', iron: 'Eisen', crystal: 'Kristall',
}));

const signed = (n: number): string => (n < 0 ? `−${-n}` : `+${n}`);

// Wie in der Engine: ein fehlender Wert (0) heißt Stufe 1.
const hubLevelOf = (s: Site): number => Math.max(1, s.hubLevel);

// Plätze mit einer Stufe außerhalb von `levels` würden sonst still aus dem Block fallen.
function checkLevels(h: Hub): void {
  const wrong = [...h.sites, ...h.islandSites].find((s) => hubLevelOf(s) > h.levels.length);
  if (wrong) {
    throw new Error(`hub.json: platz ${wrong.kind} hat hubLevel ${wrong.hubLevel}, aber nur ${h.levels.length} levels`);
  }
}

// Das Baumaterial der Stufe: das eine Material der Ausbaukosten (Gold zählt nicht), ohne Kosten Holz. Mehr als ein
// Material oder ein unbekanntes ist ein Fehler, damit die Ausgabe eindeutig bleibt.
function material(stage: number, cost: Cost): string {
  const keys = [...cost].filter(([key, v]) => key !== 'gold' && v > 0).map(([key]) => key).sort();
  if (keys.length === 0) return 'Holz';
  if (keys.length > 1) throw new Error(`hub.json: levels[${stage - 1}] hat mehrere materialien [${keys.join(' ')}]`);
  const name = MATERIAL_NAMES.get(keys[0]);
  if (!name) throw new Error(`hub.json: levels[${stage - 1}] hat das unbekannte material ${JSON.stringify(keys[0])}`);
  return name;
}

function siteText(s: Site): string {
  let text = signed(s.offsetUnits);
  if (s.fromDepth > 0) text += ` (ab Tiefe ${s.fromDepth})`;
  if (s.needsDeeper) text += ' (nur mit tieferer Stufe)';
  if (s.island) text += ' (je Insel)';
  return text;
}

// Plätze der Stufe nach Art gruppiert in Reihenfolge des ersten Auftretens (erst `sites`, dann `islandSites`).
function places(h: Hub, stage: number): string {
  const byKind = new Map<string, string[]>();
  for (const s of [...h.sites, ...h.islandSites]) {
    if (hubLevelOf(s) !== stage) continue;
    byKind.set(s.kind, [...(byKind.get(s.kind) ?? []), siteText(s)]);
  }
  const parts = [...byKind].map(([kind, offsets]) => `${KIND_NAMES.get(kind) ?? kind} ${offsets.join(', ')}`);
  return parts.length === 0 ? '–' : parts.join('; ');
}

// Mauer- und Turm-Plätze der Stufe (B-652): Zone n ist ab Hub-Stufe n sichtbar; Zone 1 hat den festen Baugrund
// (Mauer, Turm), alle weiteren Plätze der Zone würfelt der Generator.
function line(h: Hub, stage: number): string {
  const p = h.placement;
  if (stage > p.zones) return '–';
  if (stage === 1) return `Zone 1: Baugrund Mauer ±${p.wall}, Turm ±${p.tower}, dazu 0…2 Plätze je Seed`;
  return `Zone ${stage}: 1…3 Mauern, 1…3 Türme je Seed`;
}

// Der Händler hängt in der hub.json an keiner Stufe und ist von Anfang an da.
function merchant(h: Hub, stage: number): string {
  if (stage !== 1) return '–';
  const m = h.merchant;
  return `Kaufen ${signed(m.buy)}, Verkaufen ${signed(m.sell)} (nur Tiefe ${m.depth})`;
}

/** block liefert den Block samt Marken (LF-Zeilenenden, ohne Schlusszeilenumbruch) für alle `levels` der hub.json. */
export function block(hubJson: string): string {
  const h = parseHub(hubJson);
  if (h.levels.length === 0) throw new Error('hub.json: keine levels');
  checkLevels(h);
  const rows = [START, '| Hub-Stufe | Material | Hub-Plätze | Mauer- und Turm-Zone | Händler |', '|---|---|---|---|---|'];
  h.levels.forEach((cost, i) => {
    const stage = i + 1;
    const cells = [String(stage), material(stage, cost), places(h, stage), line(h, stage), merchant(h, stage)];
    rows.push(`| ${cells.join(' | ')} |`);
  });
  return [...rows, END].join('\n');
}

const count = (text: string, part: string): number => text.split(part).length - 1;

// Anfang der Start-Marke und Ende der End-Marke.
function span(doc: string): [from: number, to: number] {
  if (count(doc, START) !== 1 || count(doc, END) !== 1) {
    throw new Error(`marken ${JSON.stringify(START)} und ${JSON.stringify(END)} müssen je genau einmal vorkommen`);
  }
  const from = doc.indexOf(START);
  const to = doc.indexOf(END);
  if (to < from) throw new Error('ende-marke steht vor der Start-Marke');
  return [from, to + END.length];
}

/**
 * replaceBlock tauscht den Text von der Start- bis zur End-Marke (beide inklusive) gegen `next`; der Rest bleibt
 * byte-gleich, die Zeilenenden des Blocks folgen dem Dokument. Fehlt eine Marke, kommt sie doppelt vor oder steht das
 * Ende vor dem Start, ist das ein Fehler.
 */
export function replaceBlock(doc: string, next: string): string {
  const [from, to] = span(doc);
  let text = next.replaceAll('\r\n', '\n');
  if (doc.includes('\r\n')) text = text.replaceAll('\n', '\r\n');
  return doc.slice(0, from) + text + doc.slice(to);
}

/** compareBlock vergleicht den Block im Dokument mit dem aus hubJson erzeugten (Zeilenenden egal). */
export function compareBlock(doc: string, hubJson: string): { same: boolean; actual: string; wanted: string } {
  const wanted = block(hubJson);
  const [from, to] = span(doc);
  const actual = doc.slice(from, to).replaceAll('\r\n', '\n');
  return { same: actual === wanted, actual, wanted };
}
