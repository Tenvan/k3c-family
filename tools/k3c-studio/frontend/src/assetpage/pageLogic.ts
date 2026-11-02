import type { AssetKind, SourceProposal } from '../api';
import { moveSelection } from '@game/tools/soundtestLogic';
import type { Facets, Group } from './filter';
import type { SlotRow } from './model';

// Hilfen der Seitenlogik (B-677), getrennt von der Oberfläche, damit sie testbar bleiben.

/** Art, auf die sich Chat und Vorschlagskarten beziehen: die des gewählten Slots, sonst die einzige gewählte Art, sonst keine. */
export function pageArt(slot: { art: AssetKind } | undefined, facets: Facets): AssetKind | undefined {
  if (slot) return slot.art;
  return facets.art.length === 1 ? (facets.art[0] as AssetKind) : undefined;
}

/** Slot-ID nach `delta` Schritten in der angezeigten Reihenfolge (rundum); ohne Auswahl der erste bzw. letzte. */
export function stepSlot(groups: readonly Group<SlotRow>[], current: string, delta: number): string {
  const ids = groups.flatMap((g) => g.items.map((r) => r.slot.id));
  if (ids.length === 0) return '';
  const at = ids.indexOf(current);
  return ids[at < 0 ? (delta > 0 ? 0 : ids.length - 1) : moveSelection(at, ids.length, delta)] ?? '';
}

/** Schlüssel gewählter Dateizeilen (`sug:<id>`) als Vorschlags-IDs. */
export const proposalIds = (keys: Iterable<string>): string[] => [...keys].filter((k) => k.startsWith('sug:')).map((k) => k.slice(4));

/** Vorschläge zu den gewählten Schlüsseln (für Summe und Bestätigung). */
export function pickedProposals(all: readonly SourceProposal[], keys: Iterable<string>): SourceProposal[] {
  const ids = new Set(proposalIds(keys));
  return all.filter((p) => ids.has(p.id));
}

/** Auftrag an den Chat „Claude suchen lassen“ für alle Lücken der Art (leer = beide). */
export function searchPrompt(art: AssetKind | undefined): string {
  const what = art === 'sound' ? 'Töne' : art === 'grafik' ? 'Grafiken' : 'Töne und Grafiken';
  return `Suche freie ${what} von den erlaubten Quellen für alle Lücken. `
    + 'Nenne je Ziel bis zu drei Kandidaten mit direkter Datei-URL, Seite, Urheber und Lizenz (CC0, CC-BY, OGA-BY oder CC-BY-SA).';
}
