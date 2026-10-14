/**
 * Asset-Vorschau (B-299): reine Logik ohne Browser. Liest Asset-Slots aus dem Slot-Register `data/assets.json`
 * und löst je Slot das aktive Asset auf (Präferenz 1, zum Vergleich `wahl=n`, Ausschnitt).
 */
import spritesJson from '../../data/sprites.json';
import type { Card } from './spriteReference';

export interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}

/** Zuordnung eines Asset-Slots: Datei relativ zu public/, optional Sheet, Ausschnitt, Frame-Raster, Name im Audio-Atlas */
export interface Choice {
  datei: string;
  sheet?: string;
  ausschnitt?: Rect;
  raster?: { w: number; h: number };
  sprite?: string;
}

export interface AssetSlot {
  id: string;
  art: 'grafik' | 'sound' | string;
  kategorie: string;
  beschreibung: string;
  bus?: string;
  wahl: Choice[];
}

/** Mini-Szenen je Kategorie, in Anzeigereihenfolge */
export const SCENES = [
  { id: 'figuren', art: 'grafik' },
  { id: 'gebaeude', art: 'grafik' },
  { id: 'umgebung', art: 'grafik' },
  { id: 'objekte', art: 'grafik' },
  { id: 'hud', art: 'grafik' },
  { id: 'sfx', art: 'sound' },
  { id: 'musik', art: 'sound' },
  { id: 'ambient', art: 'sound' },
] as const;
export type SceneId = (typeof SCENES)[number]['id'];

export interface Query {
  szene?: SceneId;
  slot?: string;
  /** Präferenz des Slots `slot`, 1-basiert */
  wahl: number;
}

/** `?szene=<kategorie>[&slot=<id>&wahl=<n>]`; unbekannte Szene → keine, wahl mindestens 1 */
export function parseQuery(search: string): Query {
  const p = new URLSearchParams(search);
  const szene = SCENES.find((s) => s.id === p.get('szene'))?.id;
  const wahl = Math.max(1, Math.floor(Number(p.get('wahl') ?? 1)) || 1);
  return { szene, slot: p.get('slot') ?? undefined, wahl };
}

export const slotsOf = (slots: readonly AssetSlot[], szene: SceneId): AssetSlot[] => slots.filter((s) => s.kategorie === szene);

export interface Resolved {
  slot: AssetSlot;
  /** 1-basiert */
  pref: number;
  choice?: Choice;
  /** keine Zuordnung an dieser Präferenz: Platzhalter mit Slot-ID */
  placeholder: boolean;
}

/** Aktive Zuordnung: Präferenz 1, für den Slot aus dem Aufruf die gewünschte Präferenz. */
export function resolveSlot(slot: AssetSlot, q: Query): Resolved {
  const pref = q.slot === slot.id ? q.wahl : 1;
  const choice = slot.wahl[pref - 1];
  return { slot, pref, choice, placeholder: !choice };
}

/** Lizenz-Eintrag eines Packs (wie `Credit` aus `credits.ts`) */
export interface LicenseEntry {
  readonly root: string;
  readonly folders: readonly string[];
  readonly license: string;
}

/** Kennzeichen nach Q70: `NC` (nur markierter Lückenfüller) oder leer. */
export function licenseFlag(datei: string, credits: readonly LicenseEntry[]): 'NC' | '' {
  const [root, folder] = datei.split('/');
  const c = credits.find((e) => e.root === root && e.folders.includes(folder ?? ''));
  return c && /(^|[\s-])NC([\s-]|$)/i.test(c.license) ? 'NC' : '';
}

interface Role {
  sheet: string;
  tint?: string;
  scale?: number;
}
const SPRITES = spritesJson as unknown as {
  players: Role[];
  troops: Record<string, Role>;
  enemies: Record<string, Role>;
  mounts: Record<string, Role>;
};

/** Rolle im Spiel zur Slot-ID (Truppe, Gegner, Monarch, Reittier) aus data/sprites.json */
function roleOf(id: string): { role?: Role; mount?: string } {
  const [kind, key = ''] = id.split(':');
  if (kind === 'monarch') return { role: SPRITES.players[Number(key) - 1] };
  if (kind === 'mount') return { role: SPRITES.mounts[key], mount: key };
  return { role: SPRITES.troops[id] ?? SPRITES.enemies[id] };
}

/** Karte für die Figuren-Szene: Tönung, Größe und Reiter nur, wenn die Zuordnung das Sheet der Rolle nutzt. */
export function figureCard(slot: AssetSlot, choice: Choice & { sheet: string }): Card {
  const { role, mount } = roleOf(slot.id);
  const card: Card = { title: slot.beschreibung, note: choice.datei, spec: { sheet: choice.sheet } };
  if (!role || role.sheet !== choice.sheet) return card;
  if (mount) return { ...card, mount };
  card.spec = { sheet: choice.sheet, ...(role.tint && { tint: role.tint }), ...(role.scale && { scale: role.scale }) };
  return card;
}

/** Spielmaßstab einer Umgebungs-Grafik: 16-px-Raster ×2 auf `UNIT_PX` = 32 (Q13) */
// ponytail: ein Faktor für alle; im Spiel haben einzelne Objekte eigene Zooms (Portal ×1, Wald-Hintergrund ×3,
// src/scenes/worldSprites.ts). Erst übernehmen, wenn die Vorschau dort täuscht.
export const GRAFIK_SCALE = 2;
