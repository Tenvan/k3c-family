import { useState } from 'react';
import { Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { loadJSON, saveJSON } from '../lib/prefs';
import { CompareView } from './CompareView';
import { DataFiles } from './DataFiles';
import { Examples } from './Examples';
import { appendLine, formFromExample } from './help';
import { ConfigForm } from './ConfigForm';
import { ReportList } from './ReportList';
import { ReportView } from './ReportView';
import { RunQueue } from './RunQueue';
import { TuningView } from './TuningView';
import { EMPTY_FORM, formFromSpec, isPresetList, type BalanceForm, type Preset } from './spec';
import { useBalancing, useSummary, useTuning } from './useBalancing';

/** Abstimm-Ergebnis (WZ28.6). */
function TuningDetail({ path }: { path: string }) {
  const t = useTuning(path);
  if (t.error) return <NoticeCard title="Ergebnis nicht lesbar" tone="error">{t.error}</NoticeCard>;
  if (!t.tuning) return <Text color="gray">Lade Ergebnis …</Text>;
  return <TuningView name={path} tuning={t.tuning} />;
}

/** Bericht (B) oder Vergleich (A = Basis, B = gewählt), je nach Auswahl. */
function ReportDetail({ selected, base }: { selected: string; base: string }) {
  const b = useSummary(selected);
  const a = useSummary(base);
  if (!selected) return <NoticeCard title="Kein Bericht gewählt" tone="neutral">In der Liste einen Bericht öffnen. Mit „Vergleichsbasis“ wird ein Bericht zum Maßstab A, jeder danach geöffnete Bericht B erscheint neben ihm.</NoticeCard>;
  if (b.error) return <NoticeCard title="Bericht nicht lesbar" tone="error">{b.error}</NoticeCard>;
  if (b.loading || !b.summary) return <Text color="gray">Lade Bericht …</Text>;
  if (base && base !== selected) {
    if (a.error) return <NoticeCard title="Basis nicht lesbar" tone="error">{a.error}</NoticeCard>;
    if (!a.summary) return <Text color="gray">Lade Basis …</Text>;
    return <CompareView nameA={base} nameB={selected} a={a.summary} b={b.summary} />;
  }
  return <ReportView name={selected} summary={b.summary} />;
}

function Detail({ selected, base, tuning }: { selected: string; base: string; tuning: boolean }) {
  if (selected && tuning) return <TuningDetail path={selected} />;
  return <ReportDetail selected={selected} base={base} />;
}

/** Reiter `Balancing` (WZ28.5): Konfiguration, Warteschlange, Berichte mit Diagrammen, Vergleich, Export; Hilfen,
 *  Beispiele und Konfigurationsdateien (WZ28.10). */
export function BalancingPage({ active = true }: { active?: boolean }) {
  const data = useBalancing(active);
  const [form, setForm] = useState<BalanceForm>(EMPTY_FORM);
  const [presets, setPresets] = useState<Preset[]>(() => loadJSON('balancing.presets', isPresetList, []));
  const [selected, setSelected] = useState('');
  const [base, setBase] = useState('');
  useTicketPage('balancing', selected || data.runs[0]?.id || '', active);
  const keepPresets = (p: Preset[]) => {
    setPresets(p);
    saveJSON('balancing.presets', p);
  };
  return (
    <div className="bal-page">
      {data.error && <p className="svc-error" role="alert">{data.error}</p>}
      <div className="bal-col">
        <ConfigForm form={form} onForm={setForm} presets={presets} onPresets={keepPresets} onStart={data.start}>
          <Examples onPick={(e) => setForm(formFromExample(e, EMPTY_FORM))} />
          <DataFiles
            onVary={(line) => setForm((f) => ({ ...f, vary: appendLine(f.vary, line) }))}
            onParam={(line) => setForm((f) => ({ ...f, tune: { ...f.tune, mode: f.tune.mode || 'search', params: appendLine(f.tune.params, line) } }))}
          />
        </ConfigForm>
        <RunQueue runs={data.runs} onStop={data.stop} onOpen={setSelected} onReuse={(r) => setForm(formFromSpec(r.spec))} />
      </div>
      <div className="bal-col">
        <ReportList
          reports={data.reports} selected={selected} base={base} onSelect={setSelected} onBase={setBase} onReload={data.reloadReports}
          onRate={data.rate} onDelete={(path) => { if (path === selected) setSelected(''); if (path === base) setBase(''); void data.remove(path); }}
        />
        <Detail selected={selected} base={base} tuning={data.reports.find((r) => r.path === selected)?.kind === 'tuning'} />
      </div>
    </div>
  );
}
