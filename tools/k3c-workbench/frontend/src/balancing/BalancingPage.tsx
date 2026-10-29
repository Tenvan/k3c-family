import { useMemo, useState } from 'react';
import { Button, StatusBadge } from '@webkit/atoms';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { loadJSON, saveJSON } from '../lib/prefs';
import type { BalanceRun, BalanceSpec } from '../api';
import { BalancingDetail } from './BalancingDetail';
import { BalancingLayout, type BalancingView } from './BalancingLayout';
import type { DetailActions } from './DetailHead';
import { buildEntries, resolveSelection, selectionAfterRemoval } from './entries';
import { EntryList } from './EntryList';
import { DataFiles } from './DataFiles';
import { RunEditor } from './editor/RunEditor';
import type { Origin } from './editor/variantForm';
import { appendLine } from './help';
import { EMPTY_FORM, formFromSpec, isPresetList, type BalanceForm, type Preset } from './spec';
import { useBalancing } from './useBalancing';

type Mode = 'report' | 'editor' | 'compare';

/** Womit der Lauf-Editor geöffnet ist; `key` baut ihn bei jedem Öffnen neu auf. */
interface EditorTarget {
  form: BalanceForm;
  origin: Origin;
  key: number;
}

/** Lauf-Editor öffnen, gefüllt mit `form`; `origin` nennt den Ausgangspunkt, `onOpen` zeigt den Editor. */
function useEditorTarget(onOpen: () => void) {
  const [editor, setEditor] = useState<EditorTarget>({ form: EMPTY_FORM, origin: { kind: 'leer', label: '' }, key: 0 });
  const openEditor = (form: BalanceForm, origin: Origin) => {
    setEditor((e) => ({ form, origin, key: e.key + 1 }));
    onOpen();
  };
  return { editor, openEditor };
}

/** „Als Variante übernehmen“: die Zeile kommt als feste Variante in den Editor. */
const withVariant = (f: BalanceForm, line?: string): BalanceForm => (line ? { ...f, vary: appendLine(f.vary, line) } : f);

/** Werkzeugleiste (B-659): „Neuer Lauf“ öffnet das Formular im Detail; rechts der Zustand der Warteschlange. */
function Toolbar({ runs, editing, onNew }: { runs: BalanceRun[]; editing: boolean; onNew: () => void }) {
  const running = runs.filter((r) => r.state === 'läuft').length;
  const waiting = runs.filter((r) => r.state === 'wartet').length;
  return (
    <>
      <Button variant={editing ? 'solid' : 'soft'} aria-pressed={editing} onClick={onNew}>Neuer Lauf</Button>
      <DataFiles />
      <div className="bal-queue" aria-label="Warteschlange">
        <StatusBadge tone={running > 0 ? 'info' : 'neutral'}>{running} läuft</StatusBadge>
        <StatusBadge tone="neutral">{waiting} wartet</StatusBadge>
      </div>
    </>
  );
}

/** Voreinstellungen des Formulars, gemerkt unter `balancing.presets`. */
function usePresets(): [Preset[], (p: Preset[]) => void] {
  const [presets, setPresets] = useState<Preset[]>(() => loadJSON('balancing.presets', isPresetList, []));
  return [presets, (p) => { setPresets(p); saveJSON('balancing.presets', p); }];
}

const without = (list: string[], id: string) => list.filter((x) => x !== id);
/** Auswahl in der Reihenfolge der Wahl: ein neu Gewählter kommt ans Ende. */
const toggled = (list: string[], id: string, on: boolean) => (on ? [...without(list, id), id] : without(list, id));

/** Reiter `Balancing` (WZ28.5, Rahmen B-659, Liste und Detail B-660): links eine Liste für Läufe und Berichte, rechts
 *  Bericht in Reitern, Lauf, Vergleich, Abstimmung oder den Lauf-Editor (WZ32.4).
 *  Den Editor öffnet `openEditor(form, origin)`. */
export function BalancingPage({ active = true }: { active?: boolean }) {
  const data = useBalancing(active);
  const [presets, keepPresets] = usePresets();
  const [picked, setPicked] = useState('');
  const [checkedRaw, setChecked] = useState<string[]>([]);
  const [compare, setCompare] = useState<string[]>([]);
  const [mode, setMode] = useState<Mode>('report');
  const [view, setView] = useState<BalancingView>('list');
  const entries = useMemo(() => buildEntries(data.runs, data.reports, data.baselines), [data.runs, data.reports, data.baselines]);
  const selected = resolveSelection(picked, data.runs, entries);
  const checked = checkedRaw.filter((id) => entries.some((e) => e.id === id));
  useTicketPage('balancing', selected || data.runs[0]?.id || '', active);
  const show = (next: Mode) => { setMode(next); setView('detail'); };
  const open = (id: string) => { setPicked(id); show('report'); };
  const { editor, openEditor } = useEditorTarget(() => { data.reloadBaselines(); show('editor'); });
  const newRun = () => openEditor(EMPTY_FORM, { kind: 'leer', label: '' });
  /** Liste nur nach einem Start; eine Ablehnung wirft, damit der Editor offen bleibt und den Grund zeigt. */
  const start = async (spec: BalanceSpec) => { if (!(await data.start(spec))) throw new Error(data.startError() || 'Lauf abgelehnt'); setView('list'); };
  const removeMany = async (paths: string[]) => {
    if (!(await data.removeMany(paths))) return;
    setPicked(selectionAfterRemoval(entries, paths, selected));
    const keep = (c: string[]) => c.filter((p) => !paths.includes(p));
    setChecked(keep);
    setCompare(keep);
  };
  const actions: DetailActions = {
    onRestart: (spec) => void start(spec),
    onEdit: (spec, variant) => openEditor(withVariant(formFromSpec(spec), variant), { kind: 'bericht', label: spec?.label || 'Bericht' }),
    onCompareWith: (path) => { setChecked([path]); setView('list'); },
    onRate: (path, rating) => void data.rate(path, rating),
    onDelete: (paths) => void removeMany(paths),
    onSaveBaseline: data.saveBaseline,
  };
  const comparing = mode === 'compare' && compare.length >= 2;
  const list = (
    <EntryList
      entries={entries} selected={selected} checked={checked} onSelect={open} onReload={data.reload}
      onCheck={(id, on) => setChecked((c) => toggled(c, id, on))}
      onClear={() => setChecked([])} onCompare={(paths) => { setCompare(paths); show('compare'); }}
      onRate={(paths, rating) => void data.rateMany(paths, rating)} onDelete={(paths) => void removeMany(paths)}
    />
  );
  const detail = mode === 'editor' ? (
    <RunEditor
      key={editor.key} initial={editor.form} origin={editor.origin} baselines={data.baselines} runs={data.runs}
      presets={presets} onPresets={keepPresets} onStart={start} onClose={() => setMode('report')}
    />
  ) : (
    <BalancingDetail
      compare={comparing} paths={compare} entries={entries} selected={selected} actions={actions} onStop={data.stop}
      onNew={newRun} onOpen={open} onMakeBase={(p) => setCompare((c) => [p, ...without(c, p)])}
    />
  );
  return (
    <BalancingLayout
      toolbar={<Toolbar runs={data.runs} editing={mode === 'editor'} onNew={newRun} />}
      notice={data.error ? <p className="svc-error" role="alert">{data.error}</p> : undefined}
      list={list} detail={detail} fill={mode === 'editor'} view={view} onBack={() => setView('list')}
    />
  );
}
