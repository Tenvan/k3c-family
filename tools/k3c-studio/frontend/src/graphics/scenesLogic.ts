/**
 * Szenen des Asset-Studios (B-299, umgezogen aus der früheren Asset-Vorschau mit B-459): reine Logik ohne Browser.
 * Die Slots kommen aus AssetsList (nicht aus einem Import von data/assets.json, sonst greift das Neuladen nicht);
 * je Slot das aktive Asset (Präferenz 1, zum Vergleich `wahl=n`, Ausschnitt).
 */
import spritesJson from '@data/sprites.json';
import { HUB_GROWTH, HUB_SCAFFOLD } from '@game/scenes/render/hubGrowth';
import type { Card } from '@game/tools/spriteReference';
import type { ListFilterConfig } from '@webkit/lib/listFilter';

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

/** Vergleich: Slot `slot` zeigt Präferenz `wahl` (1-basiert) statt Präferenz 1 */
export interface Query {
  slot?: string;
  wahl: number;
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

/**
 * Suche, Filter und Sortierung der Szenen-Listen (B-599, Baustein `ListFilter`): gleich für Sound und Grafik.
 * `items` gibt die Reihenfolge im Spiel vor (Register-Reihenfolge); „Bus“ gibt es nur bei Sound-Slots und bleibt sonst aus.
 * Dimensionen ohne Daten (z. B. „vom Chat vorgeschlagen“) kommen erst hinzu, wenn es Daten dafür gibt.
 */
export function sceneFilter(items: readonly Resolved[], credits: readonly LicenseEntry[]): ListFilterConfig<Resolved> {
  const order = new Map(items.map((r, i) => [r.slot.id, i]));
  const byId = (a: Resolved, b: Resolved) => a.slot.id.localeCompare(b.slot.id, 'de');
  const status = (r: Resolved) => (r.choice ? 'zugeordnet' : 'Lücke');
  return {
    searchText: (r) => `${r.slot.id} ${r.slot.beschreibung} ${r.choice?.datei ?? ''}`,
    dimensions: [
      { key: 'status', label: 'Status', values: status, order: ['Lücke', 'zugeordnet'] },
      { key: 'art', label: 'Art', values: (r) => r.slot.id.split(':')[0] },
      { key: 'bus', label: 'Bus', values: (r) => r.slot.bus },
      { key: 'lizenz', label: 'Lizenz', values: (r) => (r.choice ? (licenseFlag(r.choice.datei, credits) === 'NC' ? 'NC' : 'frei') : undefined), order: ['NC', 'frei'] },
    ],
    sorts: [
      { key: 'spiel', label: 'Reihenfolge im Spiel', compare: (a, b) => (order.get(a.slot.id) ?? 0) - (order.get(b.slot.id) ?? 0) },
      { key: 'name', label: 'Name', compare: byId },
      { key: 'status', label: 'Status', compare: (a, b) => status(a).localeCompare(status(b), 'de') || byId(a, b) },
    ],
  };
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
// ponytail: ein Faktor für alle; im Spiel haben einzelne Objekte eigene Zooms (Portal ×1, Hintergründe aller fünf Biome ×1,
// src/scenes/render/worldSprites.ts). Erst übernehmen, wenn die Vorschau dort täuscht.
export const GRAFIK_SCALE = 2;

/** Eintrag aus `public/grafik/gebaeude/index.json` (schreibt `task gen:gebaeude`, GEB1) */
export interface GebaeudeEintrag {
  id: string;
  klasse?: string;
  kacheln: { b: number; h: number };
  dateien: Record<string, string>;
  /** Gebäude-Vorlage (Glossar, GEB8) */
  vorlage?: { motiv: string; teile: string[]; materialien?: string[]; stimmung?: string; fein?: string };
  /** Ausbaustufen (GEB3, Regeldatei § 6); Stufe 1 = Grundform */
  stufen?: { stufe: number; aenderung: string; dateien: Record<string, string> }[];
}

/** Kacheln nebeneinander in der Vorschau: kachelbare Grafik (`klasse: kachel`, z. B. Hintergrundmauer) dreimal, damit die Naht sichtbar wird (GEB3/AC-07). */
export function kachelAnzahl(e: Pick<GebaeudeEintrag, 'klasse'>): number {
  return e.klasse === 'kachel' ? 3 : 1;
}

/** Karten der Stufenreihe: jede Stufe ab 2 in 32 px mit ihrer Änderung; Stufe 1 zeigt die Hauptkarte (GEB3/AC-01). */
export function stufenKarten(e: GebaeudeEintrag): { id: string; stufe: number; datei: string; aenderung: string }[] {
  return (e.stufen ?? []).filter((s) => s.stufe > 1 && s.dateien[32]).map((s) => ({ id: e.id, stufe: s.stufe, datei: s.dateien[32]!, aenderung: s.aenderung }));
}

/** Vorlage für die Anzeige: Motiv als Titel, Teile als Zeilen, danach die gesetzten Zusatzfelder (GEB8/AC-03). */
export function vorlageZeilen(e: GebaeudeEintrag): { titel: string; zeilen: string[] } | null {
  const v = e.vorlage;
  if (!v) return null;
  const extra = [
    v.materialien?.length ? `Materialien: ${v.materialien.join(', ')}` : '',
    v.stimmung ? `Stimmung: ${v.stimmung}` : '',
    v.fein ? `fein: ${v.fein}` : '',
  ];
  return { titel: v.motiv, zeilen: [...v.teile, ...extra.filter(Boolean)] };
}

export interface GebaeudeKarte {
  id: string;
  dichte: number;
  datei: string;
  /** ganzzahliger Faktor auf 32 px je Kachel (`UNIT_PX`) */
  scale: number;
}

/** Vergleichsdichten der Vorschau: Master 32 px ×1 neben 16 px ×2 (GEB1/AC-06, Spiel-Dichte entscheidet 🧑 in GEB1.4) */
const VERGLEICH = [32, 16];

/** Karten je Gebäude in Vergleichsreihenfolge; fehlende Dichten entfallen. Hintergrund-Schichten haben ihren eigenen Streifen. */
export function gebaeudeKarten(index: readonly GebaeudeEintrag[]): GebaeudeKarte[] {
  return index.filter((e) => e.klasse !== 'hintergrund').flatMap((e) =>
    VERGLEICH.filter((d) => e.dateien[d]).map((d) => ({ id: e.id, dichte: d, datei: e.dateien[d], scale: 32 / d })),
  );
}

/** Schritt der Bäume in der Reihe: 1 Kachel bei 2–4 Kacheln Breite, also dicht überlappend wie ein Wald (GEB10, Regeldatei § 5) */
export const BAUM_ABSTAND = 1;

/** Datei eines Baums in Stufe (Look, Regeldatei § 6) und Dichte; Stufe 1 = Grundform, fehlende Stufe oder Dichte → `undefined`. */
export function baumDatei(e: GebaeudeEintrag, stufe: number, dichte: number): string | undefined {
  if (stufe === 1) return e.dateien[dichte];
  return e.stufen?.find((s) => s.stufe === stufe)?.dateien[dichte];
}

/**
 * Reihe aus den Bäumen des Index im Wechsel: je Baum ID und x in Kacheln, gezeichnet von links nach rechts.
 * Ohne `anzahl` kommt jeder Baum einmal vor (GEB6.1: alle Formen); `stufe` wählt den Look (1 Wald, 2 Höhle) und lässt Bäume ohne ihn weg.
 * `klasse` `flora` liefert dieselbe Reihe für die Höhlen-Flora (Wurzeln, Höhlenbusch, GEB10).
 */
export function baumReihe(index: readonly GebaeudeEintrag[], anzahl?: number, stufe = 1, klasse = 'baum'): { id: string; x: number }[] {
  const ids = index.filter((e) => e.klasse === klasse && (stufe === 1 || e.stufen?.some((s) => s.stufe === stufe))).map((e) => e.id);
  if (ids.length === 0) return [];
  return Array.from({ length: anzahl ?? ids.length }, (_, i) => ({ id: ids[i % ids.length]!, x: i * BAUM_ABSTAND }));
}

/** Karte der Hub-Wachstumsreihe (GEB11/AC-03): Look in 32 px, `geruest` liegt als zweites Bild darüber; `h` in Kacheln. */
export interface WachstumKarte {
  titel: string;
  wann: string;
  datei: string;
  h: number;
  geruest?: string;
}

/** Burg-Material je Hub-Stufe 2–5 und die Zusätze der Ausbauschritte 2 und 3 (`burg.json` › Stufen 6–13, GEB17.6) */
const BURG_NAME = ['Steinburg', 'Kupferburg', 'Eisenburg', 'Kristallburg'];
const SCHRITT_ZUSATZ = ['', ' mit Bannern', ' mit Türmchen'];

/**
 * Name und Spielbedingung je Look `<id>-<stufe>` (Regel materialien-gebaeude.md § 2.1, Ausbauschritte stufen.md § 8); der Spielcode
 * kennt nur die Reihe. Hub-Stufe 1 = Lagerfeuer, Zelt, Holzhütte; ab Hub-Stufe 2 je Ausbauschritt eine eigene Burg.
 */
const WACHSTUM_TEXT: Record<string, [string, string]> = {
  'lagerfeuer-1': ['Lagerfeuer mit Grill', 'Hub-Stufe 1, Ausbauschritt 1'],
  'lagerfeuer-2': ['Zelt am Feuer', 'Hub-Stufe 1, Ausbauschritt 2'],
  'lagerfeuer-3': ['Holzhütte am Feuer', 'Hub-Stufe 1, Ausbauschritt 3'],
  ...Object.fromEntries(BURG_NAME.flatMap((name, i) => [1, 2, 3].map((step) => [
    `burg-${HUB_GROWTH[(i + 1) * 3 + step - 1]!.stufe}`,
    [name + SCHRITT_ZUSATZ[step - 1], `Hub-Stufe ${i + 2}, Ausbauschritt ${step}`],
  ]))),
};

/**
 * Wachstumsreihe der Burg in Spielreihenfolge (`HUB_GROWTH` aus dem Spielcode, ein Look je Hub-Stufe und Ausbauschritt), danach die
 * Steinburg mit Baugerüst als Beispiel für „ein Bauer baut den Ausbau“; Looks ohne Datei im Index entfallen.
 */
export function hubWachstum(index: readonly GebaeudeEintrag[]): WachstumKarte[] {
  const byId = new Map(index.map((e) => [e.id, e]));
  const karte = (id: string, stufe: number, titel: string, wann: string): WachstumKarte[] => {
    const e = byId.get(id);
    const datei = e?.stufen ? e.stufen.find((s) => s.stufe === stufe)?.dateien[32] : stufe === 1 ? e?.dateien[32] : undefined;
    return e && datei ? [{ titel, wann, datei, h: e.kacheln.h }] : [];
  };
  const karten = HUB_GROWTH.flatMap((l) => {
    const [titel, wann] = WACHSTUM_TEXT[`${l.id}-${l.stufe}`] ?? [`${l.id} · Stufe ${l.stufe}`, ''];
    return karte(l.id, l.stufe, titel, wann);
  });
  const geruest = byId.get(HUB_SCAFFOLD)?.dateien[32];
  const bau = karte('burg', 2, 'Steinburg im Ausbau', 'ein Bauer baut den Ausbau (Gerüst auf Hub-Stufe 1 bis 4)');
  return geruest && bau[0] ? [...karten, { ...bau[0], geruest }] : karten;
}

/** Biom-ID → Namensteil der Hintergrund-Schichten `hg-<teil>-<fern|mitte|nah>` (GEB7, Regeldatei § 10) */
const HINTERGRUND_TEIL: Record<string, string> = { forest: 'wald', cave: 'hoehle', mine: 'mine', ironhold: 'eisenstollen', crystal: 'kristall' };

export interface HintergrundStreifen {
  biom: string;
  /** `palette.sky` des Bioms: Untergrund, auf dem die Schichten stehen */
  himmel: string;
  /** hinten (fern) zuerst */
  schichten: { id: string; dateien: Record<string, string> }[];
}

/** Je Biom (Reihenfolge von `biome`) die drei Hintergrund-Schichten fern → nah; Biome ohne alle drei Schichten entfallen. */
export function hintergrundStreifen(index: readonly GebaeudeEintrag[], biome: readonly { id: string; palette: { sky: string } }[]): HintergrundStreifen[] {
  const byId = new Map(index.filter((e) => e.klasse === 'hintergrund').map((e) => [e.id, e]));
  return biome.flatMap((b) => {
    const schichten = ['fern', 'mitte', 'nah'].flatMap((s) => byId.get(`hg-${HINTERGRUND_TEIL[b.id]}-${s}`) ?? []);
    return schichten.length === 3 ? [{ biom: b.id, himmel: b.palette.sky, schichten: schichten.map((e) => ({ id: e.id, dateien: e.dateien })) }] : [];
  });
}
