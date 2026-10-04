import { Button, Flex, SegmentedControl, Text } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { backend, type PlanDoc } from '../api';
import { errorText } from '../lib/errors';
import { loadText, savePref } from '../lib/prefs';
import { MarkdownView } from '../ui/MarkdownView';
import { NoticeCard } from '../ui/parts';
import { SprintsBacklog } from './SprintsBacklog';

const LABEL: Record<PlanDoc, string> = { plan: 'Plan', fragen: 'Fragenkatalog', glossar: 'Glossar' };

/** Reiter `Planung`: Sprints & Backlog und die vorhandenen Dokumente aus docs/ (Plan, Fragenkatalog, Glossar). Die
 *  Liste der Dokumente kommt aus Go und wird bei `planning:changed` neu gelesen: ein neues glossar.md erscheint so von selbst. */
export function PlanningPage() {
  const [docs, setDocs] = useState<PlanDoc[]>([]);
  const [view, setView] = useState(() => loadText('planning', 'sprints'));
  useEffect(() => {
    const load = () => backend.planningDocs().then(setDocs, () => setDocs([]));
    void load();
    return backend.on('planning:changed', () => void load());
  }, []);
  const choose = (v: string) => {
    setView(v);
    savePref('planning', v);
  };
  const doc = docs.find((d) => d === view); // gemerkte Ansicht ohne Datei → Sprints & Backlog
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
  useEffect(() => {
    load();
    return backend.on('planning:changed', load); // Dokument wurde gespeichert
  }, [name]);
  if (error) return <NoticeCard title="Dokument nicht geladen" tone="error">{error}</NoticeCard>;
  if (text === null) return <Text color="gray">Lade …</Text>;
  return (
    <article className="pl-doc">
      <Flex justify="end"><Button size="1" variant="soft" color="gray" onClick={load}>Neu laden</Button></Flex>
      <MarkdownView source={text} />
    </article>
  );
}
