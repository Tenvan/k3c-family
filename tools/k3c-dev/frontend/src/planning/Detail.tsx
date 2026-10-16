import { IconButton, Text } from '@radix-ui/themes';
import type { ReactNode } from 'react';
import type { PlanningData, PlanProject, PlanSprint, PlanTicket } from '../api';
import { MarkdownView } from '../ui/MarkdownView';
import { ENVS, envTone, setField } from './Backlog';
import { ModePill, Pill, prioTone, sessionTone, specTone, stateTone, type PillTone } from './Pill';
import { findSession } from './planning';
import { sessionProgress } from './projects';
import { CopyPrompt, DepLinks, FieldMenu } from './PromptParts';
import { backlogPrompt, pickable, promptProject, promptSession, promptSprint, type SprintTarget } from './prompts';
import { Reader } from './Reader';

const AGENTS = ['autonom', 'Mensch'];
const PRIOS = ['hoch', 'mittel', 'niedrig', '?'];

interface Props {
  data: PlanningData;
  sel: string;
  /** Auswahl aus dem Detail heraus: Karten klappen auf, die Zeile scrollt in Sicht. */
  onSelect: (id: string) => void;
  onClose: () => void;
  /** Ziel der Backlog-Prompts (Ziel-Auswahl im Backlog). */
  target: SprintTarget;
}

/** Detail der Auswahl unter dem Backlog (erp-qs-Look): Session, Sprint, Projekt oder Ticket mit Fakten, Prompt und
 *  Lesemodus (Doppelklick auf den Titel); verschwundene Auswahl als Hinweis. */
export function PlanDetail({ data, sel, onSelect, onClose, target }: Props) {
  const head = (id: string, pill: ReactNode, title: string, prompt: string) => (
    <header className="pl-detail-head">
      {pill}
      <Reader id={id} title={title} prompt={prompt} pill={pill} />
      {prompt ? <CopyPrompt prompt={prompt} what={id} /> : <span />}
      <IconButton size="1" variant="ghost" color="gray" onClick={onClose} aria-label="Detail schließen">×</IconButton>
    </header>
  );
  const ctx = { data, onSelect, head };
  if (findSession(data, sel)) return <SessionDetail sel={sel} {...ctx} />;
  const sprint = data.sprints.find((s) => s.id === sel);
  if (sprint) return <SprintDetail s={sprint} {...ctx} />;
  const project = data.projects?.find((p) => p.id === sel);
  if (project) return <ProjectDetail p={project} {...ctx} />;
  const ticket = data.tickets.find((t) => t.nr === sel);
  if (ticket) return <TicketDetail t={ticket} target={target} {...ctx} />;
  return <div className="pl-detail is-empty">{sel} steht nicht mehr in der Planung.</div>;
}

type Head = (id: string, pill: ReactNode, title: string, prompt: string) => ReactNode;
interface Ctx { data: PlanningData; onSelect: (id: string) => void; head: Head }

function Facts({ items }: { items: [string, ReactNode][] }) {
  return (
    <ul className="pl-facts">
      {items.filter(([, v]) => v !== '' && v !== undefined && v !== null && v !== '–').map(([k, v]) => <li key={k}><b>{k}:</b> {v}</li>)}
    </ul>
  );
}

const link = (id: string, onSelect: (id: string) => void) => <button type="button" className="pl-textlink" onClick={() => onSelect(id)}>{id}</button>;

/** Feld als Marke, die ein Menü der erlaubten Werte öffnet. */
const field = (id: string, label: string, value: string, options: string[], tone: PillTone) => (
  <FieldMenu label={label} value={value} options={options} onPick={(v) => void setField(id, label, v)}><Pill tone={tone}>{value || '?'}</Pill></FieldMenu>
);

