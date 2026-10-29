import { MINE_SYMBOL_TEXTURES } from './mineSignView';
import type { ObjectSprite, WorldFile } from './worldSprites';

/**
 * Natur und Objekte aus `tools/buildings` (GEB6, GEB10): Bäume, Höhlen-Flora, Busch, Fels, Erzbrocken, Adern, Truhe,
 * Portal, Ausgang und Rekrutierungslager, alle 32 px je Kachel (Maßstab 1). Reine Auswahl ohne Phaser und ohne Zufall:
 * Form aus der Position, Look aus dem Biom (Regeldatei `docs/standards/gebaeude-grafik.md` § 5, § 6).
 */

const ARTEN = ['tanne', 'pappel', 'birke'] as const;
/** Höhlen-Flora statt Bäumen außerhalb des Walds (GEB10): fünf Wurzel-Formen (GEB15.1: `wurzel-4` kurz und buschig, `wurzel-5` lang und dünn) */
export const WURZELN = ['wurzel', 'wurzel-2', 'wurzel-3', 'wurzel-4', 'wurzel-5'] as const;
/** Fels-Formen, je mit den Looks 1–3 (GEB15.2, B-583): Haufen, hoher Findling, flacher Steinhaufen */
export const FELSEN = ['felsen', 'felsen-b', 'felsen-c'] as const;
/** Adern in der Reihenfolge ihrer Stufen (Stufe = Material, nicht Biom) */
const ADERN = ['stone', 'copper', 'iron', 'crystal'] as const;

const datei = (name: string) => `gebaeude/${name}-32.png`;

/** Textur-Schlüssel → Datei unter `public/grafik/` (LoadScene lädt genau diese, `WORLD_TEXTURES` nimmt sie auf) */
export const NATURE_TEXTURES: Record<string, string> = {
  ...Object.fromEntries(ARTEN.flatMap((art) => [1, 2, 3].map((f) => [`grafik:tree-${art}-${f}`, datei(f === 1 ? art : `${art}-${f}`)]))),
  ...Object.fromEntries(WURZELN.map((id) => [`grafik:${id}`, datei(id)])),
  'grafik:hoehlenbusch': datei('hoehlenbusch'),
  'grafik:truhe-geschlossen': datei('truhe-geschlossen'),
  ...Object.fromEntries(FELSEN.flatMap((id) => [1, 2, 3].map((look) => [`grafik:${id}-${look}`, datei(`${id}-${look}`)]))),
  ...Object.fromEntries(ADERN.map((_, i) => [`grafik:ader-${i + 1}`, datei(`ader-${i + 1}`)])),
  'grafik:erzbrocken': datei('erzbrocken'),
  'grafik:busch': datei('busch'),
  'grafik:truhe': datei('truhe'),
  'grafik:portal': datei('portal'),
  'grafik:ausgang': datei('ausgang'),
  'grafik:hafen': datei('hafen'), // Hafen-Steg des Chunks `harbor` (BS10, B-641)
  ...Object.fromEntries([1, 2, 3].map((look) => [look === 1 ? 'grafik:lager' : `grafik:lager-${look}`, datei(`rekrutierungslager-${look}`)])),
  ...MINE_SYMBOL_TEXTURES, // Axt und Spitzhacke am Abbau-Schild (GEB16, B-588)
};

export const natureFiles = (): Record<string, WorldFile> => Object.fromEntries(Object.entries(NATURE_TEXTURES).map(([key, file]) => [key, { file }]));

/** Look des Bioms: 1 Wald, 2 Höhle, 3 Mine; Biome ohne eigenen Look (Eisenstollen, Kristallhöhle, neue) zeigen Mine, fehlende ID Wald */
export function natureLook(biomeId: string | undefined): 1 | 2 | 3 {
  if (!biomeId || biomeId === 'forest') return 1;
  return biomeId === 'cave' ? 2 : 3;
}

/** Rekrutierungslager im Look des Bioms (B-570): Wald Grundform, Höhle und Mine eigene Looks (`rekrutierungslager.json` › `stufen`) */
export function campSprite(biomeId: string | undefined): ObjectSprite {
  const look = natureLook(biomeId);
  return still(look === 1 ? 'grafik:lager' : `grafik:lager-${look}`);
}

