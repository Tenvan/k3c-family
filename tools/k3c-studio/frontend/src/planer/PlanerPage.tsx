import { Text } from '@radix-ui/themes';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { backend, type HubPlanView, type HubRulesView } from '../api';
import { LivePreview, type PlanerSource } from '../stage/LivePreview';
import { errorText } from '../lib/errors';
import { ActionButton, NoticeCard, StatusBadge } from '../ui/parts';
import { HubMatrixView } from './HubMatrixView';

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

/** Lädt Planer-Stand und Abweichung; jede Ladung ersetzt die vorige, eine veraltete Antwort wird verworfen. Lehnt das
 *  Backend ab (Binding nicht erreichbar, Absturz), steht der Text in `failed` bzw. `rulesFailed`. */
function usePlan() {
  const [plan, setPlan] = useState<HubPlanView | null>(null);
  const [rules, setRules] = useState<HubRulesView | null>(null);
  const [failed, setFailed] = useState('');
  const [rulesFailed, setRulesFailed] = useState('');
  const [round, setRound] = useState(0);
  useEffect(() => {
    let current = true;
    const rulesOf = (raw: string) => backend.hubRules(raw)
      .then((r) => { if (current) setRules(r); })
      .catch((e: unknown) => { if (current) setRulesFailed(errorText(e)); });
    void backend.hubPlan().then((next) => {
      if (!current) return;
      setPlan(next);
      setFailed('');
      setRules(null);
      setRulesFailed('');
      if (!next.error) void rulesOf(next.raw);
    }).catch((e: unknown) => {
      if (!current) return;
      setPlan(null);
      setFailed(errorText(e));
    });
    return () => { current = false; };
  }, [round]);
  const reload = useCallback(() => setRound((r) => r + 1), []);
  return { plan, rules, failed, rulesFailed, reload };
}

/** Seite Gebäude-Planer des Asset-Studios (B-462, WZ9.2): Matrix Hub-Stufe × Bauplatz aus `data/hub.json`, Befunde der
 *  Prüfung, Abweichung der Regeltabelle und die Live-Vorschau des Planer-Stands in der gewählten Hub-Stufe. Nur lesen;
 *  Bearbeiten und Speichern folgen mit WZ9.3. */
export function PlanerPage() {
  const { plan, rules, failed, rulesFailed, reload } = usePlan();
  const [chosen, setChosen] = useState<number | null>(null);
  const levels = plan?.matrix.levels ?? 0;
  const hubLevel = chosen !== null && chosen <= levels ? chosen : Math.max(1, levels);
  const source = useMemo<PlanerSource | null>(
    () => (plan && !plan.error ? { candidate: plan.raw, levels, hubLevel, onHubLevel: setChosen } : null),
    [plan, levels, hubLevel],
  );

  return (
    <div className="pl-page">
      <div className="wd-bar">
        <Text size="2">Quelle <code>{plan?.path ?? 'data/hub.json'}</code>{levels > 0 && `, ${levels} Hub-Stufen`}</Text>
        <ActionButton onClick={reload}>Neu laden</ActionButton>
      </div>
      {failed && <NoticeCard title="Gebäude-Planer nicht geladen" tone="error">{failed}</NoticeCard>}
      {plan?.error && <Text size="2" color="red" role="alert">{plan.error}</Text>}
      {source && plan && (
        <>
          <Findings plan={plan} />
          <RulesDiff rules={rules} failed={rulesFailed} />
          <HubMatrixView matrix={plan.matrix} hubLevel={hubLevel} onHubLevel={setChosen} />
          <section className="wd-live" aria-label="Live-Vorschau">
            <Text size="2" color="gray">Live-Vorschau des Planer-Stands, Seed {PREVIEW.seed}, Wald</Text>
            <LivePreview seed={PREVIEW.seed} biome={PREVIEW.biome} active planer={source} />
          </section>
        </>
      )}
    </div>
  );
}
