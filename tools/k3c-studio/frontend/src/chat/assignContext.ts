import type { AssignContext } from '../api/chatTypes';

// Kontext-Aufbereitung für den Chat-Auftrag „Zuordnung“ (B-600/AC-01, WZ29.7): Seite, gewählter Slot und aktiver Filter
// werden zum Kontext, den der Chat mit jeder Nachricht bekommt. Die Slot-Details ergänzt das Backend aus dem Register
// (assignInstruction), damit sie nicht vom Browser kommen.

/** Filter der Slot-Liste (AssetsPage); leere Werte filtern nicht. */
export interface AssignFilter {
  kategorie: string;
  typ: string;
  status: string;
  suche: string;
}

/** Aktiver Filter als kurzer Text („Kategorie sfx · Suche „ork““), leer ohne Filter. */
export function filterText(f: AssignFilter): string {
  const parts: string[] = [];
  if (f.kategorie) parts.push(`Kategorie ${f.kategorie}`);
  if (f.typ) parts.push(`Typ ${f.typ}`);
  if (f.status) parts.push(`Status ${f.status}`);
  if (f.suche.trim()) parts.push(`Suche „${f.suche.trim()}“`);
  return parts.join(' · ');
}

/** Kontext für Sound- und Grafik-Seite; andere Seiten haben keinen (null). */
export function assignContext(page: string, slot: string, filter: AssignFilter): AssignContext | null {
  if (page !== 'sound' && page !== 'grafik') return null;
  return { seite: page, slot, filter: filterText(filter) };
}

/** Kontext als Zeile für die Anzeige im Chat („Sound · Slot sfx:coin · Status luecke“). */
export function assignContextLine(c: AssignContext): string {
  return [c.seite === 'sound' ? 'Sound' : 'Grafik', c.slot ? `Slot ${c.slot}` : 'kein Slot gewählt', c.filter].filter(Boolean).join(' · ');
}