function SessionDetail({ data, sel, onSelect, head }: Ctx & { sel: string }) {
  const { sprint: s, session: x } = findSession(data, sel)!;
  const open = pickable(x);
  return (
    <div className="pl-detail">
      {head(x.nr, <Pill tone={sessionTone(x.status)}>{x.status}</Pill>, `${x.nr} · ${x.titel || s.title}`, open ? promptSession(s, x) : '')}
      <Facts items={[
        ['Typ', x.typ], ['Domäne', x.domain ?? s.domain],
        ['Agent', open ? field(x.nr, 'Agent', x.agent, AGENTS, x.agent === 'autonom' ? 'ok' : 'violet') : <ModePill agent={x.agent} env={x.env} />],
        ['Umgebung', open ? field(x.nr, 'Umgebung', x.env ?? '', ENVS, envTone(x.env)) : ''],
        ['Abhängig von', x.deps?.length ? <DepLinks ids={x.deps} title="Abhängig von" onPick={onSelect} /> : ''],
        ['Sprint', <>{link(s.id, onSelect)} {s.title}</>], ['Projekt', s.project && s.project !== '–' ? link(s.project, onSelect) : ''],
      ]} />
      <div className="pl-md">
        {x.text ? <MarkdownView source={x.text.replace(/^#\s[^\n]*\n/, '')} headingOffset={1} /> : <Text color="gray" size="2">Noch keine Session-Datei – Entwurf im Sprint {s.id}.</Text>}
      </div>
    </div>
  );
}

function SprintDetail({ s, onSelect, head }: Ctx & { s: PlanSprint }) {
  const [done, all] = sessionProgress(s);
  return (
    <div className="pl-detail">
      {head(s.id, <Pill tone={stateTone(s.status)}>{s.status}</Pill>, `${s.id} · ${s.title}`, s.status !== 'erledigt' ? promptSprint(s) : '')}
      <Facts items={[
        ['Projekt', s.project && s.project !== '–' ? link(s.project, onSelect) : ''], ['Domäne', s.domain],
        ['Spec', <Pill key="s" tone={specTone(s.spec)}>{s.spec || '–'}</Pill>], ['Reife', s.reife],
        ['Wartet auf', s.deps?.length ? <DepLinks ids={s.deps} title="Wartet auf" onPick={onSelect} /> : ''],
        ['Worktree', s.worktree], ['Sessions', `${done} von ${all} erledigt`],
        ['Tickets', s.tickets.length ? <DepLinks ids={s.tickets.map((t) => t.split(' ')[0])} lead="" title="Tickets" onPick={onSelect} /> : ''],
      ]} />
      <ul className="pl-detail-list">
        {s.sessions.map((x) => (
          <li key={x.nr}>
            <Pill tone={sessionTone(x.status)}>{x.status}</Pill>
            <button type="button" className="pl-detail-link" onClick={() => onSelect(x.nr)}>{x.nr} {x.titel || x.typ}</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

function ProjectDetail({ p, data, onSelect, head }: Ctx & { p: PlanProject }) {
  const sprints = p.sprints.flatMap((id) => data.sprints.find((s) => s.id === id) ?? []);
  const loose = data.tickets.filter((t) => t.project === p.id && (!t.sprint || t.sprint === '–'));
  return (
    <div className="pl-detail">
      {head(p.id, <Pill tone={stateTone(p.status)}>{p.status}</Pill>, `${p.id} · ${p.title}`, promptProject(p))}
      <Facts items={[
        ['Rang', p.rang], ['Sprints', `${sprints.filter((s) => s.status === 'erledigt').length} von ${sprints.length} erledigt`],
        ['Tickets ohne Sprint', loose.length ? <DepLinks ids={loose.map((t) => t.nr)} lead="" title="Tickets ohne Sprint" onPick={onSelect} /> : ''],
      ]} />
      <ul className="pl-detail-list">
        {sprints.map((s) => (
          <li key={s.id}>
            <Pill tone={stateTone(s.status)}>{s.status}</Pill>
            <button type="button" className="pl-detail-link" onClick={() => onSelect(s.id)}>{s.id} {s.title} · {sessionProgress(s).join('/')}</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

function TicketDetail({ t, data, onSelect, head, target }: Ctx & { t: PlanTicket; target: SprintTarget }) {
  return (
    <div className="pl-detail">
      {head(t.nr, <Pill tone={prioTone(t.prio)}>{t.prio || '?'}</Pill>, `${t.nr} · ${t.title}`, backlogPrompt([t], target))}
      <Facts items={[
        ['Domäne', t.domain], ['Typ', t.typ], ['Status', t.status],
        ['Prio', field(t.nr, 'Prio', t.prio, PRIOS, prioTone(t.prio))], ['Umgebung', field(t.nr, 'Umgebung', t.env, ENVS, envTone(t.env))],
        ['Sprint', t.sprint && t.sprint !== '–' ? link(t.sprint, onSelect) : ''],
        ['Projekt', t.project && t.project !== '–' && data.projects?.some((p) => p.id === t.project) ? link(t.project, onSelect) : t.project],
        ['Spec', <Pill key="s" tone={specTone(t.spec)}>{t.spec || '–'}</Pill>],
      ]} />
      <Text as="p" size="1" color="gray">Volltext: Doppelklick auf den Titel oder das Lese-Symbol.</Text>
    </div>
  );
}
