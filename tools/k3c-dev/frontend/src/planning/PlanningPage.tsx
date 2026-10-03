import { Button, Flex, SegmentedControl, Text } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { backend, type PlanDoc } from '../api';
import { errorText } from '../lib/errors';
import { loadPref, savePref } from '../lib/prefs';
import { MarkdownView } from '../ui/MarkdownView';
import { NoticeCard } from '../ui/parts';

const VIEWS = ['sprints', 'plan', 'fragen'] as const;
type View = (typeof VIEWS)[number];

/** Reiter `Planung`: Umschalter zwischen Sprints & Backlog, dem Plan und dem Fragenkatalog (docs/). */
export function PlanningPage() {
  const [view, setView] = useState<View>(() => loadPref('planning', VIEWS, 'sprints'));
  const choose = (v: string) => {
    if (!VIEWS.includes(v as View)) return;
    setView(v as View);
    savePref('planning', v);
  };
  return (
    <div className="pl-page">
      <SegmentedControl.Root value={view} onValueChange={choose} size="2">
        <SegmentedControl.Item value="sprints">Sprints &amp; Backlog</SegmentedControl.Item>
        <SegmentedControl.Item value="plan">Plan</SegmentedControl.Item>
        <SegmentedControl.Item value="fragen">Fragenkatalog</SegmentedControl.Item>
      </SegmentedControl.Root>
      {view === 'sprints' ? <SprintsAndBacklog /> : <DocView key={view} name={view} />}
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
    return backend.on('planning:changed', load); // Plan oder Fragenkatalog wurde gespeichert
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

/** Sprints & Backlog: die von Go erzeugte Seite (internal/planning/page.html) im iframe; Filter und Auswahl hält die
 *  Seite selbst im sessionStorage, damit sie das Neuladen nach einer Änderung in docs/ übersteht. */
function SprintsAndBacklog() {
  const [html, setHtml] = useState<string | null>(null);
  const [error, setError] = useState('');
  useEffect(() => {
    const load = () => backend.planningPage().then((h) => { setHtml(h); setError(''); }, (e) => setError(errorText(e)));
    void load();
    return backend.on('planning:changed', () => void load()); // Wächter in Go: Datei in docs/ geändert
  }, []);
  if (error) return <NoticeCard title="Planung nicht geladen" tone="error">{error}</NoticeCard>;
  if (html === null) return <Text color="gray">Lade Planung …</Text>;
  return <iframe className="pl-frame" title="Sprints & Backlog" srcDoc={html} />;
}
