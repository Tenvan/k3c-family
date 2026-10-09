import { Flex } from '@radix-ui/themes';
import { useState } from 'react';
import { backend, type McpOverview } from '../api';
import { errorText } from '../lib/errors';
import { formatNumber } from '../lib/format';
import { useNow } from '../lib/useNow';
import { ActionButton, StatusBadge, Tip } from '../ui/parts';
import { InstructionsDialog } from './InstructionsDialog';
import { durationText, uptimeText, weightedAvg } from './overview';

interface Props {
  overview: McpOverview;
  error: string;
  reload: () => Promise<void>;
}

/**
 * Serverkarte (Workbench-Spec § 3): Titel · URL · Status-Badge · Neustart (immer bedienbar) · Systemprompt ·
 * Aktualisieren; darunter Aufrufe/Fehler/Dauer und Verbindungen (Details im Tooltip); Fehler als Zeile.
 */
export function ServerCards({ overview, error, reload }: Props) {
  const now = useNow(30_000);
  const [restartError, setRestartError] = useState('');
  const { mcp, stats } = overview;
  const restart = async () => {
    try {
      const st = await backend.mcpRestart();
      setRestartError(st.listening ? '' : st.error || 'lauscht nicht');
      await reload();
    } catch (e) {
      setRestartError(errorText(e));
    }
  };
  const problem = restartError || (!mcp.listening && mcp.error) || '';
  const url = `http://${mcp.addr}/mcp`;
  const connections = `Clients max ${formatNumber(stats.peakClients)} · parallel max ${formatNumber(stats.peakInFlight)}`;
  return (
    <section className="mcp-card mcp-server">
      <Flex justify="between" align="center" gap="3" wrap="wrap">
        <Flex align="center" gap="3" wrap="wrap">
          <span className="mcp-card-title">MCP-Server</span>
          <code className="mcp-addr" title={url}>{url}</code>
          <Tip content={`Laufzeit ${uptimeText(stats.startedAt, now)}`}>
            <StatusBadge tone={mcp.listening ? 'ok' : 'error'}>{mcp.listening ? 'Lauscht' : 'Fehler'}</StatusBadge>
          </Tip>
          <ActionButton size="1" variant="soft" onClick={restart}>Neustart</ActionButton>
        </Flex>
        <Flex gap="2">
          <InstructionsDialog />
          <ActionButton size="1" variant="soft" color="gray" onClick={reload}>Aktualisieren</ActionButton>
        </Flex>
      </Flex>
      <div className="mcp-kpi-groups">
        <dl className="mcp-kpis">
          <Kpi label="AUFRUFE" value={formatNumber(stats.totalCalls)} />
          <Kpi label="FEHLER" value={formatNumber(stats.errors)} bad={stats.errors > 0} />
          <Kpi label="Ø DAUER" value={durationText(weightedAvg(stats.tools))} />
        </dl>
        <Tip content={connections}>
          <dl className="mcp-kpis" aria-label={`Verbindungen: ${connections}`}>
            <Kpi label="CLIENTS" value={formatNumber(stats.clients)} />
            <Kpi label="PARALLEL" value={formatNumber(stats.inFlight)} />
          </dl>
        </Tip>
      </div>
      {problem && <p className="svc-error">{problem}</p>}
      {error && <p className="svc-error">Nachladen fehlgeschlagen: {error}</p>}
    </section>
  );
}

export function Kpi({ label, value, bad }: { label: string; value: string; bad?: boolean }) {
  return (
    <div>
      <dt>{label}</dt>
      <dd className={bad ? 'svc-error-count' : undefined}>{value}</dd>
    </div>
  );
}
