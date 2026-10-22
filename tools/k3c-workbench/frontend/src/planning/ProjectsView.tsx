import { useState, type ReactNode } from 'react';
import { backend, type PlanningData, type PlanSprint, type PlanTicket } from '../api';
import { Button, Text } from '@webkit/atoms';
import { FoldButton, useFold } from './fold';
import { ProjectCard } from './ProjectCard';
import { groupProjects, moveRank, rankedCount, type ProjectGroups } from './projects';
import { DepLinks } from './PromptParts';

interface Props {
  data: PlanningData;
  /** Gefilterte Sprints und Tickets; Projekte zeigen nur diese. */
  sprints: PlanSprint[];
  tickets: PlanTicket[];
  sel: string;
  onPick: (id: string) => void;
  renderSprint: (s: PlanSprint) => ReactNode;
  /** Lädt die Planung neu (nach einer Rang-Änderung). */
  reload: () => Promise<unknown>;
}

/** Arbeit nach Projekten (B-358): aktive nach Rang, ABN, „Ohne Projekt“, ruhende und erledigte eingeklappt. */
export function ProjectsView({ data, sprints, tickets, sel, onPick, renderSprint, reload }: Props) {
  const [error, setError] = useState<{ id: string; text: string } | null>(null);
  const move = (id: string, to: number) =>
    void moveRank((i, f, v) => backend.planningSet(i, f, v), id, to, reload).then((text) => setError(text ? { id, text } : null));
  return (
    <ProjectSections groups={groupProjects(data, sprints, tickets)} count={rankedCount(data.projects)} error={error}
      onMove={move} data={data} sel={sel} onPick={onPick} renderSprint={renderSprint} />
  );
}

interface SectionProps extends Pick<Props, 'data' | 'sel' | 'renderSprint'>, Partial<Pick<Props, 'onPick'>> {
  groups: ProjectGroups;
  count: number;
  error: { id: string; text: string } | null;
  onMove: (id: string, to: number) => void;
}

/** Bereiche der Projekt-Ansicht ohne Zustand, damit ein Test sie statisch rendert. */
export function ProjectSections({ groups: g, count, error, onMove, data, sel, onPick, renderSprint }: SectionProps) {
  const card = { data, sel, onPick, renderSprint };
  return (
    <>
      {g.active.map((v) => (
        <ProjectCard key={v.project.id} view={v} count={count} error={error?.id === v.project.id ? error.text : undefined}
          onMove={(to) => onMove(v.project.id, to)} {...card} />
      ))}
      {g.abn && (
        <FoldSection id="ABN" area="abn" className="pl-card pl-project is-abn"
          title={<>Abnahmen am Gerät (ABN) <small>ohne Rang · {g.abnOpen.length} offen</small></>}>
          {g.abnOpen.map(({ sprint, session }) => (
            <Button key={session.nr} size="1" variant="ghost" color="gray" highContrast className="pl-abn-row" onClick={() => onPick?.(session.nr)}>
              {session.nr} · {session.titel} <Text color="gray">({sprint.id})</Text>
            </Button>
          ))}
          {g.abnOpen.length === 0 && <Text size="1" color="gray">Keine offene Abnahme.</Text>}
        </FoldSection>
      )}
      {(g.without.sprints.length > 0 || g.without.tickets.length > 0) && (
        <FoldSection id="ohne" area="ohne" className="pl-card pl-project is-loose"
          title={<>Ohne Projekt <small>{g.without.sprints.length} Sprints · {g.without.tickets.length} Tickets</small></>}>
          {g.without.sprints.map(renderSprint)}
          {g.without.tickets.length > 0 && (
            <span className="pl-sprint-tickets">
              Tickets: <DepLinks ids={g.without.tickets.map((t) => t.nr)} lead="" title="Tickets ohne Projekt" onPick={(id) => onPick?.(id)} />
            </span>
          )}
        </FoldSection>
      )}
      {g.folded.length > 0 && (
        <details data-area="eingeklappt">
          <summary className="pl-h">Inaktiv, ruhend und erledigt ({g.folded.length})</summary>
          <div className="pl-folded-list">{g.folded.map((v) => <ProjectCard key={v.project.id} view={v} {...card} />)}</div>
        </details>
      )}
    </>
  );
}

/** Bereich mit Pfeil zum Einklappen (ABN, Ohne Projekt; B-364). */
function FoldSection({ id, area, className, title, children }:
  { id: string; area: string; className?: string; title: ReactNode; children: ReactNode }) {
  const [folded, toggle] = useFold(`area:${id}`);
  return (
    <section className={className} data-area={area}>
      <h3 className="pl-h pl-area-head"><FoldButton folded={folded} onToggle={toggle} what={id} />{title}</h3>
      {!folded && children}
    </section>
  );
}
