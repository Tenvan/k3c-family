import { useState } from 'react';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { backend, type PlanDoc } from '../api';
import { Button, Flex, SegmentedControl, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { NoticeCard } from '@webkit/molecules';
import { PLAN_RELOAD_MS, useLiveReload } from '../lib/useLiveReload';
import { loadText, savePref } from '../lib/prefs';
import { MarkdownView } from '../ui/molecules/MarkdownView';
import { SprintsBacklog } from './SprintsBacklog';

const LABEL: Record<PlanDoc, string> = { plan: 'Plan', fragen: 'Fragenkatalog', glossar: 'Glossar' };

/** Reiter `Planung`: Sprints & Backlog und die vorhandenen Dokumente aus docs/ (Plan, Fragenkatalog, Glossar). Die
 *  Liste der Dokumente kommt aus Go und wird bei `planning:changed` neu gelesen: ein neues glossar.md erscheint so von selbst. */
export function PlanningPage() {
  const [docs, setDocs] = useState<PlanDoc[]>([]);
  const [view, setView] = useState(() => loadText('planning.view', 'sprints'));
  useLiveReload(['planning:changed'], () => void backend.planningDocs().then(setDocs, () => setDocs([])), PLAN_RELOAD_MS);
  const choose = (v: string) => {
    setView(v);
    savePref('planning.view', v);
  };
  const doc = docs.find((d) => d === view); // gemerkte Ansicht ohne Datei → Sprints & Backlog
  useTicketPage('planung', doc ?? 'sprints');
  return (
    <div className="pl-page">
      <SegmentedControl.Root value={doc ?? 'sprints'} onValueChange={choose} size="2">
        <SegmentedControl.Item value="sprints">Sprints &amp; Backlog</SegmentedControl.Item>
        {docs.map((d) => <SegmentedControl.Item key={d} value={d}>{LABEL[d]}</SegmentedControl.Item>)}
      </SegmentedControl.Root>
      {doc ? <DocView key={doc} name={doc} /> : <SprintsBacklog />}
    </div>
  );
}

function DocView({ name }: { name: PlanDoc }) {
  const [text, setText] = useState<string | null>(null);
  const [error, setError] = useState('');
  const load = () => {
    setError('');
    backend.planningDoc(name).then(setText, (e) => setError(errorText(e)));
  };
  useLiveReload(['planning:changed'], load, PLAN_RELOAD_MS); // Dokument wurde gespeichert; DocView hat key={name}
  if (error) return <NoticeCard title="Dokument nicht geladen" tone="error">{error}</NoticeCard>;
  if (text === null) return <Text color="gray">Lade …</Text>;
  return (
    <article className="pl-doc">
      <Flex justify="end"><Button size="1" variant="soft" color="gray" onClick={load}>Neu laden</Button></Flex>
      <MarkdownView source={text} />
    </article>
  );
}
