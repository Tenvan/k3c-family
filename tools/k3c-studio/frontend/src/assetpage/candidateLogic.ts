import type { Tone } from '@webkit/atoms';
import { isStale, type AssetCandidate, type AssetKind, type CandidateOrigin, type LicenseMark } from '../api';
import { formatSize } from '../lib/license';

// Reine Logik des Reiters Kandidaten (B-677): Beschriftungen, Töne, Bestätigungstext, Fehlertext der Zeile. Ohne React.

const ORIGIN_LABEL: Record<CandidateOrigin, string> = { katalog: 'Katalog', vorrat: 'Vorrat', vorschlag: 'Vorschlag', recherche: 'Recherche' };
const AUDIO = /\.(mp3|ogg|wav|m4a|flac)(\?|#|$)/i;
const IMAGE = /\.(png|jpe?g|gif|webp|svg|bmp)(\?|#|$)/i;

/** Beschriftung der Stufe: „Präf. n“ bei Zuordnungen, sonst Spiel, Vorrat oder Netz. */
export function stageText(c: Pick<AssetCandidate, 'stufe' | 'praef'>): string {
  if (c.praef > 0) return `Präf. ${c.praef}`;
  return c.stufe === 'netz' ? 'Netz' : c.stufe === 'vorrat' ? 'Vorrat' : 'Spiel';
}

/** Ton der Stufe: Zuordnung grün, Spiel blau, Vorrat grau, Netz gelb. */
export function stageTone(c: Pick<AssetCandidate, 'stufe' | 'praef'>): Tone {
  if (c.praef > 0) return 'ok';
  return c.stufe === 'netz' ? 'warn' : c.stufe === 'vorrat' ? 'neutral' : 'info';
}

/** Ton der Lizenz-Marke: frei ok, Share-Alike und prüfen gelb, gesperrt rot. */
export const markTone = (m: LicenseMark): Tone => (m === 'frei' ? 'ok' : m === 'gesperrt' ? 'error' : 'warn');

/** Tooltip der Marke: bei gesperrt der Grund, sonst der Lizenztext. */
export const markTip = (c: Pick<AssetCandidate, 'marke' | 'grund' | 'lizenz'>): string =>
  c.marke === 'gesperrt' ? c.grund || `Lizenz ${c.lizenz || 'unbekannt'} nicht erlaubt` : c.lizenz || 'Lizenz unbekannt';

/** Tooltip der Zeile: Herkünfte, Lizenztext und Urheber. */
export function originTip(c: Pick<AssetCandidate, 'herkunft' | 'lizenz' | 'urheber' | 'quelle'>): string {
  const from = c.herkunft.map((h) => ORIGIN_LABEL[h]).join(' + ');
  return [`Herkunft: ${from || '–'}`, c.quelle && `Quelle: ${c.quelle}`, c.urheber && `Urheber: ${c.urheber}`, c.lizenz && `Lizenz: ${c.lizenz}`].filter(Boolean).join(' · ');
}

/** Zeile unter dem Titel: Quelle · Urheber. */
export const sourceLine = (c: Pick<AssetCandidate, 'quelle' | 'urheber'>): string => [c.quelle, c.urheber].filter(Boolean).join(' · ');

/** Zuordnen ist erlaubt: nur ungebundene Kandidaten, die das Backend als zuordenbar meldet. */
export const canAssign = (c: Pick<AssetCandidate, 'zuordenbar' | 'praef'>): boolean => c.zuordenbar && c.praef === 0;

/** Netz-Kandidaten brauchen die Bestätigung in der Zeile (Größe, Lizenz, Ziel), Vorrat und Spiel nicht. */
export const needsConfirm = (c: Pick<AssetCandidate, 'stufe'>): boolean => c.stufe === 'netz';

/** Text der Bestätigung: „212 kB · CC0 · in tavern“. */
export const confirmText = (c: Pick<AssetCandidate, 'groesse' | 'lizenz' | 'marke'>, slotId: string): string =>
  `${c.groesse > 0 ? formatSize(c.groesse) : 'Größe unbekannt'} · ${c.lizenz || 'Lizenz unbekannt'} · in ${slotId}`;

/** Art der Vorschau: Endung der Adresse entscheidet, sonst die Art des Slots; ohne Adresse keine. */
export function previewKind(c: Pick<AssetCandidate, 'vorschau'>, art: AssetKind): 'bild' | 'ton' | 'keine' {
  if (!c.vorschau) return 'keine';
  if (AUDIO.test(c.vorschau)) return 'ton';
  if (IMAGE.test(c.vorschau)) return 'bild';
  return art === 'sound' ? 'ton' : 'bild';
}

export interface RowError {
  text: string;
  /** Slot-Register zwischenzeitlich geändert: die Liste muss neu geladen werden, dann ist „Erneut versuchen“ sinnvoll */
  stale: boolean;
}

/** Fehler einer Zeile: Stale bekommt den Hinweis „Register geändert“, alles andere seinen Text. */
export function rowError(e: unknown): RowError {
  if (isStale(e)) return { text: 'Register geändert: Liste wird neu geladen, dann erneut versuchen', stale: true };
  return { text: e instanceof Error ? e.message : String(e), stale: false };
}

/** Treffer-ID ohne Präfix `hit:` für AssetsLicenseOK. */
export const hitId = (id: string): string => id.replace(/^hit:/, '');

/** Hinweisleiste der Recherche: „n neue Treffer · Hinweis …“. */
export const researchNote = (neu: number, hinweise: readonly string[]): string => [`${neu} neue Treffer`, ...hinweise].join(' · ');
