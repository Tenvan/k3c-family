import { Text } from '@radix-ui/themes';
import { useEffect, useRef, useState } from 'react';
import { backend, type Source } from '../api';
import { errorText } from '../lib/errors';
import { loadText, savePref } from '../lib/prefs';
import { NoticeCard } from '../ui/parts';
import { ConsoleView } from './ConsoleView';
import { pickSource, upsertSource } from './lines';
import { SourceBar } from './SourceBar';

/** Reiter `Logs` (B-064): Quellenleiste links, Konsole der gewählten Quelle rechts. */
export function LogsPage() {
  const [sources, setSources] = useState<Source[] | null>(null);
  const [error, setError] = useState('');
  const [wanted, setWanted] = useState(() => loadText('source', ''));
  const [open, setOpen] = useState(() => loadText('sourceBar', 'open') === 'open');
  const pending = useRef<Source[]>([]); // Meldungen, die vor der Liste kamen

  useEffect(() => {
    const off = backend.on('source:state', (src) =>
      setSources((list) => {
        if (!list) pending.current.push(src);
        return list && upsertSource(list, src);
      }),
    );
    backend.sources().then(
      (list) => setSources(pending.current.reduce(upsertSource, list)),
      (e) => setError(errorText(e)),
    );
    return off;
  }, []);

  if (error) return <NoticeCard title="Quellen nicht geladen" tone="error">{error}</NoticeCard>;
  if (!sources) return <Text color="gray">Lade Quellen …</Text>;
  const selected = pickSource(sources, wanted);
  const choose = (name: string) => {
    setWanted(name);
    savePref('source', name);
  };
  const toggle = () => {
    setOpen(!open);
    savePref('sourceBar', open ? 'closed' : 'open');
  };
  return (
    <div className="logs-page">
      <SourceBar sources={sources} selected={selected} open={open} onSelect={choose} onToggle={toggle} />
      {selected ? (
        <ConsoleView key={selected} name={selected} />
      ) : (
        <NoticeCard title="Keine Quellen" tone="neutral">Weder Dienste noch Läufe noch Log-Dateien.</NoticeCard>
      )}
    </div>
  );
}
