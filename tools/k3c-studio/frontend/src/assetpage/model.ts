import type { AssetKind, AssetSlot, CatalogAsset, LicenseMark, SourceProposal } from '../api';
import { licenseMark } from '../lib/license';
import { typOf } from '../assets/assets';

// Datenmodell der Assets-Seite (B-677): die Zeilen der Sichten Slots und Dateien aus Slot-Register, Katalog und
// Vorschlagsliste. Reine Funktionen ohne React; Filter und Gruppen stehen in filter.ts.

/** Stufe einer Datei auf dem Weg ins Spiel (siehe api/candidateTypes.ts). */
export type Stufe = 'netz' | 'vorrat' | 'spiel';

/** Slot-Status mit den Wörtern der Seite: aktiv, Lücke, Datei fehlt. */
export type SlotState = 'aktiv' | 'luecke' | 'fehlt';
const STATE: Record<AssetSlot['status'], SlotState> = { zugeordnet: 'aktiv', luecke: 'luecke', fehlt: 'fehlt' };

/** Eine Datei: Zuordnung eines Slots (Spiel, Präferenz n) oder Eintrag der Vorschlagsliste (Vorrat, Netz). */
export interface FileRow {
  key: string;
  datei: string;
  titel: string;
  stufe: Stufe;
  /** 0 = nicht zugeordnet, n = Präferenz n des Slots */
  praef: number;
  slotId: string;
  art: AssetKind;
  kategorie: string;
  status: SlotState;
  quelle: string;
  pack: string;
  lizenz: string;
  marke: LicenseMark;
  groesse: number;
  /** Bild bzw. Ton zum Anzeigen oder Abspielen ("" = keine Vorschau) */
  vorschau: string;
  proposalId?: string;
  /** im Netz ladbar: offen, Lizenz erlaubt */
  ladbar: boolean;
  grund?: string;
}

/** Ein Asset-Slot mit den Eigenschaften seiner Dateien (für Facetten und Gruppen). */
export interface SlotRow {
  slot: AssetSlot;
  status: SlotState;
  bereich: string;
  quelle: string;
  pack: string;
  files: FileRow[];
}

const NO_SOURCE = '–';

/** Bekannte Anbieter an ihrer Adresse; alles andere zeigt den Hostnamen ohne „www.“. */
const PROVIDERS: readonly (readonly [RegExp, string])[] = [
  [/(^|\.)opengameart\.org$/, 'OpenGameArt'], [/(^|\.)kenney\.nl$/, 'Kenney'], [/(^|\.)freesound\.org$/, 'freesound'],
  [/(^|\.)itch\.io$/, 'itch.io'], [/(^|\.)(commons\.)?wikimedia\.org$/, 'Wikimedia Commons'],
];

/** Anbieter einer Quelle: Der Katalog nennt je Datei oft die genaue Seite (`https://kenney.nl/assets/digaudio#highUp`); die Facette „Quelle“
 *  soll aber den Anbieter zeigen („Kenney“), nicht jede Download-Adresse einzeln. Ohne Adresse bleibt der Text, leer ist „–“. */
export function sourceName(raw: string | undefined): string {
  const text = (raw ?? '').trim();
  if (!text) return NO_SOURCE;
  const url = /^https?:\/\//i.test(text) ? safeUrl(text) : undefined;
  if (!url) return /^selbst erzeugt|^eigene/i.test(text) ? 'eigene' : text;
  const host = url.hostname.toLowerCase().replace(/^www\./, '');
  return PROVIDERS.find(([re]) => re.test(host))?.[1] ?? host;
}

function safeUrl(text: string): URL | undefined {
  try {
    return new URL(text);
  } catch {
    return undefined;
  }
}

function choiceRow(slot: AssetSlot, i: number, meta?: CatalogAsset): FileRow {
  const c = slot.wahl[i]!;
  return {
    key: `${slot.id}#${i}`, datei: c.datei, titel: c.datei.split('/').pop() ?? c.datei, stufe: 'spiel', praef: i + 1, slotId: slot.id, art: slot.art, kategorie: slot.kategorie,
    status: STATE[slot.status], quelle: sourceName(meta?.quelle), pack: meta?.pack ?? NO_SOURCE, lizenz: meta?.lizenz ?? '',
    marke: licenseMark(meta?.lizenz ?? '', meta?.kennzeichen), groesse: 0, vorschau: c.datei, ladbar: false,
  };
}

function proposalRow(p: SourceProposal, slot: AssetSlot | undefined): FileRow {
  const loaded = p.status === 'geladen';
  const blocked = !p.lizenzErlaubt || p.status === 'gesperrt';
  return {
    key: `sug:${p.id}`, datei: p.dateiname, titel: p.titel, stufe: loaded ? 'vorrat' : 'netz', praef: 0, slotId: p.slot, art: p.art, kategorie: slot?.kategorie ?? '', status: slot ? STATE[slot.status] : 'luecke',
    quelle: sourceName(p.quelle), pack: NO_SOURCE, lizenz: p.lizenz, marke: blocked ? 'gesperrt' : licenseMark(p.lizenz), groesse: p.groesse, vorschau: loaded ? (p.vorschau ?? '') : '',
    proposalId: p.id, ladbar: p.status === 'offen' && p.lizenzErlaubt, ...(blocked && { grund: p.sperrgrund ?? `Lizenz ${p.lizenz} nicht erlaubt (Q70)` }),
  };
}

export interface Rows {
  slots: SlotRow[];
  files: FileRow[];
}

/** Zeilen beider Sichten. Vorschläge, die schon verwendet (dann sind sie eine Zuordnung) oder verworfen sind, fehlen. */
export function buildRows(slots: readonly AssetSlot[], proposals: readonly SourceProposal[], catalog: readonly CatalogAsset[]): Rows {
  const meta = new Map(catalog.map((a) => [a.datei, a]));
  const byId = new Map(slots.map((s) => [s.id, s]));
  const open = proposals.filter((p) => !p.verwendet && p.status !== 'verworfen');
  const files: FileRow[] = [
    ...slots.flatMap((s) => s.wahl.map((c, i) => choiceRow(s, i, meta.get(c.datei)))),
    ...open.map((p) => proposalRow(p, byId.get(p.slot))),
  ];
  const rows = slots.map((slot): SlotRow => {
    const mine = files.filter((f) => f.slotId === slot.id);
    const active = mine.find((f) => f.praef === 1);
    return { slot, status: STATE[slot.status], bereich: typOf(slot), quelle: active?.quelle ?? NO_SOURCE, pack: active?.pack ?? NO_SOURCE, files: mine };
  });
  return { slots: rows, files };
}

/** Zahl der Vorschläge je Stufe für die Zeile eines Slots („3 im Netz, 1 im Vorrat“). */
export function stageCounts(row: SlotRow): { netz: number; vorrat: number } {
  return { netz: row.files.filter((f) => f.stufe === 'netz').length, vorrat: row.files.filter((f) => f.stufe === 'vorrat').length };
}

/** Anzeigename der Stufe: „Netz“, „Vorrat“, „Spiel“, bei Zuordnungen „Präf. n“. */
export const stageLabel = (f: Pick<FileRow, 'stufe' | 'praef'>): string => (f.praef > 0 ? `Präf. ${f.praef}` : f.stufe === 'netz' ? 'Netz' : f.stufe === 'vorrat' ? 'Vorrat' : 'Spiel');
