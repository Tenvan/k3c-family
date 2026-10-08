import type { ToolStats } from '../api';
import { Tip } from '../ui/parts';
import { share, sortByCalls, toolLine } from './overview';

/**
 * Tool-Kacheln (Workbench-Spec § 3): kompakt mit Name, Anteilsleiste und einer Kennzahlenzeile; Beschreibung und
 * letzter Aufruf nur im Tooltip und `aria-label`. Aktivste zuerst, dann alphabetisch; jedes Tool, auch ohne Aufruf.
 */
export function ToolTiles({ tools }: { tools: ToolStats[] }) {
  const total = tools.reduce((n, t) => n + t.calls, 0);
  return (
    <section className="mcp-tools" aria-label={`Tools (${tools.length})`}>
      <div className="mcp-tool-tiles">
        {sortByCalls(tools).map((t) => {
          const tip = [t.description, t.lastCall && `Letzter Aufruf: ${t.lastCall}`].filter(Boolean).join('\n\n');
          return (
            <Tip key={t.name} content={<span className="mcp-tip">{tip}</span>}>
              <div className="mcp-tile" aria-label={`${t.name}: ${toolLine(t)}. ${tip}`}>
                <div className="mcp-tile-name">{t.name}</div>
                <div className="mcp-share" aria-hidden>
                  <span style={{ width: `${share(t.calls, total)}%` }} />
                </div>
                <div className={t.errors > 0 ? 'mcp-tile-line mcp-tile-bad' : 'mcp-tile-line'}>{toolLine(t)}</div>
              </div>
            </Tip>
          );
        })}
      </div>
    </section>
  );
}
