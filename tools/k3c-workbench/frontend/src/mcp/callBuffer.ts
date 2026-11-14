import type { McpCall } from '../api';

// Aufruf-Log der MCP-Seite aus den Live-Ereignissen (B-895): mcp:start und mcp:call tragen den Aufruf schon, die Seite
// holt die Liste deshalb nur beim Öffnen, statt sie bei jedem Aufruf ganz neu zu laden. Die Grenze entspricht dem
// Ringpuffer in Go (mcpsrv callRingSize) plus Spielraum für laufende Aufrufe.

/** Höchstzahl der gehaltenen Aufrufe. */
export const MAX_CALLS = 250;

/** Fügt einen Aufruf ein oder ersetzt ihn (gleiche id: Start, dann Ende). Neueste zuerst wie McpCalls, höchstens `max`. */
export function applyCall(calls: readonly McpCall[], c: McpCall, max = MAX_CALLS): McpCall[] {
  const rest = calls.filter((x) => x.id !== c.id);
  const at = rest.findIndex((x) => x.startSeq < c.startSeq);
  const next = at < 0 ? [...rest, c] : [...rest.slice(0, at), c, ...rest.slice(at)];
  return next.length > max ? next.slice(0, max) : next;
}
