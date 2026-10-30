import { Splitter, Text } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { NoticeCard } from '@webkit/molecules';
import { useCallback, useMemo, useState, type CSSProperties } from 'react';
import type { HubPlanView } from '../api';
import * as prefs from '../lib/prefs';
import { LivePreview, type PlanerSource } from '../stage/LivePreview';
import type { RowFocus } from '../stage/stageLogic';
import { HubMatrixView, rowId } from './HubMatrixView';
import { PlannerInspector } from './PlannerEditor';
import { PlannerToolbar } from './PlannerToolbar';
import { addSite, parseHub } from './plannerEdit';
import { rowAt, rowOffset } from './plannerLogic';
import { usePlanner } from './usePlanner';
import { usePlannerView } from './usePlannerView';

/** Seed und Biom der Live-Vorschau im Gebäude-Planer: fest, damit nur Planer-Stand und Hub-Stufe das Bild ändern. */
const PREVIEW = { seed: 'k3c', biome: 'forest' };

/** Startanteil der Matrix (Rest: Inspektor, bei etwa 1000 px Breite rund 360 px) */
const SPLIT_DEFAULT = 64;

type Planer = ReturnType<typeof usePlanner>;

/** Eine Auswahl für Bühne, Matrix und Platz-Chips (B-670): gewählte Zeile (rowId) und ob die Wahl aus der Bühne oder einem
 *  Chip kam (dann springt die Vorschau nicht zur Zeile). */
function useRowPick(rows: HubPlanView['matrix']['rows'], hubLevel: number) {
  const [pick, setPick] = useState({ id: '', stay: false });
  const select = useCallback((id: string) => setPick({ id, stay: false }), []);
  const onPick = useCallback((site: { kind: string; x: number } | null, hubX: number) => {
    const row = site ? rowAt(rows, hubLevel, site, hubX) : undefined;
    setPick({ id: row ? rowId(row) : '', stay: true });
  }, [rows, hubLevel]);
  const row = rows.find((r) => rowId(r) === pick.id) ?? null;
  const focus = useMemo<RowFocus | undefined>(
    () => (row ? { id: rowId(row), kind: row.kind, offset: rowOffset(row, hubLevel), bothSides: row.bothSides, stay: pick.stay } : undefined),
    [row, hubLevel, pick.stay],
  );
  return { selected: pick.id, row, focus, select, onPick };
}

/** Hauptteil des geladenen Planers: Live-Vorschau mit Chip-Leiste, darunter Matrix und Inspektor nebeneinander. */
function PlanerBody({ planer, plan, shown, controlsHost }: { planer: Planer; plan: HubPlanView; shown: HubPlanView; controlsHost: HTMLElement | null }) {
  const { hub, candidate, edit, saving } = planer;
  const [chosen, setChosen] = useState<number | null>(null);
  const [lookHost, setLookHost] = useState<HTMLElement | null>(null);
  const [layersHost, setLayersHost] = useState<HTMLElement | null>(null);
  const levels = shown.matrix.levels;
  const hubLevel = chosen !== null && chosen <= levels ? chosen : Math.max(1, levels);
  const { selected, row, focus, select, onPick } = useRowPick(shown.matrix.rows, hubLevel);
  useTicketPage('planer', selected);
  const view = usePlannerView();
  const [split, setSplit, resetSplit] = useSplit(prefs, 'planer.split', SPLIT_DEFAULT);
  const current = useMemo(() => hub ?? parseHub(plan.raw), [hub, plan]);
  const source = useMemo<PlanerSource>(() => ({ candidate, levels, hubLevel, onHubLevel: setChosen, focus, onPick }), [candidate, levels, hubLevel, focus, onPick]);
  const kind = plan.kinds[0];
  /** „+ Platz“: neue Zeile in `sites` (Art: erste bekannte, Platz 0, höchste Hub-Stufe), gleich gewählt */
  const add = kind && !saving ? () => { const index = current.sites.length; edit((h) => addSite(h, 'sites', kind, 0, Math.max(1, levels))); select(rowId({ source: 'sites', index, kind })); } : undefined;
  return (
    <>
      <section className="wd-live" aria-label="Live-Vorschau" title={`Live-Vorschau des Planer-Stands, Seed ${PREVIEW.seed}, Wald`}>
        <LivePreview seed={PREVIEW.seed} biome={PREVIEW.biome} position={view.position} onPan={view.onPan} onWheel={view.onWheel} planer={source}
          controlsHost={controlsHost} lookHost={lookHost} layersHost={layersHost} />
      </section>
      <div className="pl-lower" style={{ '--pl-split': `${split}%` } as CSSProperties}>
        <HubMatrixView matrix={shown.matrix} hubLevel={hubLevel} onHubLevel={setChosen} selected={selected} onSelect={select} onAdd={add} />
        <Splitter className="pl-split" value={split} onChange={setSplit} onReset={resetSplit} />
        <PlannerInspector hub={current} row={row} kinds={plan.kinds} edit={edit} saving={saving} onSelect={select} setLookHost={setLookHost} setLayersHost={setLayersHost} />
      </div>
    </>
  );
}

/** Seite Gebäude-Planer des Asset-Studios (B-462, WZ9.2, WZ9.3, B-670): Werkzeugleiste mit Quelle, Hub-Stufe, Tageszeit,
 *  Prüfung, Regeltabelle und Speichern; Live-Vorschau des Planer-Stands (auch ungespeichert); darunter die Matrix Hub-Stufe ×
 *  Bauplatz aus `data/hub.json` und der Inspektor der gewählten Zeile. Die Seite scrollt nicht, Matrix und Inspektor je für sich. */
export function PlannerPage() {
  const planer = usePlanner();
  const { plan, failed } = planer;
  const shown = planer.view ?? plan;
  const [controlsHost, setControlsHost] = useState<HTMLElement | null>(null);
  const ready = plan && !plan.error && shown ? shown : null;
  useTicketPage('planer', '', !ready); // mit geladenem Planer meldet PlanerBody die Zeile
  return (
    <div className="pl-page">
      <PlannerToolbar planer={planer} shown={ready} setControlsHost={setControlsHost} />
      {failed && <NoticeCard title="Gebäude-Planer nicht geladen" tone="error">{failed}</NoticeCard>}
      {plan?.error && <Text size="2" color="red" role="alert">{plan.error}</Text>}
      {plan && ready && <PlanerBody planer={planer} plan={plan} shown={ready} controlsHost={controlsHost} />}
    </div>
  );
}
