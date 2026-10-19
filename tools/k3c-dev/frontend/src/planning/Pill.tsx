import { IconButton } from '@radix-ui/themes';
import type { ReactNode } from 'react';
import type { Tone } from '@webkit/parts';

// Runde Statusmarke mit Farbpunkt wie in der erp-qs-Workbench (Workbench-Spec › Status-Marken): grün fertig, blau in
// Arbeit, amber Freigabe/Warnung, rot Fehler, violett Worktree, grau offen. Töne wie @webkit/parts, dazu violett.

export type PillTone = Tone | 'violet';

export function Pill({ tone, children, title }: { tone: PillTone; children: ReactNode; title?: string }) {
  return (
    <span className={`pl-pill is-${tone}`} title={title}>
      <i aria-hidden />
      {children}
    </span>
  );
}

/** Ton einer Session nach Status. */
export const sessionTone = (status: string): Tone =>
  (({ fertig: 'ok', 'in Arbeit': 'info', blockiert: 'error', verworfen: 'neutral' }) as Record<string, Tone>)[status] ?? 'neutral';

/** Ton von Sprint und Projekt nach Status: aktiv blau, erledigt grün, sonst grau. */
export const stateTone = (status: string): Tone => (status === 'aktiv' ? 'info' : status === 'erledigt' ? 'ok' : 'neutral');

export const prioTone = (p = ''): Tone => (p === 'hoch' ? 'error' : p === 'mittel' ? 'warn' : 'neutral');

export const specTone = (spec = ''): Tone => (spec === 'freigegeben' ? 'ok' : spec === 'rückwirkend' ? 'neutral' : 'warn');

/** Agent und Umgebung als eine Marke: grün = autonom · offline (worktree-tauglich), blau = autonom · live. */
export function ModePill({ agent = '', env = '' }: { agent?: string; env?: string }) {
  if (!agent && !env) return null;
  const tone: PillTone = agent !== 'autonom' ? 'violet' : env === 'offline' ? 'ok' : 'info';
  const where = env === 'offline' ? 'offline, worktree-tauglich' : env === 'live' ? 'braucht laufende Dienste, Browser oder Gerät' : 'Umgebung offen';
  return <Pill tone={tone} title={`${agent || 'Agent offen'} · ${where}`}>{agent || '?'} · {env || '?'}</Pill>;
}

/** Angehakter Wert der Filterleiste mit Entfernen-Knopf; die Gruppe steht davor, damit „hoch“ nicht allein steht. */
export function RemovableChip({ group, label, onRemove }: { group: string; label: string; onRemove: () => void }) {
  const what = `${group}: ${label}`;
  return (
    <span className="pl-fchip">
      <span className="pl-fchip-group">{group}</span>
      {label}
      <IconButton aria-label={`${what} entfernen`} color="gray" onClick={onRemove} radius="full" size="1" title={`${what} entfernen`} variant="ghost">
        ×
      </IconButton>
    </span>
  );
}
