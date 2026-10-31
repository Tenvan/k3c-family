import { Button, Flex, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { FacetRail, TransportBar } from '@webkit/organisms';
import { useRef, useState } from 'react';
import type { AssetKind, AssetSlot } from '../api';
import { endSourcesChat } from '../chat/chatBridge';
import * as prefs from '../lib/prefs';
import '../styles/assetpage.css';
import { DownloadDialog } from '../sources/SourceDialogs';
import { formatBytes } from '../sources/sourcesLogic';
import { Inspector, type InspectorProps } from './Inspector';
import { takePendingSlot } from './jump';
import { ListPane } from './ListPane';
import { PageHeader } from './PageHeader';
import { PageLayout } from './PageLayout';
import { pageArt, stepSlot } from './pageLogic';
import { ToolsPanel } from './ToolsPanel';
import { changeVolume, stopPlayback, useTransport } from './transport';
import { useAssetData } from './useAssetData';
import { useDownloads } from './useDownloads';
import { usePlayActions } from './usePlayActions';
import { usePageEffects } from './usePageEffects';
import { usePageKeys } from './usePageKeys';
import { useViewActions } from './useViewActions';
import { usePageModel } from './usePageModel';
import { chipFromView, uniqueChip, useCustomChips, useViewState } from './viewState';


/** Auswahl-Leiste der Sicht Dateien (nur bei Auswahl): Anzahl und Größe, Laden und zuordnen oder nur laden. */
function DownloadBar({ count, bytes, blocked, onAsk, onClear }: { count: number; bytes: number; blocked: boolean; onAsk: (mode: 'laden' | 'zuordnen') => void; onClear: () => void }) {
  return (
    <Flex gap="3" align="center" wrap="wrap" className="src-actions" data-testid="selection-bar">
      <Text data-testid="selection-sum">{count} gewählt · {formatBytes(bytes)}</Text>
      <Button size="1" disabled={blocked} onClick={() => onAsk('zuordnen')}>Laden und zuordnen</Button>
      <Button size="1" variant="soft" disabled={blocked} onClick={() => onAsk('laden')}>Nur laden</Button>
      <Button size="1" variant="soft" color="gray" onClick={onClear}>Auswahl leeren</Button>
    </Flex>
  );
}

/** Rechte Spalte: Werkzeuge, sonst der Inspektor des gewählten Slots, sonst ein Hinweis. */
function SidePane({ tools, slot, used, offline, onCloseTools, onSave, onReload }: { tools: boolean; slot: AssetSlot | undefined; used: ReadonlySet<string>; offline: boolean; onCloseTools: () => void; onSave: InspectorProps['onSave']; onReload: () => void }) {
  return (
    <section className="ap-side" aria-label={tools ? 'Werkzeuge' : 'Inspektor'}>
      {tools ? <ToolsPanel onClose={onCloseTools} onChanged={onReload} offline={offline} />
        : slot ? <Inspector key={slot.id} slot={slot} used={used} onSave={onSave} onReload={onReload} />
        : <NoticeCard title="Kein Asset-Slot gewählt" tone="neutral">Links einen Slot wählen: Belegung, Kandidaten und Details erscheinen hier.</NoticeCard>}
    </section>
  );
}

/** Seite Assets (B-677): ersetzt Grafik, Sound und Quellen. Fokus-Chips, Sichten Slots, Dateien und Szene, Facetten links,
 *  Inspektor oder Werkzeuge rechts, eine Abspielleiste unten. `onArt` meldet dem Rahmen die Art für die Vorschlagskarten des Chats. */
export function AssetsPage({ onArt }: { onArt?: (art: AssetKind | undefined) => void }) {
  const d = useAssetData();
  const [view, patch] = useViewState();
  const custom = useCustomChips();
  const jump = useRef(takePendingSlot());
  const [selected, setSelected] = useState(() => jump.current || prefs.loadText('assets.selected', ''));
  const [tools, setTools] = useState(false);
  const [notice, setNotice] = useState('');
  const m = usePageModel(d.rows, view, custom.chips);
  const dl = useDownloads(d.sources, d.sources.view?.vorschlaege ?? [], d.reload);
  const transport = useTransport();
  const slot = d.rows.slots.find((r) => r.slot.id === selected)?.slot;
  const art = pageArt(slot, view.facets);

  usePageEffects({ jump, view, patch, loaded: d.loaded, quellen: m.quellen, slot, art, onArt, onNotice: setNotice });

  const pick = (id: string) => { setSelected(id); prefs.savePref('assets.selected', id); endSourcesChat(); };
  const { playSlot, playFileRow, playSelected } = usePlayActions(d.rows);
  const actions = useViewActions({ view, patch, chips: m.chips, quellen: m.quellen, onNotice: setNotice });
  usePageKeys({ onMove: (delta) => pick(stepSlot(m.slotGroups, selected, delta) || selected), onPlay: () => playSelected(selected) });

  if (!d.assets.view) return d.assets.error ? <NoticeCard title="Slot-Register nicht lesbar" tone="error">{d.assets.error}</NoticeCard> : <Text color="gray">Lade Slot-Register …</Text>;
  if (d.assets.view.error) return <NoticeCard title="Slot-Register ungültig" tone="error">{d.assets.view.error}</NoticeCard>;
  const error = d.assets.error || d.sources.error || dl.error;
  return (
    <div className="ap-page" data-testid="assets-page">
      <PageHeader chips={m.chipOptions} chip={m.chip} onChip={actions.choose} onRemoveChip={custom.remove} suggestedName={m.suggested}
        onSaveChip={(name) => custom.add(uniqueChip(chipFromView(name, view), custom.chips))} query={view.query} onQuery={(query) => patch({ query })} sicht={view.sicht} onSicht={(sicht) => patch({ sicht })}
        gruppe={view.gruppe} onGruppe={(gruppe) => patch({ gruppe })} tools={tools} onTools={() => setTools((t) => !t)} onReload={d.reload} notice={notice} />
      {d.offline && <NoticeCard title="Offline" tone="warn">Kein Internet: Geladenes bleibt sichtbar und spielbar, Netz-Dateien sind nicht ladbar.</NoticeCard>}
      {error && <Text color="red" role="alert">{error}</Text>}
      {dl.picked.size > 0 && <DownloadBar count={dl.selection.count} bytes={dl.selection.bytes} blocked={d.offline || dl.busy} onAsk={dl.ask} onClear={dl.clear} />}
      <PageLayout
        rail={<FacetRail facets={m.facets} selected={view.facets} onToggle={actions.toggle} onClear={actions.clear} />}
        list={<ListPane sicht={view.sicht} art={art} slotGroups={m.slotGroups} fileGroups={m.fileGroups} selected={selected} picked={dl.picked} offline={d.offline} netFocus={m.chip === 'netz'}
          onSelect={pick} onPlaySlot={playSlot} onPlayFile={playFileRow} onPick={dl.toggle} onTools={() => setTools(true)} />}
        side={<SidePane tools={tools} slot={slot} used={d.used} offline={d.offline} onCloseTools={() => setTools(false)} onSave={d.assets.save} onReload={d.reload} />}
      />
      <TransportBar playing={transport.playing} status={transport.status} volume={transport.volume} onStop={stopPlayback} onFade={() => playSelected(selected)} onVolume={(dir) => void changeVolume(dir)} />
      <DownloadDialog open={dl.confirm !== null} selection={dl.selection} onClose={() => dl.ask(null)} onConfirm={dl.start} />
    </div>
  );
}
