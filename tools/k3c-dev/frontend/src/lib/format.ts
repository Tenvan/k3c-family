// Deutsche Formatierung für Zahlen und Zeiten, wie die MCP-Antworten in Go (internal/mcpsrv: formatBytes,
// formatUptime, formatMs), damit Oberfläche und Agenten dasselbe lesen.

const DE = 'de-DE';

/** Zahl mit festen Nachkommastellen: 1234.5 → „1.234,5“. */
export function formatNumber(n: number, digits = 0): string {
  return n.toLocaleString(DE, { minimumFractionDigits: digits, maximumFractionDigits: digits });
}

/** Prozent mit einer Nachkommastelle: 3.14 → „3,1 %“. */
export function formatPercent(p: number): string {
  return `${formatNumber(p, 1)} %`;
}

/** Bytes auf 1024er-Basis: „512 B“, „2,0 KB“, „480,0 MB“, „1,5 GB“. */
export function formatBytes(n: number): string {
  if (n < 1024) return `${n} B`;
  const units = ['KB', 'MB', 'GB'];
  let v = n / 1024;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024;
    i++;
  }
  return `${formatNumber(v, 1)} ${units[i]}`;
}

/** Laufzeit grob: „< 1 min“, „42 min“, „2 h 13 min“, „3 T 4 h“. */
export function formatUptime(ms: number): string {
  const minutes = Math.floor(ms / 60_000);
  if (minutes < 1) return '< 1 min';
  if (minutes < 60) return `${minutes} min`;
  if (minutes < 24 * 60) return `${Math.floor(minutes / 60)} h ${minutes % 60} min`;
  return `${Math.floor(minutes / (24 * 60))} T ${Math.floor((minutes % (24 * 60)) / 60)} h`;
}

/** Dauer fein: „468 ms“, „12,4 s“, „2 min 5 s“, „1 h 2 min“. */
export function formatDuration(ms: number): string {
  if (ms < 999.5) return `${Math.round(ms)} ms`; // 999,6 ms wäre sonst „1000 ms“
  if (ms < 60_000) return `${formatNumber(ms / 1000, 1)} s`;
  if (ms < 3_600_000) {
    const sec = Math.floor(ms / 1000);
    return `${Math.floor(sec / 60)} min ${sec % 60} s`;
  }
  const minutes = Math.floor(ms / 60_000);
  return `${Math.floor(minutes / 60)} h ${minutes % 60} min`;
}

/** Uhrzeit in Ortszeit: „08:03“, mit Sekunden „08:03:15“. */
export function formatTime(t: Date, seconds = false): string {
  return t.toLocaleTimeString(DE, { hour: '2-digit', minute: '2-digit', second: seconds ? '2-digit' : undefined });
}
