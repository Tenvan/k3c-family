import { IconButton, Text } from '@radix-ui/themes';
import { backend, type GitHubSprint, type PlanSprint, type PlanTicket, type PlanningData } from '../api';
import { MarkdownView } from '../ui/MarkdownView';
import { StatusBadge, Tip, type Tone } from '../ui/parts';
import { findSession, ghBadges, groupTickets, isNext, linkedTickets } from './planning';

const tone = (status: string): Tone =>
  (({ fertig: 'ok', 'in Arbeit': 'info', blockiert: 'error' }) as Record<string, Tone>)[status] ?? 'neutral';

/** Karte eines Sprints: Kopf mit Status, Spec und Worktree, Fortschritt, Sessions (klickbar), Tickets. */
export function SprintCard({ sprint: s, gh, sel, onSelect }:
  { sprint: PlanSprint; gh?: GitHubSprint; sel: string; onSelect: (nr: string) => void }) {
  const done = s.sessions.filter((x) => x.status === 'fertig').length;
  const next = s.sessions.find(isNext);
  const cls = ['pl-card', s.status === 'aktiv' && 'is-active', s.status === 'erledigt' && 'is-done', s.worktree && 'is-wt'];
  return (
    <div className={cls.filter(Boolean).join(' ')}>
      <div className="pl-head">
        <strong>{s.id}</strong>
        <span className="pl-dim">{s.domain}</span>
        {s.prio && <StatusBadge tone={s.prio === 'hoch' ? 'warn' : 'neutral'}>Prio {s.prio}</StatusBadge>}
        <span className="pl-title">{s.title}</span>
        {s.worktree && (
          <Tip content="Ein Worktree arbeitet gerade an diesem Sprint"><StatusBadge tone="ok">⚙ {s.worktree}</StatusBadge></Tip>
        )}
        <StatusBadge tone={s.status === 'aktiv' ? 'info' : 'neutral'}>{s.status}</StatusBadge>
        <StatusBadge tone={s.spec === 'freigegeben' ? 'ok' : 'warn'}>Spec: {s.spec || '–'}</StatusBadge>
      </div>
      {gh && <GitHubRow pr={gh} />}
      {s.sessions.length > 0 && (
        <div className="pl-progress" title={`${done} von ${s.sessions.length} Sessions fertig`}>
          <div style={{ width: `${(done / s.sessions.length) * 100}%` }} />
        </div>
      )}
      <ul className="pl-sessions">
        {s.sessions.map((x) => (
          <li key={x.nr} className={[x === next && 'is-next', x.nr === sel && 'is-sel'].filter(Boolean).join(' ')}>
            <button type="button" onClick={() => onSelect(x.nr)}>
              <StatusBadge tone={tone(x.status)}>{x.status}</StatusBadge>
              <code>{x.nr}</code>
              <span>{x.titel || [x.typ, x.agent].filter(Boolean).join(' · ')}</span>
            </button>
          </li>
        ))}
      </ul>
      {s.tickets.length > 0 && <Text size="1" color="gray">Tickets: {s.tickets.join(', ')}</Text>}
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

/** Backlog je Domäne; Tickets der ausgewählten Session sind hervorgehoben. */
export function BacklogList({ data, tickets, sel }: { data: PlanningData; tickets: PlanTicket[]; sel: string }) {
  const linked = linkedTickets(data, sel);
  if (tickets.length === 0) return <Text color="gray">Kein Ticket passt zum Filter.</Text>;
  return groupTickets(tickets).map(([dom, ts]) => (
    <div key={dom}>
      <div className="pl-group">{dom} <span>{ts.length}</span></div>
      {ts.map((t) => (
        <div key={t.nr} className={`pl-ticket${linked.has(t.nr) ? ' is-linked' : ''}`} title={`${t.typ} · Spec: ${t.spec}`}>
          <code>{t.nr}</code>
          <span className="pl-title">{t.title}</span>
          <StatusBadge tone={t.prio === 'hoch' ? 'warn' : 'neutral'}>{t.prio}</StatusBadge>
          <StatusBadge tone={t.status === 'eingeplant' ? 'info' : 'neutral'}>
            {t.status}{t.sprint && t.sprint !== '–' ? ` · ${t.sprint}` : ''}
          </StatusBadge>
        </div>
      ))}
    </div>
  ));
}

/** Detail-Panel der ausgewählten Session; verschwindet die Auswahl (Datei weg), bleibt es leer. */
export function SessionDetail({ data, sel, onClose }: { data: PlanningData; sel: string; onClose: () => void }) {
  const hit = findSession(data, sel);
  if (!hit) return null;
  const { sprint, session: x } = hit;
  return (
    <aside className="pl-detail">
      <div className="pl-head pl-sticky">
        <code>{x.nr}</code>
        <strong className="pl-title">{x.titel || sprint.title}</strong>
        <StatusBadge tone={tone(x.status)}>{x.status}</StatusBadge>
        {x.typ && <span className="pl-dim">{x.typ} · {x.agent}</span>}
        <IconButton size="1" variant="ghost" color="gray" onClick={onClose} aria-label="Schließen">×</IconButton>
      </div>
      <div className="pl-md">
        {x.text ? <MarkdownView source={x.text} headingOffset={1} /> : (
          <Text color="gray">Noch keine Session-Datei – Entwurf im Sprint {sprint.id}.</Text>
        )}
      </div>
    </aside>
  );
}
