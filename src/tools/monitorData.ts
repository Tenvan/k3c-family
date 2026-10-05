/**
 * Daten der Monitoring-Seite (B-282): Antwort von `/api/metrics` (B-281, docs/protocol.md › Diagnose: Verläufe über
 * /api/metrics), Puffer mit Deltas und Statistik über gelieferte Punkte. Keine Spiel-Logik (Entscheidung 001).
 */

export interface ServerPoint {
  t: number;
  heapMB: number;
  gcPauseMs: number;
  goroutines: number;
  /** Prozent einer CPU, -1 = unbekannt. */
  cpu: number;
  saveMs: number;
}

export interface RoomPoint {
  t: number;
  maxMs: number;
  p99Ms: number;
  /** Ticks länger als ein Takt im Intervall. */
  over: number;
}

export interface DevicePoint {
  t: number;
  /** -1 = kein Pong innerhalb 1 s. */
  rttMs: number;
  queue: number;
  dropped: number;
}

export interface DeviceSeries {
  room: string;
  points: DevicePoint[];
}

export type EventKind = 'crash' | 'slow' | 'drop' | 'client';

export interface DiagEvent {
  t: number;
  room: string;
  kind: EventKind;
  text: string;
}

export interface MetricsResponse {
  startedAt: number;
  now: number;
  server: ServerPoint[];
  rooms: Record<string, RoomPoint[]>;
  devices: Record<string, DeviceSeries>;
  events: DiagEvent[];
}

/** Puffer der Seite: alle Punkte im Fenster plus die Zeiten erkannter Neustarts. */
export interface MonitorState extends MetricsResponse {
  restarts: number[];
}

export const MINUTE_MS = 60_000;
export const HOUR_MS = 60 * MINUTE_MS;
/** Ein Takt (30 Hz) = Budget eines Ticks. */
export const TICK_BUDGET_MS = 1000 / 30;

export function emptyState(): MonitorState {
  return { startedAt: 0, now: 0, server: [], rooms: {}, devices: {}, events: [], restarts: [] };
}

const recent = <T extends { t: number }>(points: T[], from: number): T[] => points.filter((p) => p.t >= from);

/** Reihen je Schlüssel aneinanderhängen, nur Punkte ab `from`; leere Reihen fallen weg. */
function mergeKeyed<T extends { t: number }>(a: Record<string, T[]>, b: Record<string, T[]>, from: number): Record<string, T[]> {
  const out: Record<string, T[]> = {};
  for (const key of new Set([...Object.keys(a), ...Object.keys(b)])) {
    const pts = recent([...(a[key] ?? []), ...(b[key] ?? [])], from);
    if (pts.length > 0) out[key] = pts;
  }
  return out;
}

/**
 * Hängt eine Antwort an den Puffer. Ein neuer `startedAt` verwirft alle alten Punkte und merkt den Neustart;
 * danach bleibt nur, was jünger als `keepMs` vor `now` der Antwort ist. Leere Reihen verschwinden.
 */
export function applyDelta(state: MonitorState, delta: MetricsResponse, keepMs = HOUR_MS): MonitorState {
  const restarted = state.startedAt !== 0 && delta.startedAt !== state.startedAt;
  const base = restarted ? { ...emptyState(), restarts: [...state.restarts, delta.startedAt] } : state;
  const from = delta.now - keepMs;
  const points = (d: Record<string, DeviceSeries>) => Object.fromEntries(Object.entries(d).map(([id, s]) => [id, s.points]));
  const devicePoints = mergeKeyed(points(base.devices), points(delta.devices), from);
  const devices = Object.fromEntries(
    Object.entries(devicePoints).map(([id, pts]) => [id, { room: (delta.devices[id] ?? base.devices[id]).room, points: pts }]),
  );
  return {
    startedAt: delta.startedAt,
    now: delta.now,
    server: recent([...base.server, ...delta.server], from),
    rooms: mergeKeyed(base.rooms, delta.rooms, from),
    devices,
    events: recent([...base.events, ...delta.events], from),
    restarts: base.restarts.filter((t) => t >= from),
  };
}

/** Perzentil nach Nearest-Rank über aufsteigend sortierte Werte; NaN bei leerer Liste. */
export function percentile(sorted: number[], p: number): number {
  if (sorted.length === 0) return NaN;
  const rank = Math.ceil((p / 100) * sorted.length);
  return sorted[Math.min(sorted.length, Math.max(1, rank)) - 1];
}

