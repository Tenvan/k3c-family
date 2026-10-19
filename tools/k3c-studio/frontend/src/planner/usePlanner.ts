import { useCallback, useEffect, useMemo, useState } from 'react';
import { backend, type HubPlanView, type HubRulesView, type HubSaveResult } from '../api';
import { errorText } from '@webkit/errors';
import { candidateText, parseHub, type HubJson } from './plannerEdit';

/** Lädt data/hub.json (Planer-Stand mit Stand der Dateien); jede Ladung ersetzt die vorige und verwirft die Änderungen,
 *  eine veraltete Antwort wird verworfen. Lehnt das Backend ab, steht der Text in `failed`. */
function useLoaded() {
  const [plan, setPlan] = useState<HubPlanView | null>(null);
  const [failed, setFailed] = useState('');
  const [round, setRound] = useState(0);
  useEffect(() => {
    let current = true;
    void backend.hubPlan().then((next) => {
      if (!current) return;
      setPlan(next);
      setFailed('');
    }).catch((e: unknown) => {
      if (!current) return;
      setPlan(null);
      setFailed(errorText(e));
    });
    return () => { current = false; };
  }, [round]);
  const reload = useCallback(() => setRound((r) => r + 1), []);
  return { plan, failed, reload };
}

/** Prüfung und Abweichung des Kandidaten, beide im Go-Backend. `checked` ist die Prüfung genau dieses Kandidaten (auch
 *  mit Fehler), `lastGood` die letzte lesbare Prüfung seit dem Laden: Die Matrix zeigt sie, bis die nächste Antwort da ist. */
function useChecked(plan: HubPlanView | null, candidate: string, dirty: boolean) {
  const [checked, setChecked] = useState<HubPlanView | null>(null);
  const [good, setGood] = useState<{ base: HubPlanView; view: HubPlanView } | null>(null);
  const [rules, setRules] = useState<HubRulesView | null>(null);
  const [rulesFailed, setRulesFailed] = useState('');
  const [checkFailed, setCheckFailed] = useState('');
  useEffect(() => {
    if (!plan || plan.error) return;
    let current = true;
    setRules(null);
    setRulesFailed('');
    void backend.hubRules(candidate)
      .then((r) => { if (current) setRules(r); })
      .catch((e: unknown) => { if (current) setRulesFailed(errorText(e)); });
    if (dirty) {
      void backend.hubCheck(candidate)
        .then((c) => {
          if (!current) return;
          setChecked(c);
          setCheckFailed('');
          if (!c.error) setGood({ base: plan, view: c });
        })
        .catch((e: unknown) => { if (current) setCheckFailed(errorText(e)); });
    }
    return () => { current = false; };
  }, [plan, candidate, dirty]);
  const lastGood = good && good.base === plan ? good.view : null;
  return { checked: dirty && checked?.raw === candidate ? checked : null, lastGood, rules, rulesFailed, checkFailed };
}

/** Zustand des Gebäude-Planers (B-462, WZ9.2 und WZ9.3): geladene Datei, Änderungen als Kandidat (Live-Vorschau folgt),
 *  Prüfung und Abweichung aus dem Go-Backend, Speichern nur ohne Befund mit dem Stand beim Laden (Schreibregel). */
export function usePlanner() {
  const { plan, failed, reload: load } = useLoaded();
  const [edited, setEdited] = useState<{ base: HubPlanView; hub: HubJson } | null>(null);
  const [saveResult, setSaveResult] = useState<HubSaveResult | null>(null);
  const [saving, setSaving] = useState(false);
  const hub = edited && edited.base === plan ? edited.hub : null;
  const candidate = useMemo(() => (hub ? candidateText(hub) : plan?.raw ?? ''), [hub, plan]);
  const { checked, lastGood, rules, rulesFailed, checkFailed } = useChecked(plan, candidate, hub !== null);
  const view = hub && checked && !checked.error ? checked : hub ? lastGood ?? plan : plan;
  const edit = useCallback((change: (h: HubJson) => HubJson) => {
    if (!plan || plan.error || saving) return; // während des Speicherns gesperrt: gespeichert wird genau der Stand davor
    setEdited((e) => ({ base: plan, hub: change(e && e.base === plan ? e.hub : parseHub(plan.raw)) }));
    setSaveResult(null);
  }, [plan, saving]);
  const reload = useCallback(() => { setSaveResult(null); load(); }, [load]);
  const canSave = !!plan && hub !== null && !!checked && !checked.error && checked.findings.length === 0 && !saving;
  const save = useCallback(async () => {
    if (!plan || !canSave) return;
    setSaving(true);
    try {
      const result = await backend.hubSave(candidate, plan.stand);
      setSaveResult(result);
      if (result.saved) load();
    } catch (e: unknown) {
      setSaveResult({ saved: false, unchanged: false, stale: '', findings: [], error: errorText(e), rulesError: '', notice: '', stand: plan.stand });
    } finally {
      setSaving(false);
    }
  }, [plan, canSave, candidate, load]);
  return { plan, view, checked, failed, checkFailed, rules, rulesFailed, hub, candidate, edit, reload, save, canSave, saving, saveResult };
}
