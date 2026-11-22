import type { Tone } from '@webkit/atoms';
import { formatBytes } from '../lib/format';
import type { PlaytestAction, PlaytestPlan, PlaytestSession, PlaytestStage } from './types';

export const STAGE_TONE: Record<PlaytestStage, Tone> = {
  läuft: 'info', unvollständig: 'warn', aufgenommen: 'neutral', transkribiert: 'info', ausgewertet: 'ok',
};

/** Datum „21.11.2026“ aus „2026-11-21“. */
export function germanDate(iso: string): string {
  const [y, m, d] = iso.split('-');
  return `${d}.${m}.${y}`;
}

/** Uhrzeit „16:45“ (Ortszeit) eines Unix-Zeitpunkts in ms. */
export function clockTime(ms: number): string {
  const d = new Date(ms);
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`;
}

/** Dauer „42 min“, „1 h 5 min“; ohne Dauer „–“. */
export function sessionLength(ms: number): string {
  if (ms <= 0) return '–';
  const min = Math.max(1, Math.round(ms / 60_000));
  return min < 60 ? `${min} min` : `${Math.floor(min / 60)} h ${min % 60} min`;
}

/** Titel und Text des Bestätigungsdialogs: nennt jede Datei mit Größe, bevor etwas gelöscht wird. */
export function confirmTexts(action: PlaytestAction, s: PlaytestSession | null, plan: PlaytestPlan) {
  const list = plan.files.map((f) => `${f.name} (${formatBytes(f.size)})`).join(', ') || 'nichts';
  const dflt = { title: 'Video löschen?', action: 'Video löschen', text: `Gelöscht wird: ${list}. Das lässt sich nicht rückgängig machen.` };
  if (action === 'session') {
    return { title: 'Session löschen?', action: 'Session löschen',
      text: `Gelöscht wird der ganze Ordner ${plan.files[0]?.name ?? s?.id} mit ${s?.files.length ?? 0} Einträgen, zusammen ${formatBytes(plan.total)}. Das lässt sich nicht rückgängig machen.` };
  }
  if (action === 'ausgewertet') {
    return { title: 'Videos ausgewerteter Sessions löschen?', action: 'Videos löschen',
      text: `Gelöscht werden ${plan.files.length} Videos: ${list}; zusammen ${formatBytes(plan.total)}. Transkripte und Befunde bleiben.` };
  }
  return dflt;
}