/** Ader-Art des Knotens → Slot `vein:<Material>`; unbekannte Art `null` */
export function veinSlot(kind: string): `vein:${(typeof ADERN)[number]}` | null {
  const material = ADERN.find((m) => kind === `${m}Vein`);
  return material ? `vein:${material}` : null;
}

/** Ganzzahlige Mischung der gerundeten Position (Murmur-artig, ohne Zufall): gleiche Position, gleicher Wert */
function mix(x: number): number {
  let h = Math.imul(Math.round(x) | 0, 0x9e3779b1) ^ 0x85ebca6b;
  h = Math.imul(h ^ (h >>> 16), 0x85ebca6b);
  h = Math.imul(h ^ (h >>> 13), 0xc2b2ae35);
  return (h ^ (h >>> 16)) >>> 0;
}

/**
 * Baum-Knoten an Position `x` (Units) im Biom: im Wald Art und Form (1–3) aus der Position, außerhalb des Walds
 * (Höhle; Mine und andere zeigen wie bisher den Höhle-Look) eine der fünf Wurzel-Formen statt eines Baums (GEB10).
 */
export function treeTexture(x: number, biomeId: string | undefined): string {
  if (natureLook(biomeId) !== 1) return `grafik:${WURZELN[mix(x) % WURZELN.length]!}`;
  const wahl = mix(x) % (ARTEN.length * 3);
  return `grafik:tree-${ARTEN[wahl % ARTEN.length]!}-${Math.floor(wahl / ARTEN.length) + 1}`;
}

/** Deko-Wurzel neben einem Baum-Knoten: Textur und Versatz in Units */
export interface DecoRoot {
  key: string;
  dx: number;
}

/**
 * Zusätzliche Wurzeln je Baum-Knoten außerhalb des Walds (GEB15.1): `rootsPerNode - 1` Stück, reine Darstellung, nicht
 * abbaubar. Versatz ±0,4–1,2 Units und Form aus `mix(x)`, nie die Form des Knotens selbst; im Wald keine.
 */
export function decoRoots(x: number, biomeId: string | undefined, rootsPerNode = 1): DecoRoot[] {
  if (natureLook(biomeId) === 1) return [];
  const eigen = mix(x) % WURZELN.length;
  return Array.from({ length: Math.max(0, Math.floor(rootsPerNode) - 1) }, (_, i) => {
    const h = mix(x + (i + 1) * 7919);
    const form = (eigen + 1 + (h % (WURZELN.length - 1))) % WURZELN.length;
    const seite = ((h >>> 4) + i) & 1 ? 1 : -1; // mehrere Deko-Wurzeln wechseln die Seite
    return { key: `grafik:${WURZELN[form]!}`, dx: seite * (4 + ((h >>> 8) % 9)) / 10 };
  });
}

/** Fels-Knoten an Position `x` (Units): Form aus der Position (GEB15.2), Look aus dem Biom */
export function rockTexture(x: number, biomeId: string | undefined): string {
  return `grafik:${FELSEN[mix(x) % FELSEN.length]!}-${natureLook(biomeId)}`;
}

const still = (key: string): ObjectSprite => ({ key, frame: 0, frames: 1, zoom: 1 });

/** Bild eines Ressourcen-Knotens (Baum, Fels, Erzbrocken, Busch, Ader) an Position `x` im Biom; unbekannte Art `null` → Platzhalter */
export function nodeSprite(kind: string, x: number, biomeId: string | undefined): ObjectSprite | null {
  if (kind === 'tree') return still(treeTexture(x, biomeId));
  if (kind === 'rock') return still(rockTexture(x, biomeId));
  if (kind === 'copperOre') return still('grafik:erzbrocken');
  if (kind === 'bush') return still(natureLook(biomeId) === 1 ? 'grafik:busch' : 'grafik:hoehlenbusch');
  const slot = veinSlot(kind);
  return slot ? still(`grafik:ader-${ADERN.indexOf(slot.slice(5) as (typeof ADERN)[number]) + 1}`) : null;
}
