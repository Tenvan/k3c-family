import { Button, Flex, Select, TextField } from '@radix-ui/themes';
import { useLayoutEffect, useRef, useState } from 'react';
import type { LogEntry } from '../api';
import { formatTime } from '../lib/format';
import { useDebounced } from '../lib/useDebounced';
import { NoticeCard, StatusBadge } from '../ui/parts';
import { budgetNote, entryKey, footer, LEVEL_FILTERS, levelTone, LIMITS, oldestFirst, prettyData } from './logview';
import { useLogQuery } from './useLogQuery';

const BOTTOM_PX = 24;
const ALL = 'ALLE'; // Radix Select erlaubt keinen leeren Wert

/** Reiter `Log` (B-064): Filterleiste, Tabelle älteste oben, läuft mit, solange unten. */
export function LogTab({ name }: { name: string }) {
  const [minLevel, setMinLevel] = useState('');
  const [ns, setNs] = useState('');
  const [pattern, setPattern] = useState('');
  const [limit, setLimit] = useState<number>(LIMITS[0]);
  const bottom = useRef(true);
  const [atBottom, setAtBottom] = useState(true);
  const box = useRef<HTMLDivElement>(null);
  const q = { minLevel, ns: useDebounced(ns), pattern: useDebounced(pattern), limit };
  const { view, error, reload } = useLogQuery(name, q, bottom);
  const rows = view ? oldestFirst(view.entries) : [];

  useLayoutEffect(() => {
    if (bottom.current && box.current) box.current.scrollTop = box.current.scrollHeight;
  }, [view]);
  const onScroll = () => {
    const el = box.current;
    if (!el) return;
    bottom.current = el.scrollHeight - el.scrollTop - el.clientHeight < BOTTOM_PX;
    setAtBottom(bottom.current);
  };

  return (
    <section className="logtab">
      <Flex className="logtab-filters" gap="2" align="center" wrap="wrap">
        <Select.Root value={minLevel || ALL} onValueChange={(v) => setMinLevel(v === ALL ? '' : v)}>
          <Select.Trigger aria-label="Mindest-Level" />
          <Select.Content>
            {LEVEL_FILTERS.map((l) => (
              <Select.Item key={l || ALL} value={l || ALL}>{l ? `ab ${l}` : 'Alle'}</Select.Item>
            ))}
          </Select.Content>
        </Select.Root>
        <TextField.Root placeholder="Namespace" value={ns} onChange={(e) => setNs(e.target.value)} />
        <TextField.Root className="logtab-search" placeholder="Suche (regulärer Ausdruck auf msg)" value={pattern}
          onChange={(e) => setPattern(e.target.value)} color={error ? 'red' : undefined} />
        <Select.Root value={String(limit)} onValueChange={(v) => setLimit(Number(v))}>
          <Select.Trigger aria-label="Limit" />
          <Select.Content>
            {LIMITS.map((l) => <Select.Item key={l} value={String(l)}>{l}</Select.Item>)}
          </Select.Content>
        </Select.Root>
        <Button variant="soft" onClick={() => void reload()}>Aktualisieren</Button>
      </Flex>
      {error && <p className="svc-error">{error}</p>}
      {view?.missing && <NoticeCard title="Log-Datei fehlt" tone="neutral">logs/{name}.jsonl gibt es noch nicht.</NoticeCard>}
      <div className="logtab-body" ref={box} onScroll={onScroll}>
        <table className="logtab-table">
          <tbody>
            {rows.map((e) => <LogRow key={entryKey(e)} e={e} />)}
          </tbody>
        </table>
      </div>
      {view && (
        <p className="logtab-foot">
          {footer(view, rows.length, atBottom)}
          {budgetNote(view) && <span className="logtab-budget"> · {budgetNote(view)}</span>}
        </p>
      )}
    </section>
  );
}

function LogRow({ e }: { e: LogEntry }) {
  const [open, setOpen] = useState(false);
  const data = prettyData(e.data);
  return (
    <>
      <tr className={data ? 'logrow logrow-data' : 'logrow'} onClick={data ? () => setOpen(!open) : undefined}>
        <td className="logrow-time">{formatTime(new Date(e.time), true)}</td>
        <td><StatusBadge tone={levelTone(e.level)}>{e.level}</StatusBadge></td>
        <td className="logrow-ns">{e.ns}</td>
        <td className="logrow-msg">{data && <span className="logrow-arrow">{open ? '▾' : '▸'} </span>}{e.msg}</td>
      </tr>
      {open && (
        <tr className="logrow-detail">
          <td colSpan={4}><pre>{data}</pre></td>
        </tr>
      )}
    </>
  );
}
