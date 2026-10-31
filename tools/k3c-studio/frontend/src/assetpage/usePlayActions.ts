import type { FileRow, Rows, SlotRow } from './model';
import { playFile, playOn } from './transport';

/** ▶ der Seite: aktive Zuordnung eines Slots, eine Zuordnung aus der Sicht Dateien oder eine Vorrat-Datei ohne Slot. */
export function usePlayActions(rows: Rows) {
  const playSlot = (r: SlotRow) => {
    const c = r.slot.wahl[0];
    if (c) void playOn(r.slot, c);
  };
  const playFileRow = (f: FileRow) => {
    const own = f.praef > 0 ? rows.slots.find((r) => r.slot.id === f.slotId)?.slot : undefined;
    const c = own?.wahl[f.praef - 1];
    if (own && c) void playOn(own, c);
    else if (f.vorschau) void playFile(`${f.slotId} · ${f.titel}`, f.vorschau);
  };
  /** Spielt den gewählten Slot (Enter, X, Überblenden). */
  const playSelected = (id: string) => {
    const r = rows.slots.find((x) => x.slot.id === id);
    if (r) playSlot(r);
  };
  return { playSlot, playFileRow, playSelected };
}
