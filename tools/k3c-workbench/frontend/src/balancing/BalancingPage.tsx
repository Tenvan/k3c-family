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
import { FormEditor } from './FormEditor';
import { appendLine } from './help';
import { EMPTY_FORM, formFromSpec, isPresetList, type BalanceForm, type Preset } from './spec';
import { useBalancing } from './useBalancing';

type Mode = 'report' | 'editor' | 'compare';

/** Werkzeugleiste (B-659): „Neuer Lauf“ öffnet das Formular im Detail; rechts der Zustand der Warteschlange. */
function Toolbar({ runs, editing, onNew }: { runs: BalanceRun[]; editing: boolean; onNew: () => void }) {
  const running = runs.filter((r) => r.state === 'läuft').length;
  const waiting = runs.filter((r) => r.state === 'wartet').length;
  return (
    <>
      <Button variant={editing ? 'solid' : 'soft'} aria-pressed={editing} onClick={onNew}>Neuer Lauf</Button>
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
 *  Bericht in Reitern, Lauf, Vergleich, Abstimmung oder das Formular für einen neuen Lauf (Editor, WZ32.4).
 *  Den Editor öffnen `openEditor(form)` (Formular vorbefüllen) und `show('editor')`. */
export function BalancingPage({ active = true }: { active?: boolean }) {
  const data = useBalancing(active);
  const [form, setForm] = useState<BalanceForm>(EMPTY_FORM);
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
  /** Formular im Editor öffnen und mit `next` füllen. */
  const openEditor = (next: BalanceForm) => { setForm(next); show('editor'); };
  const start = async (spec: BalanceSpec) => { await data.start(spec); setView('list'); };
  const removeMany = async (paths: string[]) => {
    if (!(await data.removeMany(paths))) return;
    setPicked(selectionAfterRemoval(entries, paths, selected));
    setChecked((c) => c.filter((p) => !paths.includes(p)));
    setCompare((c) => c.filter((p) => !paths.includes(p)));
  };
  const actions: DetailActions = {
    onRestart: (spec) => void start(spec),
    onEdit: (spec, variant) => {
      const f = formFromSpec(spec);
      openEditor(variant ? { ...f, vary: appendLine(f.vary, variant) } : f);
    },
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
    <FormEditor form={form} onForm={setForm} presets={presets} onPresets={keepPresets} baselines={data.baselines} onReloadBaselines={data.reloadBaselines} onStart={start} />
  ) : (
    <BalancingDetail
      compare={comparing} paths={compare} entries={entries} selected={selected} actions={actions} onStop={data.stop}
      onNew={() => show('editor')} onOpen={open} onMakeBase={(p) => setCompare((c) => [p, ...without(c, p)])}
    />
  );
  return (
    <BalancingLayout
      toolbar={<Toolbar runs={data.runs} editing={mode === 'editor'} onNew={() => show('editor')} />}
      notice={data.error ? <p className="svc-error" role="alert">{data.error}</p> : undefined}
      list={list} detail={detail} view={view} onBack={() => setView('list')}
    />
  );
}
