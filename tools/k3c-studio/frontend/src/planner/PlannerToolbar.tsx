import { ActionButton, Button, StatusBadge, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { useState } from 'react';
import type { HubPlanView, HubRulesView, HubSaveResult } from '../api';
import type { usePlanner } from './usePlanner';

type Planer = ReturnType<typeof usePlanner>;
type Panel = '' | 'checks' | 'rules';

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
function Findings({ plan, failed, error }: { plan: HubPlanView; failed: string; error: string }) {
  return (
    <section className="pl-findings" aria-label="Prüfung">
      {failed && <Text size="2" color="red">Prüfung nicht erreichbar: {failed}</Text>}
      {error && <Text size="2" color="red">{error}</Text>}
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

/** Zustand des Speicherns als Text: Änderungsanzeige und Grund einer Sperre (B-462/AC-02). */
function saveState({ hub, checked, checkFailed, saving }: Planer): string {
  if (!hub) return 'keine Änderung';
  if (checkFailed) return `Prüfung nicht erreichbar: ${checkFailed}`;
  if (saving) return 'wird gespeichert …';
  if (!checked) return 'wird geprüft …';
  if (checked.findings.length > 0 || checked.error) return `nicht speicherbar: ${checked.error || `${checked.findings.length} Befunde`}`;
  return 'geändert, Prüfung ohne Befund';
}

/** Chip „Prüfung“: ok, n Befunde oder Fehler (rot) */
function checkChip(p: Planer, shown: HubPlanView): { label: string; bad: boolean } {
  if (p.checkFailed) return { label: 'Prüfung nicht erreichbar', bad: true };
  if (p.checked?.error) return { label: 'Prüfung: Fehler', bad: true };
  const n = shown.findings.length;
  return n > 0 ? { label: `Prüfung: ${n} Befunde`, bad: true } : { label: 'Prüfung ok', bad: false };
}

/** Chip „Regeltabelle“: nur bei Abweichung (Zahl der Zeilen) oder Fehler; stimmt sie, gibt es keinen Chip */
function rulesChip(rules: HubRulesView | null, failed: string): { label: string; bad: boolean } | null {
  if (failed || rules?.error) return { label: 'Regeltabelle: Fehler', bad: true };
  if (!rules || rules.same) return null;
  return { label: `Regeltabelle ${rules.lines.length}`, bad: false };
}

/** Status-Chips Prüfung und Regeltabelle; ein Klick öffnet bzw. schließt ihr Panel unter der Leiste (B-670/AC-02). */
function StatusChips({ planer, shown, panel, onPanel }: { planer: Planer; shown: HubPlanView; panel: Panel; onPanel: (p: Panel) => void }) {
  const check = checkChip(planer, shown);
  const diff = rulesChip(planer.rules, planer.rulesFailed);
  const chip = (id: Panel, c: { label: string; bad: boolean }, ok: 'green' | 'orange') => (
    <Button size="1" variant="soft" color={c.bad ? 'red' : ok} className="pl-chip" data-chip={id} aria-expanded={panel === id} onClick={() => onPanel(panel === id ? '' : id)}>{c.label}</Button>
  );
  return <>{chip('checks', check, 'green')}{diff && chip('rules', diff, 'orange')}</>;
}

/** Panel eines Chips: Befunde der Prüfung oder Abweichung der Regeltabelle (nur solange es den Chip gibt) */
function ChipPanel({ planer, shown, panel }: { planer: Planer; shown: HubPlanView; panel: Panel }) {
  if (panel === 'checks') return <div className="pl-panel"><Findings plan={shown} failed={planer.checkFailed} error={planer.checked?.error ?? ''} /></div>;
  if (panel === 'rules' && rulesChip(planer.rules, planer.rulesFailed)) return <div className="pl-panel"><RulesDiff rules={planer.rules} failed={planer.rulesFailed} /></div>;
  return null;
}

/** Werkzeugleiste des Gebäude-Planers in einer Zeile (B-670): Quelle, Hub-Stufe und Tageszeit (Live-Vorschau hängt sie in
 *  `setControlsHost` ein), Status-Chips Prüfung und Regeltabelle (Befunde und Diff erst auf Klick), rechts Änderungsanzeige,
 *  Verwerfen bzw. Neu laden und Speichern (gesperrt mit Grund als Tooltip). */
export function PlannerToolbar({ planer, shown, setControlsHost }: { planer: Planer; shown: HubPlanView | null; setControlsHost: (el: HTMLElement | null) => void }) {
  const [panel, setPanel] = useState<Panel>('');
  const { plan, hub, reload, save, canSave, saveResult } = planer;
  const state = saveState(planer);
  return (
    <>
      <div className="pl-top">
        <div className="pl-toolbar" role="toolbar" aria-label="Gebäude-Planer">
          <Text size="2" className="pl-path">Quelle <code>{plan?.path ?? 'data/hub.json'}</code></Text>
          <div ref={setControlsHost} className="wd-controls-host" />
          {shown && <StatusChips planer={planer} shown={shown} panel={panel} onPanel={setPanel} />}
          <span className="pl-grow" />
          {hub && <Text size="1" color={canSave ? 'green' : 'orange'} role="status" className="pl-state" title={state}>● {state}</Text>}
          <ActionButton size="1" variant="soft" color="gray" onClick={reload}>{hub ? 'Verwerfen' : 'Neu laden'}</ActionButton>
          <ActionButton size="1" disabled={!canSave} title={canSave ? undefined : state} onClick={save}>Speichern</ActionButton>
        </div>
        {shown && <ChipPanel planer={planer} shown={shown} panel={panel} />}
      </div>
      <SaveOutcome result={saveResult} onReload={reload} />
    </>
  );
}
