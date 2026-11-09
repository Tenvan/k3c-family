/**
 * Textur-Grenze (Testseite `texturetest.html`, B-764, RX1): Die Xbox verliert den WebGL-Kontext ab einer bestimmten Menge Texturspeicher.
 * Die Seite legt ohne Spiel stufenweise Texturen an, misst FPS und Frame-Zeit je Stufe und merkt sich, bei wie viel MiB der Kontext
 * verloren geht. Hier steht die reine Logik: Stufen, Frame-Statistik, Auswertung. Die Seite (`textureTest.ts`) führt WebGL aus.
 */

/** Kantenlängen der Testtexturen (RGBA, 4 Byte je Pixel) */
export const TEXTURE_SIZES = [1024, 2048, 4096] as const;
export type TextureSize = (typeof TEXTURE_SIZES)[number];

/** Speicher einer Textur in MiB (ohne Mip-Ebenen) */
export const textureMiB = (px: number): number => (px * px * 4) / (1024 * 1024);

export interface ProbeConfig {
  size: TextureSize;
  /** Texturen je Stufe */
  perStage: number;
  /** Dauer einer Stufe (ms): so lange wird mit dem erreichten Stand gezeichnet und gemessen */
  stageMs: number;
  /** Obergrenze in MiB: bis hierher ohne Verlust gilt als „keine Grenze gefunden“ */
  maxMiB: number;
}

/** Standard: etwa 64 MiB je Stufe, alle 2 s, bis 1 GiB */
export function defaultConfig(size: TextureSize = 2048): ProbeConfig {
  return { size, perStage: Math.max(1, Math.round(64 / textureMiB(size))), stageMs: 2000, maxMiB: 1024 };
}

/** Speicher nach `stage` Stufen (1 = erste) */
export const stageMiB = (cfg: ProbeConfig, stage: number): number => stage * cfg.perStage * textureMiB(cfg.size);

/** Zahl der Stufen bis zur Obergrenze */
export const stageCount = (cfg: ProbeConfig): number => Math.ceil(cfg.maxMiB / (cfg.perStage * textureMiB(cfg.size)));

/** Ergebnis einer Stufe */
export interface Stage {
  stage: number;
  mib: number;
  textures: number;
  fps: number;
  frameP95: number;
  /** WebGL-Fehler dieser Stufe (z. B. `OUT_OF_MEMORY`), sonst `null` */
  error: string | null;
}

/** Verlust des Kontexts */
export interface Loss {
  mib: number;
  /** Zeit seit dem Start des Laufs (ms) */
  atMs: number;
  /** Zeit bis `webglcontextrestored` (ms), `null` = nicht wiederhergestellt */
  restoredAfterMs: number | null;
}

/** Frame-Zeiten sammeln: FPS und 95-%-Perzentil einer Stufe */
export class FrameStats {
  private readonly times: number[] = [];

  add(frameMs: number): void {
    if (frameMs > 0) this.times.push(frameMs);
  }

  get count(): number {
    return this.times.length;
  }

  /** Mittlere Bilder je Sekunde */
  fps(): number {
    const total = this.times.reduce((a, b) => a + b, 0);
    return total > 0 ? Math.round((this.times.length / total) * 1000 * 10) / 10 : 0;
  }

  /** 95-%-Perzentil der Frame-Zeit (ms, auf 0,1 gerundet) */
  p95(): number {
    if (this.times.length === 0) return 0;
    const sorted = [...this.times].sort((a, b) => a - b);
    const at = Math.min(sorted.length - 1, Math.ceil(sorted.length * 0.95) - 1);
    return Math.round(sorted[at]! * 10) / 10;
  }

  reset(): void {
    this.times.length = 0;
  }
}

/** Ab dieser FPS gilt eine Stufe als flüssig genug (wie die Stress-Szene, OK_FPS) */
export const SMOOTH_FPS = 30;

/** Anteil der sicheren Grenze, den ein Spiel als Budget nutzen soll (Reserve für Puffer, Schrift, Treiber) */
export const BUDGET_SHARE = 0.7;

export interface Verdict {
  /** Letzte Stufe vor Verlust oder Fehler (MiB), `0` wenn schon die erste verlor */
  safeMiB: number;
  /** MiB beim Verlust, `null` ohne Verlust */
  lossMiB: number | null;
  /** Empfohlenes Budget für das Spiel (MiB, auf 10 abgerundet) */
  budgetMiB: number;
  /** Höchste Stufe mit mindestens `SMOOTH_FPS` (MiB), `0` wenn keine */
  smoothMiB: number;
  /** `limit` = Verlust gefunden, `none` = keine Grenze bis zur Obergrenze, `error` = Fehler ohne Verlust (z. B. Speicherfehler) */
  kind: 'limit' | 'none' | 'error';
}

/**
 * Wertet einen Lauf aus: Die sichere Grenze ist die letzte Stufe ohne Fehler vor dem Verlust (oder die letzte überhaupt, wenn nichts
 * verloren ging); das Budget ist `BUDGET_SHARE` davon, abgerundet auf 10 MiB.
 */
export function verdict(stages: readonly Stage[], loss: Loss | null): Verdict {
  const firstError = stages.findIndex((s) => s.error !== null);
  const clean = firstError < 0 ? stages : stages.slice(0, firstError);
  const safeMiB = clean.at(-1)?.mib ?? 0;
  const smoothMiB = Math.max(0, ...clean.filter((s) => s.fps >= SMOOTH_FPS).map((s) => s.mib));
  const kind: Verdict['kind'] = loss ? 'limit' : firstError >= 0 ? 'error' : 'none';
  return { safeMiB, lossMiB: loss?.mib ?? null, budgetMiB: Math.floor((safeMiB * BUDGET_SHARE) / 10) * 10, smoothMiB, kind };
}

/** Eine Stufe als Zeile für die Ausgabe: `192 MiB · 12 Texturen · 58,3 FPS · p95 21,4 ms` (mit Fehler dahinter) */
export function stageLine(s: Stage): string {
  const base = `${Math.round(s.mib)} MiB · ${s.textures} · ${s.fps} FPS · p95 ${s.frameP95} ms`;
  return s.error ? `${base} · ${s.error}` : base;
}

/** Name eines WebGL-Fehlercodes (nur die, die bei Speichermangel vorkommen) */
export function glErrorName(code: number): string | null {
  const names: Record<number, string> = { 0: '', 0x0500: 'INVALID_ENUM', 0x0501: 'INVALID_VALUE', 0x0502: 'INVALID_OPERATION', 0x0505: 'OUT_OF_MEMORY', 0x0507: 'CONTEXT_LOST_WEBGL' };
  if (code === 0) return null;
  return names[code] ?? `0x${code.toString(16)}`;
}
