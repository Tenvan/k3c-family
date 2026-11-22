import { Button, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { CompareView } from './CompareView';
import type { DetailActions } from './DetailHead';
import { entryLabel, type ListEntry } from './entries';
import { ReportDetail } from './ReportDetail';
import { RunDetail } from './RunDetail';
import { TuningView } from './TuningView';
import { useSummaries } from './useSummaries';
import { useTuning } from './useBalancing';

/** Abstimm-Ergebnis (WZ28.6). */
function TuningDetail({ path }: { path: string }) {
  const t = useTuning(path);
  if (t.error) return <NoticeCard title="Ergebnis nicht lesbar" tone="error">{t.error}</NoticeCard>;
  if (!t.tuning) return <Text color="gray">Lade Ergebnis …</Text>;
  return <TuningView name={path} tuning={t.tuning} />;
}

interface CompareProps {
  paths: string[];
  entries: ListEntry[];
  onMakeBase: (path: string) => void;
  onOpen: (path: string) => void;
}

/** Vergleich der gewählten Berichte; A ist der erste Pfad. */
function CompareDetail({ paths, entries, onMakeBase, onOpen }: CompareProps) {
  const { byPath, error } = useSummaries(paths);
  if (error) return <NoticeCard title="Bericht nicht lesbar" tone="error">{error}</NoticeCard>;
  if (!byPath) return <Text color="gray">Lade Berichte …</Text>;
  const items = paths.flatMap((path) => {
    const entry = entries.find((e) => e.id === path);
    return byPath[path] ? [{ path, name: entry ? entryLabel(entry) : path, info: entry?.type === 'report' ? entry.report : undefined, summary: byPath[path] }] : [];
  });
  return <CompareView items={items} onMakeBase={onMakeBase} onOpen={onOpen} />;
}

interface Props extends CompareProps {
  compare: boolean;
  selected: string;
  actions: DetailActions;
  onStop: (id: string) => Promise<void>;
  onNew: () => void;
}

/** Rechte Seite: Vergleich, Lauf, Abstimm-Ergebnis oder Bericht, je nach Auswahl. */
export function BalancingDetail({ compare, selected, entries, paths, actions, onStop, onNew, onMakeBase, onOpen }: Props) {
  if (compare) return <CompareDetail paths={paths} entries={entries} onMakeBase={onMakeBase} onOpen={onOpen} />;
  const entry = entries.find((e) => e.id === selected);
  if (!entry) {
    return (
      <>
        <NoticeCard title="Nichts gewählt" tone="neutral">In der Liste einen Lauf oder Bericht öffnen, oder mit dem Kästchen mehrere Berichte wählen und vergleichen.</NoticeCard>
        <div><Button onClick={onNew}>Neuer Lauf</Button></div>
      </>
    );
  }
  if (entry.type === 'run') return <RunDetail run={entry.run} onStop={onStop} />;
  if (entry.report.kind === 'tuning') return <TuningDetail path={entry.report.path} />;
  return <ReportDetail key={entry.id} entry={entry} actions={actions} />;
}