export interface Stats {
  n: number;
  p50: number;
  p95: number;
  p99: number;
  max: number;
}

export function stats(values: number[]): Stats | null {
  if (values.length === 0) return null;
  const s = [...values].sort((a, b) => a - b);
  return { n: s.length, p50: percentile(s, 50), p95: percentile(s, 95), p99: percentile(s, 99), max: s[s.length - 1] };
}

/** Grenze für Ausreißer (Tukey-Zaun Q3 + 1,5 · IQR); Werte darüber sind Ausreißer. Infinity ohne Werte. */
export function outlierLimit(values: number[]): number {
  if (values.length === 0) return Infinity;
  const s = [...values].sort((a, b) => a - b);
  const q1 = percentile(s, 25);
  const q3 = percentile(s, 75);
  return q3 + 1.5 * (q3 - q1);
}

/** Nächster Eintrag nach `current` (zyklisch); unbekanntes `current` → erster. Für Filter-Knöpfe ohne Auswahlliste. */
export const nextOf = <T>(list: T[], current: T): T => list[(list.indexOf(current) + 1) % list.length];

export const inWindow =<T extends { t: number }>(points: T[], now: number, windowMs: number): T[] =>
  recent(points, now - windowMs);

/** Messwerte im Fenster: Tick = `maxMs` je Sekunde, RTT ohne „kein Pong“; optional nur ein Raum. */
export function windowValues(state: MonitorState, windowMs: number, room = '') {
  const rooms = Object.entries(state.rooms).filter(([code]) => !room || code === room);
  const devices = Object.values(state.devices).filter((d) => !room || d.room === room);
  const ticks = rooms.flatMap(([, pts]) => inWindow(pts, state.now, windowMs));
  const rtts = devices.flatMap((d) => inWindow(d.points, state.now, windowMs)).filter((p) => p.rttMs >= 0);
  return {
    tick: ticks.map((p) => p.maxMs),
    rtt: rtts.map((p) => p.rttMs),
    over: ticks.reduce((sum, p) => sum + p.over, 0),
  };
}

export type Light = 'green' | 'yellow' | 'red' | 'grey';

export interface RoomSummary {
  code: string;
  light: Light;
  tickP99: number | null;
  /** Schlechteste letzte RTT der Geräte im Raum, -1 = kein Pong. */
  worstRtt: number | null;
  errors: number;
}

/** Schwellen der Ampel (angenommen, Glossar „Ampel“). */
export const LIGHT = { tickYellow: TICK_BUDGET_MS / 2, tickRed: TICK_BUDGET_MS, rttYellow: 100, rttRed: 250 };
export const SUMMARY_WINDOW_MS = 5 * MINUTE_MS;

function light(tickP99: number | null, worstRtt: number | null, events: DiagEvent[]): Light {
  if (tickP99 === null) return 'grey';
  const rtt = worstRtt ?? 0;
  if (events.some((e) => e.kind === 'crash') || tickP99 > LIGHT.tickRed || rtt < 0 || rtt > LIGHT.rttRed) return 'red';
  if (events.length > 0 || tickP99 > LIGHT.tickYellow || rtt > LIGHT.rttYellow) return 'yellow';
  return 'green';
}

/** Ampel je Raum über die letzten 5 min, nach Raum-Code sortiert. */
export function roomSummaries(state: MonitorState): RoomSummary[] {
  const codes = new Set([...Object.keys(state.rooms), ...Object.values(state.devices).map((d) => d.room)]);
  codes.delete('');
  return [...codes].sort().map((code) => {
    const ticks = inWindow(state.rooms[code] ?? [], state.now, SUMMARY_WINDOW_MS).map((p) => p.p99Ms);
    const tickP99 = stats(ticks)?.p99 ?? null;
    const last = Object.values(state.devices)
      .filter((d) => d.room === code)
      .map((d) => d.points[d.points.length - 1].rttMs);
    const worstRtt = last.length === 0 ? null : last.includes(-1) ? -1 : Math.max(...last);
    const events = inWindow(state.events, state.now, SUMMARY_WINDOW_MS).filter((e) => e.room === code);
    return { code, light: light(tickP99, worstRtt, events), tickP99, worstRtt, errors: events.length };
  });
}
