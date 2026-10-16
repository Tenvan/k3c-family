import type { ObjectSprite, WorldFile } from './worldSprites';

/**
 * Natur und Objekte aus `tools/gebaeude` (GEB6): Bäume, Busch, Fels, Erzbrocken, Adern, Truhe, Portal, Ausgang und
 * Rekrutierungslager, alle 32 px je Kachel (Maßstab 1). Reine Auswahl ohne Phaser und ohne Zufall: Baumform aus der
 * Position, Look aus dem Biom (Regeldatei `docs/standards/gebaeude-grafik.md` § 6: Stufe = Look).
 */

const ARTEN = ['tanne', 'pappel', 'birke'] as const;
/** Bäume stehen nur im Wald und in der Höhle: höchster Look 2 */
const BAUM_MAX_LOOK = 2;
/** Adern in der Reihenfolge ihrer Stufen (Stufe = Material, nicht Biom) */
const ADERN = ['stone', 'copper', 'iron', 'crystal'] as const;

const datei = (name: string) => `gebaeude/${name}-32.png`;

/** Textur-Schlüssel → Datei unter `public/grafik/` (LoadScene lädt genau diese, `WORLD_TEXTURES` nimmt sie auf) */
export const NATURE_TEXTURES: Record<string, string> = {
  ...Object.fromEntries(ARTEN.flatMap((art) => [[`grafik:tree-${art}-1`, datei(art)], ...[2, 3].flatMap((f) => [1, 2].map((look) => [`grafik:tree-${art}-${f}-${look}`, datei(`${art}-${f}-${look}`)]))])),
  ...Object.fromEntries([1, 2, 3].map((look) => [`grafik:felsen-${look}`, datei(`felsen-${look}`)])),
  ...Object.fromEntries(ADERN.map((_, i) => [`grafik:ader-${i + 1}`, datei(`ader-${i + 1}`)])),
  'grafik:erzbrocken': datei('erzbrocken'),
  'grafik:busch': datei('busch'),
  'grafik:truhe': datei('truhe'),
  'grafik:portal': datei('portal'),
  'grafik:ausgang': datei('ausgang'),
  'grafik:lager': datei('rekrutierungslager'),
};

export const natureFiles = (): Record<string, WorldFile> => Object.fromEntries(Object.entries(NATURE_TEXTURES).map(([key, file]) => [key, { file }]));

/** Look des Bioms: 1 Wald, 2 Höhle, 3 Mine; Biome ohne eigenen Look (Eisenstollen, Kristallhöhle, neue) zeigen Mine, fehlende ID Wald */
export function natureLook(biomeId: string | undefined): 1 | 2 | 3 {
  if (!biomeId || biomeId === 'forest') return 1;
  return biomeId === 'cave' ? 2 : 3;
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
 * Baum an Position `x` (Units) im Biom: Art und Form (1–3) aus der Position, Look aus dem Biom. Form 1 hat nur den
 * Wald-Look, daher zeigt die Höhle nur die Formen 2 und 3.
 */
export function treeTexture(x: number, biomeId: string | undefined): string {
  const look = Math.min(natureLook(biomeId), BAUM_MAX_LOOK);
  const formen = look === 1 ? [1, 2, 3] : [2, 3];
  const wahl = mix(x) % (ARTEN.length * formen.length);
  const art = ARTEN[wahl % ARTEN.length]!;
  const form = formen[Math.floor(wahl / ARTEN.length)]!;
  return form === 1 ? `grafik:tree-${art}-1` : `grafik:tree-${art}-${form}-${look}`;
}

const still = (key: string): ObjectSprite => ({ key, frame: 0, frames: 1, zoom: 1 });

/** Bild eines Ressourcen-Knotens (Baum, Fels, Erzbrocken, Busch, Ader) an Position `x` im Biom; unbekannte Art `null` → Platzhalter */
export function nodeSprite(kind: string, x: number, biomeId: string | undefined): ObjectSprite | null {
  if (kind === 'tree') return still(treeTexture(x, biomeId));
  if (kind === 'rock') return still(`grafik:felsen-${natureLook(biomeId)}`);
  if (kind === 'copperOre') return still('grafik:erzbrocken');
  if (kind === 'bush') return still('grafik:busch');
  const slot = veinSlot(kind);
  return slot ? still(`grafik:ader-${ADERN.indexOf(slot.slice(5) as (typeof ADERN)[number]) + 1}`) : null;
}
