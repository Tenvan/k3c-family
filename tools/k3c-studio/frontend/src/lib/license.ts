import type { LicenseMark } from '../api/candidateTypes';

// Lizenz-Marke einer Datei (B-677), gleiche Regeln wie im Go-Backend (Q70): frei = CC0, CC-BY, OGA-BY und eigene Dateien,
// sa = Share-Alike, gesperrt = NC, ND und Sampling Plus, sonst prüfen. Für Dateien aus dem Katalog; Kandidaten bringen ihre Marke mit.

export const MARK_LABEL: Record<LicenseMark, string> = { frei: 'CC0/BY', sa: 'SA', pruefen: 'prüfen', gesperrt: 'gesperrt' };

const tokens = (s: string): string[] => s.toLowerCase().split(/[^a-z0-9]+/).filter(Boolean);

const BY_FLAG: Record<string, LicenseMark> = { NC: 'gesperrt', ND: 'gesperrt', SA: 'sa' };

/** Marke zu Lizenztext und Katalog-Kennzeichen (NC, SA, ND). */
export function licenseMark(lizenz: string, flag = ''): LicenseMark {
  const byFlag = BY_FLAG[flag.toUpperCase()];
  if (byFlag) return byFlag;
  const t = new Set(tokens(lizenz));
  const has = (...words: string[]) => words.some((w) => t.has(w));
  if (t.size === 0 || has('unbekannt')) return 'pruefen';
  if (has('nc', 'nd') || (has('sampling') && has('plus'))) return 'gesperrt';
  if (has('sa')) return 'sa';
  return has('cc0', 'by', 'selbst', 'eigene', 'public') ? 'frei' : 'pruefen';
}
/** Größe in Bytes („1,4 MB“, „86 kB“, „–“ ohne Angabe). */
export function formatSize(bytes: number): string {
  if (bytes <= 0) return '–';
  if (bytes >= 1_000_000) return `${(bytes / 1_000_000).toFixed(1).replace('.', ',')} MB`;
  return `${Math.max(1, Math.round(bytes / 1000))} kB`;
}
