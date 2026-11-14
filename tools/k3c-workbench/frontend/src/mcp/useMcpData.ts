import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, type McpCall, type McpOverview, type McpUsage } from '../api';
import { errorText } from '@webkit/lib/errors';
import { liveReload } from '../lib/liveReload';
import { applyCall } from './callBuffer';

// Daten der MCP-Seite (B-065): Übersicht, Aufruf-Log und Statistik. Kein Abfrage-Intervall. Das Aufruf-Log folgt den
// Ereignissen mcp:start/mcp:call selbst (begrenzt, callBuffer.ts); Übersicht und Statistik lädt die Seite danach
// höchstens alle STATS_MS neu und nicht bei verborgenem Fenster. Vorher holte jeder Aufruf alle drei Listen neu
// (~100 KiB, bis zu 3× je Sekunde), und der WebView2-Renderer wuchs um über 1 GiB je Stunde (B-895).

const STATS_MS = 5000;
const CALLS_FLUSH_MS = 500; // Ereignisse eines Ausbruchs ergeben einen Render

export interface McpData {
  overview: McpOverview | null;
  calls: McpCall[];
  usage: McpUsage | null;
  /** Grund, falls das letzte Nachladen scheiterte; die letzten Zahlen bleiben stehen. */
  error: string;
  reload: () => Promise<void>;
}

export function useMcpData(): McpData {
  const [overview, setOverview] = useState<McpOverview | null>(null);
  const [calls, setCalls] = useState<McpCall[]>([]);
  const [usage, setUsage] = useState<McpUsage | null>(null);
  const [error, setError] = useState('');
  const latest = useRef(0);

  const loadStats = useCallback(async () => {
    const id = ++latest.current;
    try {
      const [o, u] = await Promise.all([backend.mcpOverview(), backend.mcpUsage()]);
      if (id !== latest.current) return;
      setOverview(o);
      setUsage(u);
      setError('');
    } catch (e) {
      if (id === latest.current) setError(errorText(e));
    }
  }, []);

  const reload = useCallback(async () => {
    await Promise.all([loadStats(), backend.mcpCalls().then(setCalls, () => undefined)]);
  }, [loadStats]);

  useEffect(() => {
    const stats = liveReload(() => void loadStats(), STATS_MS);
    let pending: McpCall[] = [];
    let flush: ReturnType<typeof setTimeout> | undefined;
    const onCall = (c: McpCall) => {
      pending.push(c);
      flush ??= setTimeout(() => {
        const batch = pending;
        pending = [];
        flush = undefined;
        setCalls((cur) => batch.reduce((acc, x) => applyCall(acc, x), cur));
      }, CALLS_FLUSH_MS);
      stats.soon();
    };
    const offs = [
      backend.on('mcp:start', onCall),
      backend.on('mcp:call', onCall),
      backend.on('mcp:state', (mcp) => setOverview((o) => o && { ...o, mcp })),
      backend.on('ui:reconnect', () => void reload()),
    ];
    void reload();
    return () => {
      stats.stop();
      clearTimeout(flush);
      offs.forEach((off) => off());
    };
  }, [loadStats, reload]);

  return { overview, calls, usage, error, reload };
}
