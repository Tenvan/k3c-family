import { Tabs } from '@webkit/organisms';
import { useState } from 'react';
import type { AssetChoice, AssetSlot } from '../api';
import { SlotInfo } from '../assets/SlotInfo';
import { loadPref, savePref } from '../lib/prefs';
import '../styles/assets.css'; // CutEditor, MiniScene und SlotInfo gestalten sich darüber (as-*)
import '../styles/assetInspector.css';
import { Belegung } from './Belegung';
import { CandidatesTab } from './CandidatesTab';

const TABS = ['belegung', 'kandidaten', 'details'] as const;
type Tab = (typeof TABS)[number];

export interface InspectorProps {
  slot: AssetSlot;
  /** alle zugeordneten Dateien; der Reiter Kandidaten braucht sie nicht, das Backend liefert die Stufe */
  used: ReadonlySet<string>;
  onSave: (id: string, wahl: AssetChoice[]) => Promise<boolean>;
  /** Slot-Register neu laden (nach Zuordnen) */
  onReload: () => void;
}

/** Inspektor der Assets-Seite (B-677): Belegung | Kandidaten | Details. Der Reiter wird gemerkt (`assets.inspector`). */
export function Inspector({ slot, onSave, onReload }: InspectorProps) {
  const [tab, setTab] = useState<Tab>(() => loadPref('assets.inspector', TABS, 'belegung'));
  const choose = (v: string) => {
    const next = TABS.find((t) => t === v) ?? 'belegung';
    setTab(next);
    savePref('assets.inspector', next);
  };
  return (
    <Tabs.Root value={tab} onValueChange={choose} className="ap-inspector">
      <Tabs.List size="1">
        <Tabs.Trigger value="belegung">Belegung</Tabs.Trigger>
        <Tabs.Trigger value="kandidaten">Kandidaten</Tabs.Trigger>
        <Tabs.Trigger value="details">Details</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="belegung" className="ap-tab"><Belegung key={slot.id} slot={slot} onSave={onSave} /></Tabs.Content>
      <Tabs.Content value="kandidaten" className="ap-tab"><CandidatesTab key={slot.id} slot={slot} onReload={onReload} /></Tabs.Content>
      <Tabs.Content value="details" className="ap-tab"><SlotInfo slot={slot} /></Tabs.Content>
    </Tabs.Root>
  );
}
