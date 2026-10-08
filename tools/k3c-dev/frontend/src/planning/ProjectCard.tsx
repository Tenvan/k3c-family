import { Button, Flex, Text } from '@radix-ui/themes';
import type { ReactNode } from 'react';
import type { PlanningData, PlanSprint } from '../api';
import { NoticeCard } from '../ui/parts';
import { BacklogList } from './Backlog';
import { CopyPrompt } from './PromptParts';
import { promptSession } from './prompts';
import { FoldButton, useFold } from './fold';
import { scrollToSprint } from './SprintCard';
import { sessionProgress, stepRank, type ProjectView } from './projects';

interface Props {
  view: ProjectView;
  /** Zahl der aktiven Projekte mit Rang; ohne Wert keine Rang-Knöpfe (ruhend, erledigt, ABN). */
  count?: number;
  /** Grund der letzten Ablehnung von planningSet. */
  error?: string;
  onMove?: (to: number) => void;
  data: PlanningData;
  sel: string;
  renderSprint: (s: PlanSprint) => ReactNode;
}

/** Karte eines Projekts: Rang mit hoch/runter, Sprints in Reihenfolge mit Status und Fortschritt, nächste Session,
 *  darunter die Sprint-Karten der offenen Sprints und aufklappbar die Tickets ohne Sprint. */
export function ProjectCard({ view, count, error, onMove, data, sel, renderSprint }: Props) {
  const { project: p, next } = view;
  const up = count === undefined ? null : stepRank(p, -1, count);
  const down = count === undefined ? null : stepRank(p, 1, count);
  const [folded, toggle] = useFold(p.id);
  return (
    <div id={`pl-pj-${p.id}`} className={`pl-card pl-project is-${p.status}${folded ? ' is-folded' : ''}`}>
      <div className="pl-head pl-project-head">
        <FoldButton folded={folded} onToggle={toggle} what={`Projekt ${p.id}`} />
        <span className="pl-rank" title={p.rang === '–' ? 'ohne Rang' : `Rang ${p.rang}`}>{p.rang === '–' ? '·' : p.rang}</span>
        <strong className="pl-project-id">{p.id}</strong>
        <span className="pl-title">{p.title}</span>
        <Text size="1" color="gray">Sprints {view.done}/{view.total}</Text>
        {count !== undefined && (
          <Flex gap="1">
            <Button size="1" variant="soft" color="gray" disabled={up === null} onClick={() => up !== null && onMove?.(up)}>hoch</Button>
            <Button size="1" variant="soft" color="gray" disabled={down === null} onClick={() => down !== null && onMove?.(down)}>runter</Button>
          </Flex>
        )}
      </div>
      {error && <NoticeCard title="Rang nicht geändert" tone="error">{error}</NoticeCard>}
      <Flex gap="1" wrap="wrap" className="pl-chips">
        {view.sprints.map((s) => <SprintChip key={s.id} s={s} onPick={() => { if (folded) toggle(); setTimeout(() => scrollToSprint(s.id), 0); }} />)}
      </Flex>
      {!folded && next && (
        <Flex gap="2" align="center" wrap="wrap">
          <Text size="1">Nächste Session: <strong>{next.session.nr}</strong> {next.session.titel}</Text>
          <CopyPrompt prompt={promptSession(next.sprint, next.session)} what={`Session ${next.session.nr}`} />
        </Flex>
      )}
      {!folded && view.sprints.filter((s) => s.status !== 'erledigt').map(renderSprint)}
      {!folded && view.loose.length > 0 && (
        <details>
          <summary><Text size="1" color="gray">Tickets ohne Sprint ({view.loose.length})</Text></summary>
          <BacklogList data={data} tickets={view.loose} sel={sel} />
        </details>
      )}
    </div>
  );
}

/** Sprint im Projektkopf: eckiger Chip, damit er sich von den runden Status-Badges der Sessions abhebt (B-364). */
function SprintChip({ s, onPick }: { s: PlanSprint; onPick: () => void }) {
  const [done, all] = sessionProgress(s);
  return (
    <button type="button" className={`pl-chip is-${s.status}`} onClick={onPick}>
      {s.id} · {s.status} · {done}/{all}
    </button>
  );
}
