/**
 * Log des Browsers: schickt Fehler, Warnungen und wichtige Ereignisse an `POST /api/clientlog`; der Server schreibt sie nach
 * `logs/k3c-client.jsonl` und auf seine Konsole. Ohne `installClientLog()` tun alle Funktionen nichts (Tests, Seiten ohne Server).
 */

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface Entry {
  level: LogLevel;
  msg: string;
  url: string;
  stack?: string;
  ctx?: string;
  device: string;
}

const ENDPOINT = '/api/clientlog';
const FLUSH_MS = 2000;
const MAX_QUEUE = 40;
const MAX_TEXT = 1500;

let installed = false;
const queue: Entry[] = [];
let timer: ReturnType<typeof setTimeout> | null = null;
/** Gleiche Meldungen innerhalb von `REPEAT_MS` werden nur gezählt (eine Fehlerschleife im Frame-Takt): Schlüssel → Anzahl weiterer Vorkommen. */
const REPEAT_MS = 5000;
const repeats = new Map<string, { entry: Entry; extra: number; at: number }>();

function deviceId(): string {
  try {
    return localStorage.getItem('k3c-client') ?? '';
  } catch {
    return '';
  }
}

function text(value: unknown): string {
  if (value instanceof Error) return value.message;
  if (typeof value === 'string') return value;
  try {
    return JSON.stringify(value) ?? String(value);
  } catch {
    return String(value);
  }
}

/** Meldung vormerken; Fehler gehen sofort raus, alles andere gebündelt nach `FLUSH_MS`. */
export function clientLog(level: LogLevel, msg: string, ctx?: Record<string, unknown>, stack?: string): void {
  if (!installed) return;
  const entry: Entry = { level, msg: msg.slice(0, MAX_TEXT), url: location.href, device: deviceId() };
  if (stack) entry.stack = stack.slice(0, MAX_TEXT * 2);
  if (ctx) entry.ctx = text(ctx).slice(0, MAX_TEXT);
  const key = `${level}|${entry.msg}|${entry.stack ?? ''}`;
  const now = Date.now();
  const seen = repeats.get(key);
  if (seen && now - seen.at < REPEAT_MS) {
    seen.extra++;
    schedule();
    return;
  }
  repeats.set(key, { entry, extra: 0, at: now });
  queue.push(entry);
  if (queue.length >= MAX_QUEUE || level === 'error') flush();
  else schedule();
}

function schedule(): void {
  timer ??= setTimeout(flush, FLUSH_MS);
}

function summarize(): void {
  const now = Date.now();
  for (const [key, seen] of repeats) {
    if (seen.extra > 0) queue.push({ ...seen.entry, msg: `${seen.entry.msg} (${seen.extra}× wiederholt)`, stack: undefined });
    seen.extra = 0;
    if (now - seen.at >= REPEAT_MS) repeats.delete(key);
  }
}

function flush(): void {
  if (timer !== null) clearTimeout(timer);
  timer = null;
  summarize();
  if (queue.length === 0) return;
  const entries = queue.splice(0, 50);
  const body = JSON.stringify({ entries });
  try {
    void fetch(ENDPOINT, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body, keepalive: true }).catch(() => undefined);
  } catch {
    // Server nicht erreichbar: das Log darf das Spiel nie stören
  }
}

/** Fängt `error`, `unhandledrejection` und `console.warn/error` ab und meldet den Seitenstart. Mehrfacher Aufruf ist harmlos. */
export function installClientLog(): void {
  if (installed || typeof window === 'undefined' || typeof fetch === 'undefined') return;
  installed = true;
  window.addEventListener('error', (e) => {
    const where = e.filename ? ` @ ${e.filename}:${e.lineno}:${e.colno}` : '';
    clientLog('error', `${e.message}${where}`, undefined, e.error instanceof Error ? e.error.stack : undefined);
  });
  window.addEventListener('unhandledrejection', (e) => {
    const reason = e.reason as unknown;
    clientLog('error', `Promise abgelehnt: ${text(reason)}`, undefined, reason instanceof Error ? reason.stack : undefined);
  });
  for (const level of ['warn', 'error'] as const) {
    const original = console[level].bind(console);
    console[level] = (...args: unknown[]) => {
      original(...args);
      clientLog(level, args.map(text).join(' '));
    };
  }
  window.addEventListener('pagehide', flush);
  document.addEventListener('visibilitychange', () => document.hidden && flush());
  clientLog('info', 'Seite geladen', {
    ua: navigator.userAgent,
    viewport: `${window.innerWidth}x${window.innerHeight}`,
    dpr: window.devicePixelRatio,
    touch: navigator.maxTouchPoints,
  });
}
