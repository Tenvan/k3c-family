import { Flex, Select, Switch, Text } from '@radix-ui/themes';
import { Fragment, useState } from 'react';
import type { McpCall } from '../api';
import { formatDuration, formatTime } from '../lib/format';
import { loadText, savePref } from '../lib/prefs';
import { StatusBadge, type Tone } from '../ui/parts';
import { buildGraph, callFooter, filterCalls, prettyArgs, type Row } from './lanes';

const LANE = 14;
const ROW = 26;
const ALL = 'ALLE'; // Radix Select erlaubt keinen leeren Wert

/** Band 3 (B-065): Aufruf-Log mit Start- und Endzeile je Aufruf und Graph-Spuren. */
export function CallLog({ calls, tools }: { calls: McpCall[]; tools: string[] }) {
  const [tool, setTool] = useState(() => loadText('callTool', ''));
  const [errorsOnly, setErrorsOnly] = useState(() => loadText('callErrors', '') === '1');
  const [open, setOpen] = useState<Set<string>>(new Set());
  const shown = filterCalls(calls, { tool, errorsOnly });
  const graph = buildGraph(shown);
  const toggle = (key: string) => setOpen((s) => {
    const next = new Set(s);
    if (!next.delete(key)) next.add(key);
    return next;
  });
  return (
    <section className="mcp-card mcp-log">
      <Flex justify="between" align="center" gap="3" wrap="wrap">
        <span className="mcp-card-title">Aufruf-Log</span>
        <Flex gap="3" align="center">
          <Select.Root size="1" value={tool || ALL} onValueChange={(v) => { const t = v === ALL ? '' : v; setTool(t); savePref('callTool', t); }}>
            <Select.Trigger aria-label="Tool" />
            <Select.Content>
              <Select.Item value={ALL}>Alle Tools</Select.Item>
              {tools.map((t) => <Select.Item key={t} value={t}>{t}</Select.Item>)}
            </Select.Content>
          </Select.Root>
          <Text as="label" size="1">
            <Flex gap="2" align="center">
              <Switch size="1" checked={errorsOnly} onCheckedChange={(v) => { setErrorsOnly(v); savePref('callErrors', v ? '1' : ''); }} />
              nur Fehler
            </Flex>
          </Text>
        </Flex>
      </Flex>
      <div className="mcp-log-body">
        <table className="mcp-log-table">
          <tbody>
            {graph.rows.map((r) => {
              const key = `${r.call.id}-${r.kind}`;
              return (
                <Fragment key={key}>
                  <LogRow r={r} lanes={graph.lanes} onClick={() => toggle(key)} />
                  {open.has(key) && <Detail call={r.call} />}
                </Fragment>
              );
            })}
          </tbody>
        </table>
        {graph.rows.length === 0 && <p className="mcp-empty">Noch keine Aufrufe{tool || errorsOnly ? ' für diesen Filter' : ''}.</p>}
      </div>
      <p className="logtab-foot">{callFooter(shown)}</p>
    </section>
  );
}

function badgeOf(r: Row): [string, Tone] {
  if (r.kind === 'start') return r.call.running ? ['läuft', 'info'] : ['Start', 'neutral'];
  return r.call.ok ? ['OK', 'ok'] : ['Fehler', 'error'];
}

function LogRow({ r, lanes, onClick }: { r: Row; lanes: number; onClick: () => void }) {
  const [label, tone] = badgeOf(r);
  const at = r.kind === 'start' ? r.call.startedAt : r.call.ts;
  const text = r.kind === 'start' ? r.call.args : r.call.summary || r.call.error || '';
  return (
    <tr className="mcp-log-row" onClick={onClick}>
      <td className="logrow-time">{at ? formatTime(new Date(at), true) : '–'}</td>
      <td className="mcp-log-tool">{r.call.tool}</td>
      <td className={`mcp-ref lane-c${r.color}`}>{r.call.ref}</td>
      <td className="mcp-graph"><GraphCell r={r} lanes={lanes} /></td>
      <td><StatusBadge tone={tone}>{label}</StatusBadge></td>
      <td className="logrow-time">{r.kind === 'end' ? formatDuration(r.call.durationMs) : ''}</td>
      <td className="mcp-log-text" title={text}>{text}</td>
    </tr>
  );
}

/** Graph-Spalte einer Zeile: durchlaufende Spuren und der eigene Knoten; laufende Aufrufe gestrichelt. */
function GraphCell({ r, lanes }: { r: Row; lanes: number }) {
  const x = (lane: number) => lane * LANE + LANE / 2;
  const mid = ROW / 2;
  const own = r.kind === 'start' ? [0, mid] : [mid, ROW]; // Start: nach oben (neuer), Ende: nach unten (älter)
  return (
    <svg width={Math.max(1, lanes) * LANE} height={ROW} aria-hidden>
      {r.through.map((t) => (
        <line key={t.lane} x1={x(t.lane)} x2={x(t.lane)} y1={0} y2={ROW}
          className={`lane lane-c${t.color}${t.running ? ' lane-open' : ''}`} />
      ))}
      <line x1={x(r.lane)} x2={x(r.lane)} y1={own[0]} y2={own[1]}
        className={`lane lane-c${r.color}${r.call.running ? ' lane-open' : ''}`} />
      <circle cx={x(r.lane)} cy={mid} r={4} className={`node lane-c${r.color}${r.kind === 'start' ? ' node-start' : ''}`} />
    </svg>
  );
}

function Detail({ call }: { call: McpCall }) {
  return (
    <tr className="logrow-detail">
      <td colSpan={7}>
        <pre>{prettyArgs(call.args)}</pre>
        {call.error && <p className="svc-error">{call.error}</p>}
      </td>
    </tr>
  );
}
