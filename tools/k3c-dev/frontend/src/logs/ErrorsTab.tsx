import { Flex, SegmentedControl } from '@radix-ui/themes';
import { useCallback, useEffect, useState } from 'react';
import { backend, type ErrorsView } from '../api';
import { errorText } from '../lib/errors';
import { formatNumber } from '../lib/format';
import { ActionButton, NoticeCard, StatusBadge } from '../ui/parts';
import { budgetNote, footer, levelTone, oneLine, timeWindow } from './logview';

const EXPLAIN =
  'Gleichartige Meldungen der letzten 24 Stunden stehen je in einer Zeile: Zahlen, Pfade, IDs und Texte sind im ' +
  'Fingerabdruck maskiert, das Beispiel ist die neueste Meldung.';

/** Reiter `Fehler (verdichtet)` (B-064): Gruppen aus logs.Digest, häufigste zuerst. */
export function ErrorsTab({ name }: { name: string }) {
  const [level, setLevel] = useState('WARN');
  const [view, setView] = useState<ErrorsView | null>(null);
  const [error, setError] = useState('');
  const load = useCallback(async () => {
    try {
      setView(await backend.logsErrors(name, level));
      setError('');
    } catch (e) {
      setError(errorText(e));
    }
  }, [name, level]);
  useEffect(() => void load(), [load]);

  return (
    <section className="errtab">
      <p className="errtab-explain">{EXPLAIN}</p>
      <Flex gap="3" align="center">
        <SegmentedControl.Root value={level} onValueChange={setLevel}>
          <SegmentedControl.Item value="WARN">ab WARN</SegmentedControl.Item>
          <SegmentedControl.Item value="ERROR">nur ERROR</SegmentedControl.Item>
        </SegmentedControl.Root>
        <ActionButton variant="soft" onClick={load}>Aktualisieren</ActionButton>
      </Flex>
      {error && <p className="svc-error">{error}</p>}
      {view?.missing && <NoticeCard title="Log-Datei fehlt" tone="neutral">logs/{name}.jsonl gibt es noch nicht.</NoticeCard>}
      {view && !view.missing && view.groups.length === 0 && (
        <NoticeCard title="Nichts gefunden" tone="ok">Keine Meldungen ab {level} in den letzten 24 Stunden.</NoticeCard>
      )}
      <div className="errtab-list">
        {view?.groups.map((g) => (
          <div key={g.ns + g.fingerprint} className="errrow">
            <span className="errrow-count">{formatNumber(g.count)}×</span>
            <StatusBadge tone={levelTone(g.level)}>{g.level}</StatusBadge>
            <span className="logrow-ns">{g.ns}</span>
            <div className="errrow-text">
              <div className="errrow-example" title={g.example}>{oneLine(g.example)}</div>
              <div className="errrow-fp">{g.fingerprint}</div>
            </div>
            <span className="errrow-window">{timeWindow(g.first, g.last)}</span>
          </div>
        ))}
      </div>
      {view && (
        <p className="logtab-foot">
          {footer(view, view.entries, false)}
          {budgetNote(view) && <span className="logtab-budget"> · {budgetNote(view)}</span>}
        </p>
      )}
    </section>
  );
}
