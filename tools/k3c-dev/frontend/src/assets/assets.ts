import type { AssetChoice, AssetKind, AssetRect, AssetSlot, AssetStatus } from '../api';

// Seite Assets (B-298): reine Logik ohne React, damit sie testbar bleibt.

/** Kategorien je Art wie im Slot-Register (Go: assets.Categories) */
export const CATEGORIES: Record<AssetKind, readonly string[]> = {
  grafik: ['figuren', 'gebaeude', 'umgebung', 'objekte', 'hud'],
  sound: ['sfx', 'musik', 'ambient'],
};

/** Spielmaßstab der Vorschau: 16-px-Raster ×2 auf UNIT_PX = 32 (Q13) */
export const UNIT_PX = 32;
export const GAME_SCALE = 2;

export interface SlotFilter {
  art: AssetKind;
  /** leer = alle Kategorien der Art */
  kategorie: string;
  /** leer = alle */
  status: '' | AssetStatus;
  query: string;
}

/** Slots der Ansicht: Art, Kategorie, Status und Suchwörter (in ID, Beschreibung oder Datei der Zuordnungen). */
export function filterSlots(slots: readonly AssetSlot[], f: SlotFilter): AssetSlot[] {
  const words = f.query.toLowerCase().split(/\s+/).filter(Boolean);
  return slots.filter((s) => {
    if (s.art !== f.art || (f.kategorie && s.kategorie !== f.kategorie) || (f.status && s.status !== f.status)) return false;
    const hay = [s.id, s.beschreibung, ...s.wahl.map((c) => c.datei)].join(' ').toLowerCase();
    return words.every((w) => hay.includes(w));
  });
}

/** Zahl je Status für die Kopfzeile */
export function countStatus(slots: readonly AssetSlot[]): Record<AssetStatus, number> {
  const out: Record<AssetStatus, number> = { zugeordnet: 0, luecke: 0, fehlt: 0 };
  for (const s of slots) out[s.status] += 1;
  return out;
}

/** Verschiebt den Eintrag von `from` nach `to` (Ziehen); ungültige Indizes ändern nichts. */
export function move<T>(list: readonly T[], from: number, to: number): T[] {
  if (from === to || from < 0 || to < 0 || from >= list.length || to >= list.length) return [...list];
  const out = [...list];
  const [item] = out.splice(from, 1);
  out.splice(to, 0, item as T);
  return out;
}

/** Hängt eine Datei als letzte Präferenz an; schon vorhanden → unverändert. */
export function addChoice(wahl: readonly AssetChoice[], datei: string): AssetChoice[] {
  return wahl.some((c) => c.datei === datei) ? [...wahl] : [...wahl, { datei }];
}

export const removeChoice = (wahl: readonly AssetChoice[], index: number): AssetChoice[] => wahl.filter((_, i) => i !== index);

/** Ausschnitt setzen (undefined = ganzes Bild) */
export function withCut(wahl: readonly AssetChoice[], index: number, cut: AssetRect | undefined): AssetChoice[] {
  return wahl.map((c, i) => {
    if (i !== index) return c;
    const { ausschnitt: _, ...rest } = c;
    return cut ? { ...rest, ausschnitt: cut } : rest;
  });
}

/** Größe der Vorschau im Spielmaßstab samt Raster in Units */
export function previewSize(cut: AssetRect): { w: number; h: number; unitsW: number; unitsH: number } {
  const w = cut.w * GAME_SCALE;
  const h = cut.h * GAME_SCALE;
  return { w, h, unitsW: Math.round((w / UNIT_PX) * 10) / 10, unitsH: Math.round((h / UNIT_PX) * 10) / 10 };
}

/** Adresse der Mini-Szene (B-299) für Slot und Präferenz; ohne Vite leer. */
export function sceneUrl(preview: string, slot: AssetSlot, pref: number): string {
  if (!preview) return '';
  const q = new URLSearchParams({ szene: slot.kategorie, slot: slot.id });
  if (pref > 1) q.set('wahl', String(pref));
  return `${preview}assetvorschau.html?${q}`;
}

/** Adresse einer Datei unter public/ über den Vite-Dienst; ohne Vite leer. */
export const fileUrl = (preview: string, datei: string): string => (preview ? `${preview}${datei}` : '');
