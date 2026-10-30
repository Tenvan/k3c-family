import { useState } from 'react';
import type { HubRow } from '../api';
import { ActionButton, SelectField, Text, TextField, plainOptions } from '@webkit/atoms';
import { CheckField, ChipSwitch } from '@webkit/molecules';
import { rowId } from './HubMatrixView';
import { rowLabel } from './plannerLogic';
import {
  integerOf, moveSite, numberOf, REMOVE_HINT, removeSite, setMerchant, setSiteField, setSiteKind, setZone1Tower, setZone1Wall, siteOf,
  type HubJson, type SiteList,
} from './plannerEdit';

const LIST_OPTIONS = [{ value: 'sites' as const, label: 'sites' }, { value: 'islandSites' as const, label: 'islandSites' }];

type Edit = (change: (h: HubJson) => HubJson) => void;

interface FieldProps {
  /** kurze Beschriftung (B-670); das Feld der Datei steht im Tooltip */
  label: string;
  field: string;
  value: number;
  onValue: (n: number) => void;
  /** nur ganze Zahlen (Stufe, Tiefe: in Go int) */
  integer?: boolean;
}

/** Zahlenfeld: behält den getippten Text, meldet nur gültige Zahlen; ein neuer Wert von außen ersetzt den Text. */
function NumberField({ label, field, value, onValue, integer = false }: FieldProps) {
  const parse = integer ? integerOf : numberOf;
  const [text, setText] = useState(String(value));
  const [seen, setSeen] = useState(value);
  if (value !== seen) {
    setSeen(value);
    if (parse(text) !== value) setText(String(value));
  }
  return (
    <label className="pl-field" title={field}>
      <span>{label}</span>
      <TextField.Root size="1" type="number" step={integer ? 1 : 'any'} aria-label={label} value={text} onChange={(e) => {
        setText(e.target.value);
        const n = parse(e.target.value);
        if (n !== null) onValue(n);
      }} />
    </label>
  );
}

interface RowProps { hub: HubJson; row: HubRow; kinds: readonly string[]; edit: Edit; onSelect: (id: string) => void }

/** Felder eines Hub- oder Insel-Platzes: Art, Liste, Stufe, Platz, Tiefe, needsDeeper; Entfernen mit Hinweis auf alte Spielstände.
 *  Ändern Art oder Liste die Kennung der Zeile, bleibt sie gewählt. */
function SiteFields({ hub, row, kinds, edit, onSelect }: RowProps) {
  const site = siteOf(hub, row);
  if (!site) return null;
  const list = row.source as SiteList;
  const set = (field: Parameters<typeof setSiteField>[3]) => (v: number | boolean) => edit((h) => setSiteField(h, list, row.index, field, v));
  const kindOf = (kind: string) => { edit((h) => setSiteKind(h, list, row.index, kind)); onSelect(rowId({ source: list, index: row.index, kind })); };
  const listOf = (to: SiteList) => {
    if (to === list) return;
    const index = hub[to]?.length ?? 0;
    edit((h) => moveSite(h, list, row.index, to));
    onSelect(rowId({ source: to, index, kind: site.kind }));
  };
  return (
    <>
      <label className="pl-field" title="kind">
        <span>Art</span>
        <SelectField size="1" aria-label="Art" value={site.kind} options={plainOptions([...new Set([site.kind, ...kinds])])} onChange={kindOf} />
      </label>
      <ChipSwitch label="Liste" caption="Liste" value={list} options={LIST_OPTIONS} onChange={listOf} />
      <NumberField label="ab Hub-Stufe" field="hubLevel" value={site.hubLevel ?? 1} onValue={set('hubLevel')} integer />
      <NumberField label="Platz (Units)" field="offsetUnits" value={site.offsetUnits} onValue={set('offsetUnits')} />
      <NumberField label="ab Tiefe" field="fromDepth" value={site.fromDepth ?? 0} onValue={set('fromDepth')} integer />
      <CheckField size="1" className="pl-check" title="needsDeeper" label="nur mit tieferer Stufe" aria-label="nur mit tieferer Stufe" checked={site.needsDeeper === true}
        onCheckedChange={(c) => set('needsDeeper')(c === true)} />
      <div className="pl-remove">
        <ActionButton size="1" color="red" variant="soft" onClick={() => { edit((h) => removeSite(h, list, row.index)); onSelect(''); }}>Entfernen</ActionButton>
        <Text size="1" color="orange">{REMOVE_HINT}</Text>
      </div>
    </>
  );
}

/** Felder je Zeile: Platz, Baugrund der Zone 1 (zone1WallUnits, zone1TowerUnits) oder Händler. */
function RowFields(props: RowProps) {
  const { hub, row, edit } = props;
  if (row.kind === 'wall') {
    return (
      <>
        <NumberField label="Mauer Zone 1" field="zone1WallUnits" value={hub.sitePlacement.zone1WallUnits} onValue={(v) => edit((h) => setZone1Wall(h, v))} />
        <NumberField label="Turm Zone 1" field="zone1TowerUnits" value={hub.sitePlacement.zone1TowerUnits} onValue={(v) => edit((h) => setZone1Tower(h, v))} />
      </>
    );
  }
  if (row.source === 'merchant') {
    const field = row.kind === 'merchantBuy' ? 'buyOffsetUnits' : 'sellOffsetUnits';
    return (
      <>
        <NumberField label="Platz (Units)" field={field} value={hub.merchant[field]} onValue={(v) => edit((h) => setMerchant(h, field, v))} />
        <NumberField label="nur Tiefe" field="onlyDepth" value={hub.merchant.onlyDepth} onValue={(v) => edit((h) => setMerchant(h, 'onlyDepth', v))} integer />
      </>
    );
  }
  return <SiteFields {...props} />;
}

/** Inspektor des Gebäude-Planers (B-670): feste Spalte rechts neben der Matrix mit den Feldern der gewählten Zeile, darunter
 *  der Vorschau-Zustand ihrer Plätze (Karte aus B-666, eingehängt von der Live-Vorschau) und die Hintergrund-Schichten. */
export function PlannerInspector({ hub, row, kinds, edit, saving, onSelect, setLookHost, setLayersHost }: {
  hub: HubJson; row: HubRow | null; kinds: readonly string[]; edit: Edit; saving: boolean; onSelect: (id: string) => void;
  setLookHost: (el: HTMLElement | null) => void; setLayersHost: (el: HTMLElement | null) => void;
}) {
  return (
    <aside className="pl-inspector" aria-label="Inspektor">
      {row ? (
        <fieldset className="pl-fieldset" disabled={saving} aria-label="Bearbeiten">
          <section className="pl-editor" aria-label="Bauplatz bearbeiten">
            <Text size="2" weight="bold">{rowLabel(row)}</Text>
            <RowFields hub={hub} row={row} kinds={kinds} edit={edit} onSelect={onSelect} />
          </section>
        </fieldset>
      ) : <Text size="2" color="gray">Bauplatz wählen: in der Bühne, in der Matrix oder auf einem Platz-Chip.</Text>}
      <div ref={setLookHost} className="pl-look" />
      <section className="pl-layers" aria-label="Hintergrund-Schichten">
        <Text size="1" color="gray">Hintergrund</Text>
        <div ref={setLayersHost} className="wd-layers" />
      </section>
    </aside>
  );
}
