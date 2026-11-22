import { StatusBadge, Tip } from '../atoms';

/** Zustand eines MCP-Servers (Go: MCPState der Wails-Apps). */
export interface McpBadgeState {
  addr: string;
  listening: boolean;
  error: string;
}

/** Badge der Kopfzeile (B-064 › Kopfzeile): MCP-Server lauscht, startet noch oder scheitert (Workbench, Studio). */
export function McpBadge({ mcp }: { mcp: McpBadgeState | null }) {
  // Noch nicht gestartet (Go: leerer Zustand vor Start(), Mock: die ersten Millisekunden).
  if (!mcp || (!mcp.listening && !mcp.error)) return <StatusBadge tone="neutral">MCP …</StatusBadge>;
  const label = `MCP ${mcp.addr}`;
  if (mcp.listening) {
    return (
      <Tip content={`lauscht an http://${mcp.addr}/mcp`}>
        <StatusBadge tone="ok">{label}</StatusBadge>
      </Tip>
    );
  }
  return (
    <Tip content={mcp.error || 'lauscht nicht'}>
      <StatusBadge tone="error">{label}</StatusBadge>
    </Tip>
  );
}
