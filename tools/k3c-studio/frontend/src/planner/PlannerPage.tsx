import { Text } from '@radix-ui/themes';
import { useMemo, useState } from 'react';
import type { HubPlanView, HubRulesView, HubSaveResult } from '../api';
import { LivePreview, type PlanerSource } from '../stage/LivePreview';
import { ActionButton, NoticeCard, StatusBadge } from '@webkit/parts';
import { HubMatrixView, rowId } from './HubMatrixView';
import { AddSite, RowEditor } from './PlannerEditor';
import { parseHub } from './plannerEdit';
import { usePlanner } from './usePlanner';
import { usePlannerView } from './usePlannerView';

/** Seed und Biom der Live-Vorschau im Gebäude-Planer: fest, damit nur Planer-Stand und Hub-Stufe das Bild ändern. */
const PREVIEW = { seed: 'k3c', biome: 'forest' };

/** Abweichung der Regeltabelle (B-462/AC-03): berechnet im Go-Backend (tabelle.Gegenprobe), hier nur angezeigt. */
function RulesDiff({ rules, failed }: { rules: HubRulesView | null; failed: string }) {
  if (failed) return <NoticeCard title="Regeltabelle nicht verglichen" tone="error">{failed}</NoticeCard>;
  if (!rules) return <Text size="2" color="gray">Regeltabelle wird verglichen …</Text>;
  if (rules.error) return <Text size="2" color="red" role="alert">{rules.error}</Text>;
  if (rules.same) return <StatusBadge tone="ok">Regeltabelle stimmt mit dem Planer-Stand überein</StatusBadge>;
  return (
    <section className="pl-diff" aria-label="Abweichung der Regeltabelle">
      <Text size="2" color="orange" role="status">
        Regeltabelle weicht ab ({rules.lines.length} Zeilen in <code>{rules.path}</code> § 3): <code>{rules.task}</code> ausführen
      </Text>
      <table className="pl-diff-table">
        <thead><tr><th scope="col">Zeile</th><th scope="col">Regeltabelle (Ist)</th><th scope="col">aus hub.json erzeugt (Soll)</th></tr></thead>
        <tbody>
          {rules.lines.map((l) => (
            <tr key={l.line}><td>{l.line}</td><td className="pl-ist">{l.ist}</td><td className="pl-soll">{l.soll}</td></tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}

/** Befunde und Hinweise der Prüfung (sim.ValidateHub im Go-Backend). */
function Findings({ plan }: { plan: HubPlanView }) {
  return (
    <section className="pl-findings" aria-label="Prüfung">
      {plan.findings.length === 0
        ? <StatusBadge tone="ok">Prüfung ohne Befund</StatusBadge>
        : <ul className="wd-warnings">{plan.findings.map((f, i) => <li key={i}>{f.rule}: {f.site}: {f.detail}</li>)}</ul>}
      {plan.notes.map((n) => <Text key={n} size="1" color="gray">Hinweis: {n}</Text>)}
    </section>
  );
}

/** Ergebnis des Speicherns: Schreibregel (extern geändert, neu laden), Fehler, REG-Hinweis, veraltete Regeltabelle. */
function SaveOutcome({ result, onReload }: { result: HubSaveResult | null; onReload: () => void }) {
  if (!result) return null;
  if (result.stale) {
    return (
      <NoticeCard title="Datei extern geändert" tone="error">
        {result.stale} <ActionButton onClick={onReload}>Neu laden (Änderungen verwerfen)</ActionButton>
      </NoticeCard>
    );
  }
  if (result.unchanged) return <NoticeCard title="Unverändert" tone="info">Der Stand entspricht der Datei, nichts gespeichert.</NoticeCard>;
  if (!result.saved) return <NoticeCard title="Nicht gespeichert" tone="error">{result.error}</NoticeCard>;
  return (
    <>
      <NoticeCard title="Gespeichert" tone="ok">{result.notice}</NoticeCard>
      {result.rulesError && <NoticeCard title="Regeltabelle veraltet" tone="warn">{result.rulesError}</NoticeCard>}
    </>
  );
}

/** Speichern: nur ein geänderter Stand ohne Befund der Prüfung im Go-Backend (B-462/AC-02). */
function SaveBar({ planer }: { planer: ReturnType<typeof usePlanner> }) {
  const { hub, checked, canSave, save, reload, saveResult, checkFailed, saving } = planer;
  const state = !hub ? 'keine Änderung'
    : checkFailed ? `Prüfung nicht erreichbar: ${checkFailed}`
      : saving ? 'wird gespeichert …'
        : !checked ? 'wird geprüft …'
          : checked.findings.length > 0 || checked.error ? `nicht speicherbar: ${checked.error || `${checked.findings.length} Befunde`}` : 'geändert, Prüfung ohne Befund';
  return (
    <section className="pl-save" aria-label="Speichern">
      <ActionButton disabled={!canSave} onClick={save}>Speichern</ActionButton>
      {hub && <ActionButton variant="soft" onClick={reload}>Änderungen verwerfen</ActionButton>}
      <Text size="2" color="gray" role="status">{state}</Text>
      <SaveOutcome result={saveResult} onReload={reload} />
    </section>
  );
}

/** Hauptteil des geladenen Planers: Befunde, Abweichung, Speichern, Matrix, Bearbeiten und Live-Vorschau. */
function PlanerBody({ planer, plan, shown }: { planer: ReturnType<typeof usePlanner>; plan: HubPlanView; shown: HubPlanView }) {
  const { rules, rulesFailed, hub, candidate, edit, saving } = planer;
  const [chosen, setChosen] = useState<number | null>(null);
  const [selected, setSelected] = useState('');
  const levels = shown.matrix.levels;
  const hubLevel = chosen !== null && chosen <= levels ? chosen : Math.max(1, levels);
  const source = useMemo<PlanerSource>(() => ({ candidate, levels, hubLevel, onHubLevel: setChosen }), [candidate, levels, hubLevel]);
  const view = usePlannerView();
  const current = useMemo(() => hub ?? parseHub(plan.raw), [hub, plan]);
  const row = shown.matrix.rows.find((r) => rowId(r) === selected) ?? null;
  return (
    <>
      <Findings plan={shown} />
      <RulesDiff rules={rules} failed={rulesFailed} />
      <SaveBar planer={planer} />
      <HubMatrixView matrix={shown.matrix} hubLevel={hubLevel} onHubLevel={setChosen} selected={selected} onSelect={setSelected} />
      <fieldset className="pl-fieldset" disabled={saving} aria-label="Bearbeiten">
        <RowEditor hub={current} row={row} edit={edit} onRemoved={() => setSelected('')} />
        <AddSite kinds={plan.kinds} levels={levels} edit={edit} />
      </fieldset>
      <section className="wd-live" aria-label="Live-Vorschau">
        <Text size="2" color="gray">Live-Vorschau des Planer-Stands, Seed {PREVIEW.seed}, Wald</Text>
        <LivePreview seed={PREVIEW.seed} biome={PREVIEW.biome} position={view.position} onPan={view.onPan} onWheel={view.onWheel} planer={source} />
      </section>
    </>
  );
}

/** Seite Gebäude-Planer des Asset-Studios (B-462, WZ9.2 und WZ9.3): Matrix Hub-Stufe × Bauplatz aus `data/hub.json`,
 *  Bearbeiten der gewählten Zeile als Kandidat, Befunde der Prüfung, Abweichung der Regeltabelle, Speichern und die
 *  Live-Vorschau des Planer-Stands (auch ungespeichert) in der gewählten Hub-Stufe. */
export function PlannerPage() {
  const planer = usePlanner();
  const { plan, failed, reload } = planer;
  const shown = planer.view ?? plan;
  const levels = shown?.matrix.levels ?? 0;
  return (
    <div className="pl-page">
      <div className="wd-bar">
        <Text size="2">Quelle <code>{plan?.path ?? 'data/hub.json'}</code>{levels > 0 && `, ${levels} Hub-Stufen`}</Text>
        <ActionButton onClick={reload}>Neu laden</ActionButton>
      </div>
      {failed && <NoticeCard title="Gebäude-Planer nicht geladen" tone="error">{failed}</NoticeCard>}
      {plan?.error && <Text size="2" color="red" role="alert">{plan.error}</Text>}
      {plan && !plan.error && shown && <PlanerBody planer={planer} plan={plan} shown={shown} />}
    </div>
  );
}
