/**
 * Vollbild halten (B-768, BD11.6): Das Spiel ist die einzige Seite und navigiert nie, deshalb geht Vollbild nur verloren, wenn der Browser
 * es beendet (Esc, Edge auf der Xbox, App-Wechsel). Der Halter fordert es bei der ersten Eingabe an und nach einem Verlust bei der
 * nächsten wieder. Browser erlauben Vollbild nur nach einer Nutzeraktion; lehnt er mehrmals ab, hört der Halter auf und zeigt einen
 * Hinweis, wie man es selbst umschaltet (Taste F, RS am Controller). Die Entscheidungen sind rein (`FullscreenKeeper`), `install…` verdrahtet.
 */
import { clientLog } from './clientLog';
import { isFullscreen, onFullscreenChange, requestLocal } from './fullscreen';
import { t } from './texts';

export type KeeperResult = 'ok' | 'denied' | 'giveup';

export class FullscreenKeeper {
  private failures = 0;
  private lastTry = -Infinity;
  private gaveUp = false;

  constructor(
    private readonly minGapMs = 1500,
    private readonly maxFailures = 3,
  ) {}

  /** Bei jeder Nutzereingabe: soll Vollbild jetzt angefordert werden? */
  wants(now: number, active: boolean): boolean {
    return !active && !this.gaveUp && now - this.lastTry >= this.minGapMs;
  }

  /** Vermerkt den Versuch (Zeit), bevor die Anfrage geht */
  tried(now: number): void {
    this.lastTry = now;
  }

  /** Ergebnis einer Anfrage: `ok` setzt die Fehler zurück, nach `maxFailures` Fehlern in Folge `giveup` */
  result(error: string | null): KeeperResult {
    if (!error) {
      this.failures = 0;
      return 'ok';
    }
    this.failures += 1;
    if (this.failures < this.maxFailures) return 'denied';
    this.gaveUp = true;
    return 'giveup';
  }

  /** Vollbild wurde beendet: der nächste Versuch ist wieder erlaubt, auch nach einem Aufgeben */
  lost(): void {
    this.failures = 0;
    this.gaveUp = false;
  }
}

const TOAST_MS = 7000;

function showHint(text: string): void {
  const el = document.createElement('div');
  el.textContent = text;
  el.style.cssText =
    'position:fixed;left:50%;bottom:4%;translate:-50% 0;z-index:2147483000;max-width:80vw;padding:0.6em 1.2em;border-radius:0.5em;' +
    'background:#f3ead4;color:#2a1d2e;font:600 clamp(14px,1.6vw,26px) "Segoe UI",system-ui,sans-serif;box-shadow:0 6px 18px rgba(26,15,20,.5)';
  document.body.append(el);
  setTimeout(() => el.remove(), TOAST_MS);
}

/** Verdrahtet den Halter mit Zeiger, Tastatur und Controller; einmal beim Start des Spiels aufrufen */
export function installFullscreenKeeper(): FullscreenKeeper {
  const keeper = new FullscreenKeeper();
  const onInput = (): void => {
    const now = performance.now();
    if (!keeper.wants(now, isFullscreen())) return;
    keeper.tried(now);
    void requestLocal().then((error) => {
      const result = keeper.result(error);
      if (result === 'ok') clientLog('info', '🚦 Vollbild an');
      else clientLog('warn', `🚫 Vollbild abgelehnt (${result}): ${error ?? ''}`);
      if (result === 'giveup') showHint(t('fs.denied'));
    });
  };
  addEventListener('pointerdown', onInput, true);
  addEventListener('keydown', onInput, true);
  onFullscreenChange((active) => {
    if (!active) keeper.lost();
  });
  // Controller: eine neu gedrückte Taste zählt als Eingabe (ob der Browser sie als Nutzeraktion gelten lässt, zeigt das Log)
  let held = false;
  const poll = (): void => {
    const pads = typeof navigator.getGamepads === 'function' ? Array.from(navigator.getGamepads()) : [];
    const down = pads.some((p) => p?.buttons.some((b) => b.pressed));
    if (down && !held) onInput();
    held = down;
    requestAnimationFrame(poll);
  };
  requestAnimationFrame(poll);
  return keeper;
}
