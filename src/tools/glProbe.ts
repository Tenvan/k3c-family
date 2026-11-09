/**
 * WebGL-Diagnose (Testseite `texturetest.html`, RX1): Die Xbox meldete „Kein WebGL“, obwohl das Spiel dort zunächst mit WebGL startet. Die Seite
 * probiert deshalb nacheinander mehrere Wege, einen Kontext zu bekommen, und hält je Versuch fest, ob er klappt und warum nicht. Hier steht
 * die reine Logik (Versuche, Ergebnisse, Auswertung); `glDiagnose.ts` führt sie im Browser aus.
 */

/** Ein Weg, einen Kontext zu bekommen */
export interface Attempt {
  id: string;
  /** Name für `getContext` */
  kind: 'webgl2' | 'webgl' | 'experimental-webgl';
  attrs: WebGLContextAttributes;
}

/** Die Einstellungen, mit denen Phaser seinen Kontext anfordert (Spiel und Stress-Seite) */
const PHASER_ATTRS: WebGLContextAttributes = {
  alpha: false, depth: true, antialias: true, premultipliedAlpha: true, stencil: false, failIfMajorPerformanceCaveat: false,
  powerPreference: 'high-performance', preserveDrawingBuffer: false,
};

/** Reihenfolge: erst wie Phaser (so bekommt das Spiel seinen Kontext), dann abgespeckt, dann die Rückfälle */
export const ATTEMPTS: readonly Attempt[] = [
  { id: 'webgl-phaser', kind: 'webgl', attrs: PHASER_ATTRS },
  { id: 'webgl2', kind: 'webgl2', attrs: {} },
  { id: 'webgl', kind: 'webgl', attrs: {} },
  { id: 'webgl-noaa', kind: 'webgl', attrs: { antialias: false } },
  { id: 'webgl-low', kind: 'webgl', attrs: { antialias: false, powerPreference: 'low-power' } },
  { id: 'webgl-min', kind: 'webgl', attrs: { alpha: false, depth: false, stencil: false, antialias: false } },
  { id: 'experimental-webgl', kind: 'experimental-webgl', attrs: {} },
];

/** Ergebnis eines Versuchs */
export interface AttemptResult {
  id: string;
  ok: boolean;
  /** Grund aus `webglcontextcreationerror`, `'null'` = `getContext` lieferte `null` ohne Grund, `'exception: …'` bei einer Ausnahme; sonst `null` */
  error: string | null;
  version: string | null;
  renderer: string | null;
  vendor: string | null;
  maxTexture: number | null;
  textureUnits: number | null;
  extensions: number | null;
  /** `EXT_disjoint_timer_query(_webgl2)` vorhanden: Auslöser des bekannten Xbox-Fehlers (WebView2Feedback 4370) */
  timerQuery: boolean | null;
  ms: number;
}

/** Ein gescheiterter Versuch ohne Werte */
export function failed(id: string, error: string, ms: number): AttemptResult {
  return { id, ok: false, error, version: null, renderer: null, vendor: null, maxTexture: null, textureUnits: null, extensions: null, timerQuery: null, ms };
}

/** Auswertung aller Versuche */
export interface Summary {
  /** Zahl der gelungenen Versuche */
  ok: number;
  /** Erster gelungener Versuch (der, mit dem die Messung läuft), sonst `null` */
  first: string | null;
  /** `none` = nichts gelungen, `partial` = nur einige, `all` = alle */
  kind: 'none' | 'partial' | 'all';
  /** Gründe der gescheiterten Versuche ohne Wiederholung */
  reasons: string[];
}

export function summarize(results: readonly AttemptResult[]): Summary {
  const ok = results.filter((r) => r.ok);
  const reasons = [...new Set(results.filter((r) => !r.ok).map((r) => r.error ?? 'null'))];
  const kind: Summary['kind'] = ok.length === 0 ? 'none' : ok.length === results.length ? 'all' : 'partial';
  return { ok: ok.length, first: ok[0]?.id ?? null, kind, reasons };
}

/** Eine Zeile je Versuch für die Ausgabe */
export function attemptLine(r: AttemptResult): string {
  if (!r.ok) return `✗ ${r.id}: ${r.error === 'null' || r.error === null ? 'getContext lieferte null (kein Grund gemeldet)' : r.error}`;
  const parts = [r.version, r.renderer, r.maxTexture === null ? null : `maxTex ${r.maxTexture}`, r.timerQuery ? 'Timer-Query' : null];
  return `✓ ${r.id}: ${parts.filter(Boolean).join(' · ')}`;
}

/** Die Umgebung, in der die Seite läuft (Teil des Berichts) */
export interface Env {
  userAgent: string;
  viewport: string;
  devicePixelRatio: number;
  /** Seite läuft als Panel (iframe) im Spiel */
  inPanel: boolean;
  hidden: boolean;
  hasWebGL1: boolean;
  hasWebGL2: boolean;
  hasOffscreenCanvas: boolean;
  hasWebGPU: boolean;
  deviceMemoryGiB: number | null;
  cores: number | null;
  jsHeapMiB: number | null;
}
