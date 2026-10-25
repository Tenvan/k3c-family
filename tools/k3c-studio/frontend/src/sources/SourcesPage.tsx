import { Badge, Button, Flex, Heading, Text } from '@webkit/atoms';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { CheckField, ListFilter, NoticeCard } from '@webkit/molecules';
import { useMemo, useState } from 'react';
import type { SourceInfo, SourceProposal } from '../api';
import { loadJSON, saveJSON } from '../lib/prefs';
import { ProposalRow } from './ProposalRow';
import { CleanupDialog, DownloadDialog } from './SourceDialogs';
import { takePendingSlot } from './jump';
import { downloadable, formatBytes, running, sourceFilter, summarize } from './sourcesLogic';
import { useSources } from './useSources';

function Overview({ quellen }: { quellen: SourceInfo[] }) {
  return (
    <Flex gap="3" wrap="wrap" data-testid="source-overview">
      {quellen.map((q) => (
        <Flex key={q.name} direction="column" gap="1" style={{ border: '1px solid var(--gray-6)', borderRadius: 6, padding: 8, minWidth: 200, maxWidth: 260 }}>
          <Text weight="bold">{q.name}</Text>
          <Text size="1" color="gray">{q.lizenzregeln}</Text>
          <Flex gap="2" wrap="wrap">
            <Badge variant="soft">{q.vorgeschlagen} vorgeschlagen</Badge>
            <Badge variant="soft" color="green">{q.geladen} geladen</Badge>
            <Badge variant="soft" color="blue">{q.verwendet} verwendet</Badge>
          </Flex>
        </Flex>
      ))}
    </Flex>
  );
}

/** Seite Quellen (B-601, WZ30.4): Übersicht je Quelle, Vorschlagsliste mit Filter, Auswahl mit Größensumme, Download nach
 *  Bestätigung mit Fortschritt, Offline-Anzeige und Aufräumen des Vorrats. Für Ton und Grafik gemeinsam. */
export function SourcesPage() {
  useTicketPage('quellen');
  const s = useSources();
  const [slot, setSlot] = useState(takePendingSlot);
  const [picked, setPicked] = useState<ReadonlySet<string>>(new Set());
  const [confirm, setConfirm] = useState(false);
  const [cleanup, setCleanup] = useState(false);
  const all = s.view?.vorschlaege;
  const items = useMemo(() => (all ?? []).filter((p) => !slot || p.slot === slot), [all, slot]);
  const sel = useMemo(() => summarize(all ?? [], picked), [all, picked]);
  if (!s.view) return s.error ? <NoticeCard title="Quellen nicht lesbar" tone="error">{s.error}</NoticeCard> : <Text color="gray">Lade Quellen …</Text>;
  const { offline } = s.view;
  const busy = running([...s.jobs.values()]);
  const toggle = (ids: string[], on: boolean) => setPicked((cur) => { const n = new Set(cur); ids.forEach((id) => (on ? n.add(id) : n.delete(id))); return n; });
  const start = async () => {
    setConfirm(false);
    const ids = [...picked];
    setPicked(new Set());
    await s.download(ids);
  };
  return (
    <Flex direction="column" gap="3" p="3" style={{ overflow: 'auto', minHeight: 0 }}>
      <Heading size="4">Quellen</Heading>
      {offline && <NoticeCard title="Offline" tone="warn">Kein Internet: Geladenes ist weiter sichtbar und spielbar, Ungeladenes ist ausgegraut und nicht ladbar.</NoticeCard>}
      {s.error && <Text color="red" role="alert">{s.error}</Text>}
      <Overview quellen={s.view.quellen} />
      <Flex gap="3" align="center" wrap="wrap">
        <Text data-testid="selection-sum">{sel.count} gewählt, {formatBytes(sel.bytes)}</Text>
        <Button disabled={offline || busy || sel.count === 0} onClick={() => setConfirm(true)}>Herunterladen</Button>
        <Button variant="soft" color="gray" disabled={busy} onClick={() => setCleanup(true)}>Vorrat aufräumen</Button>
        <Button variant="soft" color="gray" onClick={() => void s.reload()}>Neu laden</Button>
        {slot && <Button size="1" variant="soft" onClick={() => setSlot('')} aria-label="Slot-Filter entfernen">Slot: {slot} ✕</Button>}
        {busy && <Text size="2" color="gray" role="status">Download läuft …</Text>}
      </Flex>
      <ListFilter
        items={items} {...sourceFilter} itemKey={(p) => p.id} storageKey="quellen.filter" prefs={{ loadJSON, saveJSON }} defaultSort="prio"
        emptyHint="Keine Vorschläge in der Liste."
        renderList={(list) => <Rows list={list} offline={offline} picked={picked} jobs={s.jobs} onToggle={toggle} onAdopt={(id) => void s.adopt(id)} />}
      />
      <DownloadDialog open={confirm} selection={sel} onClose={() => setConfirm(false)} onConfirm={start} />
      <CleanupDialog open={cleanup} onClose={() => setCleanup(false)} onDone={() => { setCleanup(false); void s.reload(); }} />
    </Flex>
  );
}

interface RowsProps {
  list: SourceProposal[];
  offline: boolean;
  picked: ReadonlySet<string>;
  jobs: ReturnType<typeof useSources>['jobs'];
  onToggle: (ids: string[], on: boolean) => void;
  onAdopt: (id: string) => void;
}

function Rows({ list, offline, picked, jobs, onToggle, onAdopt }: RowsProps) {
  const ladbar = list.filter(downloadable).map((p) => p.id);
  const allOn = ladbar.length > 0 && ladbar.every((id) => picked.has(id));
  return (
    <Flex direction="column">
      {!offline && ladbar.length > 0 && <CheckField size="1" label={`Alle ladbaren wählen (${ladbar.length})`} checked={allOn} onCheckedChange={(c) => onToggle(ladbar, c === true)} />}
      {list.map((p) => (
        <ProposalRow key={p.id} s={p} offline={offline} checked={picked.has(p.id)} job={jobs.get(p.id)} onCheck={(on) => onToggle([p.id], on)} onAdopt={() => onAdopt(p.id)} />
      ))}
    </Flex>
  );
}
