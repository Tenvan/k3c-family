import { useState } from 'react';
import { Button, StatusBadge, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { loadJSON, saveJSON } from '../lib/prefs';
import type { BalanceRun, BalanceSpec } from '../api';
import { BalancingLayout, type BalancingView } from './BalancingLayout';
import { CompareView } from './CompareView';
import { DataFiles } from './DataFiles';
import { Examples } from './Examples';
import { appendLine, formFromExample } from './help';
import { ConfigForm } from './ConfigForm';
import { ReportList } from './ReportList';
import { ReportView } from './ReportView';
import { RunQueue } from './RunQueue';
import { TuningView } from './TuningView';
import { EMPTY_FORM, formFromBaseline, formFromSpec, isPresetList, type BalanceForm, type Preset } from './spec';
import { useBalancing, useSummary, useTuning } from './useBalancing';

/** Abstimm-Ergebnis (WZ28.6). */
function TuningDetail({ path }: { path: string }) {
  const t = useTuning(path);
  if (t.error) return <NoticeCard title="Ergebnis nicht lesbar" tone="error">{t.error}</NoticeCard>;
  if (!t.tuning) return <Text color="gray">Lade Ergebnis …</Text>;
  return <TuningView name={path} tuning={t.tuning} />;
}

/** Bericht (B) oder Vergleich (A = Basis, B = gewählt), je nach Auswahl. */
function ReportDetail({ selected, base, onNew }: { selected: string; base: string; onNew: () => void }) {
  const b = useSummary(selected);
  const a = useSummary(base);
  if (!selected) {
    return (
      <>
        <NoticeCard title="Kein Bericht gewählt" tone="neutral">In der Liste einen Bericht öffnen. Mit „Vergleichsbasis“ wird ein Bericht zum Maßstab A, jeder danach geöffnete Bericht B erscheint neben ihm.</NoticeCard>
        <div><Button onClick={onNew}>Neuer Lauf</Button></div>
      </>
    );
  }
  if (b.error) return <NoticeCard title="Bericht nicht lesbar" tone="error">{b.error}</NoticeCard>;
  if (b.loading || !b.summary) return <Text color="gray">Lade Bericht …</Text>;
  if (base && base !== selected) {
    if (a.error) return <NoticeCard title="Basis nicht lesbar" tone="error">{a.error}</NoticeCard>;
    if (!a.summary) return <Text color="gray">Lade Basis …</Text>;
    return <CompareView nameA={base} nameB={selected} a={a.summary} b={b.summary} />;
  }
  return <ReportView name={selected} summary={b.summary} />;
}

function Detail({ selected, base, tuning, onNew }: { selected: string; base: string; tuning: boolean; onNew: () => void }) {
  if (selected && tuning) return <TuningDetail path={selected} />;
  return <ReportDetail selected={selected} base={base} onNew={onNew} />;
}

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

/** Reiter `Balancing` (WZ28.5, Rahmen B-659): Werkzeugleiste, links Läufe und Berichte, rechts Bericht, Vergleich,
 *  Abstimmung oder das Formular für einen neuen Lauf; Hilfen, Beispiele und Konfigurationsdateien (WZ28.10). */
export function BalancingPage({ active = true }: { active?: boolean }) {
  const data = useBalancing(active);
  const [form, setForm] = useState<BalanceForm>(EMPTY_FORM);
  const [presets, setPresets] = useState<Preset[]>(() => loadJSON('balancing.presets', isPresetList, []));
  const [selected, setSelected] = useState('');
  const [base, setBase] = useState('');
  const [mode, setMode] = useState<'report' | 'editor'>('report');
  const [view, setView] = useState<BalancingView>('list');
  useTicketPage('balancing', selected || data.runs[0]?.id || '', active);
  const keepPresets = (p: Preset[]) => {
    setPresets(p);
    saveJSON('balancing.presets', p);
  };
  const show = (next: 'report' | 'editor') => { setMode(next); setView('detail'); };
  const open = (path: string) => { setSelected(path); show('report'); };
  const start = async (spec: BalanceSpec) => { await data.start(spec); setView('list'); };
  const list = (
    <>
      <RunQueue runs={data.runs} onStop={data.stop} onOpen={open} onReuse={(r) => { setForm(formFromSpec(r.spec)); show('editor'); }} />
      <ReportList
        reports={data.reports} selected={selected} base={base} onSelect={open} onBase={setBase} onReload={data.reloadReports}
        onRate={data.rate} onDelete={(path) => { if (path === selected) setSelected(''); if (path === base) setBase(''); void data.remove(path); }}
      />
    </>
  );
  const detail = mode === 'editor' ? (
    <ConfigForm form={form} onForm={setForm} presets={presets} onPresets={keepPresets} onStart={start}>
      <Examples
        baselines={data.baselines} onOpen={data.reloadBaselines}
        onPick={(e) => setForm(formFromExample(e, EMPTY_FORM))} onPickBaseline={(b) => setForm(formFromBaseline(b))}
      />
      <DataFiles
        onVary={(line) => setForm((f) => ({ ...f, vary: appendLine(f.vary, line) }))}
        onParam={(line) => setForm((f) => ({ ...f, tune: { ...f.tune, mode: f.tune.mode || 'search', params: appendLine(f.tune.params, line) } }))}
      />
    </ConfigForm>
  ) : (
    <Detail selected={selected} base={base} tuning={data.reports.find((r) => r.path === selected)?.kind === 'tuning'} onNew={() => show('editor')} />
  );
  return (
    <BalancingLayout
      toolbar={<Toolbar runs={data.runs} editing={mode === 'editor'} onNew={() => show('editor')} />}
      notice={data.error ? <p className="svc-error" role="alert">{data.error}</p> : undefined}
      list={list} detail={detail} view={view} onBack={() => setView('list')}
    />
  );
}
