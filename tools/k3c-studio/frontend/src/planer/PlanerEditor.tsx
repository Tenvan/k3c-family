import { Text } from '@radix-ui/themes';
import { useState } from 'react';
import type { HubRow } from '../api';
import { ActionButton } from '../ui/parts';
import { rowLabel } from './planerLogic';
import {
  addSite, integerOf, numberOf, REMOVE_HINT, removeSite, setGateHubLevel, setMerchant, setSiteField, setWallUnit, siteOf,
  type HubJson, type SiteList,
} from './planerEdit';

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
      <input type="number" step={integer ? 1 : 'any'} aria-label={label} value={text} onChange={(e) => {
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
      <label className="pl-field">
        <span>nur mit tieferer Stufe (needsDeeper)</span>
        <input type="checkbox" aria-label="needsDeeper" checked={site.needsDeeper === true} onChange={(e) => set('needsDeeper')(e.target.checked)} />
      </label>
      <div className="pl-remove">
        <ActionButton onClick={() => { edit((h) => removeSite(h, list, row.index)); onRemoved(); }}>Platz entfernen</ActionButton>
        <Text size="1" color="orange">{REMOVE_HINT}</Text>
      </div>
    </>
  );
}

/** Felder je Zeile: Platz, Mauerlinie (wallUnits[k]), Tor (gateHubLevel, gilt für alle Tore) oder Händler. */
function RowFields({ hub, row, edit, onRemoved }: { hub: HubJson; row: HubRow; edit: Edit; onRemoved: () => void }) {
  if (row.kind === 'wall') {
    return <NumberField label={`Mauer Linie ${row.line} (wallUnits[${row.index}])`} value={hub.wallLines.wallUnits[row.index] ?? 0} onValue={(v) => edit((h) => setWallUnit(h, row.index, v))} />;
  }
  if (row.kind === 'gate') {
    return <NumberField label="Tore ab Hub-Stufe (gateHubLevel)" value={hub.wallLines.gateHubLevel} onValue={(v) => edit((h) => setGateHubLevel(h, v))} integer />;
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
        <select aria-label="Art" value={kind} onChange={(e) => setKind(e.target.value)}>
          {kinds.map((k) => <option key={k} value={k}>{k}</option>)}
        </select>
      </label>
      <label className="pl-field">
        <span>Liste</span>
        <select aria-label="Liste" value={list} onChange={(e) => setList(e.target.value as SiteList)}>
          <option value="sites">sites (je Hub)</option>
          <option value="islandSites">islandSites (je Insel)</option>
        </select>
      </label>
      <NumberField label="neuer Platz (offsetUnits)" value={offset} onValue={setOffset} />
      <NumberField label="neue Hub-Stufe (hubLevel)" value={level} onValue={setLevel} integer />
      <ActionButton disabled={!kind} onClick={() => edit((h) => addSite(h, list, kind, offset, level))}>Platz hinzufügen</ActionButton>
    </section>
  );
}
