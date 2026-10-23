import type { ResOp } from '../api';
import { formatBytes, formatDuration, formatNumber } from '../lib/format';

/** Vorgänge nach Allokation: Balken relativ zum größten, dazu Aufrufe, Ø und Höchstwerte. Wer viel allokiert, belastet
 *  den Garbage Collector; wer nach einem Dauerlauf oben steht, ist der erste Verdächtige. */
export function OpsTable({ ops }: { ops: ResOp[] }) {
  if (ops.length === 0) return null;
  const top = Math.max(1, ...ops.map((o) => o.alloc));
  return (
    <section className="mon-card" aria-label="Vorgänge">
      <header className="mon-head"><h3>Vorgänge nach Allokation</h3><span className="mon-time">seit Start der Workbench</span></header>
      <table className="mon-ops">
        <thead>
          <tr><th>Vorgang</th><th /><th>Aufrufe</th><th>Σ Allokation</th><th>Ø</th><th>max</th><th>Σ Dauer</th><th>max Dauer</th></tr>
        </thead>
        <tbody>
          {ops.map((o) => (
            <tr key={o.name}>
              <td className="mon-op-name">{o.name}</td>
              <td className="mon-op-bar"><span style={{ width: `${(100 * o.alloc) / top}%` }} /></td>
              <td>{formatNumber(o.calls)}</td>
              <td>{formatBytes(o.alloc)}</td>
              <td>{formatBytes(Math.round(o.alloc / Math.max(o.calls, 1)))}</td>
              <td>{formatBytes(o.maxAlloc)}</td>
              <td>{formatDuration(o.totalMs)}</td>
              <td>{formatDuration(o.maxMs)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}
