import { Flex } from '@radix-ui/themes';
import { useState } from 'react';
import { backend, type McpOverview } from '../api';
import { errorText } from '../lib/errors';
import { formatNumber } from '../lib/format';
import { useNow } from '../lib/useNow';
import { ActionButton, StatusBadge } from '../ui/parts';
import { InstructionsDialog } from './InstructionsDialog';
import { durationText, uptimeText, weightedAvg } from './overview';

interface Props {
  overview: McpOverview;
  error: string;
  reload: () => Promise<void>;
}

/** Band 1 (B-065): Karten `MCP-Server` und `Verbindungen`. */
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
  return (
    <div className="mcp-band1">
      <section className="mcp-card">
        <Flex justify="between" align="center" gap="3" wrap="wrap">
          <Flex align="center" gap="3">
            <span className="mcp-card-title">MCP-Server</span>
            <code className="mcp-addr">http://{mcp.addr}/mcp</code>
            <StatusBadge tone={mcp.listening ? 'ok' : 'error'}>{mcp.listening ? 'Lauscht' : 'Fehler'}</StatusBadge>
          </Flex>
          <Flex gap="2">
            <ActionButton size="1" variant="soft" onClick={restart}>Neu starten</ActionButton>
            <InstructionsDialog />
            <ActionButton size="1" variant="soft" color="gray" onClick={reload}>Aktualisieren</ActionButton>
          </Flex>
        </Flex>
        <dl className="mcp-kpis">
          <Kpi label="LAUFZEIT" value={uptimeText(stats.startedAt, now)} />
          <Kpi label="AUFRUFE" value={formatNumber(stats.totalCalls)} />
          <Kpi label="FEHLER" value={formatNumber(stats.errors)} bad={stats.errors > 0} />
          <Kpi label="Ø DAUER" value={durationText(weightedAvg(stats.tools))} />
        </dl>
        {problem && <p className="svc-error">{problem}</p>}
        {error && <p className="svc-error">Nachladen fehlgeschlagen: {error}</p>}
      </section>
      <section className="mcp-card">
        <span className="mcp-card-title">Verbindungen</span>
        <dl className="mcp-kpis">
          <Kpi label="CLIENTS" value={formatNumber(stats.clients)} />
          <Kpi label="CLIENTS MAX" value={formatNumber(stats.peakClients)} />
          <Kpi label="PARALLEL" value={formatNumber(stats.inFlight)} />
          <Kpi label="PARALLEL MAX" value={formatNumber(stats.peakInFlight)} />
        </dl>
      </section>
    </div>
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
