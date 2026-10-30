import { Button, Flex, Heading, Text } from '@webkit/atoms';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { NoticeCard } from '@webkit/molecules';
import { useEffect, useMemo, useState } from 'react';
import type { SourceAddView, SourceInfo, SourceProposal } from '../api';
import '../styles/sources.css';
import { endSourcesChat, startSourcesChat } from '../chat/chatBridge';
import { loadJSON, saveJSON } from '../lib/prefs';
import { CleanupDialog, DownloadDialog, ResearchDialog } from './SourceDialogs';
import { SourceEmpty, SourceList } from './SourceList';
import { SourceTabs } from './SourceTabs';
import { SourcesFilterBar } from './SourcesFilterBar';
import { takePendingSlot } from './jump';
import { applyFilter, DEFAULT_FILTER, groupBySlot, isSourceFilter, searchPrompt, type SourceFilter } from './sourcesFilter';
import { formatBytes, running, summarize, type Selection } from './sourcesLogic';
import { useSources } from './useSources';

const FILTER_KEY = 'quellen.filter';

/** Gemerkter Filter (ohne Freitext, Workbench-Spec › Gemerkter Zustand); ein Sprung aus anderen Seiten setzt den Slot. */
function initialFilter(): SourceFilter {
  const slot = takePendingSlot();
  const saved = { ...loadJSON(FILTER_KEY, isSourceFilter, DEFAULT_FILTER), query: '' };
  return slot ? { ...saved, slot, tab: '', status: 'alle' } : saved;
}

/** Aktionsleiste (B-673): nur bei Auswahl, mit Anzahl und Größe. */
function SelectionBar({ sel, blocked, onClear, onDownload }: { sel: Selection; blocked: boolean; onClear: () => void; onDownload: () => void }) {
  return (
    <Flex gap="3" align="center" wrap="wrap" className="src-actions" data-testid="selection-bar">
      <Text data-testid="selection-sum">{sel.count} gewählt · {formatBytes(sel.bytes)}</Text>
      <Button size="1" variant="soft" color="gray" onClick={onClear}>Auswahl leeren</Button>
      <Button size="1" disabled={blocked} onClick={onDownload}>Herunterladen</Button>
    </Flex>
  );
}

function Status({ busy, done }: { busy: boolean; done: SourceAddView | null }) {
  return (
    <>
      {busy && <Text size="2" color="gray" role="status">Download läuft …</Text>}
      {done && <Text size="2" role="status" data-testid="research-done">Aus Recherche: {done.uebernommen.length} übernommen, {done.uebersprungen.length} übersprungen.</Text>}
    </>
  );
}

const NONE: SourceProposal[] = [];

/** Zahl der Vorschläge im Reiter ("" = alle). */
const countIn = (quellen: readonly SourceInfo[], tab: string, total: number): number => (tab ? quellen.find((q) => q.name === tab)?.vorgeschlagen ?? 0 : total);

const sorted = (values: Iterable<string>): string[] => [...new Set(values)].sort((a, b) => a.localeCompare(b));

/** Seite Quellen (B-601, B-673): Quellen als Reiter, eine Filterzeile, Liste nach Slot gruppiert, Aktionsleiste nur bei
 *  Auswahl, Download nach Bestätigung mit Fortschritt, Vorschläge füllen (Claude suchen lassen, aus Recherche übernehmen),
 *  Offline-Anzeige und Aufräumen des Vorrats. Für Ton und Grafik gemeinsam. */
export function SourcesPage() {
  useTicketPage('quellen');
  const s = useSources();
  const [filter, setFilter] = useState(initialFilter);
  const [folded, setFolded] = useState<ReadonlySet<string>>(new Set());
  const [picked, setPicked] = useState<ReadonlySet<string>>(new Set());
  const [confirm, setConfirm] = useState(false);
  const [cleanup, setCleanup] = useState(false);
  const [research, setResearch] = useState(false);
  const [done, setDone] = useState<SourceAddView | null>(null);
  useEffect(() => saveJSON(FILTER_KEY, { ...filter, query: '' }), [filter]);
  useEffect(() => endSourcesChat, []);
  const all = s.view?.vorschlaege ?? NONE;
  const items = useMemo(() => applyFilter(all, filter), [all, filter]);
  const groups = useMemo(() => groupBySlot(items, all), [items, all]);
  const sel = useMemo(() => summarize(all, picked), [all, picked]);
  if (!s.view) return s.error ? <NoticeCard title="Quellen nicht lesbar" tone="error">{s.error}</NoticeCard> : <Text color="gray">Lade Quellen …</Text>;
  const { offline, quellen } = s.view;
  const busy = running([...s.jobs.values()]);
  const patch = (p: Partial<SourceFilter>) => setFilter((f) => ({ ...f, ...p }));
  const toggle = (ids: string[], on: boolean) => setPicked((cur) => { const n = new Set(cur); ids.forEach((id) => (on ? n.add(id) : n.delete(id))); return n; });
  const fold = (slot: string) => setFolded((cur) => { const n = new Set(cur); if (!n.delete(slot)) n.add(slot); return n; });
  const start = async () => {
    setConfirm(false);
    const ids = [...picked];
    setPicked(new Set());
    await s.download(ids);
  };
  const inTab = countIn(quellen, filter.tab, all.length);
  const search = () => startSourcesChat(searchPrompt(filter.tab, filter.slot, filter.art));
  return (
    <Flex direction="column" gap="2" p="3" style={{ overflow: 'auto', minHeight: 0 }}>
      <Heading size="4">Quellen</Heading>
      {offline && <NoticeCard title="Offline" tone="warn">Kein Internet: Geladenes ist weiter sichtbar und spielbar, Ungeladenes ist ausgegraut und nicht ladbar; Aus Recherche übernehmen ist gesperrt (Größe per HEAD nicht möglich).</NoticeCard>}
      {s.error && <Text color="red" role="alert">{s.error}</Text>}
      <SourceTabs quellen={quellen} total={all.length} tab={filter.tab} onTab={(tab) => patch({ tab })} offline={offline} showActions={inTab > 0} onSearch={search} onResearch={() => setResearch(true)} />
      <SourcesFilterBar
        filter={filter} onChange={patch} slots={sorted(all.map((p) => p.slot))} licenses={sorted(all.map((p) => p.lizenz))}
        busy={busy} onCleanup={() => setCleanup(true)} onReload={() => void s.reload()}
      />
      {sel.count > 0 && <SelectionBar sel={sel} blocked={offline || busy} onClear={() => setPicked(new Set())} onDownload={() => setConfirm(true)} />}
      <Status busy={busy} done={done} />
      {groups.length === 0
        ? <SourceEmpty source={filter.tab} filtered={inTab > 0} offline={offline} onSearch={search} onResearch={() => setResearch(true)} onReset={() => setFilter({ ...DEFAULT_FILTER, tab: filter.tab })} />
        : <SourceList groups={groups} collapsed={folded} offline={offline} picked={picked} jobs={s.jobs} onFold={fold} onToggle={toggle} onAdopt={(id) => void s.adopt(id)} />}
      <DownloadDialog open={confirm} selection={sel} onClose={() => setConfirm(false)} onConfirm={start} />
      <CleanupDialog open={cleanup} onClose={() => setCleanup(false)} onDone={() => { setCleanup(false); void s.reload(); }} />
      <ResearchDialog open={research} source={filter.tab} onClose={() => setResearch(false)} onDone={(v) => { setResearch(false); setDone(v); void s.reload(); }} />
    </Flex>
  );
}
