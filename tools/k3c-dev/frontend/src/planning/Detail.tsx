import { IconButton, Text } from '@radix-ui/themes';
import type { PlanningData, PlanProject, PlanSprint } from '../api';
import { NoticeCard, StatusBadge } from '../ui/parts';
import { SessionDetail } from './Backlog';
import { findSession } from './planning';
import { CopyPrompt } from './PromptParts';
import { promptSprint } from './prompts';
import { tone } from './SprintCard';

interface Props {
  data: PlanningData;
  sel: string;
  /** Auswahl aus dem Detail heraus (Session, Sprint): Karten klappen auf, die Zeile scrollt in Sicht. */
  onSelect: (id: string) => void;
  onClose: () => void;
}

/** Detail der Auswahl (Workbench-Spec § 1 › Detail): Session, Sprint oder Projekt; verschwundene Auswahl als Hinweis. */
export function PlanDetail({ data, sel, onSelect, onClose }: Props) {
  if (!sel) return null;
  if (findSession(data, sel)) return <SessionDetail data={data} sel={sel} onSelect={onSelect} onClose={onClose} />;
  const sprint = data.sprints.find((s) => s.id === sel);
  if (sprint) return <SprintDetail sprint={sprint} onSelect={onSelect} onClose={onClose} />;
  const project = data.projects?.find((p) => p.id === sel);
  if (project) return <ProjectDetail project={project} data={data} onSelect={onSelect} onClose={onClose} />;
  return (
    <aside className="pl-detail">
      <NoticeCard title="Auswahl nicht mehr vorhanden" tone="neutral">{sel} steht nicht mehr in der Planung.</NoticeCard>
    </aside>
  );
}

function Head({ id, title, onClose, children }: { id: string; title: string; onClose: () => void; children?: React.ReactNode }) {
  return (
    <div className="pl-head pl-sticky">
      <code>{id}</code>
      <strong className="pl-title">{title}</strong>
      {children}
      <IconButton size="1" variant="ghost" color="gray" onClick={onClose} aria-label="Schließen">×</IconButton>
    </div>
  );
}

function SprintDetail({ sprint: s, onSelect, onClose }: { sprint: PlanSprint; onSelect: (id: string) => void; onClose: () => void }) {
  const done = s.sessions.filter((x) => x.status === 'fertig' || x.status === 'verworfen').length;
  return (
    <aside className="pl-detail">
      <Head id={s.id} title={s.title} onClose={onClose}>
        <StatusBadge tone={s.status === 'aktiv' ? 'info' : 'neutral'}>{s.status}</StatusBadge>
        <StatusBadge tone={s.spec === 'freigegeben' ? 'ok' : 'warn'}>Spec: {s.spec || '–'}</StatusBadge>
        {s.status !== 'erledigt' && <CopyPrompt prompt={promptSprint(s)} what={`Sprint ${s.id}`} />}
      </Head>
      <div className="pl-md">
        <Text as="p" size="2" color="gray">
          Projekt {s.project || '–'} · Domäne {s.domain} · Reife {s.reife || '–'} · {done}/{s.sessions.length} Sessions erledigt
          {s.worktree ? ` · Worktree ${s.worktree}` : ''}
        </Text>
        <ul className="pl-detail-list">
          {s.sessions.map((x) => (
            <li key={x.nr}>
              <button type="button" className="pl-textlink" onClick={() => onSelect(x.nr)}>{x.nr}</button>
              {' '}<StatusBadge tone={tone(x.status)}>{x.status}</StatusBadge> {x.titel}
            </li>
          ))}
        </ul>
        {s.tickets.length > 0 && <Text as="p" size="2">Tickets: {s.tickets.join(', ')}</Text>}
      </div>
    </aside>
  );
}

function ProjectDetail({ project: p, data, onSelect, onClose }:
  { project: PlanProject; data: PlanningData; onSelect: (id: string) => void; onClose: () => void }) {
  const sprints = p.sprints.map((id) => data.sprints.find((s) => s.id === id)).filter((s): s is PlanSprint => s !== undefined);
  const tickets = data.tickets.filter((t) => t.project === p.id && (!t.sprint || t.sprint === '–'));
  return (
    <aside className="pl-detail">
      <Head id={p.id} title={p.title} onClose={onClose}>
        <StatusBadge tone={p.status === 'aktiv' ? 'info' : 'neutral'}>{p.status}</StatusBadge>
        <Text size="1" color="gray">Rang {p.rang}</Text>
      </Head>
      <div className="pl-md">
        <ul className="pl-detail-list">
          {sprints.map((s) => (
            <li key={s.id}>
              <button type="button" className="pl-textlink" onClick={() => onSelect(s.id)}>{s.id}</button>
              {' '}<StatusBadge tone={s.status === 'aktiv' ? 'info' : 'neutral'}>{s.status}</StatusBadge> {s.title}
              <Text size="1" color="gray"> · {s.sessions.filter((x) => x.status === 'fertig').length}/{s.sessions.length}</Text>
            </li>
          ))}
        </ul>
        {tickets.length > 0 && <Text as="p" size="2">Tickets ohne Sprint: {tickets.map((t) => t.nr).join(', ')}</Text>}
      </div>
    </aside>
  );
}
