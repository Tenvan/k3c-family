import { backend, type GitHubSprint, type PlanSession, type PlanSprint } from '../api';
import { Checkbox, LinkButton } from '@webkit/atoms';
import { FoldButton, headClick, useFold } from './fold';
import { ModePill, Pill, sessionTone, specTone, stateTone } from './Pill';
import { ghBadges, isNext } from './planning';
import { sessionProgress } from './projects';
import { CopyPrompt, DepLinks } from './PromptParts';
import { pickable, promptSession, promptSprint } from './prompts';
import { WtPills } from './WorktreePills';

/** Sprung zu einem Sprint-Panel (Sprint-Abhängigkeit, Sprint-Chip). */
export const scrollToSprint = (id: string) =>
  document.getElementById(`pl-sp-${id}`)?.scrollIntoView({ block: 'nearest', behavior: 'smooth' });

/** Klassen des Panels: Zustand, im Worktree, gewählt, eingeklappt. */
const panelClass = (s: PlanSprint, sel: string, folded: boolean) =>
  ['pl-sprint', `is-${s.status}`, s.worktree && 'is-wt', sel === s.id && 'is-sel', folded && 'is-folded'].filter(Boolean).join(' ');

interface Props {
  sprint: PlanSprint;
  gh?: GitHubSprint;
  sel: string;
  /** Session (Nr.) auswählen; ein zweiter Klick hebt die Auswahl auf. */
  onSelect: (nr: string) => void;
  /** Sprint oder Ticket für das Detail wählen. */
  onPick?: (id: string) => void;
  /** Markierte Sessions (Nr.) für den Sammel-Prompt. */
  checked: ReadonlySet<string>;
  onCheck: (nr: string, on: boolean) => void;
}

/** Sprint als Panel in der Projektkarte (erp-qs-Look): Kopf mit Pfeil, Status, Spec, Worktree und Prompt, GitHub-Stand,
 *  3-px-Fortschritt, darunter die Sessions als Zeilen und die Tickets des Sprints als eckige Links. */
export function SprintCard({ sprint: s, gh, sel, onSelect, onPick, checked, onCheck }: Props) {
  const [done, all] = sessionProgress(s);
  const next = s.sessions.find(isNext);
  const [folded, toggle] = useFold(s.id);
  return (
    <section id={`pl-sp-${s.id}`} className={panelClass(s, sel, folded)}>
      <div className="pl-sprint-head" onClick={headClick(toggle, () => onPick?.(s.id))}>
        <FoldButton folded={folded} onToggle={toggle} what={`Sprint ${s.id}`} />
        <span className="pl-sprint-id">{s.id}</span>
        <span className="pl-dim">{s.domain}</span>
        <span className="pl-sprint-title">{s.title}</span>
        <span className="pl-pills">
          {(s.worktrees ?? (s.worktree ? [s.worktree] : [])).map((w) =>
            <Pill key={w} tone="violet" title="Ein Worktree arbeitet gerade an diesem Sprint">⎇ {w}</Pill>)}
          <WtPills item="Sprint" id={s.id} />
          <Pill tone={stateTone(s.status)}>{s.status}</Pill>
          <Pill tone={specTone(s.spec)} title={`Reife: ${s.reife || '–'}`}>Spec: {s.spec || '–'}</Pill>
          <DepLinks ids={s.deps} title="Wartet auf Sessions dieser Sprints" onPick={scrollToSprint} />
          <span className="pl-count-num">{done}/{all}</span>
          {s.status !== 'erledigt' && <CopyPrompt prompt={promptSprint(s)} what={`Sprint ${s.id}`} />}
        </span>
      </div>
      {gh && <GitHubRow pr={gh} />}
      <div className="pl-bar is-thin" aria-hidden><i className="is-done" style={{ width: `${all ? (done / all) * 100 : 0}%` }} /></div>
      {!folded && (
        <ul className="pl-sessions">
          {s.sessions.map((x) => (
            <li key={x.nr} className={[x === next && 'is-next', x.nr === sel && 'is-sel'].filter(Boolean).join(' ')}>
              <SessionRow sprint={s} x={x} onSelect={onSelect} checked={checked.has(x.nr)} onCheck={(on) => onCheck(x.nr, on)} />
            </li>
          ))}
        </ul>
      )}
      {!folded && s.tickets.length > 0 && (
        <span className="pl-sprint-tickets">Tickets: <DepLinks ids={s.tickets.map((t) => t.split(' ')[0])} lead="" title="Tickets des Sprints" onPick={(id) => onPick?.(id)} /></span>
      )}
    </section>
  );
}

/** Session als Zeile ohne eigene Fläche: Checkbox (nur offene), Status, Nr., Titel, Modus, Abhängigkeiten, Prompt. */
export function SessionRow({ sprint: s, x, onSelect, checked, onCheck }:
  { sprint: PlanSprint; x: PlanSession; onSelect: (nr: string) => void; checked: boolean; onCheck: (on: boolean) => void }) {
  const open = pickable(x);
  return (
    <div className="pl-row" role="button" tabIndex={0} onClick={() => onSelect(x.nr)}
      onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && (e.preventDefault(), onSelect(x.nr))}>
      {open ? (
        <Checkbox size="1" aria-label={`${x.nr} markieren`} checked={checked} onCheckedChange={(on) => onCheck(on === true)}
          onClick={(e) => e.stopPropagation()} onKeyDown={(e) => e.stopPropagation()} />
      ) : <span />}
      <Pill tone={sessionTone(x.status)}>{x.status}</Pill>
      <span className="pl-row-id">{x.nr}</span>
      <span className="pl-row-name" title={[x.titel, x.typ, x.agent].filter(Boolean).join(' · ')}>
        {x.titel || [x.typ, x.agent].filter(Boolean).join(' · ')}
      </span>
      <span className="pl-row-meta">
        <WtPills item="Session" id={x.nr} />
        {open && <ModePill agent={x.agent} env={x.env} />}
        {open && <DepLinks ids={x.deps} title="Abhängig von" onPick={onSelect} />}
      </span>
      {open ? <CopyPrompt prompt={promptSession(s, x)} what={x.nr} /> : <span />}
    </div>
  );
}

/** PR, CI und Merge-Stand des Sprints (B-212); die PR-Marke öffnet den PR im Browser. */
function GitHubRow({ pr }: { pr: GitHubSprint }) {
  const [first, ...rest] = ghBadges(pr);
  return (
    <div className="pl-gh">
      <LinkButton underline="none" title={`${pr.title} – auf GitHub öffnen`} onClick={() => backend.openUrl(pr.url)}>
        <Pill tone={first.tone}>{first.label}</Pill>
      </LinkButton>
      {rest.map((b) => <Pill key={b.label} tone={b.tone}>{b.label}</Pill>)}
    </div>
  );
}
