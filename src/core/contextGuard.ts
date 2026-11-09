/**
 * Schutz vor dem Verlust des WebGL-Kontexts (B-763, RX1.2): Auf der Xbox-Edge meldete der Client nach wenigen Sekunden mehrfach
 * „WebGL Context lost“, Phaser schaltete den Renderer ab, die Seite blieb weiß. Phaser baut den Kontext selbst wieder auf, wenn der
 * Browser `webglcontextrestored` meldet; bleibt das aus oder verliert die Seite ihn erneut, startet die Seite genau einmal je Sitzung
 * mit dem Canvas-Renderer neu (Spielstand und Raum bleiben auf dem Server, der Client tritt dem Raum wieder bei). Danach gibt es
 * keinen weiteren Versuch, nur einen sichtbaren Hinweis. Phaser-frei; Uhr, Speicher, Neustart und Hinweis kommen vom Aufrufer.
 */

/** So lange wartet die Seite auf `webglcontextrestored`, bevor sie neu startet */
export const RESTORE_WAIT_MS = 4000;
/** Sitzungsmarke mit Zeitstempel (ms): der automatische Neustart ist schon passiert; sie überlebt das Neuladen im selben Tab und gilt `TRIED_TTL_MS` */
export const TRIED_KEY = 'k3c-ctx-restart';
/** Danach ist ein neuer Versuch wieder erlaubt (z. B. nach Home und erneutem Öffnen im WebGL-Modus) */
export const TRIED_TTL_MS = 10 * 60_000;

export interface GuardEnv {
  /** Wert aus dem Sitzungsspeicher, `null` ohne Speicher oder Eintrag */
  read(key: string): string | null;
  write(key: string, value: string): void;
  /** Uhrzeit in ms (Date.now) */
  now(): number;
  /** Ruft `fn` nach `ms` auf; die Rückgabe bricht ab */
  after(fn: () => void, ms: number): () => void;
  /** Neustart der Seite im Canvas-Modus (nur über die Shell, `openPage`) */
  restartWithCanvas(): void;
  /** Sichtbarer Hinweis im Spiel: `restart` = Neustart folgt, `canvas` = Notbetrieb, kein weiterer Versuch */
  notice(kind: 'restart' | 'canvas'): void;
  log(msg: string, ctx: Record<string, unknown>): void;
}

export type GuardState = 'ok' | 'waiting' | 'restarting' | 'gaveUp';

export class ContextGuard {
  private state: GuardState = 'ok';
  private cancel: (() => void) | null = null;
  private lossesThisPage = 0;

  constructor(private readonly env: GuardEnv) {}

  get current(): GuardState {
    return this.state;
  }

  /** `webglcontextlost` */
  lost(): void {
    if (this.state !== 'ok' || this.cancel) return; // läuft schon (Warten, Neustart, Notbetrieb): keine Schleife, weitere Meldungen zählen nicht
    if (this.tried()) return this.giveUp('Verlust nach dem Neustart');
    this.lossesThisPage++;
    if (this.lossesThisPage > 1) return this.restart('Verlust wiederholt sich'); // schon einmal wiederhergestellt: der Kontext ist instabil
    this.state = 'waiting';
    this.cancel = this.env.after(() => {
      this.cancel = null;
      this.restart(`keine Wiederherstellung nach ${RESTORE_WAIT_MS} ms`);
    }, RESTORE_WAIT_MS);
    this.env.log('🩹 Warte auf Wiederherstellung des WebGL-Kontexts', { waitMs: RESTORE_WAIT_MS });
  }

  /** `webglcontextrestored` (Phaser baut Texturen und Puffer neu auf) */
  restored(): void {
    if (this.state !== 'waiting') return;
    this.cancel?.();
    this.cancel = null;
    this.state = 'ok';
    this.env.log('✅ WebGL-Kontext wiederhergestellt', { losses: this.lossesThisPage });
  }

  /** Gab es in den letzten `TRIED_TTL_MS` schon einen automatischen Neustart? */
  private tried(): boolean {
    const at = Number(this.env.read(TRIED_KEY));
    return Number.isFinite(at) && at > 0 && this.env.now() - at < TRIED_TTL_MS;
  }

  private restart(why: string): void {
    this.cancel?.();
    this.cancel = null;
    this.env.write(TRIED_KEY, String(this.env.now()));
    this.state = 'restarting';
    this.env.notice('restart');
    this.env.log('🔁 Neustart mit Canvas-Renderer', { why });
    this.env.restartWithCanvas();
  }

  private giveUp(why: string): void {
    this.state = 'gaveUp';
    this.env.notice('canvas');
    this.env.log('🚫 Kein weiterer Neustart (Notbetrieb)', { why });
  }
}

/** Adresse für den Neustart: dieselbe Seite mit `renderer=canvas`; ein offener Raum wird per `room` wieder betreten (kein neues Spiel mit `fresh`/`save`) */
export function canvasRestartHref(page: string, search: string, room: string | null): string {
  const p = new URLSearchParams(search);
  p.set('renderer', 'canvas');
  if (room) {
    p.delete('fresh');
    p.delete('save');
    p.set('room', room);
    p.set('autostart', '1');
  }
  return `${page}?${p.toString()}`;
}
