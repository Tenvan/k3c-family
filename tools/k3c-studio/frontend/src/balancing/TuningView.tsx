import { useState } from 'react';
import { Button, StatusBadge, Text } from '@webkit/atoms';
import { BarChart, LineChart } from '@webkit/molecules';
import { errorText } from '@webkit/lib/errors';
import type { Tuning } from '../api';
import { bestIndex, paramName, scoreBars, scoreSeries } from './tune';

/** „Diff kopieren“: Zwischenablage; ein Fehler (kein Zugriff) wird als Text gezeigt statt zu werfen. */
function CopyDiff({ diff }: { diff: string }) {
  const [note, setNote] = useState('');
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(diff);
      setNote('Diff kopiert.');
    } catch (e) {
      setNote(`Kopieren fehlgeschlagen: ${errorText(e)}`);
    }
  };
  return (
    <div className="bal-run-head">
      <Button size="1" variant="soft" onClick={copy}>Diff kopieren</Button>
      {note && <Text size="1" color="gray" role="status">{note}</Text>}
    </div>
  );
}

function Points({ tuning }: { tuning: Tuning }) {
  const best = bestIndex(tuning);
  return (
    <table className="bal-table">
      <thead>
        <tr>
          {tuning.params.map((p) => <th key={paramName(p)}>{paramName(p)}</th>)}
          <th>Score</th><th>Pass</th><th>Verfehlt</th>
        </tr>
      </thead>
      <tbody>
        {tuning.points.map((p, i) => (
          <tr key={p.values.join('|')} data-tipped={i === best ? 'true' : undefined}>
            {p.values.map((v, k) => <td key={tuning.params[k] ? paramName(tuning.params[k]) : k}>{v}</td>)}
            <td>{p.score}</td>
            <td>{p.pass ? 'Pass' : 'Fail'}</td>
            <td>{p.failed.join(', ') || '–'}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function Chart({ tuning }: { tuning: Tuning }) {
  if (tuning.params.length === 1) {
    return <LineChart series={scoreSeries(tuning)} xLabel={paramName(tuning.params[0])} yLabel="Score (0 = alle Korridore getroffen)" />;
  }
  return <BarChart bars={scoreBars(tuning)} />;
}

/** Abstimm-Ergebnis: bester Punkt mit Begründung, Diagramm, Punkte-Tabelle, Diff zum Kopieren (data/ bleibt unberührt). */
export function TuningView({ name, tuning }: { name: string; tuning: Tuning }) {
  const best = tuning.best;
  return (
    <article className="bal-report" aria-label="Abstimmung">
      <div className="bal-run-head">
        <Text weight="bold">{name}</Text>
        <StatusBadge tone={best.pass ? 'ok' : 'error'}>{best.pass ? 'Pass' : 'Fail'}</StatusBadge>
        <Text size="2" color="gray">{tuning.mode} · {tuning.points.length} Punkte</Text>
      </div>
      <Text size="2" weight="medium">
        Bester Punkt: {tuning.params.map((p, i) => `${paramName(p)} = ${best.values[i]} (bisher ${p.base})`).join(', ')} · Score {best.score}
      </Text>
      <Text size="2">{tuning.reason}</Text>
      <figure className="bal-charts"><figcaption>Score je Punkt</figcaption><Chart tuning={tuning} /></figure>
      <Points tuning={tuning} />
      <Text weight="medium">Vorschlag (Diff)</Text>
      {tuning.diff === '' ? <Text size="2" color="gray">Kein Diff: der beste Wert entspricht dem bisherigen.</Text> : <pre className="bal-diff">{tuning.diff}</pre>}
      <CopyDiff diff={tuning.diff} />
    </article>
  );
}
