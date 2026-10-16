import { Button, SegmentedControl, Select, Text, TextField } from '@radix-ui/themes';
import { useMemo, useState, type CSSProperties } from 'react';
import type { AssetKind, AssetSlot, AssetStatus } from '../api';
import { loadPref, loadText, savePref } from '../lib/prefs';
import { Splitter, useSplit } from '../tasks/TaskBars';
import { NoticeCard, StatusBadge, type Tone } from '../ui/parts';
import { CATEGORIES, countStatus, filterSlots } from './assets';
import { SlotDetail } from './SlotDetail';
import { useAssets } from './useAssets';

const KINDS = ['grafik', 'sound'] as const;
const STATUS_TONE: Record<AssetStatus, Tone> = { zugeordnet: 'ok', luecke: 'warn', fehlt: 'error' };
const STATUS_LABEL: Record<AssetStatus, string> = { zugeordnet: 'zugeordnet', luecke: 'Lücke', fehlt: 'Datei fehlt' };
const ALL = 'alle';
const SPLIT_DEFAULT = 38;

/** Reiter Assets (B-298): getrennte Ansichten Grafik und Sound, Slot-Liste mit Status | Splitter (B-445) | Detail mit Zuordnungen. */
export function AssetsPage() {
  const data = useAssets();
  const [art, setArt] = useState<AssetKind>(() => loadPref('assets.art', KINDS, 'grafik'));
  const [kategorie, setKategorie] = useState('');
  const [status, setStatus] = useState<'' | AssetStatus>('');
  const [query, setQuery] = useState('');
  const [selected, setSelected] = useState(() => loadText('assets.selected', ''));
  const [split, setSplit, resetSplit] = useSplit('assets.split', SPLIT_DEFAULT);
  const slots = useMemo(() => data.view?.slots ?? [], [data.view]);
  const visible = useMemo(() => filterSlots(slots, { art, kategorie, status, query }), [slots, art, kategorie, status, query]);

  if (!data.view) return data.error ? <NoticeCard title="Slot-Register nicht lesbar" tone="error">{data.error}</NoticeCard> : <Text color="gray">Lade Slot-Register …</Text>;
  if (data.view.error) return <NoticeCard title="Slot-Register ungültig" tone="error">{data.view.error}</NoticeCard>;
  const slot = slots.find((s) => s.id === selected && s.art === art);
  const pickArt = (v: string) => {
    const next = v === 'sound' ? 'sound' : 'grafik';
    setArt(next);
    setKategorie('');
    savePref('assets.art', next);
  };
  const pick = (id: string) => {
    setSelected(id);
    savePref('assets.selected', id);
  };
  return (
    <div className="as-page">
      <div className="as-head">
        <SegmentedControl.Root value={art} onValueChange={pickArt}>
          <SegmentedControl.Item value="grafik">Grafik</SegmentedControl.Item>
          <SegmentedControl.Item value="sound">Sound</SegmentedControl.Item>
        </SegmentedControl.Root>
        <Select.Root value={kategorie || ALL} onValueChange={(v) => setKategorie(v === ALL ? '' : v)}>
          <Select.Trigger aria-label="Kategorie" />
          <Select.Content>
            <Select.Item value={ALL}>alle Kategorien</Select.Item>
            {CATEGORIES[art].map((k) => <Select.Item key={k} value={k}>{k}</Select.Item>)}
          </Select.Content>
        </Select.Root>
        <StatusFilter slots={slots.filter((s) => s.art === art)} status={status} onStatus={setStatus} />
        <TextField.Root className="as-filter" placeholder="Slot, Beschreibung oder Datei suchen" value={query} onChange={(e) => setQuery(e.target.value)} />
        {!data.view.preview && <Button variant="soft" onClick={() => void data.startVite()}>Vite starten (Bilder, Mini-Szene)</Button>}
      </div>
      {data.error && <p className="svc-error">{data.error}</p>}
      <div className="as-body" style={{ '--as-split': `${split}%` } as CSSProperties}>
        <SlotList slots={visible} selected={slot?.id ?? ''} onSelect={pick} />
        <Splitter className="pl-split-v" value={split} onChange={setSplit} onReset={resetSplit} />
        <section className="as-detail">
          {slot ? <SlotDetail key={slot.id} slot={slot} preview={data.view.preview} onSave={data.save} onReload={() => void data.reload()} /> : (
            <NoticeCard title="Kein Asset-Slot gewählt" tone="neutral">Links einen Slot wählen: Zuordnungen, Ausschnitt und Mini-Szene erscheinen hier.</NoticeCard>
          )}
        </section>
      </div>
    </div>
  );
}

function StatusFilter({ slots, status, onStatus }: { slots: AssetSlot[]; status: '' | AssetStatus; onStatus: (s: '' | AssetStatus) => void }) {
  const n = countStatus(slots);
  return (
    <SegmentedControl.Root value={status || ALL} onValueChange={(v) => onStatus(v === ALL ? '' : (v as AssetStatus))}>
      <SegmentedControl.Item value={ALL}>alle {slots.length}</SegmentedControl.Item>
      {(Object.keys(STATUS_LABEL) as AssetStatus[]).map((s) => (
        <SegmentedControl.Item key={s} value={s}>{STATUS_LABEL[s]} {n[s]}</SegmentedControl.Item>
      ))}
    </SegmentedControl.Root>
  );
}

function SlotList({ slots, selected, onSelect }: { slots: AssetSlot[]; selected: string; onSelect: (id: string) => void }) {
  if (slots.length === 0) return <div className="as-list"><Text color="gray">Keine Asset-Slots für diesen Filter.</Text></div>;
  return (
    <div className="as-list" role="listbox" aria-label="Asset-Slots">
      {slots.map((s) => (
        <button key={s.id} type="button" role="option" aria-selected={s.id === selected} className={`as-row${s.id === selected ? ' as-row-on' : ''}`} onClick={() => onSelect(s.id)}>
          <StatusBadge tone={STATUS_TONE[s.status]}>{STATUS_LABEL[s.status]}</StatusBadge>
          <span className="as-row-id">{s.id}</span>
          <span className="as-row-desc">{s.beschreibung}</span>
          <span className="as-row-file">{s.wahl[0]?.datei ?? '–'}{s.wahl.length > 1 ? ` (+${s.wahl.length - 1})` : ''}</span>
        </button>
      ))}
    </div>
  );
}
