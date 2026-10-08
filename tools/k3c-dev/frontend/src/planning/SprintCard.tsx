import { Checkbox } from '@radix-ui/themes';
import { backend, type GitHubSprint, type PlanSession, type PlanSprint } from '../api';
import { StatusBadge, Tip, type Tone } from '../ui/parts';
import { FoldButton, headClick, useFold } from './fold';
import { ghBadges, isNext } from './planning';
import { CopyPrompt, DepLinks } from './PromptParts';
import { pickable, promptSession, promptSprint } from './prompts';

export const tone = (status: string): Tone =>
  (({ fertig: 'ok', 'in Arbeit': 'info', blockiert: 'error' }) as Record<string, Tone>)[status] ?? 'neutral';

/** Agent und Umgebung als eine Marke: grün = autonom · offline (worktree-tauglich). */
export function ModeBadge({ agent = '', env = '' }: { agent?: string; env?: string }) {
  if (!agent && !env) return null;
  const t: Tone = agent !== 'autonom' ? 'neutral' : env === 'offline' ? 'ok' : 'info';
  const where = env === 'offline' ? 'offline, worktree-tauglich' : env === 'live' ? 'braucht laufende Dienste, Browser oder Gerät' : 'Umgebung offen';
  return <Tip content={`${agent || 'Agent offen'} · ${where}`}><StatusBadge tone={t}>{agent || '?'} · {env || '?'}</StatusBadge></Tip>;
}

export const prioTone = (p = ''): Tone => (p === 'hoch' ? 'error' : p === 'mittel' ? 'warn' : 'neutral');

/** Sprung zu einer Sprint-Karte (Sprint-Abhängigkeit). */
export const scrollToSprint = (id: string) =>
  document.getElementById(`pl-sp-${id}`)?.scrollIntoView({ block: 'nearest', behavior: 'smooth' });

interface Props {
  sprint: PlanSprint;
  gh?: GitHubSprint;
  sel: string;
  onSelect: (nr: string) => void;
  /** Wählt den Sprint für das Detail (Klick auf den Kopf). */
  onPick?: (id: string) => void;
  /** Markierte Sessions (Nr.) für den Sammel-Prompt. */
  checked: ReadonlySet<string>;
  onCheck: (nr: string, on: boolean) => void;
}

/** Karte eines Sprints: Kopf mit Prio, Abhängigkeiten, Status, Spec, Worktree und Prompt, Fortschritt, Sessions
 *  (klickbar, markierbar, je mit Prompt), Tickets. */
export function SprintCard({ sprint: s, gh, sel, onSelect, onPick, checked, onCheck }: Props) {
  const done = s.sessions.filter((x) => x.status === 'fertig').length;
  const next = s.sessions.find(isNext);
  const [folded, toggle] = useFold(s.id);
  const cls = ['pl-card', 'pl-sprint', s.status === 'aktiv' && 'is-active', s.status === 'erledigt' && 'is-done', s.worktree && 'is-wt',
    sel === s.id && 'is-sel'];
  return (
    <div id={`pl-sp-${s.id}`} className={cls.filter(Boolean).join(' ')}>
      <div className="pl-head pl-clickable" onClick={headClick(toggle, () => onPick?.(s.id))}>
        <FoldButton folded={folded} onToggle={toggle} what={`Sprint ${s.id}`} />
        <strong>{s.id}</strong>
        <span className="pl-dim">{s.domain}</span>
        {s.prio && <Tip content="Prio: höchste der Tickets"><StatusBadge tone={prioTone(s.prio)}>Prio {s.prio}</StatusBadge></Tip>}
        <DepLinks ids={s.deps} title="Wartet auf Sessions dieser Sprints" onPick={scrollToSprint} />
        <span className="pl-title">{s.title}</span>
        {s.worktree && (
          <Tip content="Ein Worktree arbeitet gerade an diesem Sprint"><StatusBadge tone="ok">⚙ {s.worktree}</StatusBadge></Tip>
        )}
        <StatusBadge tone={s.status === 'aktiv' ? 'info' : 'neutral'}>{s.status}</StatusBadge>
        <StatusBadge tone={s.spec === 'freigegeben' ? 'ok' : 'warn'}>Spec: {s.spec || '–'}</StatusBadge>
        {s.status !== 'erledigt' && <CopyPrompt prompt={promptSprint(s)} what={`Sprint ${s.id}`} />}
      </div>
      {gh && <GitHubRow pr={gh} />}
      {s.sessions.length > 0 && (
        <div className="pl-progress" title={`${done} von ${s.sessions.length} Sessions fertig`}>
          <div style={{ width: `${(done / s.sessions.length) * 100}%` }} />
        </div>
      )}
      {!folded && <ul className="pl-sessions">
        {s.sessions.map((x) => (
          <li key={x.nr} className={[x === next && 'is-next', x.nr === sel && 'is-sel'].filter(Boolean).join(' ')}>
            <SessionRow sprint={s} x={x} onSelect={onSelect} checked={checked.has(x.nr)} onCheck={(on) => onCheck(x.nr, on)} />
          </li>
        ))}
      </ul>}
      {!folded && s.tickets.length > 0 && <span className="pl-dim">Tickets: {s.tickets.join(', ')}</span>}
    </div>
  );
}

function SessionRow({ sprint: s, x, onSelect, checked, onCheck }:
  { sprint: PlanSprint; x: PlanSession; onSelect: (nr: string) => void; checked: boolean; onCheck: (on: boolean) => void }) {
  const open = pickable(x);
  return (
    <div className="pl-row" role="button" tabIndex={0} onClick={() => onSelect(x.nr)}
      onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && (e.preventDefault(), onSelect(x.nr))}>
      {open ? (
        <Checkbox size="1" aria-label={`${x.nr} markieren`} checked={checked} onCheckedChange={(on) => onCheck(on === true)}
          onClick={(e) => e.stopPropagation()} onKeyDown={(e) => e.stopPropagation()} />
      ) : <span className="pl-nocheck" />}
      <StatusBadge tone={tone(x.status)}>{x.status}</StatusBadge>
      <code>{x.nr}</code>
      <span className="pl-title">{x.titel || [x.typ, x.agent].filter(Boolean).join(' · ')}</span>
      {open && <ModeBadge agent={x.agent} env={x.env} />}
      {open && <DepLinks ids={x.deps} title="Abhängig von" onPick={onSelect} />}
      {open && <CopyPrompt prompt={promptSession(s, x)} what={x.nr} />}
    </div>
  );
}

/** PR, CI und Merge-Stand des Sprints (B-212); der PR-Badge öffnet den PR im Browser. */
function GitHubRow({ pr }: { pr: GitHubSprint }) {
  const [first, ...rest] = ghBadges(pr);
  return (
    <div className="pl-gh">
      <Tip content={`${pr.title} – auf GitHub öffnen`}>
        <button type="button" className="pl-link" onClick={() => backend.openUrl(pr.url)}>
          <StatusBadge tone={first.tone}>{first.label}</StatusBadge>
        </button>
      </Tip>
      {rest.map((b) => <StatusBadge key={b.label} tone={b.tone}>{b.label}</StatusBadge>)}
    </div>
  );
}
