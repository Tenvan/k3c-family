/**
 * Leistungswerte des Clients (B-351, PM1): Die Sammel-Funktion liest FPS, Latenz und Puffer – dieselben Werte wie das
 * Debug-Overlay – und die Diagnose-Zeile schreibt sie alle `DIAG_EVERY_MS` ins Client-Log. `sim_test` liest sie aus
 * `k3c-client` (`tools/k3c-workbench/internal/mcpsrv/simtest_clientlog.go`: `ctx` mit `fps`, `latencyMs`, `bufferMs`).
 * Phaser-frei; der Performance-Modus (`clientPerfReport.ts`) nutzt dieselbe Sammel-Funktion.
 */
import { clientLog } from '../core/clientLog';
import type { LatencyStats } from './clientLatency';

/** Takt der Diagnose-Zeile (B-351) */
export const DIAG_EVERY_MS = 5000;

/** Gemessene Werte eines Augenblicks; `latencyMs` ist null ohne Messung (wie `Latenz –` im Overlay). */
export interface PerfValues {
  fps: number;
  latencyMs: number | null;
  bufferMs: number;
}

/** Rohwerte wie im Debug-Overlay: `game.loop.actualFps`, `client.latency`, `GameScene.delayMs`. */
export interface PerfSource {
  fps: number;
  latency: LatencyStats | null;
  delayMs: number;
}

/** Sammel-Funktion: ganze Zahlen wie im Overlay (`toFixed(0)`), Latenz ist das Mittel. */
export function collectPerf(s: PerfSource): PerfValues {
  return {
    fps: Math.round(s.fps),
    latencyMs: s.latency ? Math.round(s.latency.mean) : null,
    bufferMs: Math.round(s.delayMs),
  };
}

/**
 * Startet die Diagnose-Zeile im Takt `everyMs`. `read` liefert null, solange nichts zu messen ist (kein Raum, Tab im
 * Hintergrund); dann entfällt die Zeile. Die laufende Nummer macht jeden Text einzigartig: Die Drosselung von `clientLog`
 * zählt gleiche Texte binnen `REPEAT_MS` nur (Schlüssel ohne `ctx`) und würde sonst Zeilen schlucken (B-351 › Regeln).
 * Die Rückgabe beendet den Takt.
 */
export function startDiag(read: () => PerfValues | null, everyMs = DIAG_EVERY_MS): () => void {
  let n = 0;
  const id = setInterval(() => {
    const v = read();
    if (!v) return;
    n++;
    clientLog('info', `💓 Diagnose #${n}: ${v.fps} FPS, Latenz ${v.latencyMs ?? '–'} ms, Puffer ${v.bufferMs} ms`, { ...v });
  }, everyMs);
  return () => clearInterval(id);
}
