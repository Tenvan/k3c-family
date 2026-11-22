import { Splitter, Text } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { NoticeCard } from '@webkit/molecules';
import { useMemo, useState, type CSSProperties } from 'react';
import type { HubPlanView } from '../api';
import * as prefs from '../lib/prefs';
import { HubMatrixView, rowId } from './HubMatrixView';
import { PlannerInspector } from './PlannerEditor';
import { PlannerToolbar } from './PlannerToolbar';
import { addSite, parseHub } from './plannerEdit';
import { usePlanner } from './usePlanner';

/** Startanteil der Matrix (Rest: Inspektor, bei etwa 1000 px Breite rund 360 px) */
const SPLIT_DEFAULT = 64;

type Planer = ReturnType<typeof usePlanner>;

/** Hauptteil des geladenen Planers: Matrix und Inspektor der gewählten Zeile nebeneinander. */
function PlanerBody({ planer, plan, shown, hubLevel, onHubLevel }: { planer: Planer; plan: HubPlanView; shown: HubPlanView; hubLevel: number; onHubLevel: (n: number) => void }) {
  const { hub, edit, saving } = planer;
  const [selected, select] = useState('');
  const levels = shown.matrix.levels;
  const row = shown.matrix.rows.find((r) => rowId(r) === selected) ?? null;
  useTicketPage('planer', selected);
  const [split, setSplit, resetSplit] = useSplit(prefs, 'planer.split', SPLIT_DEFAULT);
  const current = useMemo(() => hub ?? parseHub(plan.raw), [hub, plan]);
  const kind = plan.kinds[0];
  /** „+ Platz“: neue Zeile in `sites` (Art: erste bekannte, Platz 0, höchste Hub-Stufe), gleich gewählt */
  const add = kind && !saving ? () => { const index = current.sites.length; edit((h) => addSite(h, 'sites', kind, 0, Math.max(1, levels))); select(rowId({ source: 'sites', index, kind })); } : undefined;
  return (
    <div className="pl-lower" style={{ '--pl-split': `${split}%` } as CSSProperties}>
      <HubMatrixView matrix={shown.matrix} hubLevel={hubLevel} onHubLevel={onHubLevel} selected={selected} onSelect={select} onAdd={add} />
      <Splitter className="pl-split" value={split} onChange={setSplit} onReset={resetSplit} />
      <PlannerInspector hub={current} row={row} kinds={plan.kinds} edit={edit} saving={saving} onSelect={select} />
    </div>
  );
}

/** Seite Gebäude-Planer des Studios (B-923, Port aus dem Asset-Studio, B-462): Werkzeugleiste mit Quelle, Hub-Stufe, Prüfung,
 *  Regeltabelle und Speichern; darunter die Matrix Hub-Stufe × Bauplatz aus `data/hub.json` und der Inspektor der gewählten
 *  Zeile. Die Seite scrollt nicht, Matrix und Inspektor je für sich. Die Live-Vorschau des Studios gibt es hier nicht. */
export function PlannerPage() {
  const planer = usePlanner();
  const { plan, failed } = planer;
  const shown = planer.view ?? plan;
  const [chosen, setChosen] = useState<number | null>(null);
  const ready = plan && !plan.error && shown ? shown : null;
  const levels = ready?.matrix.levels ?? 0;
  const hubLevel = chosen !== null && chosen <= levels ? chosen : Math.max(1, levels);
  useTicketPage('planer', '', !ready); // mit geladenem Planer meldet PlanerBody die Zeile
  return (
    <div className="pl-page">
      <PlannerToolbar planer={planer} shown={ready} hubLevel={hubLevel} onHubLevel={setChosen} />
      {failed && <NoticeCard title="Gebäude-Planer nicht geladen" tone="error">{failed}</NoticeCard>}
      {plan?.error && <Text size="2" color="red" role="alert">{plan.error}</Text>}
      {plan && ready && <PlanerBody planer={planer} plan={plan} shown={ready} hubLevel={hubLevel} onHubLevel={setChosen} />}
    </div>
  );
}
