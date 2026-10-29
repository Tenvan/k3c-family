import { useState } from 'react';
import { Badge, Button, Code, Text, TONE_COLOR, type Tone } from '@webkit/atoms';
import { CorridorBar } from '@webkit/molecules';
import { errorText } from '@webkit/lib/errors';
import { backend, type BalanceInsights, type BalanceReportInfo, type Light, type TargetInsight } from '../api';
import { corridorText } from './export';
import { leverLine, leverText } from './variant';

const TONE: Record<Light, Tone> = { green: 'ok', yellow: 'warn', red: 'error', grey: 'neutral' };
const LIGHT_TEXT: Record<Light, string> = { green: 'gut', yellow: 'knapp', red: 'verletzt', grey: 'ungültig' };

/** Ampel-Zähler und, mit Baseline, die gekippten Ziele. */
function Counts({ insights }: { insights: BalanceInsights }) {
  const tipped = insights.baseline.present ? insights.baseline.changes.filter((c) => c.tipped).length : 0;
  return (
    <div className="bal-stats" aria-label="Ampeln">
      {(['green', 'yellow', 'red', 'grey'] as Light[]).map((l) => (
        <div key={l} className="bal-stat">
          <Text size="6" weight="bold" color={TONE_COLOR[TONE[l]]}>{insights.counts[l]}</Text>
          <Text size="1" color="gray">{LIGHT_TEXT[l]}</Text>
        </div>
      ))}
      {tipped > 0 && (
        <div className="bal-stat">
          <Text size="6" weight="bold" color="blue">{tipped}</Text>
          <Text size="1" color="gray">gegen die Baseline gekippt</Text>
        </div>
      )}
    </div>
  );
}

function Bullets({ title, items, empty }: { title: string; items: string[]; empty: string }) {
  return (
    <div>
      <Text weight="bold" size="2">{title}</Text>
      {items.length === 0 ? <Text as="p" size="2" color="gray">{empty}</Text> : <ul className="bal-insight-list">{items.map((s) => <li key={s}>{s}</li>)}</ul>}
    </div>
  );
}

/** Die drei Ziele am weitesten außerhalb des Korridors (kleinster Abstand, negativ = außerhalb). */
export function worstTargets(insights: BalanceInsights, n = 3): TargetInsight[] {
  return insights.targets.filter((t) => t.light !== 'green' && t.distance !== null).sort((a, b) => a.distance! - b.distance!).slice(0, n);
}

interface Props {
  info: BalanceReportInfo;
  insights: BalanceInsights;
  onTargets: () => void;
  onEdit: (variant?: string) => void;
}

/** Überblick (B-660): Ampel-Zähler, Kurzfazit, Ziele am weitesten daneben, nächster Lauf mit „Im Editor übernehmen“. */
export function OverviewTab({ info, insights, onTargets, onEdit }: Props) {
  const [note, setNote] = useState('');
  const worst = worstTargets(insights);
  const lever = insights.hints.flatMap((h) => h.levers)[0];
  // Der erste Hebel wird als Variante vorgeschlagen; sein aktueller Wert kommt aus den Dateien in data/.
  const take = async () => {
    if (!lever) return onEdit();
    try {
      const line = leverLine(lever, await backend.balanceDataValuesAll());
      if (!line) setNote(`Wert von ${lever.path} nicht gefunden, der Auftrag öffnet ohne Variante.`);
      onEdit(line ?? undefined);
    } catch (e) {
      setNote(`Fehlgeschlagen: ${errorText(e)}`);
    }
  };
  return (
    <>
      <Counts insights={insights} />
      <section className="bal-card" aria-label="Kurzfazit">
        <Text weight="bold" size="2">Kurzfazit</Text>
        <ul className="bal-insight-summary">{insights.summary.map((s) => <li key={s}>{s}</li>)}</ul>
        <div className="bal-insight-grid">
          <Bullets title="Stärken" items={insights.strengths} empty="Keine erkannt." />
          <Bullets title="Schwächen" items={insights.weaknesses} empty="Keine erkannt." />
        </div>
      </section>
      <section className="bal-card" aria-label="Am weitesten daneben">
        <div className="bal-run-head">
          <Text weight="bold" size="2">Am weitesten daneben</Text>
          <Button size="1" variant="ghost" onClick={onTargets}>Alle {insights.targets.length} Ziele</Button>
        </div>
        {worst.length === 0 && <Text size="2" color="gray">Kein Ziel außerhalb des Korridors.</Text>}
        {worst.map((t) => (
          <div key={t.name + t.scenario} className="bal-target">
            <Text weight="medium">{t.name}</Text>
            <Badge color={TONE_COLOR[TONE[t.light]]} variant="soft">{LIGHT_TEXT[t.light]}</Badge>
            <Text size="1" color="gray">{t.scenario} · Korridor {corridorText(t)}</Text>
            <CorridorBar value={t.value} lower={t.lower ?? undefined} upper={t.upper ?? undefined} unit={t.unit} />
          </div>
        ))}
      </section>
      <section className="bal-card" aria-label="Nächster Lauf">
        <Text weight="bold" size="2">Nächster Lauf</Text>
        <ol className="bal-insight-list">{insights.next.steps.map((s) => <li key={s}>{s}</li>)}</ol>
        {lever && <Text size="2">Vorschlag: gleicher Auftrag mit der Variante <Code size="1">{lever.path}</Code> ({leverText(lever)}).</Text>}
        <div className="bal-run-actions">
          <Button size="1" disabled={!info.spec} title={info.spec ? undefined : 'Älterer Bericht ohne Auftrag'} onClick={() => void take()}>Im Editor übernehmen</Button>
        </div>
        {note && <Text size="1" color="gray" role="status">{note}</Text>}
      </section>
    </>
  );
}
