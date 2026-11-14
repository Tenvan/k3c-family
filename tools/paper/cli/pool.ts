// Worker-Pool für resvg (B-751): resvg rastert je Aufruf auf einem Kern und braucht 98 % der Zeit von gen:paper. Der Pool
// verteilt die Ausgaben (Animation/Variante) auf Worker-Threads; das SVG baut der Haupt-Thread erst, wenn ein Worker frei
// ist, damit große Assets (Monarch: 195 Ausgaben) nicht alle SVGs zugleich im Speicher halten.
import { availableParallelism } from 'node:os';
import { Worker } from 'node:worker_threads';
import type { Raster } from '../render/raster';
import { Aufruf } from '../render/fehler';

/** Anzahl Worker: `K3C_PAPER_JOBS`, sonst verfügbare Kerne minus 1, höchstens 8, mindestens 1 (Default PT1, B-751). */
export function jobs(env: Record<string, string | undefined> = process.env, kerne = availableParallelism()): number {
  const wert = env.K3C_PAPER_JOBS?.trim();
  if (!wert) return Math.max(1, Math.min(8, kerne - 1));
  const n = Number(wert);
  if (!Number.isInteger(n) || n < 1) throw new Aufruf(`K3C_PAPER_JOBS muss eine ganze Zahl ab 1 sein, nicht „${wert}“`);
  return n;
}

type Auftrag = { name: string; svg: () => string; ok: (r: Raster) => void; fehler: (e: unknown) => void };
type Antwort = { png: Uint8Array; rgba: Uint8Array; breite: number; hoehe: number } | { fehler: string };

const neuerWorker = () => new Worker(new URL('./pool.worker.ts', import.meta.url));

export class RasterPool {
  private alle: Worker[] = [];
  private frei: Worker[] = [];
  private warte: Auftrag[] = [];
  private laufend = new Map<Worker, Auftrag>();
  private zu = false;

  constructor(readonly anzahl: number, neu: () => Worker = neuerWorker) {
    for (let i = 0; i < anzahl; i++) {
      const w = neu();
      w.on('message', (m: Antwort) => this.fertig(w, m));
      w.on('error', (e: Error) => this.absturz(w, e));
      w.on('exit', (code) => this.absturz(w, new Error(`Exit ${code}`)));
      this.alle.push(w);
      this.frei.push(w);
    }
  }

  /** Rastert das SVG, das `svg()` erst beim Start liefert; `name` (gruppe/id:ausgabe) steht in jeder Fehlermeldung. */
  raster(name: string, svg: () => string): Promise<Raster> {
    return new Promise((ok, fehler) => {
      this.warte.push({ name, svg, ok, fehler });
      this.weiter();
    });
  }

  private weiter() {
    while (!this.zu && this.frei.length && this.warte.length) {
      const a = this.warte.shift()!, w = this.frei.pop()!;
      let svg: string;
      try {
        svg = a.svg();
      } catch (e) {
        this.frei.push(w);
        a.fehler(e);
        continue;
      }
      this.laufend.set(w, a);
      w.postMessage(svg);
    }
  }

  private fertig(w: Worker, m: Antwort) {
    if (this.zu || !this.alle.includes(w)) return; // geschlossen oder Worker schon als abgestürzt entfernt
    const a = this.laufend.get(w);
    this.laufend.delete(w);
    this.frei.push(w);
    if (a && 'fehler' in m) a.fehler(new Error(`resvg bei ${a.name}: ${m.fehler}`));
    else if (a && !('fehler' in m))
      a.ok({ png: Buffer.from(m.png.buffer, m.png.byteOffset, m.png.byteLength), rgba: m.rgba, breite: m.breite, hoehe: m.hoehe });
    this.weiter();
  }

  /** Ein Worker ist weg: sein Auftrag scheitert mit Namen; ohne Worker scheitern auch die wartenden. */
  private absturz(w: Worker, e: Error) {
    if (this.zu || !this.alle.includes(w)) return;
    const a = this.laufend.get(w);
    this.laufend.delete(w);
    this.alle = this.alle.filter((x) => x !== w);
    this.frei = this.frei.filter((x) => x !== w);
    const fehler = new Error(`💥 Worker abgestürzt${a ? ` bei ${a.name}` : ''}: ${e.message}`);
    a?.fehler(fehler);
    if (!this.alle.length) for (const x of this.warte.splice(0)) x.fehler(fehler);
  }

  async schliessen(): Promise<void> {
    this.zu = true;
    await Promise.all(this.alle.map((w) => w.terminate()));
  }
}
