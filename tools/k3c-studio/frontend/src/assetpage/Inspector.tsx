import { Button, Flex, StatusBadge, Text } from '@webkit/atoms';
import { Tabs } from '@webkit/organisms';
import { useRef, useState } from 'react';
import type { AssetChoice, AssetSlot, AssetTarget } from '../api';
import { fileUrl } from '../assets/assets';
import { SlotInfo } from '../assets/SlotInfo';
import { TargetField } from '../assets/TargetField';
import { loadPref, loadText, savePref } from '../lib/prefs';
import '../styles/assets.css'; // CutEditor und SlotInfo gestalten sich darüber (as-*)
import '../styles/assetInspector.css';
import { Belegung } from './Belegung';
import { CandidatesTab } from './CandidatesTab';
import { LABEL, TONE } from './SlotsView';

const TABS = ['belegung', 'kandidaten'] as const;
type Tab = (typeof TABS)[number];

export interface InspectorProps {
  slot: AssetSlot;
  /** alle zugeordneten Dateien; der Reiter Kandidaten braucht sie nicht, das Backend liefert die Stufe */
  used: ReadonlySet<string>;
  onSave: (id: string, wahl: AssetChoice[]) => Promise<boolean>;
  /** Slot-Register neu laden (nach Zuordnen) */
  onReload: () => void;
  /** gewählte Präferenz (geteilt mit der Bühne); ohne Angabe hält Belegung sie selbst */
  active?: number;
  onActive?: (i: number) => void;
  /** Zielmaß speichern (B-685); ohne Angabe kein Zielmaß-Feld */
  onTarget?: (id: string, ziel: AssetTarget | null) => Promise<boolean>;
}

/** Kopf der Leiste: Vorschau der aktiven Zuordnung, Slot-Name, Status und die Hauptaktion „Datei zuordnen …“. */
function SlotHead({ slot, onAssign }: { slot: AssetSlot; onAssign: () => void }) {
  const first = slot.wahl[0];
  return (
    <div className="ap-head-slot">
      <span className="ap-head-thumb" aria-hidden>
        {first && slot.art === 'grafik' ? <img src={fileUrl(first.datei)} alt="" /> : <Text size="1" color="gray">{slot.art === 'sound' ? '♪' : '—'}</Text>}
      </span>
      <Flex direction="column" gap="1" className="ap-head-text">
        <Text size="3" weight="bold" className="ap-head-id" title={slot.id}>{slot.id}</Text>
        <Text size="1" color="gray" className="ap-head-desc" title={slot.beschreibung}>
          {slot.beschreibung} · {slot.art}/{slot.kategorie}{slot.bus ? ` · Bus ${slot.bus}` : ''}
        </Text>
        <Flex gap="2" align="center" wrap="wrap">
          <StatusBadge tone={TONE[slot.status]}>{LABEL[slot.status]}</StatusBadge>
          <Button size="1" onClick={onAssign}>Datei zuordnen …</Button>
        </Flex>
      </Flex>
    </div>
  );
}

/** Inspektor der Assets-Seite (B-677, Variante „Bühne“): Kopf mit Hauptaktion, Reiter Belegung | Kandidaten (gemerkt als
 *  `assets.inspector`), darunter Details zum Aufklappen (gemerkt als `assets.details`). Die Leiste scrollt als Ganzes. */
export function Inspector({ slot, onSave, onReload, active, onActive, onTarget }: InspectorProps) {
  const [tab, setTab] = useState<Tab>(() => loadPref('assets.inspector', TABS, 'belegung'));
  const [details, setDetails] = useState(() => loadText('assets.details', '') === '1');
  const root = useRef<HTMLDivElement>(null);
  const choose = (v: string) => {
    const next = TABS.find((t) => t === v) ?? 'belegung';
    setTab(next);
    savePref('assets.inspector', next);
  };
  const assign = () => {
    choose('kandidaten');
    setTimeout(() => root.current?.querySelector<HTMLInputElement>('input[aria-label="Katalog durchsuchen"]')?.focus(), 0);
  };
  const toggle = (open: boolean) => { setDetails(open); savePref('assets.details', open ? '1' : '0'); };
  return (
    <div className="ap-inspector" ref={root}>
      <SlotHead slot={slot} onAssign={assign} />
      {onTarget && slot.art === 'grafik' && slot.kategorie === 'gebaeude' && <TargetField slot={slot} onTarget={onTarget} />}
      <Tabs.Root value={tab} onValueChange={choose}>
        <Tabs.List size="1">
          <Tabs.Trigger value="belegung">Belegung</Tabs.Trigger>
          <Tabs.Trigger value="kandidaten">Kandidaten</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="belegung" className="ap-tab"><Belegung key={slot.id} slot={slot} onSave={onSave} active={active} onActive={onActive} /></Tabs.Content>
        <Tabs.Content value="kandidaten" className="ap-tab"><CandidatesTab key={slot.id} slot={slot} onReload={onReload} /></Tabs.Content>
      </Tabs.Root>
      <details className="ap-details" open={details} onToggle={(e) => toggle(e.currentTarget.open)}>
        <summary>Details</summary>
        {details && <SlotInfo slot={slot} />}
      </details>
    </div>
  );
}
