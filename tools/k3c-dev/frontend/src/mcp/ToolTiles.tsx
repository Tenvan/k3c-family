import type { ToolStats } from '../api';
import { Tip } from '../ui/parts';
import { share, sortByCalls, toolLine } from './overview';

/** Übersicht (B-350): eine Kachel je registriertem Tool, auch ohne Aufruf, mit Anteil, kleiner Statistik und letztem
 *  Aufruf; die Beschreibung steht im Tooltip. */
export function ToolTiles({ tools }: { tools: ToolStats[] }) {
  const total = tools.reduce((n, t) => n + t.calls, 0);
  return (
    <section className="mcp-card mcp-tools">
      <span className="mcp-card-title">Tools ({tools.length})</span>
      <div className="mcp-tool-tiles">
        {sortByCalls(tools).map((t) => (
          <Tip key={t.name} content={t.description}>
            <div className="mcp-tile" aria-label={`${t.name}: ${toolLine(t)}`}>
              <div className="mcp-tile-name">{t.name}</div>
              <div className="mcp-share" aria-hidden>
                <span style={{ width: `${share(t.calls, total)}%` }} />
              </div>
              <div className={t.errors > 0 ? 'mcp-tile-line mcp-tile-bad' : 'mcp-tile-line'}>{toolLine(t)}</div>
              {t.lastCall && <div className="mcp-tile-last" title={t.lastCall}>{t.lastCall}</div>}
            </div>
          </Tip>
        ))}
      </div>
    </section>
  );
}
