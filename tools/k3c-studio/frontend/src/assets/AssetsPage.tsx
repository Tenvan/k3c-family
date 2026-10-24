import { Button, SegmentedControl, Select, Splitter, Text, TextField, type Tone } from '@webkit/atoms';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { useMemo, useState, type CSSProperties } from 'react';
import type { AssetKind, AssetSlot, AssetStatus } from '../api';
import * as prefs from '../lib/prefs';
import { NoticeCard } from '@webkit/molecules';
import { useSplit } from '@webkit/lib/split';
import { CATEGORIES, countStatus, filterSlots, selectionOf, typen, typOf } from './assets';
import { SlotDetail } from './SlotDetail';
import { useAssets } from './useAssets';

const STATUS_TONE: Record<AssetStatus, Tone> = { zugeordnet: 'ok', luecke: 'warn', fehlt: 'error' };
const STATUS_LABEL: Record<AssetStatus, string> = { zugeordnet: 'zugeordnet', luecke: 'Lücke', fehlt: 'Datei fehlt' };
const ALL = 'alle';
const SPLIT_DEFAULT = 38;
/** Schmale Seite: Anteil des Details unter der Liste (waagerechter Splitter) */
const STACK_DEFAULT = 60;

/** Slots einer Art (Seite Grafik oder Sound, B-459): Slot-Liste mit Status | Splitter | Detail mit Zuordnungen.
 *  Neuladen ersetzt nur die Daten (useAssets); Auswahl, Filter und Eingaben bleiben. */
export function AssetsPage({ art }: { art: AssetKind }) {
  const data = useAssets();
  const [kategorie, setKategorie] = useState('');
  const [typ, setTyp] = useState('');
  const [status, setStatus] = useState<'' | AssetStatus>('');
  const [query, setQuery] = useState('');
  const [selected, setSelected] = useState(() => prefs.loadText(`${art}.selected`, ''));
  const [split, setSplit, resetSplit] = useSplit(prefs, `${art}.split`, SPLIT_DEFAULT);
  const [stack, setStack, resetStack] = useSplit(prefs, `${art}.stack`, STACK_DEFAULT);
  const slots = useMemo(() => data.view?.slots ?? [], [data.view]);
  const typList = useMemo(() => typen(slots.filter((s) => s.art === art && (!kategorie || s.kategorie === kategorie))), [slots, art, kategorie]);
  const aktivTyp = typList.includes(typ) ? typ : ''; // Typ, der nach einem Reload fehlt, filtert nicht unsichtbar weiter
  const visible = useMemo(() => filterSlots(slots, { art, kategorie, typ: aktivTyp, status, query }), [slots, art, kategorie, aktivTyp, status, query]);
  const used = useMemo(() => new Set(slots.flatMap((s) => s.wahl.map((c) => c.datei))), [slots]);
  const { slot, missing } = selectionOf(slots, art, selected);
  useTicketPage(art, slot?.id ?? '');

  if (!data.view) return data.error ? <NoticeCard title="Slot-Register nicht lesbar" tone="error">{data.error}</NoticeCard> : <Text color="gray">Lade Slot-Register …</Text>;
  if (data.view.error) return <NoticeCard title="Slot-Register ungültig" tone="error">{data.view.error}</NoticeCard>;
  const pick = (id: string) => {
    setSelected(id);
    prefs.savePref(`${art}.selected`, id);
  };
  return (
    <div className="as-page">
      <div className="as-head">
        <Select.Root value={kategorie || ALL} onValueChange={(v) => { setKategorie(v === ALL ? '' : v); setTyp(''); }}>
          <Select.Trigger aria-label="Kategorie" />
          <Select.Content>
            <Select.Item value={ALL}>alle Kategorien</Select.Item>
            {CATEGORIES[art].map((k) => <Select.Item key={k} value={k}>{k}</Select.Item>)}
          </Select.Content>
        </Select.Root>
        <TypFilter typen={typList} typ={typ} onTyp={setTyp} />
        <StatusFilter slots={slots.filter((s) => s.art === art)} status={status} onStatus={setStatus} />
        <TextField.Root className="as-filter" placeholder="Slot, Beschreibung oder Datei suchen" value={query} onChange={(e) => setQuery(e.target.value)} />
        <Button variant="soft" onClick={() => void data.reload()} title="Slot-Register neu lesen, z. B. nach Änderungen durch k3c-workbench oder Agenten">Neu laden</Button>
      </div>
      {data.error && <p className="svc-error">{data.error}</p>}
      <div className="as-body" style={{ '--as-split': `${split}%`, '--as-stack': `${stack}%` } as CSSProperties}>
        <SlotList slots={visible} selected={slot?.id ?? ''} onSelect={pick} />
        <Splitter className="as-split" value={split} onChange={setSplit} onReset={resetSplit} />
        <Splitter horizontal className="as-split-stack" value={stack} onChange={setStack} onReset={resetStack} />
        <section className="as-detail">
          {slot ? <SlotDetail key={slot.id} slot={slot} used={used} onSave={data.save} onReload={() => void data.reload()} /> : <NoSlot art={art} missing={missing ? selected : ''} />}
        </section>
      </div>
    </div>
  );
}

function NoSlot({ art, missing }: { art: AssetKind; missing: string }) {
  if (missing) return <NoticeCard title={`Asset-Slot ${missing} fehlt`} tone="warn">Nach dem Neuladen gibt es den gewählten Slot nicht mehr. Links einen anderen wählen.</NoticeCard>;
  const was = art === 'sound' ? 'Zuordnungen und Hörprobe' : 'Zuordnungen, Ausschnitt und Mini-Szene';
  return <NoticeCard title="Kein Asset-Slot gewählt" tone="neutral">Links einen Slot wählen: {was} erscheinen hier.</NoticeCard>;
}

function TypFilter({ typen: list, typ, onTyp }: { typen: string[]; typ: string; onTyp: (t: string) => void }) {
  if (list.length < 2) return null;
  return (
    <Select.Root value={typ || ALL} onValueChange={(v) => onTyp(v === ALL ? '' : v)}>
      <Select.Trigger aria-label="Typ" />
      <Select.Content>
        <Select.Item value={ALL}>alle Typen</Select.Item>
        {list.map((t) => <Select.Item key={t} value={t}>{t}</Select.Item>)}
      </Select.Content>
    </Select.Root>
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
        <Button key={s.id} variant={s.id === selected ? 'soft' : 'ghost'} color={s.id === selected ? undefined : 'gray'} highContrast={s.id !== selected} role="option" aria-selected={s.id === selected} className="as-row"
          title={`${s.beschreibung}
${s.wahl[0]?.datei ?? '–'}${s.wahl.length > 1 ? ` (+${s.wahl.length - 1})` : ''}`} onClick={() => onSelect(s.id)}>
          <span className={`as-dot as-dot-${STATUS_TONE[s.status]}`} role="img" aria-label={STATUS_LABEL[s.status]} />
          <span className="as-row-id">{s.id}</span>
          <span className="as-row-desc">{typOf(s)}{s.wahl.length > 1 ? ` · ${s.wahl.length} Präf.` : ''}</span>
        </Button>
      ))}
    </div>
  );
}
