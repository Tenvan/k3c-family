import type { ReactNode } from 'react';
import type { PlanningData, PlanSprint } from '../api';
import { Button } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { FoldButton, headClick, unfold, useFold } from './fold';
import { Pill, stateTone } from './Pill';
import { sessionProgress, stepRank, type ProjectView } from './projects';
import { CopyPrompt, DepLinks } from './PromptParts';
import { promptProject, promptSession } from './prompts';
import { scrollToSprint } from './SprintCard';
import { WtPills } from './WorktreePills';

interface Props {
  view: ProjectView;
  /** Zahl der aktiven Projekte mit Rang; ohne Wert keine Rang-Knöpfe (ruhend, erledigt, ABN). */
  count?: number;
  /** Grund der letzten Ablehnung von planningSet. */
  error?: string;
  onMove?: (to: number) => void;
  data: PlanningData;
  sel: string;
  /** Wählt Projekt, Sprint oder Ticket für das Detail. */
  onPick?: (id: string) => void;
  renderSprint: (s: PlanSprint) => ReactNode;
}

/** Sessions aller Sprints des Projekts (ungefiltert): erledigt, blockiert, alle – für Balken und Fußzeile. */
function sessionTotals(data: PlanningData, ids: string[]) {
  const all = data.sprints.filter((s) => ids.includes(s.id)).flatMap((s) => s.sessions);
  const done = all.filter((x) => x.status === 'fertig' || x.status === 'verworfen').length;
  return { done, blocked: all.filter((x) => x.status === 'blockiert').length, total: all.length };
}

/** Projekt als Karte (erp-qs-Look): Kopf mit Pfeil, Rang-Kachel, Kürzel, Titel, Zähler, hoch/runter und Pills; darunter
 *  Sprint-Chips, Fortschritt, nächste Session, die Panels der offenen Sprints, Tickets ohne Sprint und die Fußzeile. */
export function ProjectCard({ view, count, error, onMove, data, sel, onPick, renderSprint }: Props) {
  const { project: p, next } = view;
  const [folded, toggle] = useFold(p.id);
  const sum = sessionTotals(data, p.sprints);
  const pct = (n: number) => `${sum.total ? (n / sum.total) * 100 : 0}%`;
  // Ohne offenen Sprint zeigt die Karte alle Sprints als Chips, auch die vom Filter ausgeblendeten erledigten; Panels bleiben gefiltert.
  const chips = view.open === 0 ? p.sprints.flatMap((id) => data.sprints.find((s) => s.id === id) ?? []) : view.sprints;
  return (
    <article id={`pl-pj-${p.id}`} className={`pl-card pl-project ${cardState(view, folded, sel)}`}>
      <header className="pl-project-head" onClick={headClick(toggle, () => onPick?.(p.id))}>
        <FoldButton folded={folded} onToggle={toggle} what={`Projekt ${p.id}`} />
        {/^\d+$/.test(p.rang) && <span className="pl-rank" title={`Rang ${p.rang}`}>{p.rang}</span>}
        <span className="pl-project-id">{p.id}</span>
        <span className="pl-project-title">{p.title}</span>
        <span className="pl-project-meta">
          Sprints {view.done}/{view.total}{chips.length < view.total ? ` · ${chips.length} sichtbar` : ''}
        </span>
        {count !== undefined && !p.wt && <RankButtons view={view} count={count} onMove={onMove} />}
        <span className="pl-pills">
          <WtPills item="Projekt" id={p.id} />
          <StatePill view={view} />
          <CopyPrompt prompt={promptProject(p)} what={`Projekt ${p.id}`} />
        </span>
      </header>
      {error && <NoticeCard title="Rang nicht geändert" tone="error">{error}</NoticeCard>}
      {chips.length > 0 && (
        <div className="pl-chips">
          {chips.map((s) => <SprintChip key={s.id} s={s} onPick={() => { onPick?.(s.id); unfold([p.id, s.id]); setTimeout(() => scrollToSprint(s.id), 0); }} />)}
        </div>
      )}
      <div className="pl-bar" role="img" aria-label={`${sum.done} von ${sum.total} Sessions erledigt`}>
        <i className="is-done" style={{ width: pct(sum.done) }} />
        <i className="is-blocked" style={{ width: pct(sum.blocked) }} />
      </div>
      {!folded && next && (
        <div className="pl-next">
          <span>Nächste Session: <strong>{next.session.nr}</strong> {next.session.titel}</span>
          <CopyPrompt prompt={promptSession(next.sprint, next.session)} what={`Session ${next.session.nr}`} />
        </div>
      )}
      {!folded && view.sprints.filter((s) => s.status !== 'erledigt').map(renderSprint)}
      {!folded && view.loose.length > 0 && (
        <span className="pl-sprint-tickets">
          Tickets ohne Sprint: <DepLinks ids={view.loose.map((t) => t.nr)} lead="" title="Tickets des Projekts ohne Sprint" onPick={(id) => onPick?.(id)} />
        </span>
      )}
      <footer className="pl-project-foot">
        {sum.done}/{sum.total} Sessions erledigt{sum.blocked ? ` · ${sum.blocked} blockiert` : ''} · {view.open} Sprints offen
      </footer>
    </article>
  );
}

/** Zustand des Projekts; aktiv ohne offenen Sprint heißt „inaktiv“. */
function StatePill({ view: { project: p, open } }: { view: ProjectView }) {
  const inactive = open === 0 && p.status === 'aktiv';
  return <Pill tone={inactive ? 'neutral' : stateTone(p.status)}>{inactive ? 'inaktiv' : p.status}</Pill>;
}

function RankButtons({ view, count, onMove }: { view: ProjectView; count: number; onMove?: (to: number) => void }) {
  const up = stepRank(view.project, -1, count);
  const down = stepRank(view.project, 1, count);
  return (
    <span className="pl-move" onClick={(e) => e.stopPropagation()}>
      <Button size="1" variant="soft" color="gray" disabled={up === null} onClick={() => up !== null && onMove?.(up)}>hoch</Button>
      <Button size="1" variant="soft" color="gray" disabled={down === null} onClick={() => down !== null && onMove?.(down)}>runter</Button>
    </span>
  );
}

/** Zustandsklassen der Projektkarte: Status, inaktiv (aktiv ohne offenen Sprint), eingeklappt, gewählt. */
function cardState({ project: p, open }: ProjectView, folded: boolean, sel: string): string {
  return [`is-${p.status}`, p.status === 'aktiv' && open === 0 && 'is-inactive', folded && 'is-folded', sel === p.id && 'is-sel']
    .filter(Boolean).join(' ');
}

/** Farbe des Sprint-Chips: aktiv in der Akzentfarbe, blockiert amber, erledigt grün, sonst grau. */
const chipColor = (kind: string) => (kind === 'aktiv' ? undefined : kind === 'blockiert' ? 'amber' : kind === 'erledigt' ? 'green' : 'gray');

/** Sprint im Projektkopf: eckiger Chip, damit er sich von den runden Status-Pills abhebt. */
function SprintChip({ s, onPick }: { s: PlanSprint; onPick: () => void }) {
  const [done, all] = sessionProgress(s);
  const kind = s.sessions.some((x) => x.status === 'blockiert') ? 'blockiert' : s.status;
  return (
    <Button size="1" variant="outline" radius="small" color={chipColor(kind)} className="pl-chip" data-kind={kind} title={s.title} onClick={onPick}>
      {s.id} · {s.status} · {done}/{all}
    </Button>
  );
}
