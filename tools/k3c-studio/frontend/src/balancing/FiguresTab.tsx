import { Badge, Text, TONE_COLOR, type Tone } from '@webkit/atoms';
import { BarChart, LineChart } from '@webkit/molecules';
import type { BalanceInsights, BalanceSummary, Light } from '../api';
import { goldSeries, idleSeries, resourceBars, survivalSeries } from './figures';

const TONE: Record<Light, Tone> = { green: 'ok', yellow: 'warn', red: 'error', grey: 'neutral' };

/** Kennzahlen mit Ampel (aus der Auswertung), falls geladen. */
function FigureLights({ insights }: { insights?: BalanceInsights }) {
  if (!insights?.figures.lights.length) return null;
  return (
    <div className="bal-run-actions" aria-label="Kennzahlen-Ampeln">
      {insights.figures.lights.map((f) => (
        <Badge key={f.key + (f.grade ?? '')} color={TONE_COLOR[TONE[f.light]]} variant="outline" size="1" title={f.text}>
          {f.name}{f.grade && f.grade !== 'normal' ? ` (${f.grade})` : ''}: {f.value ?? '–'} {f.unit}
        </Badge>
      ))}
    </div>
  );
}

/** Kennzahl-Diagramme eines Berichts (Überleben, Gold, Rohstoffe, Wartezeit). */
export function FiguresTab({ summary, insights }: { summary: BalanceSummary; insights?: BalanceInsights }) {
  const f = summary.kennzahlen;
  if (!f || f.nights.length === 0) return <Text size="2" color="gray">Dieser Bericht enthält keine Kennzahlen (älterer Lauf).</Text>;
  return (
    <>
      <FigureLights insights={insights} />
      <div className="bal-charts">
        <figure><figcaption>Überleben je Nacht</figcaption>
          <LineChart series={survivalSeries(f)} xLabel="Nacht" yLabel="Überleben" /></figure>
        <figure><figcaption>Gold-Bilanz (Einnahmen − Ausgaben)</figcaption>
          <LineChart series={goldSeries(f)} xLabel="Nacht" yLabel="Gold" /></figure>
        <figure><figcaption>Rohstoff-Fluss (Zufluss − Abfluss)</figcaption>
          <BarChart bars={resourceBars(f).map((b) => ({ ...b, tone: b.value < 0 ? 'warn' : 'ok' }))} /></figure>
        <figure><figcaption>Wartezeit-Anteil</figcaption>
          <LineChart series={idleSeries(f)} xLabel="Nacht" yLabel="Wartezeit in %" /></figure>
      </div>
    </>
  );
}
