/**
 * Latenz Eingabe → Zustand (B-181): Zeit von einer gesendeten `input` (`seq`) bis zum ersten Zustand mit `ack` ≥ `seq`,
 * Mittel und p95 über ein gleitendes Fenster. Nur Anzeige (Debug-Overlay).
 */

/** Fenster der Messung (B-181) */
export const LATENCY_WINDOW_MS = 60_000;

export interface LatencyStats {
  mean: number;
  p95: number;
}

export class LatencyMeter {
  private pending: { seq: number; at: number }[] = [];
  private samples: { at: number; ms: number }[] = [];

  sent(seq: number, at: number): void {
    this.pending.push({ seq, at });
  }

  acked(ack: number, now: number): void {
    while (this.pending.length > 0 && this.pending[0]!.seq <= ack) this.samples.push({ at: now, ms: now - this.pending.shift()!.at });
    while (this.pending.length > 0 && this.pending[0]!.at < now - LATENCY_WINDOW_MS) this.pending.shift(); // nie bestätigt
  }

  /** Mittel und p95 (Rang-Methode) der Messungen im Fenster; null ohne Messung. */
  stats(now: number): LatencyStats | null {
    this.samples = this.samples.filter((s) => s.at >= now - LATENCY_WINDOW_MS);
    const ms = this.samples.map((s) => s.ms).sort((a, b) => a - b);
    if (ms.length === 0) return null;
    return { mean: ms.reduce((a, b) => a + b, 0) / ms.length, p95: ms[Math.ceil(ms.length * 0.95) - 1]! };
  }

  /** Neue Verbindung: `seq` beginnt neu, offene Eingaben zählen nicht mehr. */
  reset(): void {
    this.pending = [];
  }
}
