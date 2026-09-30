import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, type McpCall, type McpOverview, type McpUsage } from '../api';
import { errorText } from '../lib/errors';

// Daten der MCP-Seite (B-065): Übersicht, Aufruf-Log und Statistik. Kein Abfrage-Intervall: nach mcp:start und
// mcp:call wird neu geladen, gebündelt, damit eine Flut von Aufrufen die Oberfläche nicht zudeckt.

const RELOAD_MS = 300;

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

  const reload = useCallback(async () => {
    const id = ++latest.current;
    try {
      const [o, c, u] = await Promise.all([backend.mcpOverview(), backend.mcpCalls(), backend.mcpUsage()]);
      if (id !== latest.current) return;
      setOverview(o);
      setCalls(c);
      setUsage(u);
      setError('');
    } catch (e) {
      if (id === latest.current) setError(errorText(e));
    }
  }, []);

  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const soon = () => {
      if (timer === undefined) timer = setTimeout(() => {
        timer = undefined;
        void reload();
      }, RELOAD_MS);
    };
    const offs = [
      backend.on('mcp:start', soon),
      backend.on('mcp:call', soon),
      backend.on('mcp:state', (mcp) => setOverview((o) => o && { ...o, mcp })),
    ];
    void reload();
    return () => {
      clearTimeout(timer);
      offs.forEach((off) => off());
    };
  }, [reload]);

  return { overview, calls, usage, error, reload };
}
