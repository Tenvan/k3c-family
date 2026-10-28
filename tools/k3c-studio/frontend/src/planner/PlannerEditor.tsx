import { useState } from 'react';
import type { HubRow } from '../api';
import { ActionButton, SelectField, Text, TextField, plainOptions, type SelectOption } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { rowLabel } from './plannerLogic';
import {
  addSite, integerOf, numberOf, REMOVE_HINT, removeSite, setMerchant, setSiteField, setZone1Tower, setZone1Wall, siteOf,
  type HubJson, type SiteList,
} from './plannerEdit';

const LIST_OPTIONS: SelectOption<SiteList>[] = [{ value: 'sites', label: 'sites (je Hub)' }, { value: 'islandSites', label: 'islandSites (je Insel)' }];

type Edit = (change: (h: HubJson) => HubJson) => void;

interface FieldProps {
  label: string;
  value: number;
  onValue: (n: number) => void;
  /** nur ganze Zahlen (Stufe, Tiefe: in Go int) */
  integer?: boolean;
}

/** Zahlenfeld: behält den getippten Text, meldet nur gültige Zahlen; ein neuer Wert von außen ersetzt den Text. */
function NumberField({ label, value, onValue, integer = false }: FieldProps) {
  const parse = integer ? integerOf : numberOf;
  const [text, setText] = useState(String(value));
  const [seen, setSeen] = useState(value);
  if (value !== seen) {
    setSeen(value);
    if (parse(text) !== value) setText(String(value));
  }
  return (
    <label className="pl-field">
      <span>{label}</span>
      <TextField.Root type="number" step={integer ? 1 : 'any'} aria-label={label} value={text} onChange={(e) => {
        setText(e.target.value);
        const n = parse(e.target.value);
        if (n !== null) onValue(n);
      }} />
    </label>
  );
}

/** Felder eines Hub- oder Insel-Platzes: Stufe, Platz, Tiefe, needsDeeper; Entfernen mit Hinweis auf alte Spielstände. */
function SiteFields({ hub, row, edit, onRemoved }: { hub: HubJson; row: HubRow; edit: Edit; onRemoved: () => void }) {
  const site = siteOf(hub, row);
  if (!site) return null;
  const list = row.source as SiteList;
  const set = (field: Parameters<typeof setSiteField>[3]) => (v: number | boolean) => edit((h) => setSiteField(h, list, row.index, field, v));
  return (
    <>
      <NumberField label="Hub-Stufe (hubLevel)" value={site.hubLevel ?? 1} onValue={set('hubLevel')} integer />
      <NumberField label="Platz (offsetUnits)" value={site.offsetUnits} onValue={set('offsetUnits')} />
      <NumberField label="ab Tiefe (fromDepth)" value={site.fromDepth ?? 0} onValue={set('fromDepth')} integer />
      <CheckField className="pl-field" label="nur mit tieferer Stufe (needsDeeper)" aria-label="needsDeeper" checked={site.needsDeeper === true}
        onCheckedChange={(c) => set('needsDeeper')(c === true)} />
      <div className="pl-remove">
        <ActionButton onClick={() => { edit((h) => removeSite(h, list, row.index)); onRemoved(); }}>Platz entfernen</ActionButton>
        <Text size="1" color="orange">{REMOVE_HINT}</Text>
      </div>
    </>
  );
}

/** Felder je Zeile: Platz, Baugrund der Zone 1 (zone1WallUnits, zone1TowerUnits) oder Händler. */
function RowFields({ hub, row, edit, onRemoved }: { hub: HubJson; row: HubRow; edit: Edit; onRemoved: () => void }) {
  if (row.kind === 'wall') {
    return (
      <>
        <NumberField label="Mauer Zone 1 (zone1WallUnits)" value={hub.sitePlacement.zone1WallUnits} onValue={(v) => edit((h) => setZone1Wall(h, v))} />
        <NumberField label="Turm Zone 1 (zone1TowerUnits)" value={hub.sitePlacement.zone1TowerUnits} onValue={(v) => edit((h) => setZone1Tower(h, v))} />
      </>
    );
  }
  if (row.source === 'merchant') {
    const field = row.kind === 'merchantBuy' ? 'buyOffsetUnits' : 'sellOffsetUnits';
    return (
      <>
        <NumberField label={`Platz (${field})`} value={hub.merchant[field]} onValue={(v) => edit((h) => setMerchant(h, field, v))} />
        <NumberField label="nur Tiefe (onlyDepth)" value={hub.merchant.onlyDepth} onValue={(v) => edit((h) => setMerchant(h, 'onlyDepth', v))} integer />
      </>
    );
  }
  return <SiteFields hub={hub} row={row} edit={edit} onRemoved={onRemoved} />;
}

/** Bearbeiten der gewählten Zeile (B-462 › Bearbeitbar); ohne Wahl ein Hinweis. */
export function RowEditor({ hub, row, edit, onRemoved }: { hub: HubJson; row: HubRow | null; edit: Edit; onRemoved: () => void }) {
  if (!row) return <Text size="2" color="gray">Bauplatz in der Matrix wählen, um ihn zu bearbeiten.</Text>;
  return (
    <section className="pl-editor" aria-label="Bauplatz bearbeiten">
      <Text size="2" weight="bold">{rowLabel(row)}</Text>
      <RowFields hub={hub} row={row} edit={edit} onRemoved={onRemoved} />
    </section>
  );
}

/** Platz einer bekannten Art hinzufügen (Arten aus dem Go-Backend); Stufe, Platz und Liste wählbar. */
export function AddSite({ kinds, levels, edit }: { kinds: string[]; levels: number; edit: Edit }) {
  const [kind, setKind] = useState(kinds[0] ?? '');
  const [list, setList] = useState<SiteList>('sites');
  const [offset, setOffset] = useState(0);
  const [level, setLevel] = useState(Math.max(1, levels));
  return (
    <section className="pl-add" aria-label="Platz hinzufügen">
      <label className="pl-field">
        <span>Art</span>
        <SelectField aria-label="Art" value={kind} options={plainOptions(kinds)} onChange={setKind} />
      </label>
      <label className="pl-field">
        <span>Liste</span>
        <SelectField aria-label="Liste" value={list} options={LIST_OPTIONS} onChange={setList} />
      </label>
      <NumberField label="neuer Platz (offsetUnits)" value={offset} onValue={setOffset} />
      <NumberField label="neue Hub-Stufe (hubLevel)" value={level} onValue={setLevel} integer />
      <ActionButton disabled={!kind} onClick={() => edit((h) => addSite(h, list, kind, offset, level))}>Platz hinzufügen</ActionButton>
    </section>
  );
}
