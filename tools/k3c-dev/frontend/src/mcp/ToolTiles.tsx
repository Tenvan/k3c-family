import type { ToolStats } from '../api';
import { Tip } from '../ui/parts';
import { share, sortTools, toolHint, toolLine } from './overview';

/** Band 2 links (B-065): eine Kachel je Tool, Tool-Menge nur aus den Server-Zählern. */
export function ToolTiles({ tools, total }: { tools: ToolStats[]; total: number }) {
  return (
    <section className="mcp-card mcp-tools">
      <span className="mcp-card-title">Tools</span>
      <div className="mcp-tiles">
        {sortTools(tools).map((t) => (
          <Tip key={t.name} content={toolHint(t)}>
            <div className="mcp-tile" aria-label={toolHint(t)}>
              <div className="mcp-tile-name">{t.name}</div>
              <div className="mcp-share" aria-hidden>
                <span style={{ width: `${share(t.calls, total)}%` }} />
              </div>
              <div className={t.errors > 0 ? 'mcp-tile-line mcp-tile-bad' : 'mcp-tile-line'}>{toolLine(t)}</div>
            </div>
          </Tip>
        ))}
      </div>
    </section>
  );
}
