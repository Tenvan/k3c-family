import type { Backend, ResourceView } from './types';

const MIB = 1024 * 1024;
const POINTS = 120;

/** Erfundene Messwerte für `?mock=1`: ein leichter Anstieg mit Sägezahn, damit Diagramme und Trend etwas zeigen. */
export function mockResources(): Pick<Backend, 'resources'> {
  const started = Date.now() - 3 * 3_600_000;
  return {
    resources: async (windowMin: number): Promise<ResourceView> => {
      const now = Date.now();
      const step = (windowMin * 60_000) / POINTS;
      const points = Array.from({ length: POINTS }, (_, i) => {
        const t = now - (POINTS - 1 - i) * step;
        const age = (t - started) / 3_600_000;
        const saw = (i % 12) * 2 * MIB;
        return {
          t, heapLive: (40 + age * 6) * MIB + saw, total: (90 + age * 8) * MIB, private: (120 + age * 10) * MIB,
          rss: (150 + age * 10) * MIB, childrenRss: (300 + 200 * Math.sin(i / 9) ** 2) * MIB, children: 3 + (i % 7 === 0 ? 2 : 0),
          goroutines: 60 + (i % 9), handles: 400 + age * 5, gcCycles: i * 3,
          gaugeValues: [1.2 * MIB + (i % 20) * 4096, 12 + (i % 5), 30_000 + i * 50, 18 * MIB + age * MIB],
        };
      });
      return {
        enabled: true, started, intervalS: 10, windowMin, points,
        gauges: [{ name: 'Konsole-Text', bytes: true }, { name: 'Prüfläufe', bytes: false }, { name: 'Konsole-Zeilen', bytes: false },
          { name: 'Log-Dateien', bytes: true }],
        ops: [
          { name: 'tool:check_run', calls: 41, totalMs: 612_000, maxMs: 255_000, alloc: 380 * MIB, maxAlloc: 22 * MIB },
          { name: 'ui:PlanningData', calls: 910, totalMs: 41_000, maxMs: 380, alloc: 260 * MIB, maxAlloc: 1.4 * MIB },
          { name: 'tool:logs_query', calls: 77, totalMs: 9_000, maxMs: 410, alloc: 90 * MIB, maxAlloc: 3 * MIB },
        ],
        trends: [
          { name: 'Live-Heap', bytes: true, perHour: 6 * MIB, windowMin: 120 }, { name: 'Goroutinen', bytes: false, perHour: 0.4, windowMin: 120 },
        ],
        children: [{ name: 'node.exe', pid: 4120, rss: 320 * MIB }, { name: 'go.exe', pid: 8812, rss: 140 * MIB }],
        profiles: [],
      };
    },
  };
}
