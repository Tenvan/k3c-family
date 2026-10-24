import type { AssetSlotDetails } from '../api';

export interface InfoRow {
  label: string;
  value: string;
}

export interface InfoGroup {
  title: 'Definition' | 'Sheet' | 'Atlas' | 'Zuordnung' | 'Verwendung';
  rows: InfoRow[];
}

const row = (label: string, value: string | number | undefined): InfoRow[] =>
  value === undefined || value === '' ? [] : [{ label, value: String(value) }];

function definition(d: AssetSlotDetails): InfoRow[] {
  return [
    ...row('Art', `${d.art}/${d.kategorie}`),
    ...row('Beschreibung', d.beschreibung),
    ...row('Status', d.status),
    ...row('Datei', d.wahl?.datei),
    ...row('Sheet', d.wahl?.sheet),
    ...row('Pack', d.pack?.id),
    ...row('Quelle', d.pack?.quelle),
    ...row('Lizenz', d.pack?.lizenz),
  ];
}

function sheet(d: AssetSlotDetails): InfoRow[] {
  const s = d.sheet;
  if (!s) return [];
  return [
    ...row('Name', s.label ? `${s.name} (${s.label})` : s.name),
    ...row('Pack', s.pack),
    ...row('Frame', `${s.frameW}×${s.frameH}`),
    ...row('Skalierung', s.scale),
    ...row('Höhe', s.height),
    ...row('Ursprung', `${s.originX}, ${s.originY}`),
    ...row('Tönung', s.tint),
    ...s.anims.map((a) => ({ label: `Animation ${a.name}`, value: `${a.frames} Frames, ${a.fps} fps` })),
  ];
}

const atlas = (d: AssetSlotDetails): InfoRow[] =>
  (d.atlas?.anims ?? []).map((a) => ({ label: `Animation ${a.name}`, value: `${a.frames} Frames, ${a.page} @ ${a.x},${a.y} ${a.w}×${a.h}` }));

const zuordnung = (d: AssetSlotDetails): InfoRow[] =>
  d.zuordnung.flatMap((z) => Object.entries(z.zellen).map(([k, v]) => ({ label: `${z.abschnitt} · ${k}`, value: `${v} (${z.datei})` })));

const verwendung = (d: AssetSlotDetails): InfoRow[] => d.verwendung.map((v, i) => ({ label: String(i + 1), value: v }));

/** Gruppen der Slot-Anzeige in fester Reihenfolge; Gruppen ohne Zeilen fallen weg. Hinweise liegen getrennt in `d.hinweise`. */
export function infoGroups(d: AssetSlotDetails): InfoGroup[] {
  const all: InfoGroup[] = [
    { title: 'Definition', rows: definition(d) },
    { title: 'Sheet', rows: sheet(d) },
    { title: 'Atlas', rows: atlas(d) },
    { title: 'Zuordnung', rows: zuordnung(d) },
    { title: 'Verwendung', rows: verwendung(d) },
  ];
  return all.filter((g) => g.rows.length > 0);
}
