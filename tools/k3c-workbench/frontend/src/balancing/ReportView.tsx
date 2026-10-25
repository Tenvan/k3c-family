import { Button, StatusBadge, Text } from '@webkit/atoms';
import { BarChart, CorridorBar, LineChart } from '@webkit/molecules';
import { NoticeCard } from '@webkit/molecules';
import type { BalanceSummary } from '../api';
import { corridorText, downloadText, summaryJSON, summaryMarkdown } from './export';
import { goldSeries, idleSeries, resourceBars, survivalSeries } from './figures';
import { HtmlButton, InsightsView } from './InsightsView';

/** Export als Markdown und JSON (Download über Blob). */
export function ExportButtons({ name, summary }: { name: string; summary: BalanceSummary }) {
  const base = name.replace(/\.json$/, '').split('/').pop() || 'balancing';
  return (
    <div className="bal-run-actions">
      <Button size="1" variant="soft" onClick={() => downloadText(`${base}.md`, summaryMarkdown(summary, base), 'text/markdown')}>Export Markdown</Button>
      <Button size="1" variant="soft" onClick={() => downloadText(`${base}.json`, summaryJSON(summary), 'application/json')}>Export JSON</Button>
    </div>
  );
}

function Targets({ summary }: { summary: BalanceSummary }) {
  if (summary.targets.length === 0) {
    return <NoticeCard title="Keine Ziele im Bericht" tone="warn">Der Lauf hat keine Ziele bewertet; Korridore fehlen (data/balance-targets.json prüfen).</NoticeCard>;
  }
  return (
    <div className="bal-targets">
      {summary.targets.map((e) => (
        <div key={`${e.measure}|${e.name}|${e.players}|${e.bot}|${e.depth}|${e.days}`} className="bal-target" data-status={e.status}>
          <Text weight="medium">{e.name}</Text>
          <StatusBadge tone={e.pass ? (e.status === 'knapp' ? 'warn' : 'ok') : 'error'}>{e.status}</StatusBadge>
          <Text size="1" color="gray">{e.players} Sp. · {e.bot} · Tiefe {e.depth} · {e.days} T. · Korridor {corridorText(e)}</Text>
          <CorridorBar value={e.value} lower={e.lower ?? undefined} upper={e.upper ?? undefined} />
        </div>
      ))}
    </div>
  );
}

function Figures({ summary }: { summary: BalanceSummary }) {
  const f = summary.kennzahlen;
  if (!f || f.nights.length === 0) return <Text size="2" color="gray">Dieser Bericht enthält keine Kennzahlen (älterer Lauf).</Text>;
  return (
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
  );
}

/** Ein Bericht: Kopf, Auswertung (B-632), Korridor-Prüfung je Ziel, Kennzahl-Diagramme, Varianten, Export. */
export function ReportView({ name, summary }: { name: string; summary: BalanceSummary }) {
  return (
    <article className="bal-report" aria-label="Bericht">
      <div className="bal-run-head">
        <Text weight="bold">{name}</Text>
        <StatusBadge tone={summary.pass ? 'ok' : 'error'}>{summary.pass ? 'Pass' : 'Fail'}</StatusBadge>
        <Text size="2" color="gray">{summary.seeds} Seeds</Text>
      </div>
      {summary.varianten?.length ? <Text size="2">Varianten: {summary.varianten.join(', ')}</Text> : null}
      <div className="bal-run-actions"><HtmlButton path={name} /></div>
      <ExportButtons name={name} summary={summary} />
      <InsightsView path={name} />
      <Targets summary={summary} />
      <Figures summary={summary} />
    </article>
  );
}
